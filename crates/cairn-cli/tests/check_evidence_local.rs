#![cfg(target_os = "linux")]
use cairn_core::verified::check_evidence::*;
use cairn_core::{
    authoritative_command_hash, authoritative_selection_hash, canonical_json_hash,
    AgentRequiredCheck, AgentVerificationContract,
};
use serde_json::Value;
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn cli(args: &[&str]) -> Result<Value, Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(repo())
        .args(args)
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}
fn save(path: &Path, value: &impl serde::Serialize) -> Result<(), Box<dyn Error>> {
    fs::write(path, serde_json::to_vec(value)?)?;
    Ok(())
}
struct Fixture {
    root: PathBuf,
    out: PathBuf,
    plan: CheckEvidencePlan,
    dependency_projection: Option<&'static str>,
    _workspace: tempfile::TempDir,
}
impl Fixture {
    fn new(name: &str, body: &str, harness: bool, deadline: u64) -> Result<Self, Box<dyn Error>> {
        let workspace = tempfile::Builder::new()
            .prefix(&format!("cairn-producer-{name}-"))
            .tempdir()?;
        let root = workspace.path().to_path_buf();
        fs::create_dir_all(root.join("src"))?;
        let out = root.join("evidence");
        fs::create_dir(&out)?;
        fs::write(root.join("Cargo.toml"), format!("[package]\nname=\"producer-control\"\nversion=\"0.1.0\"\nedition=\"2021\"\n[workspace]\n[lib]\nharness={harness}\n"))?;
        fs::write(root.join("src/lib.rs"), body)?;
        let cargo = fs::canonicalize(env!("CARGO"))?;
        assert!(Command::new(&cargo)
            .current_dir(&root)
            .args(["generate-lockfile", "--offline"])
            .status()?
            .success());
        let paths = vec!["Cargo.lock", "Cargo.toml", "src/lib.rs"];
        save(&root.join("projection.json"), &paths)?;
        fs::write(root.join("cancel"), b"0")?;
        let observed = cli(&[
            "check-evidence",
            "source",
            "--root",
            root.to_str().ok_or("path")?,
            "--projection",
            "projection.json",
        ])?;
        let mut environment = BTreeMap::new();
        for key in [
            "PATH",
            "HOME",
            "CARGO_HOME",
            "RUSTC",
            "CC",
            "NIX_CC",
            "NIX_LDFLAGS",
            "NIX_CFLAGS_COMPILE",
        ] {
            if let Ok(value) = std::env::var(key) {
                environment.insert(key.into(), value);
            }
        }
        let scratch = root.join("tmp");
        fs::create_dir(&scratch)?;
        environment.insert("TMPDIR".into(), scratch.to_str().ok_or("path")?.into());
        // The cleared environment already drops the ambient sccache wrapper;
        // inserting an empty RUSTC_WRAPPER breaks rustc's linker discovery.
        environment.insert(
            "CARGO_TARGET_DIR".into(),
            root.join("build").to_str().ok_or("path")?.into(),
        );
        let hash = blake3::hash(&fs::read(&cargo)?).to_hex().to_string();
        let p = CheckProfile {
            schema: PROFILE_SCHEMA.into(),
            check_id: "tests".into(),
            kind: CheckKind::CargoTest,
            executable: cargo.to_str().ok_or("path")?.into(),
            executable_hash: hash.clone(),
            tool_cohort_hash: hash,
            target: "x86_64-unknown-linux-gnu".into(),
            features: vec![],
            environment,
            working_root: ".".into(),
            bounds: ExecutionBounds {
                id: "live".into(),
                deadline_ms: deadline,
                teardown_ms: 2000,
                stdout_bytes: 4096,
                stderr_bytes: 16384,
            },
        };
        let argv = profile_command(&p);
        let checks = vec![AgentRequiredCheck {
            check_id: "tests".into(),
            command_hash: authoritative_command_hash(&argv),
            command: argv,
            prerequisites: vec![],
        }];
        let policy: Value = serde_json::from_slice(&fs::read(
            repo().join("cairn-policy/generated/cairn-policy.json"),
        )?)?;
        let contract = AgentVerificationContract {
            schema: "cairn.agent-completion.v2".into(),
            repository_id: "live-cairn-consumer".into(),
            registry_id: "checks".into(),
            registry_hash: "a".repeat(64),
            selection_hash: authoritative_selection_hash(&checks),
            policy_hash: canonical_json_hash(&policy),
            final_subject_hash: observed["subject_hash"].as_str().ok_or("subject")?.into(),
            required_checks: checks,
        };
        let applicability = Applicability {
            subject_hash: contract.final_subject_hash.clone(),
            policy_hash: contract.policy_hash.clone(),
            dependency_hashes: vec![],
            source_projection_hash: observed["source_projection_hash"]
                .as_str()
                .ok_or("projection")?
                .into(),
        };
        let plan = admit_plan(&contract, std::slice::from_ref(&p), &applicability)?;
        save(&root.join("contract.json"), &contract)?;
        save(&root.join("profiles.json"), &vec![p])?;
        save(&root.join("applicability.json"), &applicability)?;
        Ok(Self {
            root,
            out,
            plan,
            dependency_projection: None,
            _workspace: workspace,
        })
    }
    fn bind_dependencies(&mut self, paths: &[&str]) -> Result<Value, Box<dyn Error>> {
        save(&self.root.join("dependencies.json"), &paths)?;
        let observed = cli(&[
            "check-evidence",
            "dependencies",
            "--root",
            self.root.to_str().ok_or("path")?,
            "--projection",
            "dependencies.json",
        ])?;
        let contract: AgentVerificationContract =
            serde_json::from_slice(&fs::read(self.root.join("contract.json"))?)?;
        let profiles: Vec<CheckProfile> =
            serde_json::from_slice(&fs::read(self.root.join("profiles.json"))?)?;
        let mut applicability = self.plan.applicability().clone();
        applicability.dependency_hashes =
            serde_json::from_value(observed["dependency_hashes"].clone())?;
        self.plan = admit_plan(&contract, &profiles, &applicability)?;
        save(&self.root.join("applicability.json"), &applicability)?;
        self.dependency_projection = Some("dependencies.json");
        Ok(observed)
    }
    fn command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
        command
            .current_dir(repo())
            .args(["check-evidence", "run", "--root"])
            .arg(&self.root)
            .args([
                "--contract",
                "contract.json",
                "--profiles",
                "profiles.json",
                "--applicability",
                "applicability.json",
                "--projection",
                "projection.json",
                "--plan-hash",
                self.plan.identity(),
                "--attempt",
                &"b".repeat(64),
                "--cancel-file",
                "cancel",
                "--out",
            ])
            .arg(&self.out);
        if let Some(projection) = self.dependency_projection {
            command.args(["--dependencies", projection]);
        }
        command
    }
    fn run(&self) -> Result<Value, Box<dyn Error>> {
        let output = self.command().arg("--execute").output()?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(serde_json::from_slice(&output.stdout)?)
    }
    fn verification_command(&self) -> Command {
        let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
        command
            .current_dir(repo())
            .args(["check-evidence", "verify", "--root"])
            .arg(&self.root)
            .arg("--out")
            .arg(&self.out)
            .args([
                "--contract",
                "contract.json",
                "--profiles",
                "profiles.json",
                "--applicability",
                "applicability.json",
                "--projection",
                "projection.json",
                "--plan-hash",
                self.plan.identity(),
                "--attempt",
                &"b".repeat(64),
            ]);
        if let Some(projection) = self.dependency_projection {
            command.args(["--dependencies", projection]);
        }
        command
    }
    fn verify(&self) -> Result<Value, Box<dyn Error>> {
        let output = self.verification_command().output()?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(serde_json::from_slice(&output.stdout)?)
    }
    fn completion(&self) -> Result<Value, Box<dyn Error>> {
        cli(&[
            "agent-receipt",
            "complete",
            "--contract",
            self.root.join("contract.json").to_str().ok_or("path")?,
            "--results",
            self.out.join("results.json").to_str().ok_or("path")?,
            "--subject-hash",
            &self.plan.applicability().subject_hash,
        ])
    }
}

