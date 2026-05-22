use super::*;

pub(super) fn evidence_registry_consume_registry(
    root: &Path,
    registry_path: &Path,
    prefix: &'static str,
) -> (Value, Value, Vec<Value>) {
    let mut issues = Vec::new();
    let registry = match read_json_artifact(registry_path, AGENT_RECEIPT_MAX_BYTES) {
        Ok(value) => value,
        Err(_) => {
            issues.push(evidence_registry_issue(
                &format!("{prefix}.registry_unreadable"),
                "registry",
                "readable registry",
                "unreadable",
            ));
            return (Value::Null, Value::Null, issues);
        }
    };
    let verification = evidence_registry_verify_receipt(&EvidenceRegistryVerifyOptions {
        root: root.to_path_buf(),
        registry: Some(registry_path.to_path_buf()),
    });
    if verification.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_registry_issue(
            &format!("{prefix}.registry_verify_failed"),
            "registry",
            "valid registry verification",
            "invalid",
        ));
    }
    (registry, verification, issues)
}
