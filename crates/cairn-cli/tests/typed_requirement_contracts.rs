use cairn_core::{
    evaluate_content_bound_requirement_registry, ContentBoundRevision,
    ContentBoundSpecificationInput,
};
use serde_json::{json, Value};
use std::{env, fs, path::PathBuf, process::Command, time::SystemTime};

struct Fixture {
    root: PathBuf,
    inventory: Value,
}
impl Fixture {
    fn new() -> Self {
        let root = env::temp_dir().join(format!(
            "cairn-typed-requirements-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let sources = [
            (
                "agent-completion",
                include_str!("../../../.cairn/specs/agent-completion/spec.md"),
            ),
            (
                "verification-obligation-radicle-cob",
                include_str!("../../../.cairn/specs/verification-obligation-radicle-cob/spec.md"),
            ),
        ];
        let specifications = sources
            .iter()
            .map(|(name, markdown)| {
                let path = format!(".cairn/specs/{name}/spec.md");
                fs::create_dir_all(root.join(&path).parent().unwrap()).unwrap();
                fs::write(root.join(&path), markdown).unwrap();
                ContentBoundSpecificationInput {
                    path,
                    markdown: (*markdown).into(),
                }
            })
            .collect();
        fs::write(root.join("policy.json"), b"{}").unwrap();
        let registry = evaluate_content_bound_requirement_registry(
            specifications,
            b"{}",
            "typed-cli-fixture",
            ContentBoundRevision {
                kind: "opaque".into(),
                value: "fixture".into(),
            },
        )
        .registry
        .unwrap();
        fs::write(
            root.join("registry.json"),
            serde_json::to_vec(&registry).unwrap(),
        )
        .unwrap();
        let mut inventory: Value = serde_json::from_str(include_str!(
            "../../../cairn-policy/generated/typed-requirement-contracts.json"
        ))
        .unwrap();
        // Isolate admission cases from independently retained production review policy.
        inventory["reviews"] = json!([]);
        inventory["authorized_reviewers"] = json!([]);
        let fixture = Self { root, inventory };
        fixture.save_inventory();
        fixture
    }
    fn save_inventory(&self) {
        fs::write(
            self.root.join("inventory.json"),
            serde_json::to_vec(&self.inventory).unwrap(),
        )
        .unwrap();
    }
    fn inspect(&self, requirement: &str, options: &[&str], success: bool) -> Value {
        let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .args([
                "requirement-contract",
                "inspect",
                "--root",
                self.root.to_str().unwrap(),
                "--policy",
                self.root.join("policy.json").to_str().unwrap(),
                "--registry",
                self.root.join("registry.json").to_str().unwrap(),
                "--inventory",
                self.root.join("inventory.json").to_str().unwrap(),
                "--requirement",
                requirement,
            ])
            .args(options)
            .output()
            .unwrap();
        assert_eq!(
            output.status.success(),
            success,
            "{}\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        serde_json::from_slice(&output.stdout).unwrap()
    }
    fn first_id(&self) -> String {
        self.inventory["contracts"][0]["requirement"]["requirement_id"]
            .as_str()
            .unwrap()
            .into()
    }
    fn identity_example(&self) -> Value {
        let mut tables = serde_json::Map::new();
        tables.insert(
            "repository_contract_present".into(),
            json!([{"arguments": [], "value": {"domain": "bool", "value": "true"}}]),
        );
        for name in ["repository_identity", "decision_identity"] {
            tables.insert(name.into(), Value::Array(self.inventory["contracts"][0]["domains"]["component"]["values"].as_array().unwrap().iter().map(|component| json!({"arguments": [{"domain": "component", "value": component}], "value": {"domain": "identity", "value": "a"}})).collect()));
        }
        json!({"observables": tables})
    }
}
impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

#[test]
// r[verify cairn.typed_requirements.inspection]
fn inspection_exposes_all_pilots_and_keeps_missing_contracts_informal() {
    let fixture = Fixture::new();
    let before = fs::read(fixture.root.join("registry.json")).unwrap();
    for contract in fixture.inventory["contracts"].as_array().unwrap() {
        let output = fixture.inspect(
            contract["requirement"]["requirement_id"].as_str().unwrap(),
            &[],
            true,
        );
        assert_eq!(output["well_formedness"], "well_formed");
        assert_eq!(output["correspondence"], "not_reviewed");
        assert_eq!(output["admitted_proof_evidence"], "not_supplied");
        assert_eq!(
            output["normalized"]["contract_blake3"]
                .as_str()
                .unwrap()
                .len(),
            64
        );
    }
    let informal = fixture.inspect(
        "cairn.supersede.agent_run_receipts.actionable_diagnostics",
        &[],
        true,
    );
    assert_eq!(informal["meaning"], "informal");
    assert_eq!(informal["well_formedness"], "not_applicable");
    assert_eq!(
        before,
        fs::read(fixture.root.join("registry.json")).unwrap()
    );
    assert!(!fixture.root.join(".cairn/changes").exists());
}

#[test]
fn examples_distinguish_equal_and_drifted_identities_without_becoming_proofs() {
    let fixture = Fixture::new();
    let id = fixture.first_id();
    let path = fixture.root.join("example.json");
    let good = fixture.identity_example();
    fs::write(&path, serde_json::to_vec(&good).unwrap()).unwrap();
    let output = fixture.inspect(&id, &["--example", path.to_str().unwrap()], true);
    assert_eq!(
        output["example_evaluation"]["result"]["guarantee_holds"],
        true
    );
    assert_eq!(output["correspondence"], "not_reviewed");
    assert_eq!(output["admitted_proof_evidence"], "not_supplied");
    let mut bad = good.clone();
    bad["observables"]["decision_identity"][0]["value"]["value"] = json!("b");
    fs::write(&path, serde_json::to_vec(&bad).unwrap()).unwrap();
    let output = fixture.inspect(&id, &["--example", path.to_str().unwrap()], true);
    assert_eq!(
        output["example_evaluation"]["result"]["guarantee_holds"],
        false
    );
    bad["observables"]["decision_identity"]
        .as_array_mut()
        .unwrap()
        .pop();
    fs::write(&path, serde_json::to_vec(&bad).unwrap()).unwrap();
    let output = fixture.inspect(&id, &["--example", path.to_str().unwrap()], false);
    assert_eq!(
        output["example_evaluation"]["error"]["outcome"],
        "invalid_example"
    );
}

#[test]
fn stale_revisions_unknown_profiles_unsupported_operators_and_bounds_fail_visibly() {
    for failure in [
        "revision",
        "profile",
        "unsupported",
        "bound",
        "unbound",
        "ill_typed",
    ] {
        let mut fixture = Fixture::new();
        let id = fixture.first_id();
        match failure {
            "revision" => {
                fixture.inventory["contracts"][0]["requirement"]["row_blake3"] =
                    json!("f".repeat(64))
            }
            "profile" => fixture.inventory["profile"]["interpretation"] = json!("unknown"),
            "unsupported" => {
                fixture.inventory["contracts"][0]["guarantee"] =
                    json!({"kind": "always", "expression": {"kind": "bool", "value": true}})
            }
            "bound" => fixture.inventory["profile"]["max_nodes"] = json!(1),
            "unbound" => {
                fixture.inventory["contracts"][0]["guarantee"] = json!({"kind": "equal", "left": {"kind": "variable", "name": "free"}, "right": {"kind": "literal", "value": {"domain": "identity", "value": "a"}}})
            }
            "ill_typed" => {
                fixture.inventory["contracts"][0]["guarantee"] = json!({"kind": "equal", "left": {"kind": "literal", "value": {"domain": "bool", "value": "true"}}, "right": {"kind": "literal", "value": {"domain": "identity", "value": "a"}}})
            }
            _ => unreachable!(),
        }
        fixture.save_inventory();
        let output = fixture.inspect(&id, &[], false);
        let expected = match failure {
            "revision" => "requirement_revision_mismatch",
            "profile" => "unknown_profile",
            "unsupported" => "unsupported_construct",
            "bound" => "bound_exhaustion",
            "unbound" => "unbound_variable",
            _ => "ill_typed",
        };
        assert_eq!(output["error"]["outcome"], expected);
    }
    let fixture = Fixture::new();
    fs::write(
        fixture.root.join(".cairn/specs/agent-completion/spec.md"),
        "changed",
    )
    .unwrap();
    let output = fixture.inspect(&fixture.first_id(), &[], false);
    assert_eq!(output["registry"], "invalid_or_stale");
}

#[test]
fn explicit_revision_bound_review_declarations_do_not_upgrade_examples_to_proofs() {
    let mut fixture = Fixture::new();
    let id = fixture.first_id();
    let first = fixture.inspect(&id, &[], true);
    let normalized = &first["normalized"];
    fixture.inventory["authorized_reviewers"] = json!(["fixture-independent-reviewer"]);
    fixture.save_inventory();
    // Synthetic admission test only: this is not an approval of shipped pilot mappings.
    let mut review = json!({
        "requirement_row_blake3": normalized["contract"]["requirement"]["row_blake3"],
        "contract_blake3": normalized["contract_blake3"], "interpretation_blake3": normalized["interpretation_blake3"], "mapping_blake3": normalized["mapping_blake3"],
        "proposed_by": normalized["contract"]["proposed_by"], "reviewer": "fixture-independent-reviewer", "review_artifact_blake3": "a".repeat(64), "decision": "approved",
    });
    let path = fixture.root.join("review.json");
    fs::write(&path, serde_json::to_vec(&review).unwrap()).unwrap();
    let output = fixture.inspect(&id, &["--review", path.to_str().unwrap()], true);
    assert_eq!(output["correspondence"], "reviewed_declaration");
    assert_eq!(output["admitted_proof_evidence"], "not_supplied");
    review["mapping_blake3"] = json!("b".repeat(64));
    fs::write(&path, serde_json::to_vec(&review).unwrap()).unwrap();
    let output = fixture.inspect(&id, &["--review", path.to_str().unwrap()], false);
    assert_eq!(output["correspondence"], "not_reviewed");
    assert_eq!(output["reviews"][0]["error"]["outcome"], "invalid_review");
}
