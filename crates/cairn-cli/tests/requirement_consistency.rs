use std::{
    collections::BTreeMap, env, fs, path::Path, path::PathBuf, process::Command, time::SystemTime,
};

use cairn_core::{
    evaluate_content_bound_requirement_registry, ContentBoundRevision,
    ContentBoundSpecificationInput, ConsistencyCase, ConsistencyPolicy,
};
use serde_json::{json, Value};

const REQUIREMENT_ID: &str = "cairn.supersede.agent_run_receipts.verification_contract";
const INVENTORY: &str = include_str!("../../../cairn-policy/generated/typed-requirement-contracts.json");
const ANALYSIS_POLICY: &str = r#"{
  "schema": "cairn.requirement-consistency-policy.v1",
  "queries": [
    "assumptions_satisfiable",
    "activation_reachable",
    "guarantees_compatible",
    "guarantee_constraining"
  ],
  "bounds": {
    "max_contracts": 8,
    "max_guarantees": 16,
    "max_cells": 4096,
    "max_states": 250000,
    "max_evaluations": 4000000,
    "max_depth": 32
  }
}"#;

struct Fixture {
    root: PathBuf,
    case: PathBuf,
    registry: PathBuf,
    lifecycle_policy: PathBuf,
}

fn tree(root: &Path) -> BTreeMap<PathBuf, u64> {
    fn walk(root: &Path, current: &Path, into: &mut BTreeMap<PathBuf, u64>) {
        let Ok(entries) = fs::read_dir(current) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let Ok(metadata) = entry.metadata() else {
                continue;
            };
            if metadata.is_dir() {
                walk(root, &path, into);
            } else {
                let relative = path.strip_prefix(root).unwrap_or(&path).to_path_buf();
                into.insert(relative, metadata.len());
            }
        }
    }
    let mut into = BTreeMap::new();
    walk(root, root, &mut into);
    into
}

