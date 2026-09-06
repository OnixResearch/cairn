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
