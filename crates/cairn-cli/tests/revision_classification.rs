use std::{collections::BTreeMap, env, fs, path::PathBuf, process::Command, time::SystemTime};

use cairn_core::{
    normalize_contract, ClaimContext, ClaimKind, ContentBoundRequirementRow, ContentBoundSourceSpan,
    CorrespondenceInputs, DomainCorrespondence, FiniteDomain, FiniteExpr, FiniteTerm, FiniteValue,
    ObservableCorrespondence, ObservableMapping, PolicyRevisionBinding, PolicyRevisionRequest,
    Proposition, RequirementRevisionRequest, RetainedEvidence, RevisionBinding, RevisionContext,
    RevisionMapping, ScopedClaim, SemanticContract, SemanticProfile, ACCEPTANCE_FRAGMENT_SCHEMA_V1,
    CLAIM_SCHEMA_V1, FINITE_INTERPRETATION_V1, FINITE_SEMANTICS_PROFILE_V1,
    POLICY_REVISION_REQUEST_SCHEMA_V1, REQUIREMENT_REVISION_REQUEST_SCHEMA_V1,
    SUPPORTED_POLICY_FRAGMENT_V1,
};
use serde_json::{json, Value};

fn digest(character: char) -> String {
    character.to_string().repeat(64)
}

fn profile() -> SemanticProfile {
    SemanticProfile {
        id: FINITE_SEMANTICS_PROFILE_V1.into(),
        interpretation: FINITE_INTERPRETATION_V1.into(),
        max_nodes: 4096,
        max_depth: 64,
        max_domain_values: 256,
        max_steps: 1_000_000,
    }
}

fn row(seed: char) -> ContentBoundRequirementRow {
    ContentBoundRequirementRow {
        requirement_id: "test.requirement".into(),
        specification_path: ".cairn/specs/revision/spec.md".into(),
        specification_blake3: digest(seed),
        source_span: ContentBoundSourceSpan {
            start_byte: 1,
            end_byte: 2,
            start_line: 1,
            end_line: 1,
        },
        requirement_body_blake3: digest(seed),
        row_blake3: digest(seed),
    }
}

fn observable() -> ObservableMapping {
    ObservableMapping {
        arguments: Vec::new(),
        result: "bool".into(),
        phase: "state".into(),
        source: "cli test observable".into(),
        projection: "cli test projection".into(),
    }
}

fn contract(row: ContentBoundRequirementRow, guarantee: FiniteExpr) -> SemanticContract {
    SemanticContract {
        profile: FINITE_SEMANTICS_PROFILE_V1.into(),
        proposed_by: "cli-revision-test".into(),
        requirement: row,
        domains: BTreeMap::from([(
            "bool".to_string(),
            FiniteDomain {
                values: vec!["false".into(), "true".into()],
            },
        )]),
        observables: BTreeMap::from([("flag".to_string(), observable())]),
        assumptions: Vec::new(),
        activation: FiniteExpr::Bool { value: true },
        guarantee,
        supported_portion: "finite cli portion".into(),
        informal_remainder: "cli remainder".into(),
    }
}

fn is_true(name: &str) -> FiniteExpr {
    FiniteExpr::Equal {
        left: FiniteTerm::Observable {
            mapping: name.into(),
            arguments: Vec::new(),
        },
        right: FiniteTerm::Literal {
            value: FiniteValue {
                domain: "bool".into(),
                value: "true".into(),
            },
        },
    }
}

fn mapping() -> RevisionMapping {
    RevisionMapping {
        domains: BTreeMap::from([(
            "bool".into(),
            DomainCorrespondence {
                old_domain: Some("bool".into()),
                new_domain: Some("bool".into()),
                values: vec![
                    ValueCorrespondence {
                        common: "false".into(),
                        old: Some("false".into()),
                        new: Some("false".into()),
                    },
                    ValueCorrespondence {
                        common: "true".into(),
                        old: Some("true".into()),
                        new: Some("true".into()),
                    },
                ],
            },
        )]),
        observables: BTreeMap::from([(
            "flag".into(),
            ObservableCorrespondence {
                old: Some("flag".into()),
                new: Some("flag".into()),
                arguments: Vec::new(),
                result: "bool".into(),
            },
        )]),
    }
}

fn context() -> RevisionContext {
    RevisionContext {
        subject_blake3: digest('c'),
        policy_blake3: digest('d'),
        target: "test-target".into(),
        environment: "test-environment".into(),
    }
}