impl Fixture {
    fn new() -> Self {
        let root = env::temp_dir().join(format!(
            "cairn-requirement-consistency-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let markdown = include_str!("../../../.cairn/specs/agent-completion/spec.md");
        let specification = ".cairn/specs/agent-completion/spec.md".to_string();
        fs::create_dir_all(root.join(".cairn/specs/agent-completion")).unwrap();
        fs::write(root.join(&specification), markdown).unwrap();
        fs::write(root.join("policy.json"), b"{}").unwrap();
        fs::create_dir_all(root.join("cairn-policy/generated")).unwrap();
        fs::write(
            root.join("cairn-policy/generated/consistency-analysis.json"),
            ANALYSIS_POLICY.as_bytes(),
        )
        .unwrap();
        let repository_id = "requirement-consistency-fixture".to_string();
        let registry = evaluate_content_bound_requirement_registry(
            vec![ContentBoundSpecificationInput {
                path: specification,
                markdown: markdown.into(),
            }],
            b"{}",
            &repository_id,
            ContentBoundRevision {
                kind: "opaque".into(),
                value: "fixture".into(),
            },
        )
        .registry
        .expect("native registry admits the packaged specification");
        assert!(
            registry
                .rows
                .iter()
                .any(|row| row.requirement_id == REQUIREMENT_ID),
            "native requirement row exists"
        );
        let row = registry
            .rows
            .iter()
            .find(|row| row.requirement_id == REQUIREMENT_ID)
            .expect("native requirement row exists")
            .clone();
        let registry_path = root.join("requirement-registry.json");
        fs::write(&registry_path, serde_json::to_vec(&registry).unwrap()).unwrap();
        let inventory: Value = serde_json::from_str(INVENTORY).unwrap();
        let mut contract = inventory["contracts"]
            .as_array()
            .unwrap()
            .iter()
            .find(|contract| contract["requirement"]["requirement_id"] == REQUIREMENT_ID)
            .expect("pilot contract exists")
            .clone();
        contract["requirement"] = serde_json::to_value(&row).unwrap();
        let case = json!({
            "schema": "cairn.requirement-consistency-case.v1",
            "profile": inventory["profile"],
            "subject": {
                "subject_blake3": "c".repeat(64),
                "policy_blake3": "b".repeat(64),
                "target": repository_id,
                "environment": "linux-x86_64"
            },
            "contracts": [{
                "native_requirement": row,
                "contract": contract,
                "correspondence_review": null
            }],
            "selected": [REQUIREMENT_ID],
            "correspondence": [],
            "authorized_reviewers": [],
            "native_obligations": [{
                "requirement_id": REQUIREMENT_ID,
                "obligation_id": "obligation.consistency-fixture",
                "finding_id": "finding.consistency-fixture"
            }],
            "producer": {
                "producer": "cairn-cli-test",
                "adapter": "finite-enumeration",
                "version": "1"
            }
        });
        let case_path = root.join("case.json");
        fs::write(&case_path, serde_json::to_vec(&case).unwrap()).unwrap();
        Self {
            case: case_path,
            registry: registry_path,
            lifecycle_policy: root.join("policy.json"),
            root,
        }
    }

    fn inspect(&self, name: &str, witness: Option<&Value>, success: bool) -> Value {
        let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
        command.args([
            "requirement-analysis",
            "inspect",
            "--case",
            self.case.to_str().unwrap(),
            "--registry",
            self.registry.to_str().unwrap(),
            "--root",
            self.root.to_str().unwrap(),
            "--policy",
            self.lifecycle_policy.to_str().unwrap(),
        ]);
        if let Some(witness) = witness {
            let path = self.root.join(format!("{name}-witness.json"));
            fs::write(&path, serde_json::to_vec(witness).unwrap()).unwrap();
            command.args(["--witness", path.to_str().unwrap()]);
        }
        let output = command.output().unwrap();
        assert_eq!(
            output.status.success(),
            success,
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice(&output.stdout).unwrap()
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
// r[verify cairn.requirement_consistency.inspection]
fn inspection_reports_bounded_consistency_without_granting_authority() {
    let fixture = Fixture::new();
    let before = tree(&fixture.root);
    let output = fixture.inspect("positive", None, true);
    assert_eq!(output["schema_version"], "cairn.requirement-analysis.inspection.v1");
    assert_eq!(output["outcome"], "analyzed");
    assert_eq!(output["advisory"], true);
    assert_eq!(output["admitted_proof_evidence"], "not_supplied");
    assert_eq!(output["lifecycle_authority"], "none");
    assert!(output["witness_replay"].is_null());
    let report = &output["report"];
    assert_eq!(report["schema"], "cairn.requirement-consistency-report.v1");
    assert_eq!(report["advisory"], true);
    let queries = report["queries"].as_array().unwrap();
    assert!(queries.len() >= 3);
    for outcome in queries {
        assert!(
            !outcome["status"].is_null(),
            "every query reports an explicit status"
        );
        assert!(!outcome["basis"]["kind"].is_null());
    }
    let compatible = queries
        .iter()
        .find(|outcome| outcome["query"]["kind"] == "guarantees_compatible")
        .expect("compatibility query is reported");
    assert_eq!(compatible["status"], "consistent");
    assert_eq!(compatible["basis"]["kind"], "witness");
    let assumptions = queries
        .iter()
        .find(|outcome| outcome["query"]["kind"] == "assumptions_satisfiable")
        .expect("assumption query is reported");
    assert_eq!(assumptions["status"], "consistent");
    let findings = report["findings"].as_array().unwrap();
    assert!(!findings.is_empty());
    for finding in findings {
        assert_eq!(finding["disposition"], "unresolved");
        assert_eq!(finding["boundary"], report["boundary"]);
        assert_eq!(
            finding["native_obligations"][0]["obligation_id"],
            "obligation.consistency-fixture"
        );
    }
    assert_eq!(
        before,
        tree(&fixture.root),
        "inspection reads bounded inputs without mutating the repository"
    );
}

#[test]
// r[verify cairn.requirement_consistency.replay]
fn retained_witness_replays_and_tampering_is_reported() {
    let fixture = Fixture::new();
    let output = fixture.inspect("witness", None, true);
    let witness = output["report"]["queries"]
        .as_array()
        .unwrap()
        .iter()
        .find(|outcome| outcome["query"]["kind"] == "guarantees_compatible")
        .and_then(|outcome| outcome["witness"].as_object())
        .cloned()
        .expect("compatibility witness is retained");
    let replay = fixture.inspect("replay", Some(&Value::Object(witness.clone())), true);
    assert_eq!(replay["outcome"], "analyzed");
    assert_eq!(replay["witness_replay"]["replays"], true);
    assert_eq!(
        replay["witness_replay"]["contract_evaluations"][0]["requirement_id"],
        REQUIREMENT_ID
    );

    let mut tampered = witness;
    tampered.insert(
        "model_blake3".into(),
        Value::String("0".repeat(64)),
    );
    let tampered = fixture.inspect("tampered", Some(&Value::Object(tampered)), true);
    assert_eq!(tampered["witness_replay"]["replays"], false);
    assert!(tampered["witness_replay"]["detail"]
        .as_str()
        .unwrap()
        .contains("model identity"));
}

#[test]
// r[verify cairn.requirement_consistency.inspection]
fn stale_native_revisions_and_invalid_policies_are_rejected() {
    let fixture = Fixture::new();
    let mut case: ConsistencyCase =
        serde_json::from_slice(&fs::read(&fixture.case).unwrap()).unwrap();
    case.contracts[0].native_requirement.row_blake3 = "0".repeat(64);
    fs::write(&fixture.case, serde_json::to_vec(&case).unwrap()).unwrap();
    let output = fixture.inspect("stale", None, false);
    assert_eq!(output["outcome"], "rejected");
    assert_eq!(output["error"]["outcome"], "scope_mismatch");

    let fixture = Fixture::new();
    let invalid = fixture.root.join("invalid-analysis-policy.json");
    fs::write(
        &invalid,
        serde_json::to_vec(&json!({
            "schema": "cairn.requirement-consistency-policy.v2",
            "queries": ["assumptions_satisfiable"],
            "bounds": {
                "max_contracts": 1,
                "max_guarantees": 1,
                "max_cells": 1,
                "max_states": 1,
                "max_evaluations": 1,
                "max_depth": 1
            }
        }))
        .unwrap(),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args([
            "requirement-analysis",
            "inspect",
            "--case",
            fixture.case.to_str().unwrap(),
            "--registry",
            fixture.registry.to_str().unwrap(),
            "--root",
            fixture.root.to_str().unwrap(),
            "--policy",
            fixture.lifecycle_policy.to_str().unwrap(),
            "--analysis-policy",
            invalid.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let output: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(output["error"]["outcome"], "unsupported");

    let _policy: ConsistencyPolicy = serde_json::from_str(ANALYSIS_POLICY).unwrap();
}