#[test]
fn real_developer_loop_profile_receipt_passes_the_original_validator() -> Result<(), Box<dyn Error>>
{
    let workspace = tempfile::tempdir()?;
    let root = workspace.path();
    for directory in [
        "crates",
        ".cairn/specs/producer-loop",
        ".cairn/changes",
        ".cairn/archive",
        "cairn-policy/generated",
        "evidence",
    ] {
        fs::create_dir_all(root.join(directory))?;
    }
    fs::write(
        root.join("Cargo.toml"),
        "[package]\nname = \"producer-loop\"\nversion = \"0.1.0\"\nedition = \"2021\"\n[lib]\npath = \"crates/lib.rs\"\n[workspace]\n",
    )?;
    fs::write(
        root.join("crates/lib.rs"),
        format!("// {}[impl producer.loop.checked_add]\npub fn checked_add(left: u64, right: u64) -> Option<u64> {{\n    left.checked_add(right)\n}}\n", "r"),
    )?;
    fs::write(
        root.join(".cairn/specs/producer-loop/spec.md"),
        format!("# Producer loop\n\n## Purpose\n\nExercise the real producer loop on an isolated consumer.\n\n## Requirements\n\n### Requirement: Checked arithmetic\n\n{}[producer.loop.checked_add] The library MUST reject overflowing addition.\n\n#### Scenario: Overflow\n\n- GIVEN the maximum u64 value\n- WHEN one is added\n- THEN the operation returns no value.\n", "r"),
    )?;
    fs::copy(
        repo().join("cairn-policy/generated/cairn-policy.json"),
        root.join("cairn-policy/generated/cairn-policy.json"),
    )?;
    let out = root.join("evidence");
    let subject = "c".repeat(64);
    let denied = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .args([
            "check-evidence",
            "developer-loop",
            "--root",
            root.to_str().ok_or("path")?,
            "--profile",
            "cairn-local",
            "--subject-hash",
            &subject,
            "--out",
            out.to_str().ok_or("path")?,
        ])
        .output()?;
    assert!(!denied.status.success());
    assert!(!out.join("developer-loop-receipt.json").exists());

    let produced = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .args([
            "check-evidence",
            "developer-loop",
            "--execute",
            "--root",
            root.to_str().ok_or("path")?,
            "--profile",
            "cairn-local",
            "--subject-hash",
            &subject,
            "--out",
            out.to_str().ok_or("path")?,
        ])
        .output()?;
    assert!(
        produced.status.success(),
        "{}",
        String::from_utf8_lossy(&produced.stderr)
    );
    let receipt: Value = serde_json::from_slice(&produced.stdout)?;
    assert_eq!(receipt["valid"], true, "{receipt}");
    assert_eq!(receipt["checks_executed"], true);
    assert!(out.join("developer-loop-receipt.json").exists());
    assert!(out.join("developer-loop-producer.json").exists());

    let accepted = cli(&[
        "developer-loop",
        "verify",
        "cairn-local",
        "--root",
        root.to_str().ok_or("path")?,
        "--receipt",
        out.join("developer-loop-receipt.json")
            .to_str()
            .ok_or("path")?,
        "--subject-hash",
        &subject,
    ])?;
    assert_eq!(accepted["valid"], true, "{accepted}");
    let stale = cli(&[
        "developer-loop",
        "verify",
        "cairn-local",
        "--root",
        root.to_str().ok_or("path")?,
        "--receipt",
        out.join("developer-loop-receipt.json")
            .to_str()
            .ok_or("path")?,
        "--subject-hash",
        &"d".repeat(64),
    ])?;
    assert_eq!(stale["valid"], false);
    assert!(stale["issues"]
        .as_array()
        .ok_or("issues")?
        .iter()
        .any(|issue| issue["code"] == "developer_loop.subject.stale"));
    Ok(())
}

