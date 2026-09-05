use std::error::Error;

use serde_json::Value;

const EXPECTED_REVISION: &str = "2f8213a88781d9d38bb81319d06a50805d48ccd3";
const EXPECTED_NAR_HASH: &str = "sha256-OvqB00mDIsGDOcuK5jvPJUModVz9WbIBuM/l06TABHc=";
const WORKSPACE_MANIFEST: &str = include_str!("../../../Cargo.toml");
const LINT_CONFIG: &str = include_str!("../../../dylint.toml");
const NIX_INTEGRATION: &str = include_str!("../../../nix/octet-integration.nix");

fn reviewed_source(source: &Value) -> bool {
    source["owner"] == "OnixResearch"
        && source["repo"] == "octet"
        && source["rev"] == EXPECTED_REVISION
        && source["narHash"] == EXPECTED_NAR_HASH
}

#[test]
fn committed_octet_lock_preserves_the_reviewed_source() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.octet_integration.canonical]
    // r[verify cairn.octet_integration.canonical.valid]
    // r[verify cairn.octet_integration.evidence]
    // r[verify cairn.octet_integration.evidence.valid]
    let lock: Value = serde_json::from_str(include_str!("../../../flake.lock"))?;
    let inputs = &lock["nodes"]["root"]["inputs"];
    assert!(inputs.get("slotcar").is_none());
    let node_name = inputs["octet"].as_str().ok_or("direct Octet input")?;
    let node = &lock["nodes"][node_name];
    assert!(reviewed_source(&node["locked"]));
    assert_eq!(node["original"]["rev"], EXPECTED_REVISION);
    assert_eq!(node["original"]["repo"], "octet");
    Ok(())
}

#[test]
fn legacy_and_changed_sources_do_not_match_the_reviewed_source() {
    // r[verify cairn.octet_integration.canonical.legacy]
    let source = serde_json::json!({
        "owner": "OnixResearch",
        "repo": "octet",
        "rev": EXPECTED_REVISION,
        "narHash": EXPECTED_NAR_HASH
    });
    assert!(reviewed_source(&source));
    for (field, changed) in [
        ("owner", "unowned"),
        ("repo", "slotcar"),
        ("rev", "changed-revision"),
        ("narHash", "changed-source"),
    ] {
        let mut candidate = source.clone();
        candidate[field] = Value::String(changed.into());
        assert!(!reviewed_source(&candidate), "accepted drift in {field}");
    }
    assert!(!reviewed_source(&Value::Null));
}

#[test]
fn current_config_uses_octet_tables() {
    assert!(WORKSPACE_MANIFEST.contains("[workspace.metadata.octet]"));
    assert!(!WORKSPACE_MANIFEST.contains("[workspace.metadata.slotcar]"));
    assert!(LINT_CONFIG.contains("[octet]"));
    assert!(LINT_CONFIG.contains("[octet.lint_levels]"));
    assert!(!LINT_CONFIG.contains("[slotcar"));
}

#[test]
fn integration_controls_do_not_claim_producer_execution() {
    // r[verify cairn.octet_integration.evidence.blocked]
    // This checks the declared output boundary. The recorded Nix build owns
    // the actual blocked result, not this source-level regression control.
    assert!(NIX_INTEGRATION.contains("not Octet execution or Rust correctness"));
}
