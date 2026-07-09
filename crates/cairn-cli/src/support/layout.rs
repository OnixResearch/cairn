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

pub(crate) fn load_policy_file(path: PathBuf) -> Result<Policy, Box<dyn Error>> {
    let text = fs::read_to_string(&path)
        .map_err(|source| format!("failed to read policy {}: {source}", path.display()))?;
    let value: Value = serde_json::from_str(&text)
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
