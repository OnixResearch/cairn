use super::*;
use cairn_core::verified::consumer_bootstrap::cohort_identity;
use serde_json::Value;

fn receipt(output: Output) -> Result<Value, Box<dyn Error>> {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout)?;
    println!(
        "bootstrap_consumer_receipt={}",
        serde_json::to_string(&value)?
    );
    Ok(value)
}
fn denied(output: Output, category: &str) {
    assert!(!output.status.success());
    let diagnostic = String::from_utf8_lossy(&output.stderr);
    assert!(diagnostic.contains(category), "{diagnostic}");
}
fn ordinary(root: &Path, command: &str) -> Result<Output, Box<dyn Error>> {
    Ok(Command::new(env!("CARGO_BIN_EXE_cairn"))
        .env_clear()
        .current_dir(root)
        .env("HOME", root)
        .env("CAIRN_DATA_DIR", root.join("data"))
        .args([command, "--root"])
        .arg(root)
        .output()?)
}
fn inspect(root: &Path, command: &str) -> Result<Value, Box<dyn Error>> {
    receipt(
        Command::new(env!("CARGO_BIN_EXE_cairn"))
            .env_clear()
            .current_dir(root)
            .env("HOME", root)
            .args(["setup", command, "--root"])
            .arg(root)
            .output()?,
    )
}

