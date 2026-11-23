//! A checked-declaration replay, not a setup installation receipt.
use cairn_core::verified::consumer_bootstrap::{
    BootstrapCompleteSelection, BootstrapIdentity, BootstrapProjectDeclaration,
};
use std::{error::Error, fs, path::PathBuf, process::Command};

const MAX_DECLARATION_BYTES: usize = 65_536;

// r[verify cairn.consumer_bootstrap.policy]
// r[verify cairn.consumer_bootstrap.boundary]
#[test]
fn declaration_resolves_from_selected_root_and_fails_closed() -> Result<(), Box<dyn Error>> {
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root = std::env::temp_dir().join(format!("cairn-project-policy-{}", std::process::id()));
    fs::create_dir(&root)?;
    for dir in [
        ".cairn/specs",
        ".cairn/changes",
        ".cairn/archive",
        ".cairn/bootstrap",
        "elsewhere",
    ] {
        fs::create_dir_all(root.join(dir))?;
    }
    let policy_bytes = fs::read(source.join("cairn-policy/generated/cairn-policy.json"))?;
    let manifest = BootstrapCompleteSelection {
        artifacts: vec![],
        schema: "cairn.bootstrap-complete-selection.v1".into(),
        cohort: BootstrapIdentity::from_bytes(b"cohort"),
        policy: BootstrapIdentity::from_bytes(&policy_bytes),
        policy_locator: ".cairn/bootstrap/policy.json".into(),
    };
    let manifest_bytes = serde_json::to_vec(&manifest)?;
    let declaration = BootstrapProjectDeclaration {
        schema: "cairn.bootstrap-project.v1".into(),
        policy_locator: manifest.policy_locator.clone(),
        policy: manifest.policy.clone(),
        cohort: manifest.cohort.clone(),
        complete_manifest: BootstrapIdentity::from_bytes(&manifest_bytes),
    };
    manifest.validate_for(&declaration)?;
    fs::write(root.join(".cairn/bootstrap/complete.json"), &manifest_bytes)?;
    fs::write(
        root.join(".cairn/bootstrap/selection.json"),
        serde_json::to_vec(&declaration)?,
    )?;
    fs::write(root.join(&declaration.policy_locator), &policy_bytes)?;
    let run = |extra: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(root.join("elsewhere"))
            .env("HOME", &root)
            .env("CAIRN_DATA_DIR", root.join("data"))
            .args(["validate", "--root"])
            .arg(&root)
            .args(extra)
            .output()
    };
    let good = run(&[])?;
    assert!(
        good.status.success(),
        "{}",
        String::from_utf8_lossy(&good.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&good.stdout)?;
    assert_eq!(result["valid"], true);
    assert_eq!(result["policy_selection"]["source"], "project");
    assert_eq!(
        result["policy_selection"]["blake3"],
        declaration.policy.as_str()
    );
    assert_eq!(result["policy_selection"]["installation_receipt"], false);
    let doctor = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root.join("elsewhere"))
        .env("HOME", &root)
        .env("CAIRN_DATA_DIR", root.join("data"))
        .args(["doctor", "--root"])
        .arg(&root)
        .output()?;
    assert!(doctor.status.success());
    let doctor: serde_json::Value = serde_json::from_slice(&doctor.stdout)?;
    assert_eq!(doctor["valid"], true);
    assert_eq!(doctor["policy_selection"]["source"], "project");
    for args in [
        vec!["--policy"],
        vec!["--policy", ""],
        vec!["--policy", "--strict"],
        vec!["--policy", "a", "--policy", "b"],
        vec!["--policy=missing.json"],
    ] {
        assert!(!run(&args)?.status.success());
    }
    fs::create_dir(root.join(".cairn/changes/demo"))?;
    for file in ["proposal.md", "design.md", "tasks.md"] {
        fs::copy(
            source.join("fixtures/basic/.cairn/changes/demo").join(file),
            root.join(".cairn/changes/demo").join(file),
        )?;
    }
    let gate = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root.join("elsewhere"))
        .args(["gate", "proposal", "demo", "--root"])
        .arg(&root)
        .output()?;
    assert!(
        gate.status.success(),
        "{}",
        String::from_utf8_lossy(&gate.stderr)
    );
    let gate: serde_json::Value = serde_json::from_slice(&gate.stdout)?;
    assert_eq!(gate["policy_selection"]["source"], "project");
    // Gate payload is intentionally not asserted passing: these copied files
    // are a resolver control, not a complete consumer change.
    fs::remove_dir_all(root.join(".cairn/changes/demo"))?;
    let invalid_explicit = run(&["--policy", "missing.json"])?;
    assert!(!invalid_explicit.status.success());
    assert!(String::from_utf8_lossy(&invalid_explicit.stderr).contains("failed to read policy"));
    fs::write(root.join(&declaration.policy_locator), b"modified")?;
    let modified = run(&[])?;
    assert!(!modified.status.success());
    assert!(
        String::from_utf8_lossy(&modified.stderr).contains("bootstrap.policy.identity_mismatch")
    );
    let explicit = source
        .join("cairn-policy/generated/cairn-policy.json")
        .canonicalize()?;
    assert!(run(&["--policy", explicit.to_str().ok_or("UTF-8 path")?])?
        .status
        .success());
    fs::write(root.join(&declaration.policy_locator), &policy_bytes)?;
    fs::write(root.join(".cairn/bootstrap/complete.json"), b"changed")?;
    assert!(!run(&[])?.status.success());
    fs::write(root.join(".cairn/bootstrap/complete.json"), &manifest_bytes)?;
    // A present malformed or oversized declaration never becomes absence.
    let declaration_path = root.join(".cairn/bootstrap/selection.json");
    for bytes in [b"not JSON".to_vec(), vec![b' '; MAX_DECLARATION_BYTES + 1]] {
        fs::write(&declaration_path, bytes)?;
        assert!(!run(&[])?.status.success());
    }
    fs::write(&declaration_path, serde_json::to_vec(&declaration)?)?;
    #[cfg(unix)]
    {
        // Place a usable legacy default in the working directory. Dangling
        // selected paths must still fail rather than silently using it.
        fs::create_dir_all(root.join("elsewhere/cairn-policy/generated"))?;
        fs::write(
            root.join("elsewhere/cairn-policy/generated/cairn-policy.json"),
            &policy_bytes,
        )?;
        fs::remove_file(&declaration_path)?;
        std::os::unix::fs::symlink("absent", &declaration_path)?;
        assert!(!run(&[])?.status.success());
        fs::remove_file(&declaration_path)?;
        fs::write(&declaration_path, serde_json::to_vec(&declaration)?)?;
        fs::remove_file(root.join(&declaration.policy_locator))?;
        std::os::unix::fs::symlink(&explicit, root.join(&declaration.policy_locator))?;
        assert!(!run(&[])?.status.success());
        fs::remove_file(root.join(&declaration.policy_locator))?;
        fs::write(root.join(&declaration.policy_locator), &policy_bytes)?;
        fs::rename(
            root.join(".cairn/bootstrap"),
            root.join(".cairn/real-bootstrap"),
        )?;
        std::os::unix::fs::symlink("real-bootstrap", root.join(".cairn/bootstrap"))?;
        assert!(!run(&[])?.status.success());
    }
    fs::remove_dir_all(root)?;
    Ok(())
}

