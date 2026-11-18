// r[verify cairn.concurrent_footprints.inspection]
use std::{collections::BTreeMap, env, fs, path::PathBuf, process::Command, time::SystemTime};

use cairn_core::{
    analyze_concurrent_changes, footprint_guarantee_identity, normalize_contract,
    AffectedInterface, AssumptionDependency, AssuranceScope, CaseContext, ChangeFootprint,
    ChangeIdentity, ChangedGuarantee, CompositionCandidate, ConcurrentChangeCase,
    ConsumedAssumption, ContentBoundRequirementRow, ContentBoundSourceSpan, EvidenceScope,
    FiniteDomain, FiniteExpr, FiniteTerm, FiniteValue, FootprintBounds, FootprintPolicy,
    FootprintRevision, GuaranteeChange, GuaranteeStatement, IntegrationCondition, InterfaceChange,
    JointEvidence, ObservableMapping, PreservedInvariant, PropositionEvidence, PropositionKind,
    PropositionState, SemanticContract, SemanticProfile, StatementForm, CASE_SCHEMA_V1,
    FINITE_INTERPRETATION_V1, FINITE_SEMANTICS_PROFILE_V1, FOOTPRINT_SCHEMA_V1, POLICY_SCHEMA_V1,
};
use serde_json::Value;

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

fn policy() -> FootprintPolicy {
    FootprintPolicy {
        schema: POLICY_SCHEMA_V1.into(),
        bounds: FootprintBounds {
            max_changes: 8,
            max_revisions: 8,
            max_assumptions: 8,
            max_guarantees: 8,
            max_invariants: 8,
            max_invalidations: 8,
            max_interfaces: 8,
            max_unknown: 8,
            max_compositions: 8,
            max_evidence: 8,
            max_conditions: 8,
            max_domain_values: 8,
            max_worlds: 1024,
            max_steps: 100_000,
            max_depth: 32,
        },
    }
}

