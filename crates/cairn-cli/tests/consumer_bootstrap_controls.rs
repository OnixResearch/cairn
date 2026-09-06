//! Preserve the original installed-consumer policy gap while setup is additive.
use std::{error::Error, fs, path::PathBuf, process::Command};

const POLICY_RELATIVE: &str = "cairn-policy/generated/cairn-policy.json";

// r[verify cairn.consumer_bootstrap.compatibility]
#[test]
fn nickel_bootstrap_contract_accepts_cohort_and_rejects_excessive_bound(
) -> Result<(), Box<dyn Error>> {
    use cairn_core::verified::consumer_bootstrap::{
        admit_cohort, BootstrapCohort, CohortObservations,
    };
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let export = |fixture: &str| {
        Command::new("nickel")
            .current_dir(&source)
            .args(["export", "--format", "json", fixture])
            .output()
    };
    let good = export("fixtures/consumer-bootstrap/cohort.ncl")?;
    assert!(
        good.status.success(),
        "{}",
        String::from_utf8_lossy(&good.stderr)
    );
    let cohort: BootstrapCohort = serde_json::from_slice(&good.stdout)?;
    let observed = CohortObservations {
        tool: cohort.tool.clone(),
        policy: cohort.policy.clone(),
        export: cohort.export.clone(),
        evaluator: cohort.evaluator.clone(),
        source: cohort.source.clone(),
        workflow: cohort.workflow.clone(),
        export_current: true,
    };
    assert_eq!(admit_cohort(&cohort, &observed, None), Ok(()));
    let records = export("fixtures/consumer-bootstrap/ownership-upgrade.ncl")?;
    assert!(
        records.status.success(),
        "{}",
        String::from_utf8_lossy(&records.stderr)
    );
    let records: serde_json::Value = serde_json::from_slice(&records.stdout)?;
    let _: cairn_core::verified::consumer_bootstrap::BootstrapOwnership =
        serde_json::from_value(records["ownership"].clone())?;
    let _: cairn_core::verified::consumer_bootstrap::BootstrapUpgrade =
        serde_json::from_value(records["upgrade"].clone())?;
    let rejected_ownership = export("fixtures/consumer-bootstrap/rejected-ownership.ncl")?;
    assert!(!rejected_ownership.status.success());
    assert!(String::from_utf8_lossy(&rejected_ownership.stderr).contains("contract broken"));
    let project = export("fixtures/consumer-bootstrap/project.ncl")?;
    assert!(
        project.status.success(),
        "{}",
        String::from_utf8_lossy(&project.stderr)
    );
    let declaration: cairn_core::verified::consumer_bootstrap::BootstrapProjectDeclaration =
        serde_json::from_slice(&project.stdout)?;
    declaration.validate()?;
    let complete = export("fixtures/consumer-bootstrap/complete-selection.ncl")?;
    assert!(
        complete.status.success(),
        "{}",
        String::from_utf8_lossy(&complete.stderr)
    );
    let complete: cairn_core::verified::consumer_bootstrap::BootstrapCompleteSelection =
        serde_json::from_slice(&complete.stdout)?;
    complete.validate_for(&declaration)?;
    let rejected_complete = export("fixtures/consumer-bootstrap/rejected-complete-selection.ncl")?;
    assert!(!rejected_complete.status.success());
    assert!(String::from_utf8_lossy(&rejected_complete.stderr).contains("contract broken"));
    let rejected_project = export("fixtures/consumer-bootstrap/rejected-project.ncl")?;
    assert!(!rejected_project.status.success());
    assert!(String::from_utf8_lossy(&rejected_project.stderr).contains("contract broken"));
    let rejected = export("fixtures/consumer-bootstrap/rejected-bound.ncl")?;
    assert!(!rejected.status.success());
    assert!(String::from_utf8_lossy(&rejected.stderr).contains("contract broken"));
    Ok(())
}

// r[verify cairn.consumer_bootstrap.conformance]
// r[verify cairn.consumer_bootstrap.boundary]
#[test]
fn low_level_init_preserves_missing_policy_and_explicit_policy_controls(
) -> Result<(), Box<dyn Error>> {
    let source = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let root =
        std::env::temp_dir().join(format!("cairn-bootstrap-controls-{}", std::process::id()));
    if root.exists() {
        fs::remove_dir_all(&root)?;
    }
    fs::create_dir_all(&root)?;
    let invoke = |args: &[&str]| {
        Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(&root)
            .env("HOME", &root)
            .env("XDG_DATA_HOME", root.join("data"))
            .env("CAIRN_DATA_DIR", root.join("data/cairn"))
            .env_remove("CAIRN_GLOBAL_REGISTRY")
            .args(args)
            .output()
    };
    let init = invoke(&["init", "--root", "."])?;
    assert!(
        init.status.success(),
        "{}",
        String::from_utf8_lossy(&init.stderr)
    );
    assert!(root.join(".cairn/specs").is_dir());
    assert!(!root.join(POLICY_RELATIVE).exists());
    let missing = invoke(&["validate", "--root", "."])?;
    assert!(!missing.status.success());
    assert!(String::from_utf8_lossy(&missing.stderr).contains("failed to read policy"));
    let explicit = source.join(POLICY_RELATIVE).canonicalize()?;
    let good = invoke(&[
        "validate",
        "--root",
        ".",
        "--policy",
        explicit.to_str().ok_or("policy path is not UTF-8")?,
    ])?;
    assert!(
        good.status.success(),
        "{}",
        String::from_utf8_lossy(&good.stderr)
    );
    fs::create_dir_all(root.join("cairn-policy/generated"))?;
    fs::copy(&explicit, root.join(POLICY_RELATIVE))?;
    fs::write(root.join("invalid-policy.json"), b"not JSON")?;
    let invalid = invoke(&["validate", "--root", ".", "--policy", "invalid-policy.json"])?;
    assert!(!invalid.status.success());
    assert!(String::from_utf8_lossy(&invalid.stderr).contains("failed to parse policy"));
    let valid_default = invoke(&["validate", "--root", "."])?;
    assert!(valid_default.status.success());
    fs::remove_dir_all(root)?;
    Ok(())
}