fn requirement_request(
    old_guarantee: FiniteExpr,
    new_guarantee: FiniteExpr,
    include_evidence: bool,
) -> RequirementRevisionRequest {
    let old_row = row('a');
    let new_row = row('b');
    let old_contract = contract(old_row.clone(), old_guarantee);
    let new_contract = contract(new_row.clone(), new_guarantee);
    let normalized_old = normalize_contract(&profile(), &old_row, &old_contract).unwrap();
    let normalized_new = normalize_contract(&profile(), &new_row, &new_contract).unwrap();
    let retained_evidence = if include_evidence {
        vec![RetainedEvidence {
            evidence_blake3: digest('e'),
            receipt_blake3: digest('f'),
            claim: ScopedClaim {
                schema: CLAIM_SCHEMA_V1.into(),
                kind: ClaimKind::Observation,
                requirement: normalized_old.contract.requirement.clone(),
                interpretation_blake3: normalized_old.interpretation_blake3.clone(),
                contract_blake3: normalized_old.contract_blake3.clone(),
                context: ClaimContext {
                    subject_blake3: context().subject_blake3,
                    policy_blake3: context().policy_blake3,
                    target: context().target,
                    environment: context().environment,
                    bounds: BTreeMap::from([("finite-worlds".into(), "1".into())]),
                },
                proposition: Proposition::CheckPassed {
                    check_id: "check-a".into(),
                },
            },
        }]
    } else {
        Vec::new()
    };
    RequirementRevisionRequest {
        schema: REQUIREMENT_REVISION_REQUEST_SCHEMA_V1.into(),
        profile: profile(),
        old_native_requirement: old_row,
        new_native_requirement: new_row,
        old_contract,
        new_contract,
        binding: RevisionBinding {
            old_requirement_row_blake3: normalized_old.contract.requirement.row_blake3.clone(),
            new_requirement_row_blake3: normalized_new.contract.requirement.row_blake3.clone(),
            old_interpretation_blake3: normalized_old.interpretation_blake3.clone(),
            new_interpretation_blake3: normalized_new.interpretation_blake3.clone(),
            old_mapping_blake3: normalized_old.mapping_blake3.clone(),
            new_mapping_blake3: normalized_new.mapping_blake3.clone(),
            old_contract_blake3: normalized_old.contract_blake3.clone(),
            new_contract_blake3: normalized_new.contract_blake3.clone(),
        },
        context: context(),
        mapping: mapping(),
        correspondence: CorrespondenceInputs {
            old: None,
            new: None,
            authorized_reviewers: Vec::new(),
        },
        retained_evidence,
    }
}