#[test]
fn real_cargo_passes_original_completion_and_denial_never_spawns() -> Result<(), Box<dyn Error>> {
    let f = Fixture::new(
        "pass",
        "#[test] fn adds() { assert_eq!(2+2,4); }",
        true,
        30000,
    )?;
    let denied = f.command().output()?;
    assert!(!denied.status.success());
    assert!(!f.root.join("build").exists());
    assert!(!f.out.join("attempt.json").exists());
    let result = f.run()?;
    assert_eq!(result["valid"], true, "{result}");
    assert_eq!(f.completion()?["valid"], true);
    assert_eq!(f.verify()?["valid"], true);
    let saved = fs::read(f.out.join("results.json"))?;
    fs::write(f.out.join("results.json"), b"[]")?;
    assert_eq!(f.verify()?["valid"], false);
    fs::write(f.out.join("results.json"), saved)?;
    assert_eq!(f.verify()?["valid"], true);
    let journal = fs::read(f.out.join("attempt.json"))?;
    fs::remove_file(f.out.join("attempt.json"))?;
    assert_eq!(f.verify()?["recovery"], "inspect_and_use_new_output_root");
    fs::write(f.out.join("attempt.json"), journal)?;
    let manifest = fs::read(f.out.join("manifest.json"))?;
    fs::remove_file(f.out.join("manifest.json"))?;
    assert_eq!(f.verify()?["valid"], false);
    fs::write(f.out.join("manifest.json"), manifest)?;
    // The unchanged loop validator must not reinterpret Cargo rows as fmt/validate/traceability evidence.
    let loop_receipt = cli(&[
        "developer-loop",
        "verify",
        "cairn-local",
        "--root",
        f.root.to_str().ok_or("path")?,
        "--receipt",
        f.out.join("results.json").to_str().ok_or("path")?,
        "--subject-hash",
        &f.plan.applicability().subject_hash,
    ])?;
    assert_eq!(loop_receipt["valid"], false);
    assert!(loop_receipt["issues"]
        .as_array()
        .ok_or("issues")?
        .iter()
        .any(|issue| issue["code"] == "developer_loop.receipt.schema"));
    assert!(f.out.join("manifest.json").exists());
    let old = fs::read(f.out.join("manifest.json"))?;
    assert!(!f.command().arg("--execute").output()?.status.success());
    assert_eq!(old, fs::read(f.out.join("manifest.json"))?);
    Ok(())
}

