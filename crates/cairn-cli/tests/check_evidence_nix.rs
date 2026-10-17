#![cfg(target_os = "linux")]
use cairn_core::verified::check_evidence::*;
use cairn_core::{
    authoritative_command_hash, authoritative_selection_hash, canonical_json_hash,
    AgentRequiredCheck, AgentVerificationContract,
};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const FIXTURE_CONTENT: &str = "native-build\n";
const FIXTURE_CONFIG: &str = "experimental-features = nix-command flakes\nsandbox = false\nbuild-users-group =\nbuilders =\n";

fn binary(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    for directory in std::env::split_paths(&std::env::var_os("PATH").ok_or("missing PATH")?) {
        let path = directory.join(name);
        if path.is_file() {
            return Ok(fs::canonicalize(path)?);
        }
    }
    Err("required fixture executable unavailable".into())
}

fn cli(repo: &Path, args: &[&str]) -> Result<Value, Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(repo)
        .args(args)
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn derivation(name: &str, fail: bool) -> String {
    // Nix's fixed-output builder requires an interoperability SHA-256.
    // Applicability, tool identities, attempts, and retained evidence use BLAKE3.
    let expected = if fail {
        "different-content\n"
    } else {
        FIXTURE_CONTENT
    };
    format!(
        "let content = {}; source = builtins.toFile \"producer-input\" content; in builtins.derivation {{ name = \"producer-{name}\"; system = builtins.currentSystem; builder = \"builtin:fetchurl\"; url = \"file://${{source}}\"; outputHashAlgo = \"sha256\"; outputHashMode = \"flat\"; outputHash = builtins.hashString \"sha256\" {}; }}\n",
        json!(FIXTURE_CONTENT), json!(expected)
    )
}

// r[verify cairn.check_evidence_producer.native_nix_build]
#[test]
fn real_nix_build_reaches_original_consumers_without_test_coverage() -> Result<(), Box<dyn Error>> {
    nix_fixture("pass", &derivation("pass", false), "exited_zero", true)
}

// r[verify cairn.check_evidence_producer.native_nix_build]
#[test]
fn real_nix_build_failure_cannot_complete() -> Result<(), Box<dyn Error>> {
    nix_fixture("fail", &derivation("fail", true), "exited_nonzero", false)
}

// r[verify cairn.check_evidence_producer.native_nix_build]
#[test]
fn real_nix_empty_build_set_cannot_complete_after_zero_exit() -> Result<(), Box<dyn Error>> {
    nix_fixture("empty", "[]", "exited_zero", false)
}