// r[verify cairn.consumer_bootstrap.publication]
// r[verify cairn.consumer_bootstrap.policy]
// r[verify cairn.consumer_bootstrap.conformance]
// r[verify cairn.consumer_bootstrap.guidance]
#[test]
fn fresh_apply_upgrade_and_recovery_controls() -> Result<(), Box<dyn Error>> {
    let temp = std::env::temp_dir().join(format!("cairn-bootstrap-apply-{}", std::process::id()));
    fs::create_dir(&temp)?;
    let root = temp.join("empty");
    let sources = temp.join("source");
    fs::create_dir(&root)?;
    fs::create_dir(&sources)?;
    let mut bundle = bundle(&sources)?;
    fs::write(sources.join("bundle.json"), serde_json::to_vec(&bundle)?)?;
    let preview = receipt(invoke(&root, &sources, &[])?)?;
    assert_eq!(fs::read_dir(&root)?.count(), 0);
    let hash = preview["plan_hash"].as_str().ok_or("plan hash")?;
    assert!(!setup(&root, &sources, "apply", &["--execute"])?
        .status
        .success());
    assert!(!setup(
        &root,
        &sources,
        "apply",
        &["--execute", "--plan-hash", &"0".repeat(64)]
    )?
    .status
    .success());
    assert_eq!(fs::read_dir(&root)?.count(), 0);
    // A previously valid token must fail even when the changed source bundle
    // remains semantically admissible (only its exact serialization changed).
    fs::write(
        sources.join("bundle.json"),
        serde_json::to_vec_pretty(&bundle)?,
    )?;
    let changed = receipt(invoke(&root, &sources, &[])?)?;
    assert_eq!(changed["valid"], true);
    assert_ne!(changed["plan_hash"], preview["plan_hash"]);
    let stale = setup(
        &root,
        &sources,
        "apply",
        &["--execute", "--plan-hash", hash],
    )?;
    assert!(!stale.status.success());
    assert!(String::from_utf8_lossy(&stale.stderr).contains("bootstrap.setup.stale_plan"));
    assert_eq!(fs::read_dir(&root)?.count(), 0);
    fs::write(sources.join("bundle.json"), serde_json::to_vec(&bundle)?)?;
    let original_source = fs::read(sources.join("policy.ncl"))?;
    fs::write(sources.join("policy.ncl"), b"changed source")?;
    let stale = setup(
        &root,
        &sources,
        "apply",
        &["--execute", "--plan-hash", hash],
    )?;
    assert!(!stale.status.success());
    assert!(String::from_utf8_lossy(&stale.stderr).contains("bootstrap.export.stale"));
    assert_eq!(fs::read_dir(&root)?.count(), 0);
    fs::write(sources.join("policy.ncl"), original_source)?;
    let original_root = temp.join("original-root");
    fs::rename(&root, &original_root)?;
    fs::create_dir(&root)?;
    let substituted = receipt(invoke(&root, &sources, &[])?)?;
    assert_ne!(substituted["root_identity"], preview["root_identity"]);
    let stale = setup(
        &root,
        &sources,
        "apply",
        &["--execute", "--plan-hash", hash],
    )?;
    assert!(!stale.status.success());
    assert!(String::from_utf8_lossy(&stale.stderr).contains("bootstrap.setup.stale_plan"));
    assert_eq!(fs::read_dir(&root)?.count(), 0);
    assert_eq!(fs::read_dir(&original_root)?.count(), 0);
    fs::remove_dir(&root)?;
    fs::rename(original_root, &root)?;
    assert_eq!(
        receipt(invoke(&root, &sources, &[])?)?["plan_hash"],
        preview["plan_hash"]
    );
    fs::write(root.join(".cairn-bootstrap.lock"), b"another owner")?;
    assert!(!setup(
        &root,
        &sources,
        "apply",
        &["--execute", "--plan-hash", hash]
    )?
    .status
    .success());
    fs::remove_file(root.join(".cairn-bootstrap.lock"))?;
    let applied = receipt(setup(
        &root,
        &sources,
        "apply",
        &["--execute", "--plan-hash", hash],
    )?)?;
    assert_eq!(applied["installation_receipt"], true);
    assert_eq!(applied["durability"], "parent_synchronized");
    assert_eq!(inspect(&root, "check")?["valid"], true);
    let doctor = receipt(ordinary(&root, "doctor")?)?;
    assert_eq!(doctor["valid"], true);
    assert_eq!(doctor["bootstrap_guidance"]["valid"], true);
    let declaration_path = root.join(".cairn/bootstrap/guidance.json");
    let declaration_bytes = fs::read(&declaration_path)?;
    fs::remove_file(&declaration_path)?;
    assert_eq!(receipt(ordinary(&root, "doctor")?)?["valid"], false);
    fs::write(&declaration_path, b"{}")?;
    assert_eq!(receipt(ordinary(&root, "doctor")?)?["valid"], false);
    fs::write(&declaration_path, declaration_bytes)?;
    let valid = receipt(ordinary(&root, "validate")?)?;
    assert_eq!(valid["valid"], true);
    assert_eq!(valid["policy_selection"]["source"], "project");
    let guidance_path = preview["guidance"]["files"][0]["path"]
        .as_str()
        .ok_or("guidance path")?;
    let guidance_bytes = fs::read(root.join(guidance_path))?;
    let unmanaged_root = temp.join("unmanaged-guidance");
    let unmanaged_file = unmanaged_root.join(guidance_path);
    fs::create_dir_all(unmanaged_file.parent().ok_or("guidance parent")?)?;
    fs::write(&unmanaged_file, b"unowned guidance")?;
    denied(
        invoke(&unmanaged_root, &sources, &[])?,
        "bootstrap.guidance.manual_repair",
    );
    assert_eq!(fs::read(&unmanaged_file)?, b"unowned guidance");
    assert!(!unmanaged_root
        .join(".cairn/bootstrap/selection.json")
        .exists());
    fs::write(root.join(guidance_path), b"modified guidance")?;
    assert_eq!(receipt(ordinary(&root, "doctor")?)?["valid"], false);
    denied(
        invoke(&root, &sources, &[])?,
        "bootstrap.state.modified_managed",
    );
    assert_eq!(fs::read(root.join(guidance_path))?, b"modified guidance");
    fs::remove_file(root.join(guidance_path))?;
    denied(
        invoke(&root, &sources, &[])?,
        "bootstrap.state.missing_or_unsafe_managed",
    );
    assert!(!root.join(guidance_path).exists());
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(sources.join("policy.json"), root.join(guidance_path))?;
        denied(
            invoke(&root, &sources, &[])?,
            "bootstrap.state.missing_or_unsafe_managed",
        );
        assert!(fs::symlink_metadata(root.join(guidance_path))?.is_symlink());
        assert_eq!(receipt(ordinary(&root, "doctor")?)?["valid"], false);
        fs::remove_file(root.join(guidance_path))?;
    }
    fs::write(root.join(guidance_path), guidance_bytes)?;
    let policy_bytes = fs::read(root.join(POLICY_DESTINATION))?;
    let complete_bytes = fs::read(root.join(".cairn/bootstrap/complete.json"))?;
    fs::write(root.join(POLICY_DESTINATION), b"consumer modified")?;
    assert!(!invoke(&root, &sources, &[])?.status.success());
    assert_eq!(
        fs::read(root.join(".cairn/bootstrap/complete.json"))?,
        complete_bytes
    );
    fs::write(root.join(POLICY_DESTINATION), &policy_bytes)?;
    // Consumer-owned policy is outside the managed set and survives a declared upgrade.
    fs::write(
        root.join("consumer-policy.ncl"),
        b"{ consumer_owned = true }",
    )?;
    let prior = cohort_identity(&bundle.cohort);
    let mut updated_policy: Value = serde_json::from_slice(&policy_bytes)?;
    updated_policy["name"] = Value::String("consumer-upgrade-policy".into());
    bundle = bundle_material(&sources, serde_json::to_vec(&updated_policy)?)?;
    bundle.cohort.upgrade_from = vec![prior];
    const NEXT_POLICY: &str = ".cairn/bootstrap/policy-v2.json";
    bundle.policy_locator = NEXT_POLICY.into();
    bundle.artifacts[0].destination = NEXT_POLICY.into();
    bundle.cohort.managed_locators = vec![id(NEXT_POLICY.as_bytes())];
    fs::write(sources.join("bundle.json"), serde_json::to_vec(&bundle)?)?;
    let upgrade = receipt(invoke(&root, &sources, &[])?)?;
    assert_eq!(upgrade["operation"], "upgrade");
    assert_eq!(upgrade["artifacts"][0]["action"], "create");
    assert_eq!(upgrade["guidance"]["manifest_freshness"], "stale-source");
    let hash = upgrade["plan_hash"].as_str().ok_or("upgrade hash")?;
    assert_eq!(
        receipt(setup(
            &root,
            &sources,
            "apply",
            &["--execute", "--plan-hash", hash]
        )?)?["valid"],
        true
    );
    assert_eq!(
        fs::read(root.join("consumer-policy.ncl"))?,
        b"{ consumer_owned = true }"
    );
    assert_eq!(receipt(ordinary(&root, "validate")?)?["valid"], true);
    assert_eq!(fs::read(root.join(POLICY_DESTINATION))?, policy_bytes);
    assert_eq!(
        receipt(ordinary(&root, "doctor")?)?["bootstrap_guidance"]["valid"],
        true
    );
    let selected_before_recovery = fs::read(root.join(".cairn/bootstrap/selection.json"))?;
    let complete_before_recovery = fs::read(root.join(".cairn/bootstrap/complete.json"))?;
    fs::write(root.join(".cairn/bootstrap/prepared.json"), b"interrupted")?;
    for command in ["check", "recover", "recover"] {
        let state = inspect(&root, command)?;
        assert_eq!(state["valid"], false);
        assert_eq!(state["category"], "bootstrap.recovery.manual_repair");
        assert_eq!(state["blocked"], true);
        assert_eq!(state["mutated"], false);
        assert_eq!(state["installation_receipt"], false);
        assert_eq!(
            fs::read(root.join(".cairn/bootstrap/selection.json"))?,
            selected_before_recovery
        );
        assert_eq!(
            fs::read(root.join(".cairn/bootstrap/complete.json"))?,
            complete_before_recovery
        );
    }
    assert!(!ordinary(&root, "validate")?.status.success());
    assert!(!invoke(&root, &sources, &[])?.status.success());
    assert_eq!(
        fs::read(root.join(".cairn/bootstrap/prepared.json"))?,
        b"interrupted"
    );
    fs::remove_dir_all(temp)?;
    Ok(())
}