#[test]
fn real_empty_failed_payload_timeout_and_flood_never_complete() -> Result<(), Box<dyn Error>> {
    for (name, body, harness, deadline, expected) in [
        ("empty", "pub fn no_tests() {}", true, 30000, "failed"),
        ("failed", "#[test] fn fails() { assert_eq!(2,3); }", true, 30000, "failed"),
        ("payload", "fn main() { println!(\"running 1 test\\ntest result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s\"); }", false, 30000, "failed"),
        ("timeout", "#[test] fn hangs() { loop { std::thread::sleep(std::time::Duration::from_millis(50)); } }", true, 2000, "timed_out"),
        ("flood", "fn main() { use std::io::Write; std::io::stdout().write_all(&vec![b'x';100000]).unwrap(); std::io::stderr().write_all(&vec![b'y';100000]).unwrap(); }", false, 30000, "infrastructure_error"),
    ] {
        let f = Fixture::new(name, body, harness, deadline)?;
        let result = f.run()?;
        assert_eq!(result["valid"], false, "{name}: {result}");
        assert_eq!(result["results"][0]["status"], expected, "{name}: {result}");
        assert_eq!(f.completion()?["valid"], false);
        assert!(!f.out.join("manifest.json").exists());
    }
    Ok(())
}

#[test]
fn partial_publication_preserves_existing_bytes_and_withholds_manifest(
) -> Result<(), Box<dyn Error>> {
    let f = Fixture::new("publication", "#[test] fn collides() { std::fs::write(\"evidence/results.json\", \"untrusted-existing\").unwrap(); }", true, 30000)?;
    let result = f.run()?;
    assert_eq!(result["valid"], false);
    assert!(result["blockers"]
        .as_array()
        .ok_or("blockers")?
        .contains(&serde_json::json!("artifact_publication_failed")));
    assert!(!f.out.join("manifest.json").exists());
    assert!(f.out.join("attempt.json").exists());
    assert!(f.out.join("incomplete.json").exists());
    assert_eq!(fs::read(f.out.join("results.json"))?, b"untrusted-existing");
    assert!(!f.command().arg("--execute").output()?.status.success());
    Ok(())
}

