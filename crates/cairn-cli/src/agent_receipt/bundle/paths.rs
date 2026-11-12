use std::path::{Path, PathBuf};

use serde_json::Value;

use super::issues::agent_bundle_issue;

fn safe_bundle_path(text: &str, base: &Path) -> Option<PathBuf> {
    let path = PathBuf::from(text);
    if path.is_absolute() {
        Some(path)
    } else if crate::support::is_safe_relative_reference(text) {
        Some(base.join(path))
    } else {
        None
    }
}

pub(super) fn bundle_artifact_path(
    bundle: &Value,
    base: &Path,
    field: &str,
    issues: &mut Vec<Value>,
) -> Option<PathBuf> {
    match bundle.get(field).and_then(Value::as_str) {
        Some(text) if !text.is_empty() => match safe_bundle_path(text, base) {
            Some(path) => Some(path),
            None => {
                issues.push(agent_bundle_issue(
                    "bundle.path_unsafe",
                    field,
                    "safe relative path or absolute path",
                    "unsafe",
                ));
                None
            }
        },
        _ => {
            issues.push(agent_bundle_issue(
                "bundle.missing_field",
                field,
                "non-empty string",
                "missing",
            ));
            None
        }
    }
}

pub(super) fn optional_bundle_artifact_path(
    bundle: &Value,
    base: &Path,
    field: &str,
    issues: &mut Vec<Value>,
) -> Option<PathBuf> {
    match bundle.get(field) {
        None | Some(Value::Null) => None,
        Some(value) => match value.as_str() {
            Some(text) if !text.is_empty() => match safe_bundle_path(text, base) {
                Some(path) => Some(path),
                None => {
                    issues.push(agent_bundle_issue(
                        "bundle.path_unsafe",
                        field,
                        "safe relative path or absolute path",
                        "unsafe",
                    ));
                    None
                }
            },
            _ => {
                issues.push(agent_bundle_issue(
                    "bundle.invalid_field",
                    field,
                    "non-empty string",
                    "invalid",
                ));
                None
            }
        },
    }
}
