use super::*;
use crate::support::observation_budget::{ObservationBudget, MAX_REPORT_BYTES, MAX_REPORT_ENTRIES};

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn lifecycle_root(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let root = std::env::temp_dir().join(format!(
        "cairn-drain-readiness-{name}-{}",
        std::process::id()
    ));
    let _ = std::fs::remove_dir_all(&root);
    copy_tree(&repository_root().join("fixtures/basic/.cairn"), &root.join(".cairn"))?;
    std::fs::create_dir_all(root.join(".cairn/archive"))?;
    Ok(root)
}

fn copy_tree(source: &Path, destination: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(destination)?;
    for entry in std::fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_tree(&entry.path(), &target)?;
        } else {
            std::fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn duplicate_change(root: &Path, source: &str, destination: &str) -> std::io::Result<()> {
    copy_tree(
        &root.join(".cairn/changes").join(source),
        &root.join(".cairn/changes").join(destination),
    )
}

// r[verify cairn.agent_workflow_readiness.shared]
#[test]
fn shared_budget_bounds_the_complete_multi_change_observation() -> Result<(), Box<dyn Error>> {
    let root = lifecycle_root("shared-budget")?;
    duplicate_change(&root, "demo", "second")?;
    let input = json!({"root": path_string(&root)});
    let measured = ObservationBudget::new();
    let baseline = readiness_drain_with_budget(&input, "drain-plan test", &measured)?;
    assert_eq!(baseline["active_change_count"], 2);
    assert_eq!(baseline["truncated"], false);
    let consumed = MAX_REPORT_BYTES - measured.read_limit(MAX_REPORT_BYTES);
    assert!(consumed > 0);

    let exact = ObservationBudget::with_limits(consumed, MAX_REPORT_ENTRIES);
    let reproduced = readiness_drain_with_budget(&input, "drain-plan test", &exact)?;
    assert_eq!(reproduced, baseline);
    assert_eq!(exact.read_limit(MAX_REPORT_BYTES), 0);

    let short = ObservationBudget::with_limits(consumed - 1, MAX_REPORT_ENTRIES);
    let degraded = readiness_drain_with_budget(&input, "drain-plan test", &short)?;
    assert_ne!(degraded, baseline);
    assert_eq!(degraded["active_change_count"], 2);
    assert_ne!(degraded["changes"], baseline["changes"]);
    for change in degraded["changes"].as_array().ok_or("changes")? {
        assert_eq!(change["readiness"]["mutation_authorized"], false);
    }
    std::fs::remove_dir_all(root)?;
    Ok(())
}

// r[verify cairn.agent_workflow_readiness.compatibility]
#[test]
fn manifest_admission_requires_exact_active_change_keys_and_complete_selectors(
) -> Result<(), Box<dyn Error>> {
    let root = lifecycle_root("manifest-admission")?;
    let policy = cairn_core::workflow_readiness_policy().map_err(|code| {
        format!("{code}: generated workflow readiness policy is not admitted")
    })?;
    let status = scan_repo_with_budget(root.clone(), detect_layout(&root).layout, None)?;
    let budget = ObservationBudget::new();
    let manifest = |changes: Value| EvidenceManifest {
        schema: policy.manifest_schema.clone(),
        changes: serde_json::from_value(changes).expect("manifest entries"),
    };
    let error = |manifest: EvidenceManifest| {
        admit_evidence_manifest(manifest, &status.changes, policy)
            .expect_err("invalid manifest")
            .to_string()
    };
    assert!(error(manifest(json!([{"change": "absent"}]))).contains(MANIFEST_UNKNOWN_CHANGE));
    assert!(error(manifest(json!([{"change": "demo"}, {"change": "demo"}])))
        .contains(MANIFEST_DUPLICATE_CHANGE));
    assert!(error(manifest(
        json!([{"change": "demo", "contract": "contract.json"}])
    ))
    .contains(MANIFEST_PARTIAL_COMPLETION));
    assert!(error(manifest(
        json!([{"change": "demo", "contract": "../escape.json", "results": "results.json", "subject_hash": "c"}])
    ))
    .contains(MANIFEST_UNSAFE_PATH));
    assert!(error(manifest(json!([{"change": "demo", "cross_repo_evidence": ["/etc/passwd"]}])))
        .contains(MANIFEST_UNSAFE_PATH));
    assert!(error(EvidenceManifest {
        schema: "cairn.workflow-readiness-manifest.v0".into(),
        changes: serde_json::from_value(json!([{"change": "demo"}])).expect("entries"),
    })
    .contains(MANIFEST_SCHEMA));

    let admitted = admit_evidence_manifest(
        manifest(json!([{
            "change": "demo",
            "contract": "contract.json",
            "results": "results.json",
            "subject_hash": "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
            "cross_repo_evidence": ["upstream.json"],
        }])),
        &status.changes,
        policy,
    )?;
    assert_eq!(admitted.len(), 1);
    assert!(admitted["demo"].contract.is_some());
    assert_eq!(admitted["demo"].cross_repo_evidence, vec!["upstream.json"]);

    std::fs::write(
        root.join("manifest.json"),
        serde_json::to_vec(&json!({
            "schema": policy.manifest_schema,
            "changes": [{"change": "demo"}, {"unexpected": true}],
        }))?,
    )?;
    let error = load_evidence_manifest(
        &root,
        "manifest.json",
        &status.changes,
        policy,
        &budget,
    )
    .expect_err("unknown manifest field")
    .to_string();
    assert!(error.contains(MANIFEST_INVALID), "{error}");
    std::fs::remove_dir_all(root)?;
    Ok(())
}

// r[verify cairn.agent_workflow_readiness.compatibility]
#[test]
fn manifest_selection_matches_direct_readiness_and_absent_evidence_stays_unavailable(
) -> Result<(), Box<dyn Error>> {
    let root = lifecycle_root("direct-parity")?;
    for file in ["contract.json", "results.json"] {
        std::fs::copy(
            repository_root().join("fixtures/agent-completion").join(file),
            root.join(file),
        )?;
    }
    let subject = "c".repeat(64);
    std::fs::write(
        root.join("manifest.json"),
        serde_json::to_vec(&json!({
            "schema": cairn_core::workflow_readiness_policy().expect("policy").manifest_schema,
            "changes": [{
                "change": "demo",
                "contract": "contract.json",
                "results": "results.json",
                "subject_hash": subject,
            }],
        }))?,
    )?;
    let direct_input = json!({
        "root": path_string(&root),
        "change": "demo",
        "contract": "contract.json",
        "results": "results.json",
        "subject_hash": subject,
    });
    let direct = crate::readiness::readiness_value(&direct_input)?;
    assert_eq!(direct["stages"][4]["state"], "ready");
    let value = readiness_drain_with_budget(
        &json!({"root": path_string(&root), "view": "readiness", "evidence_manifest": "manifest.json"}),
        "drain-plan test",
        &ObservationBudget::new(),
    )?;
    let change = &value["changes"][0];
    assert_eq!(change["name"], "demo");
    assert_eq!(change["evidence"]["declared"], true);
    assert_eq!(change["evidence"]["completion_inputs"], true);
    assert_eq!(change["readiness"], direct);
    assert_eq!(change["classification"], direct["classification"]);

    let no_manifest = readiness_drain_with_budget(
        &json!({"root": path_string(&root), "view": "readiness"}),
        "drain-plan test",
        &ObservationBudget::new(),
    )?;
    let unavailable = &no_manifest["changes"][0];
    assert_eq!(unavailable["evidence"]["declared"], false);
    assert_eq!(unavailable["evidence"]["completion_inputs"], false);
    assert_eq!(unavailable["readiness"]["stages"][4]["state"], "unavailable");
    assert!(no_manifest["next_actions"]
        .as_array()
        .ok_or("next actions")?
        .iter()
        .all(|action| action["kind"] != "sync_preview" && action["kind"] != "archive_preview"));
    std::fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn manifest_paths_must_be_repository_relative_and_bounded() {
    assert!(manifest_path_is_safe("contract.json", 4096));
    assert!(manifest_path_is_safe("nested/results.json", 4096));
    assert!(!manifest_path_is_safe("../outside.json", 4096));
    assert!(!manifest_path_is_safe("/absolute.json", 4096));
    assert!(!manifest_path_is_safe("nested\\windows.json", 4096));
    assert!(!manifest_path_is_safe("", 4096));
    assert!(!manifest_path_is_safe(&"a".repeat(4097), 4096));
}
