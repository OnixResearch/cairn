//! Saved producer observations are fixtures, not executed Nickel evidence.
#[path = "bootstrap_preview/apply.rs"]
mod apply;
#[path = "bootstrap_preview/material.rs"]
mod material;
use cairn_core::verified::consumer_bootstrap::BootstrapSourceBundle;
use material::{id, POLICY_DESTINATION};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

fn bundle(root: &Path) -> Result<BootstrapSourceBundle, Box<dyn Error>> {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let policy = fs::read(repo.join("cairn-policy/generated/cairn-policy.json"))?;
    bundle_material(root, policy)
}

fn bundle_material(root: &Path, policy: Vec<u8>) -> Result<BootstrapSourceBundle, Box<dyn Error>> {
    material::bundle_material(root, policy, &fs::read(env!("CARGO_BIN_EXE_cairn"))?)
}

fn invoke(root: &Path, sources: &Path, extra: &[&str]) -> Result<Output, Box<dyn Error>> {
    setup(root, sources, "plan", extra)
}

fn setup(
    root: &Path,
    sources: &Path,
    command: &str,
    extra: &[&str],
) -> Result<Output, Box<dyn Error>> {
    Ok(Command::new(env!("CARGO_BIN_EXE_cairn"))
        .env_clear()
        .current_dir(root)
        .env("HOME", root)
        .env("CAIRN_DATA_DIR", root.join("data"))
        .args(["setup", command, "--root"])
        .arg(root)
        .arg("--source-root")
        .arg(sources)
        .args(["--bundle", "bundle.json"])
        .args(extra)
        .output()?)
}

// r[verify cairn.consumer_bootstrap.boundary]
// r[verify cairn.consumer_bootstrap.compatibility]
#[test]
fn preview_binds_observations_without_authorizing_installation() -> Result<(), Box<dyn Error>> {
    let temp = std::env::temp_dir().join(format!("cairn-bootstrap-preview-{}", std::process::id()));
    fs::create_dir(&temp)?;
    let root = temp.join("consumer");
    let sources = temp.join("sources");
    fs::create_dir(&root)?;
    fs::create_dir(&sources)?;
    let bundle = bundle(&sources)?;
    let bytes = serde_json::to_vec(&bundle)?;
    fs::write(sources.join("bundle.json"), &bytes)?;
    let good = invoke(&root, &sources, &[])?;
    assert!(
        good.status.success(),
        "{}",
        String::from_utf8_lossy(&good.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&good.stdout)?;
    assert_eq!(result["valid"], true);
    assert_eq!(result["mutated"], false);
    assert_eq!(result["installation_receipt"], false);
    assert_eq!(result["execution_supported"], true);
    assert_eq!(result["artifacts"][0]["action"], "create");
    assert_eq!(result["policy"], bundle.cohort.policy.as_str());
    assert_eq!(result["producer_non_claim"], nickel_export_core::NON_CLAIM);
    assert_eq!(invoke(&root, &sources, &[])?.stdout, good.stdout);
    assert_eq!(fs::read_dir(&root)?.count(), 0);
    for args in [
        vec!["--execute"],
        vec!["--bundle", "bundle.json"],
        vec!["--unknown"],
    ] {
        assert!(!invoke(&root, &sources, &args)?.status.success());
        assert_eq!(fs::read_dir(&root)?.count(), 0);
    }
    let mut bad = bundle.clone();
    bad.cohort.tool = id(b"other tool");
    fs::write(sources.join("bundle.json"), serde_json::to_vec(&bad)?)?;
    assert!(!invoke(&root, &sources, &[])?.status.success());
    bad = bundle.clone();
    bad.cohort.profile = "unknown".into();
    fs::write(sources.join("bundle.json"), serde_json::to_vec(&bad)?)?;
    assert!(!invoke(&root, &sources, &[])?.status.success());
    fs::write(sources.join("bundle.json"), &bytes)?;
    let original = fs::read(sources.join("policy.ncl"))?;
    fs::write(sources.join("policy.ncl"), b"changed")?;
    let stale = invoke(&root, &sources, &[])?;
    assert!(!stale.status.success());
    assert!(String::from_utf8_lossy(&stale.stderr).contains("bootstrap.export.stale"));
    fs::write(sources.join("policy.ncl"), original)?;
    fs::create_dir_all(root.join(".cairn/bootstrap"))?;
    fs::write(root.join(POLICY_DESTINATION), b"unowned")?;
    assert!(!invoke(&root, &sources, &[])?.status.success());
    assert_eq!(fs::read(root.join(POLICY_DESTINATION))?, b"unowned");
    fs::remove_file(root.join(POLICY_DESTINATION))?;
    fs::write(root.join(".cairn/bootstrap/prepared.json"), b"{}")?;
    assert!(!invoke(&root, &sources, &[])?.status.success());
    fs::remove_file(root.join(".cairn/bootstrap/prepared.json"))?;
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(sources.join("policy.json"), root.join(POLICY_DESTINATION))?;
        assert!(!invoke(&root, &sources, &[])?.status.success());
        fs::remove_file(root.join(POLICY_DESTINATION))?;
        fs::rename(
            sources.join("policy.json"),
            sources.join("real-policy.json"),
        )?;
        std::os::unix::fs::symlink("real-policy.json", sources.join("policy.json"))?;
        assert!(!invoke(&root, &sources, &[])?.status.success());
    }
    fs::remove_dir_all(temp)?;
    Ok(())
}