#[test]
fn real_owned_descendant_teardown_and_escaped_pipe_uncertainty() -> Result<(), Box<dyn Error>> {
    for (name, escape, expected) in [
        ("descendant", "", true),
        ("escaped", ".process_group(0)", false),
    ] {
        let source = format!("use std::os::unix::process::CommandExt;\n#[test] fn parent() {{ std::process::Command::new(std::env::current_exe().unwrap()).args([\"--exact\",\"descendant\",\"--ignored\"]) {escape}.stdout(std::process::Stdio::null()).spawn().unwrap(); }}\n#[test] #[ignore] fn descendant() {{ std::thread::sleep(std::time::Duration::from_millis(4000)); }}");
        let f = Fixture::new(name, &source, true, 30000)?;
        let result = f.run()?;
        assert_eq!(result["valid"], expected, "{result}");
        assert_eq!(f.completion()?["valid"], expected);
        let observed: Value = serde_json::from_slice(&fs::read(f.out.join("observations.json"))?)?;
        assert_eq!(observed[0]["execution_observed"], expected);
        if !expected {
            assert_eq!(result["results"][0]["status"], "cleanup_failed");
            // The escaped child is deliberately finite. Wait for its exit before returning.
            std::thread::sleep(std::time::Duration::from_millis(4200));
        }
    }
    Ok(())
}