fn nix_fixture(
    name: &str,
    script: &str,
    expected_outcome: &str,
    expected_complete: bool,
) -> Result<(), Box<dyn Error>> {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let workspace = tempfile::Builder::new()
        .prefix(&format!("cairn-native-nix-{name}-"))
        .tempdir()?;
    let root = workspace.path();
    fs::create_dir_all(root.join("evidence"))?;
    fs::create_dir(root.join("nix-conf"))?;
    fs::write(root.join("check.nix"), script)?;
    fs::write(root.join("nix-conf/nix.conf"), FIXTURE_CONFIG)?;
    fs::write(
        root.join("projection.json"),
        b"[\"check.nix\",\"nix-conf/nix.conf\"]",
    )?;
    fs::write(root.join("dependencies.json"), b"[\"dependency.lock\"]")?;
    fs::write(root.join("cancel"), b"0")?;
    let nix = binary("nix")?;
    let mut executable_hash = blake3::Hasher::new();
    executable_hash.update_reader(fs::File::open(&nix)?)?;
    let hash = executable_hash.finalize().to_hex().to_string();
    fs::write(
        root.join("dependency.lock"),
        format!("nix-executable-blake3={hash}\n"),
    )?;
    let source = cli(
        &repo,
        &[
            "check-evidence",
            "source",
            "--root",
            root.to_str().ok_or("path")?,
            "--projection",
            "projection.json",
        ],
    )?;
    let dependencies = cli(
        &repo,
        &[
            "check-evidence",
            "dependencies",
            "--root",
            root.to_str().ok_or("path")?,
            "--projection",
            "dependencies.json",
        ],
    )?;
    // The private store and its private config work inside a sandboxed Nix check.
    // No daemon, host configuration change, shell closure, or external download is needed.
    let environment = BTreeMap::from([
        (
            "PATH".into(),
            nix.parent()
                .ok_or("nix directory")?
                .to_str()
                .ok_or("path")?
                .into(),
        ),
        ("HOME".into(), root.to_str().ok_or("path")?.into()),
        (
            "NIX_CONF_DIR".into(),
            root.join("nix-conf").to_str().ok_or("path")?.into(),
        ),
        (
            "NIX_USER_CONF_FILES".into(),
            root.join("nix-conf/nix.conf")
                .to_str()
                .ok_or("path")?
                .into(),
        ),
        (
            "NIX_REMOTE".into(),
            format!("local?root={}", root.join("store").display()),
        ),
    ]);
    let profile = CheckProfile {
        schema: PROFILE_SCHEMA.into(),
        check_id: "build".into(),
        kind: CheckKind::NixCheck,
        executable: nix.to_str().ok_or("path")?.into(),
        executable_hash: hash.clone(),
        tool_cohort_hash: hash,
        target: format!("{}-linux", std::env::consts::ARCH),
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
        registry_hash: canonical_json_hash(
            &json!({"schema":"cairn.nix-fixture-registry.v1","checks":checks}),
        ),
        selection_hash: authoritative_selection_hash(&checks),
        policy_hash: canonical_json_hash(&policy),
        final_subject_hash: source["subject_hash"].as_str().ok_or("subject")?.into(),
        required_checks: checks,
    };
    let applicability = Applicability {
        subject_hash: contract.final_subject_hash.clone(),
        policy_hash: contract.policy_hash.clone(),
        dependency_hashes: serde_json::from_value(dependencies["dependency_hashes"].clone())?,
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
    let attempt = blake3::hash(format!("cairn-live-nix:{name}:{}", std::process::id()).as_bytes())
        .to_hex()
        .to_string();
    let result = cli(
        &repo,
        &[
            "check-evidence",
            "run",
            "--execute",
            "--root",
            root.to_str().ok_or("path")?,
            "--profiles",
            "profiles.json",
            "--contract",
            "contract.json",
            "--applicability",
            "applicability.json",
            "--projection",
            "projection.json",
            "--dependencies",
            "dependencies.json",
            "--plan-hash",
            plan.identity(),
            "--attempt",
            &attempt,
            "--cancel-file",
            "cancel",
            "--out",
            root.join("evidence").to_str().ok_or("path")?,
        ],
    )?;
    let facts: Value = serde_json::from_slice(&fs::read(root.join("evidence/build.facts.json"))?)?;
    assert_eq!(
        result["valid"],
        expected_complete,
        "result={result} facts={facts} root={}",
        root.display()
    );
    assert_eq!(
        root.join("evidence/manifest.json").exists(),
        expected_complete
    );
    let completion = cli(
        &repo,
        &[
            "agent-receipt",
            "complete",
            "--contract",
            root.join("contract.json").to_str().ok_or("path")?,
            "--results",
            root.join("evidence/results.json").to_str().ok_or("path")?,
            "--subject-hash",
            &contract.final_subject_hash,
        ],
    )?;
    assert_eq!(completion["valid"], expected_complete, "{completion}");
    let verification = cli(
        &repo,
        &[
            "check-evidence",
            "verify",
            "--root",
            root.to_str().ok_or("path")?,
            "--out",
            root.join("evidence").to_str().ok_or("path")?,
            "--profiles",
            "profiles.json",
            "--contract",
            "contract.json",
            "--applicability",
            "applicability.json",
            "--projection",
            "projection.json",
            "--dependencies",
            "dependencies.json",
            "--plan-hash",
            plan.identity(),
            "--attempt",
            &attempt,
        ],
    )?;
    assert_eq!(verification["valid"], expected_complete, "{verification}");
    let observations: Value =
        serde_json::from_slice(&fs::read(root.join("evidence/observations.json"))?)?;
    assert_eq!(observations[0]["role"], "build");
    assert_eq!(observations[0]["outcome"], expected_outcome, "{facts}");
    assert_eq!(observations[0]["payload_passed"], expected_complete);
    assert_eq!(observations[0]["executed_tests"], json!(0));
    if expected_complete {
        assert!(nix_build_payload_passed(&facts["facts"]["nix_build"]));
        let output = facts["facts"]["nix_build"][0]["outputs"]["out"]
            .as_str()
            .ok_or("native output")?;
        let realized = root
            .join("store")
            .join(output.strip_prefix('/').ok_or("absolute output")?);
        assert_eq!(fs::read_to_string(realized)?, FIXTURE_CONTENT);
    }
    Ok(())
}
