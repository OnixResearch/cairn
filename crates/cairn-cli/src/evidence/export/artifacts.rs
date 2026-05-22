use super::issue::evidence_export_issue;
use super::*;

pub(super) fn evidence_export_artifacts(out_dir: &Path, issues: &mut Vec<Value>) -> Vec<Value> {
    let roles = [
        ("agent_evidence_bundle", "agent-evidence-bundle.json", true),
        (
            "agent_evidence_bundle_verification",
            "agent-evidence-bundle-verification.json",
            true,
        ),
        ("agent_receipt", "agent-receipt.json", true),
        ("coverage_audit", "coverage-audit.json", true),
        ("evidence_manifest", "evidence-manifest.json", true),
        (
            "evidence_manifest_verification",
            "evidence-manifest-verification.json",
            true,
        ),
        ("release_readiness", "release-readiness.json", true),
        ("replay_receipt", "replay-agent.json", true),
        ("replay_verification", "replay-verification.json", true),
    ];
    roles
        .iter()
        .map(|(role, relative, required)| {
            let path = out_dir.join(relative);
            let hash = match fs::read(&path) {
                Ok(bytes) => Value::String(blake3_hex(&bytes)),
                Err(_) => {
                    issues.push(evidence_export_issue(
                        "export.artifact_missing",
                        role,
                        "generated artifact file",
                        "missing",
                    ));
                    Value::Null
                }
            };
            json!({
                "role": role,
                "path": relative,
                "hash": hash,
                "required": required,
            })
        })
        .collect()
}