fn incompatible_requirement_request() -> RequirementRevisionRequest {
    let old_row = row('a');
    let new_row = row('b');
    let domains = BTreeMap::from([(
        "bool".to_string(),
        FiniteDomain {
            values: vec!["false".into(), "true".into()],
        },
    )]);
    let observables = BTreeMap::from([
        ("left".to_string(), observable()),
        ("right".to_string(), observable()),
    ]);
    let old_contract = SemanticContract {
        profile: FINITE_SEMANTICS_PROFILE_V1.into(),
        proposed_by: "cli-revision-test".into(),
        requirement: old_row.clone(),
        domains: domains.clone(),
        observables: observables.clone(),
        assumptions: Vec::new(),
        activation: FiniteExpr::Bool { value: true },
        guarantee: is_true("left"),
        supported_portion: "finite cli portion".into(),
        informal_remainder: "cli remainder".into(),
    };
    let new_contract = SemanticContract {
        profile: FINITE_SEMANTICS_PROFILE_V1.into(),
        proposed_by: "cli-revision-test".into(),
        requirement: new_row.clone(),
        domains,
        observables,
        assumptions: Vec::new(),
        activation: FiniteExpr::Bool { value: true },
        guarantee: is_true("right"),
        supported_portion: "finite cli portion".into(),
        informal_remainder: "cli remainder".into(),
    };
    let normalized_old = normalize_contract(&profile(), &old_row, &old_contract).unwrap();
    let normalized_new = normalize_contract(&profile(), &new_row, &new_contract).unwrap();
    let observables = BTreeMap::from([
        (
            "left".to_string(),
            ObservableCorrespondence {
                old: Some("left".into()),
                new: Some("left".into()),
                arguments: Vec::new(),
                result: "bool".into(),
            },
        ),
        (
            "right".to_string(),
            ObservableCorrespondence {
                old: Some("right".into()),
                new: Some("right".into()),
                arguments: Vec::new(),
                result: "bool".into(),
            },
        ),
    ]);
    RequirementRevisionRequest {
        schema: REQUIREMENT_REVISION_REQUEST_SCHEMA_V1.into(),
        profile: profile(),
        old_native_requirement: old_row,
        new_native_requirement: new_row,
        old_contract,
        new_contract,
        binding: RevisionBinding {
            old_requirement_row_blake3: normalized_old.contract.requirement.row_blake3.clone(),
            new_requirement_row_blake3: normalized_new.contract.requirement.row_blake3.clone(),
            old_interpretation_blake3: normalized_old.interpretation_blake3.clone(),
            new_interpretation_blake3: normalized_new.interpretation_blake3.clone(),
            old_mapping_blake3: normalized_old.mapping_blake3.clone(),
            new_mapping_blake3: normalized_new.mapping_blake3.clone(),
            old_contract_blake3: normalized_old.contract_blake3.clone(),
            new_contract_blake3: normalized_new.contract_blake3.clone(),
        },
        context: context(),
        mapping: RevisionMapping {
            domains: BTreeMap::from([(
                "bool".into(),
                DomainCorrespondence {
                    old_domain: Some("bool".into()),
                    new_domain: Some("bool".into()),
                    values: vec![
                        ValueCorrespondence {
                            common: "false".into(),
                            old: Some("false".into()),
                            new: Some("false".into()),
                        },
                        ValueCorrespondence {
                            common: "true".into(),
                            old: Some("true".into()),
                            new: Some("true".into()),
                        },
                    ],
                },
            )]),
            observables,
        },
        correspondence: CorrespondenceInputs {
            old: None,
            new: None,
            authorized_reviewers: Vec::new(),
        },
        retained_evidence: Vec::new(),
    }
}

struct Fixture {
    dir: PathBuf,
}

impl Fixture {
    fn new(name: &str) -> Self {
        let nanos = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = env::temp_dir().join(format!(
            "cairn-revision-cli-{name}-{}-{nanos}",
            std::process::id()
        ));
        fs::create_dir_all(&dir).unwrap();
        Self { dir }
    }

    fn write(&self, name: &str, value: &Value) -> String {
        let path = self.dir.join(name);
        fs::write(&path, serde_json::to_vec(value).unwrap()).unwrap();
        path.to_str().unwrap().to_string()
    }