fn row(requirement_id: &str, seed: char) -> ContentBoundRequirementRow {
    ContentBoundRequirementRow {
        requirement_id: requirement_id.into(),
        specification_path: ".cairn/specs/concurrent/spec.md".into(),
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

fn bool_domain() -> FiniteDomain {
    FiniteDomain {
        values: vec!["false".into(), "true".into()],
    }
}

fn boolean_observable() -> ObservableMapping {
    ObservableMapping {
        arguments: Vec::new(),
        result: "bool".into(),
        phase: "state".into(),
        source: "cli test observable".into(),
        projection: "cli test projection".into(),
    }
}

fn attempt_observable() -> ObservableMapping {
    ObservableMapping {
        arguments: vec!["attempt".into()],
        result: "bool".into(),
        phase: "state".into(),
        source: "cli test observable".into(),
        projection: "cli test projection".into(),
    }
}

fn revision(
    requirement: ContentBoundRequirementRow,
    domains: BTreeMap<String, FiniteDomain>,
    observables: BTreeMap<String, ObservableMapping>,
    guarantee: FiniteExpr,
) -> FootprintRevision {
    let contract = SemanticContract {
        profile: FINITE_SEMANTICS_PROFILE_V1.into(),
        proposed_by: "cli-footprints-test".into(),
        requirement: requirement.clone(),
        domains,
        observables,
        assumptions: Vec::new(),
        activation: FiniteExpr::Bool { value: true },
        guarantee,
        supported_portion: "finite cli portion".into(),
        informal_remainder: "cli remainder".into(),
    };
    let normalized = normalize_contract(&profile(), &requirement, &contract).unwrap();
    FootprintRevision {
        requirement,
        contract,
        interpretation_blake3: normalized.interpretation_blake3,
        mapping_blake3: normalized.mapping_blake3,
        contract_blake3: normalized.contract_blake3,
    }
}

fn simple_revision(requirement_id: &str, seed: char) -> FootprintRevision {
    revision(
        row(requirement_id, seed),
        BTreeMap::from([("bool".to_string(), bool_domain())]),
        BTreeMap::from([("flag".to_string(), boolean_observable())]),
        FiniteExpr::Bool { value: true },
    )
}

fn is_true(mapping: &str) -> FiniteExpr {
    FiniteExpr::Equal {
        left: FiniteTerm::Observable {
            mapping: mapping.into(),
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

fn and(left: FiniteExpr, right: FiniteExpr) -> FiniteExpr {
    FiniteExpr::And {
        expressions: vec![left, right],
    }
}

fn authorized(variable: &str) -> FiniteExpr {
    FiniteExpr::Equal {
        left: FiniteTerm::Observable {
            mapping: "authorized".into(),
            arguments: vec![FiniteTerm::Variable {
                name: variable.into(),
            }],
        },
        right: FiniteTerm::Literal {
            value: FiniteValue {
                domain: "bool".into(),
                value: "true".into(),
            },
        },
    }
}

fn authorized_first() -> FiniteExpr {
    FiniteExpr::Equal {
        left: FiniteTerm::Observable {
            mapping: "authorized".into(),
            arguments: vec![FiniteTerm::Literal {
                value: FiniteValue {
                    domain: "attempt".into(),
                    value: "first".into(),
                },
            }],
        },
        right: FiniteTerm::Literal {
            value: FiniteValue {
                domain: "bool".into(),
                value: "true".into(),
            },
        },
    }
}

fn forall_attempt() -> FiniteExpr {
    FiniteExpr::Forall {
        variable: "attempt".into(),
        domain: "attempt".into(),
        expression: Box::new(authorized("attempt")),
    }
}

fn context() -> CaseContext {
    CaseContext {
        subject_blake3: digest('a'),
        policy_blake3: digest('b'),
        target: "test-target".into(),
        environment: "test-environment".into(),
        configurations: BTreeMap::from([("effect.dispatch".into(), digest('c'))]),
    }
}

fn interface(
    id: &str,
    configuration: &str,
    change_kind: InterfaceChange,
    order_sensitive: bool,
) -> AffectedInterface {
    AffectedInterface {
        id: id.into(),
        configuration_blake3: configuration.into(),
        change_kind,
        operation: format!("{}-operation", change_kind.tag()),
        order_sensitive,
    }
}

fn footprint(change_id: &str, revisions: Vec<FootprintRevision>) -> ChangeFootprint {
    ChangeFootprint {
        schema: FOOTPRINT_SCHEMA_V1.into(),
        change: ChangeIdentity {
            change_id: change_id.into(),
            source_revision: format!("source-{change_id}"),
            intent_revision: format!("intent-{change_id}"),
        },
        revisions,
        assumptions: Vec::new(),
        guarantees: Vec::new(),
        invariants: Vec::new(),
        invalidated: Vec::new(),
        interfaces: Vec::new(),
        evidence_basis: Vec::new(),
        provided_conditions: Vec::new(),
        unknown: Vec::new(),
    }
}

fn composition(
    id: &str,
    participants: &[&str],
    order: &[&str],
    fixed: bool,
) -> CompositionCandidate {
    CompositionCandidate {
        id: id.into(),
        participants: participants.iter().map(|value| (*value).into()).collect(),
        order: order.iter().map(|value| (*value).into()).collect(),
        fixed,
        admitted: true,
    }
}

fn compatible_case() -> ConcurrentChangeCase {
    let alpha_row = row("alpha.requirement", '1');
    let alpha_revision = revision(
        alpha_row.clone(),
        BTreeMap::from([("bool".to_string(), bool_domain())]),
        BTreeMap::from([
            ("flag".to_string(), boolean_observable()),
            ("guard".to_string(), boolean_observable()),
        ]),
        and(is_true("flag"), is_true("guard")),
    );
    let previous_form = StatementForm::Finite {
        expression: and(is_true("flag"), is_true("guard")),
    };
    let previous_identity =
        footprint_guarantee_identity(&alpha_row.row_blake3, "alpha.feature", &previous_form)
            .unwrap();

    let mut alpha = footprint("alpha", vec![alpha_revision]);
    alpha.guarantees.push(ChangedGuarantee {
        id: "alpha.feature".into(),
        previous: Some(GuaranteeStatement {
            requirement_id: "alpha.requirement".into(),
            form: previous_form,
        }),
        current: Some(GuaranteeStatement {
            requirement_id: "alpha.requirement".into(),
            form: StatementForm::Finite {
                expression: is_true("flag"),
            },
        }),
        change: GuaranteeChange::Replaced,
        affected_claims: vec!["claim.alpha".into()],
        interfaces: vec!["effect.dispatch".into()],
        configurations: vec![digest('c')],
        unknown: None,
    });
    alpha.interfaces.push(interface(
        "effect.dispatch",
        &digest('c'),
        InterfaceChange::Modify,
        false,
    ));

    let mut beta = footprint("beta", vec![simple_revision("beta.requirement", '2')]);
    beta.assumptions.push(ConsumedAssumption {
        id: "beta.feature".into(),
        requirement_id: "beta.requirement".into(),
        consumed_change: "alpha".into(),
        consumed_guarantee: "alpha.feature".into(),
        consumed_identity: previous_identity,
        dependency: AssumptionDependency::Monotone,
        meaning: "the feature flag stays enabled".into(),
        finite_predicate: Some(is_true("flag")),
        evidence_basis: vec!["claim.beta".into()],
        configurations: vec![digest('c')],
        unknown: None,
    });
    beta.interfaces.push(interface(
        "effect.dispatch",
        &digest('c'),
        InterfaceChange::Modify,
        false,
    ));

    ConcurrentChangeCase {
        schema: CASE_SCHEMA_V1.into(),
        profile: profile(),
        context: context(),
        changes: vec![alpha, beta],
        compositions: vec![composition(
            "c1",
            &["alpha", "beta"],
            &["alpha", "beta"],
            true,
        )],
        joint_evidence: Vec::new(),
        individual_receipts: Vec::new(),
        prior_obligations: Vec::new(),
    }
}

fn retry_case() -> ConcurrentChangeCase {
    let authorization_row = row("authorization.requirement", '3');
    let authorization_revision = revision(
        authorization_row.clone(),
        BTreeMap::from([
            ("bool".to_string(), bool_domain()),
            (
                "attempt".to_string(),
                FiniteDomain {
                    values: vec!["first".into(), "second".into()],
                },
            ),
        ]),
        BTreeMap::from([("authorized".to_string(), attempt_observable())]),
        authorized_first(),
    );
    let previous_form = StatementForm::Finite {
        expression: forall_attempt(),
    };
    let previous_identity = footprint_guarantee_identity(
        &authorization_row.row_blake3,
        "authorization.dispatch",
        &previous_form,
    )
    .unwrap();

    let mut authorization = footprint("authorization", vec![authorization_revision]);
    authorization.guarantees.push(ChangedGuarantee {
        id: "authorization.dispatch".into(),
        previous: Some(GuaranteeStatement {
            requirement_id: "authorization.requirement".into(),
            form: previous_form,
        }),
        current: Some(GuaranteeStatement {
            requirement_id: "authorization.requirement".into(),
            form: StatementForm::Finite {
                expression: authorized_first(),
            },
        }),
        change: GuaranteeChange::Weakened,
        affected_claims: vec!["claim.authorization".into()],
        interfaces: vec!["effect.dispatch".into()],
        configurations: vec![digest('c')],
        unknown: None,
    });
    authorization.interfaces.push(interface(
        "effect.dispatch",
        &digest('c'),
        InterfaceChange::Modify,
        true,
    ));

    let mut retry = footprint("retry", vec![simple_revision("retry.requirement", '4')]);
    retry.assumptions.push(ConsumedAssumption {
        id: "retry.authorization".into(),
        requirement_id: "retry.requirement".into(),
        consumed_change: "authorization".into(),
        consumed_guarantee: "authorization.dispatch".into(),
        consumed_identity: previous_identity,
        dependency: AssumptionDependency::Monotone,
        meaning: "authorization is established immediately before every attempted effect".into(),
        finite_predicate: Some(forall_attempt()),
        evidence_basis: vec!["claim.retry".into()],
        configurations: vec![digest('c')],
        unknown: None,
    });
    retry.invariants.push(PreservedInvariant {
        id: "authorization-before-effect".into(),
        requirement_id: "retry.requirement".into(),
        commitment: StatementForm::Informal {
            statement: "authorization precedes every attempted effect".into(),
        },
        interfaces: vec!["effect.dispatch".into()],
        configurations: vec![digest('c')],
        under_assumptions: vec!["retry.authorization".into()],
        required_conditions: vec![IntegrationCondition {
            id: "revalidate-authorization-per-attempt".into(),
            statement: "retry must revalidate authorization immediately before each retry attempt"
                .into(),
        }],
    });
    retry.interfaces.push(interface(
        "effect.dispatch",
        &digest('c'),
        InterfaceChange::Modify,
        true,
    ));

    ConcurrentChangeCase {
        schema: CASE_SCHEMA_V1.into(),
        profile: profile(),
        context: context(),
        changes: vec![authorization, retry],
        compositions: vec![composition(
            "authorization-then-retry",
            &["authorization", "retry"],
            &["authorization", "retry"],
            true,
        )],
        joint_evidence: Vec::new(),
        individual_receipts: Vec::new(),
        prior_obligations: Vec::new(),
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
            "cairn-footprints-cli-{name}-{}-{nanos}",
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

    fn write_raw(&self, name: &str, bytes: &[u8]) -> String {
        let path = self.dir.join(name);
        fs::write(&path, bytes).unwrap();
        path.to_str().unwrap().to_string()
    }

    fn inspect(&self, case: &str, extra: &[&str], success: bool) -> Value {
        let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .args(["footprints", "inspect", "--case", case])
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
        serde_json::from_slice(&output.stdout).unwrap_or(Value::Null)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

#[test]
// r[verify cairn.concurrent_footprints.inspection]
fn cli_analyzes_a_compatible_footprint_pair_as_advisory() {
    let fixture = Fixture::new("compatible");
    let policy_path = fixture.write("policy.json", &serde_json::to_value(policy()).unwrap());
    let case = compatible_case();
    let case_path = fixture.write("case.json", &serde_json::to_value(&case).unwrap());
    let value = fixture.inspect(&case_path, &["--policy", &policy_path], true);
    assert_eq!(
        value["schema_version"],
        "cairn.concurrent-change-footprints.inspection.v1"
    );
    assert_eq!(value["outcome"], "analyzed");
    assert_eq!(value["advisory"], true);
    assert_eq!(value["lifecycle_authority"], "none");
    assert_eq!(
        value["report"]["obligations"][0]["outcome"],
        "needs_joint_evidence"
    );
    assert_eq!(
        value["report"]["interactions"][0]["kind"],
        "changed_guarantee_affects_assumption"
    );
    assert!(value["report"]["report_blake3"].as_str().unwrap().len() == 64);
    assert!(value["boundary"]
        .as_str()
        .unwrap()
        .contains("remain independently usable"));
}

#[test]
// r[verify cairn.concurrent_footprints.inspection]
fn cli_explains_retry_authorization_conflict_and_missing_condition() {
    let fixture = Fixture::new("retry");
    let policy_path = fixture.write("policy.json", &serde_json::to_value(policy()).unwrap());
    let case_path = fixture.write("case.json", &serde_json::to_value(retry_case()).unwrap());
    let value = fixture.inspect(&case_path, &["--policy", &policy_path], true);
    assert_eq!(value["outcome"], "analyzed");
    assert_eq!(value["report"]["obligations"][0]["outcome"], "conflict");
    assert_eq!(
        value["report"]["obligations"][0]["affected_invariants"][0],
        "authorization-before-effect"
    );
    assert_eq!(
        value["report"]["obligations"][0]["integration_conditions"][0]["state"],
        "missing"
    );
    assert!(value["report"]["interactions"][0]["finite"]["witness"].is_object());
    let path = value["report"]["obligations"][0]["path"]
        .as_array()
        .unwrap()
        .iter()
        .map(|step| step["kind"].as_str().unwrap_or_default())
        .collect::<Vec<&str>>();
    assert!(path.contains(&"guarantee_change"));
    assert!(path.contains(&"assumption_consumption"));
    assert!(path.contains(&"outcome"));
}

#[test]
// r[verify cairn.concurrent_footprints.inspection]
// r[verify cairn.concurrent_footprints.outcomes]
fn cli_accepts_joint_evidence_for_the_exact_combined_candidate() {
    let fixture = Fixture::new("supported");
    let policy_path = fixture.write("policy.json", &serde_json::to_value(policy()).unwrap());
    let mut case = compatible_case();
    let report = analyze_concurrent_changes(&policy(), &case).unwrap();
    let obligation = &report.obligations[0];
    case.joint_evidence = vec![JointEvidence {
        id: "joint.evidence".into(),
        obligation: obligation.id.clone(),
        combined_candidate_blake3: obligation.candidate.combined_candidate_blake3.clone(),
        participants: obligation.participants.clone(),
        compositions: obligation.candidate.compositions.clone(),
        context: case.context.clone(),
        affected_claims: obligation.affected_claims.clone(),
        propositions: vec![
            PropositionEvidence {
                proposition: PropositionKind::Coexistence,
                state: PropositionState::Supported,
                basis: "bounded joint check".into(),
            },
            PropositionEvidence {
                proposition: PropositionKind::InvariantPreservation,
                state: PropositionState::Supported,
                basis: "bounded invariant check".into(),
            },
        ],
        assurance: AssuranceScope::BoundedCheck,
        scope: EvidenceScope::AllAdmittedCompositions,
        artifact_blake3: digest('e'),
    }];
    let case_path = fixture.write("case.json", &serde_json::to_value(&case).unwrap());
    let value = fixture.inspect(&case_path, &["--policy", &policy_path], true);
    assert_eq!(
        value["report"]["obligations"][0]["outcome"],
        "supported_within_declared_scope"
    );
    assert_eq!(
        value["report"]["obligations"][0]["propositions"]["invariant_preservation"],
        "supported"
    );
    assert_eq!(
        value["report"]["obligations"][0]["propositions"]["commutativity"],
        "not_applicable"
    );
    assert_eq!(
        value["report"]["obligations"][0]["evidence"]["admitted"][0],
        "joint.evidence"
    );
    assert_eq!(
        value["report"]["individual_receipts_do_not_discharge"],
        true
    );
}

#[test]
// r[verify cairn.concurrent_footprints.inspection]
fn cli_rejects_malformed_input_and_unknown_options() {
    let fixture = Fixture::new("rejections");
    let policy_path = fixture.write("policy.json", &serde_json::to_value(policy()).unwrap());
    let malformed = fixture.write_raw("malformed.json", b"{not json");
    let rejected = fixture.inspect(&malformed, &["--policy", &policy_path], false);
    assert_eq!(rejected["outcome"], "rejected");
    assert_eq!(rejected["error"]["outcome"], "invalid_input");
    assert_eq!(rejected["lifecycle_authority"], "none");

    let unsupported_policy = fixture.write_raw("bad-policy.json", b"{\"schema\":\"other\"}");
    let case = fixture.write(
        "case.json",
        &serde_json::to_value(compatible_case()).unwrap(),
    );
    let rejected = fixture.inspect(&case, &["--policy", &unsupported_policy], false);
    assert_eq!(rejected["outcome"], "rejected");
    assert_eq!(rejected["error"]["outcome"], "unsupported");

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args(["footprints", "inspect", "--case", &case, "--bogus"])
        .output()
        .unwrap();
    assert!(!output.status.success());

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args(["footprints", "inspect"])
        .output()
        .unwrap();
    assert!(!output.status.success());

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args([
            "footprints",
            "inspect",
            "--case",
            &case,
            "--root",
            fixture.dir.to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
}

#[test]
// r[verify cairn.concurrent_footprints.inspection]
fn cli_preserves_order_sensitive_commutativity_as_a_separate_proposition() {
    let fixture = Fixture::new("order");
    let policy_path = fixture.write("policy.json", &serde_json::to_value(policy()).unwrap());
    let mut case = compatible_case();
    case.compositions.push(composition(
        "c2",
        &["alpha", "beta"],
        &["beta", "alpha"],
        false,
    ));
    for change in &mut case.changes {
        if let Some(interface) = change.interfaces.first_mut() {
            interface.order_sensitive = true;
        }
    }
    let case_path = fixture.write("case.json", &serde_json::to_value(&case).unwrap());
    let value = fixture.inspect(&case_path, &["--policy", &policy_path], true);
    assert_eq!(
        value["report"]["obligations"][0]["propositions"]["commutativity"],
        "not_established"
    );
    assert_eq!(
        value["report"]["obligations"][0]["commutativity_required"],
        true
    );
    assert_eq!(
        value["report"]["obligations"][0]["outcome"],
        "needs_joint_evidence"
    );
}