#[test]
fn real_cancellation_and_source_drift_withhold_success() -> Result<(), Box<dyn Error>> {
    let f = Fixture::new(
        "cancel",
        "#[test] fn hangs() { loop { std::thread::sleep(std::time::Duration::from_millis(50)); } }",
        true,
        10000,
    )?;
    let mut child = f
        .command()
        .arg("--execute")
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()?;
    for _ in 0..200 {
        if f.out.join("attempt.json").exists() {
            break;
        }
        if child.try_wait()?.is_some() {
            break;
        }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    std::thread::sleep(std::time::Duration::from_millis(700));
    fs::write(f.root.join("cancel"), b"1")?;
    let output = child.wait_with_output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(result["valid"], false);
    assert_eq!(result["results"][0]["status"], "cancelled");
    assert_eq!(f.completion()?["valid"], false);
    assert!(!f.out.join("manifest.json").exists());
    let drift = Fixture::new(
        "drift",
        "#[test] fn writes() { std::fs::write(\"src/lib.rs\", \"changed\").unwrap(); }",
        true,
        30000,
    )?;
    assert_eq!(drift.run()?["valid"], false);
    assert!(!drift.out.join("manifest.json").exists());
    let observed: Value = serde_json::from_slice(&fs::read(drift.out.join("observations.json"))?)?;
    assert_eq!(observed.as_array().ok_or("observations")?.len(), 1);
    assert_ne!(
        observed[0]["source_after_hash"],
        drift.plan.applicability().subject_hash
    );
    Ok(())
}

#[test]
fn unavailable_source_preserves_executed_facts_without_completion() -> Result<(), Box<dyn Error>> {
    let f = Fixture::new(
        "source-removed",
        "#[test] fn removes() { std::fs::remove_file(\"src/lib.rs\").unwrap(); }",
        true,
        30000,
    )?;
    let receipt = f.run()?;
    assert_eq!(receipt["valid"], false);
    assert!(receipt["blockers"]
        .as_array()
        .ok_or("blockers")?
        .contains(&serde_json::json!("source_unavailable")));
    assert_eq!(f.completion()?["valid"], false);
    assert!(f.out.join("tests.facts.json").exists());
    let observed: Value = serde_json::from_slice(&fs::read(f.out.join("observations.json"))?)?;
    assert_eq!(observed.as_array().ok_or("observations")?.len(), 1);
    assert_eq!(observed[0]["source_after_hash"], "");
    assert_eq!(observed[0]["outcome"], "exited_zero");
    assert!(!f.out.join("manifest.json").exists());
    assert!(f.out.join("incomplete.json").exists());
    Ok(())
}

#[test]
fn facts_collision_preserves_observation_and_existing_bytes() -> Result<(), Box<dyn Error>> {
    let f = Fixture::new("facts-collision",
        "#[test] fn collides() { std::fs::write(\"evidence/tests.facts.json\", \"existing\").unwrap(); }", true, 30000)?;
    let receipt = f.run()?;
    assert_eq!(receipt["valid"], false);
    assert_eq!(receipt["results"], serde_json::json!([]));
    assert_eq!(receipt["blockers"][0], "facts_publication_failed");
    assert_eq!(fs::read(f.out.join("tests.facts.json"))?, b"existing");
    let observed: Value = serde_json::from_slice(&fs::read(f.out.join("observations.json"))?)?;
    assert_eq!(observed.as_array().ok_or("observations")?.len(), 1);
    assert!(!f.out.join("manifest.json").exists());
    assert!(f.out.join("incomplete.json").exists());
    Ok(())
}

// r[verify cairn.check_evidence_producer.dependency_observation]
#[test]
fn observed_dependencies_reach_original_consumers_and_reject_stale_verification(
) -> Result<(), Box<dyn Error>> {
    let mut f = Fixture::new(
        "dependencies-pass",
        "#[test] fn passes() { assert_eq!(2+2,4); }",
        true,
        30000,
    )?;
    fs::write(f.root.join("dependency-a"), b"reviewed bytes")?;
    fs::write(f.root.join("dependency-b"), b"reviewed bytes")?;
    let observed = f.bind_dependencies(&["dependency-a", "dependency-b"])?;
    let mut expected: Vec<_> = ["dependency-a", "dependency-b"]
        .iter()
        .map(|path| {
            canonical_json_hash(
                &serde_json::json!({"schema":"cairn.check-dependency.v1", "path":path,
            "content_hash":blake3::hash(b"reviewed bytes").to_hex().to_string()}),
            )
        })
        .collect();
    expected.sort();
    assert_eq!(observed["dependency_hashes"], serde_json::json!(expected));
    assert_eq!(
        observed["dependency_projection_hash"],
        canonical_json_hash(&serde_json::json!(["dependency-a", "dependency-b"]))
    );
    assert_eq!(observed["checks_executed"], false);
    assert_eq!(f.run()?["complete"], true);
    assert_eq!(f.completion()?["valid"], true);
    assert_eq!(f.verify()?["valid"], true);
    fs::write(f.root.join("dependency-a"), b"changed bytes")?;
    assert!(!f.verification_command().output()?.status.success());
    fs::write(f.root.join("dependency-a"), b"reviewed bytes")?;
    assert_eq!(f.verify()?["valid"], true);
    // The same bytes at a different path are not the declared dependency.
    fs::write(f.root.join("substitute"), b"reviewed bytes")?;
    save(
        &f.root.join("dependencies.json"),
        &vec!["dependency-b", "substitute"],
    )?;
    assert!(!f.verification_command().output()?.status.success());
    save(&f.root.join("dependencies.json"), &Vec::<String>::new())?;
    assert!(!f.verification_command().output()?.status.success());
    f.dependency_projection = None;
    assert!(!f.verification_command().output()?.status.success());
    f.dependency_projection = Some("dependencies.json");
    save(
        &f.root.join("dependencies.json"),
        &vec!["dependency-a", "dependency-b"],
    )?;
    fs::remove_file(f.root.join("dependency-a"))?;
    assert!(!f.verification_command().output()?.status.success());
    Ok(())
}

// r[verify cairn.check_evidence_producer.dependency_observation]
#[test]
fn dependency_preflight_rejects_missing_stale_omitted_and_substituted_scope(
) -> Result<(), Box<dyn Error>> {
    let mut f = Fixture::new(
        "dependencies-preflight",
        "#[test] fn passes() { assert!(true); }",
        true,
        30000,
    )?;
    fs::write(f.root.join("dependency"), b"reviewed")?;
    fs::write(f.root.join("substitute"), b"reviewed")?;
    f.bind_dependencies(&["dependency"])?;
    f.dependency_projection = None;
    assert!(!f.command().arg("--execute").output()?.status.success());
    f.dependency_projection = Some("dependencies.json");
    save(&f.root.join("dependencies.json"), &Vec::<String>::new())?;
    assert!(!f.command().arg("--execute").output()?.status.success());
    save(&f.root.join("dependencies.json"), &vec!["substitute"])?;
    assert!(!f.command().arg("--execute").output()?.status.success());
    save(&f.root.join("dependencies.json"), &vec!["dependency"])?;
    fs::write(f.root.join("dependency"), b"stale")?;
    assert!(!f.command().arg("--execute").output()?.status.success());
    fs::remove_file(f.root.join("dependency"))?;
    assert!(!f.command().arg("--execute").output()?.status.success());
    assert!(!f.root.join("build").exists());
    assert!(!f.out.join("attempt.json").exists());
    // A supplied nonempty projection cannot be ignored by an empty declaration either.
    fs::write(f.root.join("dependency"), b"reviewed")?;
    let contract: AgentVerificationContract =
        serde_json::from_slice(&fs::read(f.root.join("contract.json"))?)?;
    let profiles: Vec<CheckProfile> =
        serde_json::from_slice(&fs::read(f.root.join("profiles.json"))?)?;
    let mut applicability = f.plan.applicability().clone();
    applicability.dependency_hashes.clear();
    f.plan = admit_plan(&contract, &profiles, &applicability)?;
    save(&f.root.join("applicability.json"), &applicability)?;
    assert!(!f.command().arg("--execute").output()?.status.success());
    assert!(!f.out.join("attempt.json").exists());
    Ok(())
}

// r[verify cairn.check_evidence_producer.dependency_observation]
#[test]
fn dependency_drift_after_child_preserves_facts_but_never_completion() -> Result<(), Box<dyn Error>>
{
    for (name, body, unavailable) in [
        (
            "dependencies-changed",
            "#[test] fn changes() { std::fs::write(\"dependency\", b\"changed\").unwrap(); }",
            false,
        ),
        (
            "dependencies-removed",
            "#[test] fn removes() { std::fs::remove_file(\"dependency\").unwrap(); }",
            true,
        ),
    ] {
        let mut f = Fixture::new(name, body, true, 30000)?;
        fs::write(f.root.join("dependency"), b"reviewed")?;
        f.bind_dependencies(&["dependency"])?;
        let receipt = f.run()?;
        assert_eq!(receipt["complete"], false);
        assert_eq!(receipt["results"], serde_json::json!([]));
        assert_eq!(f.completion()?["valid"], false);
        assert!(!f.out.join("manifest.json").exists());
        assert!(f.out.join("incomplete.json").exists());
        let facts: Value = serde_json::from_slice(&fs::read(f.out.join("tests.facts.json"))?)?;
        assert_eq!(facts["payload_passed"], true);
        assert_eq!(facts["executed_tests"], 1);
        let observations: Value =
            serde_json::from_slice(&fs::read(f.out.join("observations.json"))?)?;
        assert_eq!(observations[0]["outcome"], "exited_zero");
        assert_ne!(
            observations[0]["applicability"]["dependency_hashes"],
            serde_json::json!(f.plan.applicability().dependency_hashes)
        );
        if unavailable {
            assert_eq!(
                observations[0]["applicability"]["dependency_hashes"],
                serde_json::json!([])
            );
        } else {
            let expected = canonical_json_hash(
                &serde_json::json!({"schema":"cairn.check-dependency.v1", "path":"dependency",
                "content_hash":blake3::hash(b"changed").to_hex().to_string()}),
            );
            assert_eq!(
                observations[0]["applicability"]["dependency_hashes"],
                serde_json::json!([expected])
            );
        }
    }
    Ok(())
}

// r[verify cairn.check_evidence_producer.dependency_observation]
#[test]
fn dependency_projection_rejects_invalid_and_no_follow_members_and_bounds(
) -> Result<(), Box<dyn Error>> {
    use std::os::{
        fd::AsRawFd,
        unix::{fs::symlink, net::UnixListener},
    };
    let f = Fixture::new(
        "dependencies-invalid",
        "#[test] fn passes() { assert!(true); }",
        true,
        30000,
    )?;
    fs::create_dir(f.root.join("nested"))?;
    fs::write(f.root.join("nested/dependency"), b"bytes")?;
    symlink("nested", f.root.join("linked"))?;
    symlink("nested/dependency", f.root.join("leaf"))?;
    // Bind through a held directory descriptor, independent of checkout path length.
    let socket_directory = fs::File::open(&f.root)?;
    let _socket = UnixListener::bind(format!(
        "/proc/self/fd/{}/socket",
        socket_directory.as_raw_fd()
    ))?;
    let observe = || {
        Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(repo())
            .args(["check-evidence", "dependencies", "--root"])
            .arg(&f.root)
            .args(["--projection", "dependencies.json"])
            .output()
    };
    for paths in [
        vec!["linked/dependency"],
        vec!["leaf"],
        vec!["nested"],
        vec!["socket"],
        vec!["absent"],
        vec!["../dependency"],
        vec!["/etc/passwd"],
        vec![""],
        vec!["./nested/dependency"],
        vec!["nested//dependency"],
        vec!["nested/../nested/dependency"],
        vec!["nested/dependency", "nested/dependency"],
        vec!["socket", "nested/dependency"],
        vec!["nul\0member"],
    ] {
        save(&f.root.join("dependencies.json"), &paths)?;
        let output = observe()?;
        assert!(!output.status.success(), "admitted {paths:?}");
        assert!(!String::from_utf8_lossy(&output.stderr).contains("/etc/passwd"));
    }
    let paths: Vec<_> = (0..64).map(|index| format!("dep-{index:02}")).collect();
    for path in &paths {
        fs::write(f.root.join(path), b"bounded")?;
    }
    save(&f.root.join("dependencies.json"), &paths)?;
    let accepted: Value = serde_json::from_slice(&observe()?.stdout)?;
    let mut expected: Vec<_> = paths.iter().map(|path| canonical_json_hash(&serde_json::json!({
        "schema":"cairn.check-dependency.v1", "path":path, "content_hash":blake3::hash(b"bounded").to_hex().to_string()
    }))).collect();
    expected.sort();
    assert_eq!(accepted["dependency_hashes"], serde_json::json!(expected));
    let mut too_many = paths;
    too_many.push("dep-64".into());
    save(&f.root.join("dependencies.json"), &too_many)?;
    assert!(!observe()?.status.success());
    let oversized = fs::File::create(f.root.join("oversized"))?;
    oversized.set_len(16 * 1024 * 1024 + 1)?;
    save(&f.root.join("dependencies.json"), &vec!["oversized"])?;
    assert!(!observe()?.status.success());
    let aggregate: Vec<_> = (0..5).map(|index| format!("large-{index}")).collect();
    for path in &aggregate {
        fs::File::create(f.root.join(path))?.set_len(16 * 1024 * 1024)?;
    }
    save(&f.root.join("dependencies.json"), &aggregate)?;
    assert!(!observe()?.status.success());
    save(&f.root.join("dependencies.json"), &Vec::<String>::new())?;
    let empty: Value = serde_json::from_slice(&observe()?.stdout)?;
    assert_eq!(empty["dependency_hashes"], serde_json::json!([]));
    fs::rename(
        f.root.join("dependencies.json"),
        f.root.join("real-projection.json"),
    )?;
    symlink("real-projection.json", f.root.join("dependencies.json"))?;
    assert!(!observe()?.status.success());
    assert!(!f.out.join("attempt.json").exists());
    Ok(())
}
