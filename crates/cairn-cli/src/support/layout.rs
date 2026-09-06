use super::*;

#[derive(Debug, Clone, Copy)]
pub(crate) struct SelectedLayout {
    pub(crate) layout: LifecycleLayout,
}

pub(crate) fn detect_layout(_root: &Path) -> SelectedLayout {
    SelectedLayout {
        layout: LifecycleLayout::Native,
    }
}

fn lifecycle_subdirectory_is_present(root: &Path, layout: LifecycleLayout) -> bool {
    [
        specs_dir(layout),
        changes_dir(layout),
        archive_dir(layout).to_string(),
    ]
    .iter()
    .any(|relative| root.join(relative).is_dir())
}

pub(crate) fn legacy_only_lifecycle_root(root: &Path) -> bool {
    // r[impl cairn.lifecycle_directory.legacy_diagnostic]
    lifecycle_subdirectory_is_present(root, LifecycleLayout::LegacyCairn)
        && !lifecycle_subdirectory_is_present(root, LifecycleLayout::Native)
}

pub(crate) fn reject_legacy_only_lifecycle_root(root: &Path) -> Result<(), Box<dyn Error>> {
    if legacy_only_lifecycle_root(root) {
        return Err(format!(
            "cairn.lifecycle_directory.migration_required: {} contains old cairn/ lifecycle content but no native .cairn/ lifecycle content; preview `cairn migrate cairn --root {}`",
            root.display(),
            root.display()
        )
        .into());
    }
    Ok(())
}

const MAX_POLICY_BYTES: u64 = 16 * 1024 * 1024;

pub(crate) fn load_policy_file(path: PathBuf) -> Result<Policy, Box<dyn Error>> {
    load_policy_file_with_budget(path, None)
}

pub(crate) fn load_policy_file_with_budget(
    path: PathBuf,
    budget: Option<&super::observation_budget::ObservationBudget>,
) -> Result<Policy, Box<dyn Error>> {
    let parent = path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let leaf = path.file_name().ok_or("policy path has no file name")?;
    let capability =
        CapabilityRoot::open_nofollow(CapabilityRootKind::Scan, parent)?.with_budget(budget);
    let bytes = capability
        .read_bounded(Path::new(leaf), MAX_POLICY_BYTES)
        .map_err(|source| format!("failed to read policy {}: {source}", path.display()))?;
    let value: Value = serde_json::from_slice(&bytes)
        .map_err(|source| format!("failed to parse policy {}: {source}", path.display()))?;
    validate_policy_json(value.clone()).map_err(|source| {
        let diagnostic = policy_schema_compatibility_diagnostic(&value, &source)
            .unwrap_or_else(|| source.to_string());
        format!("failed to parse policy {}: {diagnostic}", path.display()).into()
    })
}

pub(crate) fn default_policy_path() -> &'static str {
    "cairn-policy/generated/cairn-policy.json"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn policy_admission_accepts_exact_bound_and_rejects_overflow() -> Result<(), Box<dyn Error>> {
        let root = std::env::temp_dir().join(format!("cairn-policy-bound-{}", std::process::id()));
        fs::create_dir_all(root.join("child"))?;
        let path = root.join("policy.json");
        let mut bytes =
            include_bytes!("../../../../cairn-policy/generated/cairn-policy.json").to_vec();
        bytes.resize(MAX_POLICY_BYTES as usize, b' ');
        fs::write(&path, &bytes)?;
        assert!(load_policy_file(root.join("child/../policy.json")).is_ok());
        bytes.push(b' ');
        fs::write(&path, bytes)?;
        assert!(load_policy_file(path).is_err());
        fs::remove_dir_all(root)?;
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn policy_admission_rejects_linked_leaf_parent_and_link_parent_cancellation(
    ) -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;
        let root = std::env::temp_dir().join(format!("cairn-policy-links-{}", std::process::id()));
        fs::create_dir_all(root.join("real"))?;
        let policy = include_bytes!("../../../../cairn-policy/generated/cairn-policy.json");
        fs::write(root.join("real/policy.json"), policy)?;
        fs::write(root.join("policy.json"), policy)?;
        symlink("real", root.join("alias"))?;
        symlink("real/policy.json", root.join("leaf"))?;
        assert!(load_policy_file(root.join("real/policy.json")).is_ok());
        for path in ["alias/policy.json", "alias/../policy.json", "leaf"] {
            assert!(load_policy_file(root.join(path)).is_err(), "{path}");
        }
        fs::remove_dir_all(root)?;
        Ok(())
    }
}