// r[verify cairn.consumer_bootstrap.policy]
// r[verify cairn.consumer_bootstrap.boundary]
#[test]
fn sync_and_archive_honor_bootstrapped_project_policy_and_fail_closed() -> Result<(), Box<dyn Error>>
{
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root = std::env::temp_dir().join(format!(
        "cairn-project-policy-lifecycle-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(&root)?;

    let copy_fixture = |project: &PathBuf| -> Result<(), Box<dyn Error>> {
        let _ = fs::remove_dir_all(project);
        for dir in [
            ".cairn/specs/demo",
            ".cairn/changes/ready/specs/demo",
            ".cairn/archive",
            ".cairn/bootstrap",
        ] {
            fs::create_dir_all(project.join(dir))?;
        }
        let fixture = source.join("fixtures/archive-ready");
        fs::copy(
            fixture.join(".cairn/specs/demo/spec.md"),
            project.join(".cairn/specs/demo/spec.md"),
        )?;
        for file in ["proposal.md", "design.md", "specs/demo/spec.md"] {
            fs::copy(
                fixture.join(".cairn/changes/ready").join(file),
                project.join(".cairn/changes/ready").join(file),
            )?;
        }
        // Exactly one substantive completed checkbox task: passes Q (min=1), fails P (min=2).
        fs::write(
            project.join(".cairn/changes/ready/tasks.md"),
            "## Phase 1\n\n- [x] [serial] r[demo.ready] Define single-task fixture work.\n",
        )?;
        Ok(())
    };

    let elsewhere = root.join("elsewhere");
    fs::create_dir_all(&elsewhere)?;

    let q_path = source
        .join("cairn-policy/generated/cairn-policy.json")
        .canonicalize()?;
    let q_bytes = fs::read(&q_path)?;
    let q_hash = BootstrapIdentity::from_bytes(&q_bytes).as_str().to_string();

    let mut p_json: serde_json::Value = serde_json::from_slice(&q_bytes)?;
    p_json["name"] = serde_json::json!("cairn-project-strict-tasks");
    p_json["gate_policy"]["substance"]["minimum_substantive_tasks"] = serde_json::json!(2);
    let p_bytes = serde_json::to_vec(&p_json)?;
    let p_hash = BootstrapIdentity::from_bytes(&p_bytes).as_str().to_string();
    let p_external_path = root.join("policy-p.json");
    fs::write(&p_external_path, &p_bytes)?;

    let install_project_policy = |project: &PathBuf, bytes: &[u8]| -> Result<(), Box<dyn Error>> {
        let manifest = BootstrapCompleteSelection {
            artifacts: vec![],
            schema: "cairn.bootstrap-complete-selection.v1".into(),
            cohort: BootstrapIdentity::from_bytes(b"cohort-lifecycle"),
            policy: BootstrapIdentity::from_bytes(bytes),
            policy_locator: ".cairn/bootstrap/policy.json".into(),
        };
        let manifest_bytes = serde_json::to_vec(&manifest)?;
        let declaration = BootstrapProjectDeclaration {
            schema: "cairn.bootstrap-project.v1".into(),
            policy_locator: manifest.policy_locator.clone(),
            policy: manifest.policy.clone(),
            cohort: manifest.cohort.clone(),
            complete_manifest: BootstrapIdentity::from_bytes(&manifest_bytes),
        };
        manifest.validate_for(&declaration)?;
        fs::write(
            project.join(".cairn/bootstrap/complete.json"),
            &manifest_bytes,
        )?;
        fs::write(
            project.join(".cairn/bootstrap/selection.json"),
            serde_json::to_vec(&declaration)?,
        )?;
        fs::write(project.join(&declaration.policy_locator), bytes)?;
        Ok(())
    };

    let run_cmd = |project: &PathBuf,
                   cairn_policy_env: Option<&PathBuf>,
                   args: &[&str]|
     -> Result<std::process::Output, Box<dyn Error>> {
        let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
        command
            .current_dir(&elsewhere)
            .env("HOME", &root)
            .env("CAIRN_DATA_DIR", root.join("data"))
            .env_remove("CAIRN_GLOBAL_REGISTRY")
            .args(args)
            .arg("--root")
            .arg(project);
        match cairn_policy_env {
            Some(path) => {
                command.env("CAIRN_POLICY", path);
            }
            None => {
                command.env_remove("CAIRN_POLICY");
            }
        }
        Ok(command.output()?)
    };

    let parse_ok = |output: std::process::Output| -> Result<serde_json::Value, Box<dyn Error>> {
        assert!(
            output.status.success(),
            "stderr: {}\nstdout: {}",
            String::from_utf8_lossy(&output.stderr),
            String::from_utf8_lossy(&output.stdout)
        );
        Ok(serde_json::from_slice(&output.stdout)?)
    };

    // 1. P/Q counterexample for gate, sync, and archive (preview + execute).
    let project = root.join("project-pq");
    copy_fixture(&project)?;
    install_project_policy(&project, &p_bytes)?;
    let q_arg = q_path.to_str().ok_or("UTF-8 Q path")?;
    let p_arg = p_external_path.to_str().ok_or("UTF-8 P path")?;

    // Control: explicit Q unblocks sync and archive preflight.
    let sync_q = parse_ok(run_cmd(
        &project,
        Some(&q_path),
        &["sync", "ready", "--policy", q_arg],
    )?)?;
    assert_eq!(sync_q["blocked"], false);
    assert_eq!(sync_q["policy"], "cairn-default");
    assert_eq!(sync_q["policy_selection"]["source"], "explicit");
    assert_eq!(sync_q["policy_selection"]["blake3"], q_hash);

    let archive_q = parse_ok(run_cmd(
        &project,
        Some(&q_path),
        &["archive", "ready", "--policy", q_arg],
    )?)?;
    assert_eq!(archive_q["blocked"], false);
    assert_eq!(archive_q["policy"], "cairn-default");
    assert_eq!(archive_q["policy_selection"]["source"], "explicit");
    assert_eq!(archive_q["policy_selection"]["blake3"], q_hash);

    // Without --policy and with CAIRN_POLICY=Q, gate, sync, and archive all select project policy P.
    let gate_p = parse_ok(run_cmd(
        &project,
        Some(&q_path),
        &["gate", "tasks", "ready"],
    )?)?;
    assert_eq!(gate_p["valid"], false);
    assert_eq!(gate_p["policy"], "cairn-project-strict-tasks");
    assert_eq!(gate_p["policy_selection"]["source"], "project");
    assert_eq!(gate_p["policy_selection"]["blake3"], p_hash);
    assert!(gate_p["issues"].as_array().is_some_and(|issues| issues
        .iter()
        .any(|issue| { issue["code"] == "lifecycle_validation.task_substance.minimum" })));

    for (args, expected_source) in [
        (&["sync", "ready"][..], "project"),
        (&["sync", "ready", "--execute"][..], "project"),
        (&["archive", "ready"][..], "project"),
        (&["archive", "ready", "--execute"][..], "project"),
        (&["sync", "ready", "--policy", p_arg][..], "explicit"),
        (&["archive", "ready", "--policy", p_arg][..], "explicit"),
    ] {
        let receipt = parse_ok(run_cmd(&project, Some(&q_path), args)?)?;
        assert_eq!(receipt["blocked"], true, "args={args:?}");
        assert_eq!(receipt["mutated"], false, "args={args:?}");
        assert_eq!(
            receipt["policy"], "cairn-project-strict-tasks",
            "args={args:?}"
        );
        assert_eq!(
            receipt["policy_selection"]["source"], expected_source,
            "args={args:?}"
        );
        assert_eq!(
            receipt["policy_selection"]["blake3"], p_hash,
            "args={args:?}"
        );
        if expected_source == "project" {
            assert_eq!(
                receipt["policy_selection"]["locator"],
                ".cairn/bootstrap/policy.json"
            );
        }
        assert!(
            receipt["reasons"]
                .as_array()
                .is_some_and(
                    |reasons| reasons.iter().any(|reason| reason.as_str().is_some_and(
                        |text| text.contains("lifecycle_validation.task_substance.minimum")
                    ))
                ),
            "expected task_substance.minimum blocker for args={args:?}: {receipt}"
        );
    }
    assert!(
        project.join(".cairn/changes/ready").exists(),
        "blocked archive --execute must not move active change"
    );

    // 2. Execution from another working directory with NO CAIRN_POLICY still resolves project policy.
    let sync_no_env = parse_ok(run_cmd(&project, None, &["sync", "ready"])?)?;
    assert_eq!(sync_no_env["policy_selection"]["source"], "project");
    assert_eq!(sync_no_env["policy_selection"]["blake3"], p_hash);
    let archive_no_env = parse_ok(run_cmd(&project, None, &["archive", "ready"])?)?;
    assert_eq!(archive_no_env["policy_selection"]["source"], "project");
    assert_eq!(archive_no_env["policy_selection"]["blake3"], p_hash);

    // 3. Explicit --policy Q overrides project policy and allows execution.
    let project_override = root.join("project-override");
    copy_fixture(&project_override)?;
    install_project_policy(&project_override, &p_bytes)?;
    let sync_exec_q = parse_ok(run_cmd(
        &project_override,
        Some(&q_path),
        &["sync", "ready", "--policy", q_arg, "--execute"],
    )?)?;
    assert_eq!(sync_exec_q["blocked"], false);
    assert_eq!(sync_exec_q["mutated"], true);
    assert_eq!(sync_exec_q["policy_selection"]["source"], "explicit");
    let archive_exec_q = parse_ok(run_cmd(
        &project_override,
        Some(&q_path),
        &["archive", "ready", "--policy", q_arg, "--execute"],
    )?)?;
    assert_eq!(archive_exec_q["blocked"], false);
    assert_eq!(archive_exec_q["mutated"], true);
    assert_eq!(archive_exec_q["policy_selection"]["source"], "explicit");
    assert!(!project_override.join(".cairn/changes/ready").exists());

    // 4. Tampered policy bytes, tampered manifest, and recovery markers fail closed without mutating.
    let project_fail = root.join("project-fail-closed");
    copy_fixture(&project_fail)?;
    install_project_policy(&project_fail, &q_bytes)?;
    let spec_before = fs::read_to_string(project_fail.join(".cairn/specs/demo/spec.md"))?;

    // Tampered policy bytes -> identity_mismatch.
    fs::write(
        project_fail.join(".cairn/bootstrap/policy.json"),
        b"tampered",
    )?;
    for args in [
        &["sync", "ready"][..],
        &["sync", "ready", "--execute"][..],
        &["archive", "ready"][..],
        &["archive", "ready", "--execute"][..],
    ] {
        let output = run_cmd(&project_fail, Some(&q_path), args)?;
        assert!(
            !output.status.success(),
            "tampered policy must fail for {args:?}"
        );
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("bootstrap.policy.identity_mismatch"),
            "stderr for {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert_eq!(
        fs::read_to_string(project_fail.join(".cairn/specs/demo/spec.md"))?,
        spec_before
    );
    assert!(project_fail.join(".cairn/changes/ready").exists());

    // Restore policy, tamper complete manifest -> complete_manifest_mismatch.
    install_project_policy(&project_fail, &q_bytes)?;
    fs::write(
        project_fail.join(".cairn/bootstrap/complete.json"),
        b"tampered",
    )?;
    for args in [
        &["sync", "ready", "--execute"][..],
        &["archive", "ready", "--execute"][..],
    ] {
        let output = run_cmd(&project_fail, Some(&q_path), args)?;
        assert!(
            !output.status.success(),
            "tampered manifest must fail for {args:?}"
        );
        assert!(
            String::from_utf8_lossy(&output.stderr)
                .contains("bootstrap.policy.complete_manifest_mismatch"),
            "stderr for {args:?}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    assert!(project_fail.join(".cairn/changes/ready").exists());

    // Recovery markers (.cairn/bootstrap/prepared.json and .cairn-bootstrap.lock) -> recovery_required.
    install_project_policy(&project_fail, &q_bytes)?;
    for marker in [".cairn/bootstrap/prepared.json", ".cairn-bootstrap.lock"] {
        fs::write(project_fail.join(marker), b"{}")?;
        for args in [
            &["sync", "ready"][..],
            &["sync", "ready", "--execute"][..],
            &["archive", "ready"][..],
            &["archive", "ready", "--execute"][..],
        ] {
            let output = run_cmd(&project_fail, Some(&q_path), args)?;
            assert!(
                !output.status.success(),
                "recovery marker {marker} must fail for {args:?}"
            );
            assert!(
                String::from_utf8_lossy(&output.stderr)
                    .contains("bootstrap.policy.recovery_required"),
                "stderr for {marker} {args:?}: {}",
                String::from_utf8_lossy(&output.stderr)
            );
        }
        fs::remove_file(project_fail.join(marker))?;
    }
    assert_eq!(
        fs::read_to_string(project_fail.join(".cairn/specs/demo/spec.md"))?,
        spec_before
    );
    assert!(project_fail.join(".cairn/changes/ready").exists());

    // 5. Legacy fallback when no project declaration exists.
    let project_legacy = root.join("project-legacy");
    copy_fixture(&project_legacy)?;
    let sync_legacy = parse_ok(run_cmd(&project_legacy, Some(&q_path), &["sync", "ready"])?)?;
    assert_eq!(sync_legacy["policy_selection"]["source"], "legacy_default");
    assert_eq!(sync_legacy["policy_selection"]["blake3"], q_hash);
    assert_eq!(sync_legacy["blocked"], false);
    let archive_legacy = parse_ok(run_cmd(
        &project_legacy,
        Some(&q_path),
        &["archive", "ready"],
    )?)?;
    assert_eq!(
        archive_legacy["policy_selection"]["source"],
        "legacy_default"
    );
    assert_eq!(archive_legacy["policy_selection"]["blake3"], q_hash);
    assert_eq!(archive_legacy["blocked"], false);

    fs::remove_dir_all(root)?;
    Ok(())
}
