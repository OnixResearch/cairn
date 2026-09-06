#![cfg(target_os = "linux")]
use cairn_core::verified::check_evidence::*;
use cairn_core::{
    authoritative_command_hash, authoritative_selection_hash, canonical_json_hash,
    AgentRequiredCheck, AgentVerificationContract,
};
use serde_json::{json, Value};
use std::{collections::BTreeMap, error::Error, fs, path::PathBuf, process::Command};

fn binary(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    for directory in std::env::split_paths(&std::env::var_os("PATH").ok_or("missing PATH")?) {
        let path = directory.join(name);
        if path.is_file() {
            return Ok(fs::canonicalize(path)?);
        }
    }
    Err("required fixture executable unavailable".into())
}
/// Fixtures supply an expression file for the fixed `nix eval` grammar: the
/// pass fixture evaluates to a value (zero exit) and the fail fixture throws
/// during evaluation (nonzero exit). This keeps the controlled property —
/// disposition classification of a real Nix process — while working inside a
/// sandboxed Nix check derivation, where no store daemon is reachable and
/// real builds cannot run.
#[test]
fn real_nix_success_is_not_an_admitted_payload_or_test_coverage() -> Result<(), Box<dyn Error>> {
    nix_fixture("pass", "42", true)
}

#[test]
fn real_nix_build_failure_cannot_complete() -> Result<(), Box<dyn Error>> {
    nix_fixture("fail", "builtins.throw \"producer-control failed\"", false)
}

fn nix_fixture(name: &str, script: &str, expected: bool) -> Result<(), Box<dyn Error>> {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root = repo
        .join("target")
        .join(format!("producer-nix-{name}-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    fs::create_dir_all(root.join("evidence"))?;
    fs::write(root.join("check.nix"), format!("{script}\n"))?;
    fs::write(root.join("projection.json"), b"[\"check.nix\"]")?;
    fs::write(root.join("cancel"), b"0")?;
    let source = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&repo)
        .args(["check-evidence", "source", "--root"])
        .arg(&root)
        .args(["--projection", "projection.json"])
        .output()?;
    assert!(source.status.success());
    let source: Value = serde_json::from_slice(&source.stdout)?;
    let nix = binary("nix")?;
    let hash = blake3::hash(&fs::read(&nix)?).to_hex().to_string();
    let mut environment = BTreeMap::new();
    for key in ["PATH", "HOME", "NIX_REMOTE", "NIX_SSL_CERT_FILE"] {
        if let Ok(v) = std::env::var(key) {
            environment.insert(key.into(), v);
        }
    }
    environment.insert(
        "NIX_CONFIG".into(),
        "experimental-features = nix-command flakes".into(),
    );
    let profile = CheckProfile {
        schema: PROFILE_SCHEMA.into(),
        check_id: "build".into(),
        kind: CheckKind::NixCheck,
        executable: nix.to_str().ok_or("path")?.into(),
        executable_hash: hash.clone(),
        tool_cohort_hash: hash,
        target: "x86_64-linux".into(),
        features: vec![],
        environment,
        working_root: ".".into(),
        bounds: ExecutionBounds {
            id: "nix-local".into(),
            deadline_ms: 30000,
            teardown_ms: 2000,
            stdout_bytes: 16384,
            stderr_bytes: 16384,
        },
    };
    let command = profile_command(&profile);
    let checks = vec![AgentRequiredCheck {
        check_id: "build".into(),
        command_hash: authoritative_command_hash(&command),
        command,
        prerequisites: vec![],
    }];
    let policy: Value = serde_json::from_slice(&fs::read(
        repo.join("cairn-policy/generated/cairn-policy.json"),
    )?)?;
    let contract = AgentVerificationContract {
        schema: "cairn.agent-completion.v2".into(),
        repository_id: "nix-live".into(),
        registry_id: "checks".into(),
        registry_hash: "a".repeat(64),
        selection_hash: authoritative_selection_hash(&checks),
        policy_hash: canonical_json_hash(&policy),
        final_subject_hash: source["subject_hash"].as_str().ok_or("subject")?.into(),
        required_checks: checks,
    };
    let applicability = Applicability {
        subject_hash: contract.final_subject_hash.clone(),
        policy_hash: contract.policy_hash.clone(),
        dependency_hashes: vec![],
        source_projection_hash: source["source_projection_hash"]
            .as_str()
            .ok_or("projection")?
            .into(),
    };
    let plan = admit_plan(&contract, std::slice::from_ref(&profile), &applicability)?;
    fs::write(
        root.join("profiles.json"),
        serde_json::to_vec(&vec![profile])?,
    )?;
    fs::write(root.join("contract.json"), serde_json::to_vec(&contract)?)?;
    fs::write(
        root.join("applicability.json"),
        serde_json::to_vec(&applicability)?,
    )?;
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&repo)
        .args(["check-evidence", "run", "--execute", "--root"])
        .arg(&root)
        .args([
            "--profiles",
            "profiles.json",
            "--contract",
            "contract.json",
            "--applicability",
            "applicability.json",
            "--projection",
            "projection.json",
            "--plan-hash",
            plan.identity(),
            "--attempt",
            &"b".repeat(64),
            "--cancel-file",
            "cancel",
            "--out",
        ])
        .arg(root.join("evidence"))
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(result["valid"], false, "{result}");
    assert!(!root.join("evidence/manifest.json").exists());
    let completion = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&repo)
        .args(["agent-receipt", "complete", "--contract"])
        .arg(root.join("contract.json"))
        .arg("--results")
        .arg(root.join("evidence/results.json"))
        .args(["--subject-hash", &contract.final_subject_hash])
        .output()?;
    assert!(completion.status.success());
    let completion: Value = serde_json::from_slice(&completion.stdout)?;
    assert_eq!(completion["valid"], false);
    let observations: Value =
        serde_json::from_slice(&fs::read(root.join("evidence/observations.json"))?)?;
    assert_eq!(observations[0]["role"], "build");
    assert_eq!(
        observations[0]["outcome"],
        if expected {
            "exited_zero"
        } else {
            "exited_nonzero"
        }
    );
    assert_eq!(observations[0]["payload_passed"], false);
    assert_eq!(observations[0]["executed_tests"], json!(0));
    Ok(())
}