    fn classify(&self, option: &str, path: &str, extra: &[&str], success: bool) -> Value {
        let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .args(["revision", "classify", option, path])
            .args(extra)
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
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

#[test]
// r[verify cairn.revision_classification.inspection]
fn cli_classifies_editorial_and_strengthening_requirements() {
    let fixture = Fixture::new("requirement");
    let editorial = fixture.write(
        "editorial.json",
        &serde_json::to_value(requirement_request(is_true("flag"), is_true("flag"), false)).unwrap(),
    );
    let output = fixture.classify("--requirement", &editorial, &[], true);
    assert_eq!(output["outcome"], "classified");
    assert_eq!(output["advisory"], true);
    assert_eq!(output["report"]["classification"], "editorial");
    assert_eq!(output["report"]["relation"], "equivalent");
    assert_eq!(
        output["report"]["transfer"]["automated_transfer"],
        "disabled_until_separately_qualified"
    );

    let strengthening = fixture.write(
        "strengthening.json",
        &serde_json::to_value(requirement_request(
            FiniteExpr::Bool { value: true },
            is_true("flag"),
            true,
        ))
        .unwrap(),
    );
    let output = fixture.classify("--requirement", &strengthening, &[], true);
    assert_eq!(output["report"]["classification"], "strengthening");
    assert_eq!(
        output["report"]["evidence_reuse"][0]["decision"],
        "not_reusable"
    );
    assert_eq!(
        output["report"]["evidence_reuse"][0]["old_receipt_current"],
        false
    );
}

#[test]
// r[verify cairn.revision_classification.evidence_reuse]
fn cli_weakening_and_stale_evidence_are_visible_nonaccepting_outcomes() {
    let fixture = Fixture::new("stale");
    let weakening = fixture.write(
        "weakening.json",
        &serde_json::to_value(requirement_request(
            is_true("flag"),
            FiniteExpr::Bool { value: true },
            true,
        ))
        .unwrap(),
    );
    let output = fixture.classify("--requirement", &weakening, &[], true);
    assert_eq!(output["report"]["classification"], "weakening");
    assert_eq!(
        output["report"]["evidence_reuse"][0]["decision"],
        "not_reusable"
    );

    let mut stale: Value =
        serde_json::from_str(&fs::read_to_string(&weakening).unwrap()).unwrap();
    stale["retained_evidence"][0]["claim"]["context"]["subject_blake3"] =
        json!(digest('9'));
    let path = fixture.write("stale.json", &stale);
    let output = fixture.classify("--requirement", &path, &[], true);
    assert_eq!(
        output["report"]["evidence_reuse"][0]["decision"],
        "not_reusable"
    );
    assert_eq!(
        output["report"]["evidence_reuse"][0]["applicability_current"],
        false
    );
}

#[test]
// r[verify cairn.revision_classification.policy_fragment]
fn cli_compares_policy_fragments_and_rejects_tampered_bindings() {
    let fixture = Fixture::new("policy");
    let old = json!({
        "schema": ACCEPTANCE_FRAGMENT_SCHEMA_V1,
        "fragment": SUPPORTED_POLICY_FRAGMENT_V1,
        "required_checks": ["c1"],
        "evidence_roles": ["r1", "r2"],
        "assumption_sets": [["a"]],
        "universe": {"checks": ["c1", "c2"], "evidence_roles": ["r1", "r2"], "assumptions": ["a"]}
    });
    let mut new = old.clone();
    new["evidence_roles"] = json!(["r1"]);
    let request = PolicyRevisionRequest {
        schema: POLICY_REVISION_REQUEST_SCHEMA_V1.into(),
        binding: PolicyRevisionBinding {
            old_policy_blake3: digest('1'),
            new_policy_blake3: digest('2'),
        },
        old: serde_json::from_value(old).unwrap(),
        new: serde_json::from_value(new).unwrap(),
    };
    let path = fixture.write(
        "tightened.json",
        &serde_json::to_value(request.clone()).unwrap(),
    );
    let output = fixture.classify("--policy-fragment", &path, &[], true);
    assert_eq!(output["kind"], "policy_fragment");
    assert_eq!(output["report"]["classification"], "strengthening");
    assert_eq!(output["report"]["relation"], "tightened");
    assert_eq!(output["report"]["old"]["accepted_count"], json!(4));
    assert_eq!(output["report"]["new"]["accepted_count"], json!(2));
    assert!(output["report"]["inclusion"][1]["witness"].is_object());

    let mut tampered: Value = serde_json::from_str(&fs::read_to_string(&path).unwrap()).unwrap();
    tampered["binding"]["old_policy_blake3"] = json!("not-a-digest");
    let bad = fixture.write("bad.json", &tampered);
    let output = fixture.classify("--policy-fragment", &bad, &[], false);
    assert_eq!(output["outcome"], "rejected");
    assert_eq!(output["error"]["outcome"], "invalid_input");
}

#[test]
// r[verify cairn.revision_classification.requirement_relation]
fn cli_classifies_incompatible_requirements_with_witnesses() {
    let fixture = Fixture::new("incompatible");
    let path = fixture.write(
        "incompatible.json",
        &serde_json::to_value(incompatible_requirement_request()).unwrap(),
    );
    let output = fixture.classify("--requirement", &path, &[], true);
    assert_eq!(output["report"]["classification"], "incompatible");
    assert_eq!(output["report"]["relation"], "incomparable");
    for implication in output["report"]["implications"].as_array().unwrap() {
        assert_eq!(implication["holds"], false);
        assert!(implication["witness"].is_object());
    }
}

#[test]
fn cli_requires_exactly_one_input_and_rejects_unknown_options() {
    let fixture = Fixture::new("arguments");
    let path = fixture.write(
        "request.json",
        &serde_json::to_value(requirement_request(is_true("flag"), is_true("flag"), false)).unwrap(),
    );
    let both = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args([
            "revision",
            "classify",
            "--requirement",
            path.as_str(),
            "--policy-fragment",
            path.as_str(),
        ])
        .output()
        .unwrap();
    assert!(!both.status.success());

    let unknown = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args(["revision", "classify", "--unknown"])
        .output()
        .unwrap();
    assert!(!unknown.status.success());

    let missing = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args(["revision", "classify"])
        .output()
        .unwrap();
    assert!(!missing.status.success());
}
