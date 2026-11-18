use std::{collections::BTreeMap, env, fs, path::PathBuf, process::Command, time::SystemTime};

use cairn_core::{
    authoritative_command_hash, authoritative_selection_hash, claim_identity,
    evaluate_content_bound_requirement_registry, normalize_contract, AgentRequiredCheck,
    ClaimContext, ContentBoundRequirementRow, ContentBoundRevision, ContentBoundSpecificationInput,
    CorrespondenceReview, Derivation, EvaluationRequest, FiniteDomain, FiniteExpr, ProducerInput,
    Proposition, Rule, ScopedClaim, SemanticContract, SemanticProfile, CLAIM_SCHEMA_V1,
};
use serde_json::{json, Value};

const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const REVIEWER: &str = "fixture-independent-reviewer";
const REQUIREMENT_ID: &str = "cairn.supersede.agent_run_receipts.verification_contract";
const POLICY: &str = include_str!("../../../cairn-policy/generated/scoped-claim-derivations.json");

struct Fixture {
    root: PathBuf,
    row: ContentBoundRequirementRow,
    repository_id: String,
}

impl Fixture {
    fn new() -> Self {
        let root = env::temp_dir().join(format!(
            "cairn-scoped-claims-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let markdown = include_str!("../../../.cairn/specs/agent-completion/spec.md");
        let path = ".cairn/specs/agent-completion/spec.md".to_string();
        fs::create_dir_all(root.join(".cairn/specs/agent-completion")).unwrap();
        fs::write(root.join(&path), markdown).unwrap();
        fs::write(root.join("policy.json"), b"{}").unwrap();
        let repository_id = "scoped-claims-cli-fixture".to_string();
        let registry = evaluate_content_bound_requirement_registry(
            vec![ContentBoundSpecificationInput {
                path: path.clone(),
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
        let row = registry
            .rows
            .into_iter()
            .find(|row| row.requirement_id == REQUIREMENT_ID)
            .expect("native requirement row exists");
        Self {
            root,
            row,
            repository_id,
        }
    }

    fn profile(&self) -> SemanticProfile {
        SemanticProfile {
            id: "cairn.requirement-semantics.finite.v1".into(),
            interpretation:
                "finite-enumeration.equality.nominal-domains.total-observables.single-step.v1"
                    .into(),
            max_nodes: 64,
            max_depth: 8,
            max_domain_values: 8,
            max_steps: 1000,
        }
    }

    fn contract(&self) -> SemanticContract {
        SemanticContract {
            profile: self.profile().id,
            proposed_by: "scoped-claims-cli-author".into(),
            requirement: self.row.clone(),
            domains: BTreeMap::from([(
                "bool".to_string(),
                FiniteDomain {
                    values: vec!["false".into(), "true".into()],
                },
            )]),
            observables: BTreeMap::new(),
            assumptions: Vec::new(),
            activation: FiniteExpr::Bool { value: true },
            guarantee: FiniteExpr::Bool { value: true },
            supported_portion: "scoped claim derivation CLI case".into(),
            informal_remainder: "not an admitted behavior claim".into(),
        }
    }

    fn context(&self) -> ClaimContext {
        ClaimContext {
            subject_blake3: HASH_C.into(),
            policy_blake3: HASH_B.into(),
            target: self.repository_id.clone(),
            environment: "linux-x86_64".into(),
            bounds: BTreeMap::from([("max_checks".to_string(), "2".to_string())]),
        }
    }

    fn completion(&self, clippy_status: &str) -> ProducerInput {
        let tests_command = vec!["cargo".to_string(), "test".to_string()];
        let clippy_command = vec!["cargo".to_string(), "clippy".to_string()];
        let mut checks = vec![
            AgentRequiredCheck {
                check_id: "tests".into(),
                command: tests_command.clone(),
                command_hash: authoritative_command_hash(&tests_command),
                prerequisites: Vec::new(),
            },
            AgentRequiredCheck {
                check_id: "clippy".into(),
                command: clippy_command.clone(),
                command_hash: authoritative_command_hash(&clippy_command),
                prerequisites: vec!["tests".into()],
            },
        ];
        checks.sort_by(|left, right| left.check_id.cmp(&right.check_id));
        let selection_hash = authoritative_selection_hash(&checks);
        let contract = json!({
            "schema": "cairn.agent-completion.v2",
            "repository_id": self.repository_id,
            "registry_id": "cairn-release-checks",
            "registry_hash": HASH_A,
            "selection_hash": selection_hash.clone(),
            "policy_hash": HASH_B,
            "final_subject_hash": HASH_C,
            "required_checks": checks.clone(),
        });
        let results = json!([
            {
                "check_id": "tests", "status": "passed",
                "command": tests_command, "command_hash": checks[1].command_hash,
                "subject_hash": HASH_C, "registry_hash": HASH_A,
                "selection_hash": selection_hash.clone(), "policy_hash": HASH_B,
                "evidence_hash": HASH_A, "execution_receipt_hash": HASH_B
            },
            {
                "check_id": "clippy", "status": clippy_status,
                "command": clippy_command, "command_hash": checks[0].command_hash,
                "subject_hash": HASH_C, "registry_hash": HASH_A,
                "selection_hash": selection_hash.clone(), "policy_hash": HASH_B,
                "evidence_hash": HASH_A, "execution_receipt_hash": HASH_B
            }
        ]);
        ProducerInput::AuthoritativeCompletion {
            context: self.context(),
            contract,
            results,
            trust_dependencies: std::collections::BTreeSet::from([
                "native-test-oracle:shared-cargo-test-harness".to_string(),
            ]),
        }
    }

    fn claim(
        &self,
        interpretation_blake3: &str,
        contract_blake3: &str,
        proposition: Proposition,
    ) -> ScopedClaim {
        ScopedClaim {
            schema: CLAIM_SCHEMA_V1.into(),
            kind: proposition.claim_kind(),
            requirement: self.row.clone(),
            interpretation_blake3: interpretation_blake3.into(),
            contract_blake3: contract_blake3.into(),
            context: self.context(),
            proposition,
        }
    }

    /// Builds the request and the derivation scopes admitted by the generated policy.
    fn case(&self, clippy_status: &str) -> (EvaluationRequest, BTreeMap<&'static str, String>) {
        let profile = self.profile();
        let contract = self.contract();
        let normalized = normalize_contract(&profile, &self.row, &contract).unwrap();
        let clippy = self.claim(
            &normalized.interpretation_blake3,
            &normalized.contract_blake3,
            Proposition::CheckPassed {
                check_id: "clippy".into(),
            },
        );
        let tests = self.claim(
            &normalized.interpretation_blake3,
            &normalized.contract_blake3,
            Proposition::CheckPassed {
                check_id: "tests".into(),
            },
        );
        let eligible = self.claim(
            &normalized.interpretation_blake3,
            &normalized.contract_blake3,
            Proposition::NativeCompletionEligible,
        );
        let reviewed = self.claim(
            &normalized.interpretation_blake3,
            &normalized.contract_blake3,
            Proposition::ReviewedCorrespondence,
        );
        let decision = self.claim(
            &normalized.interpretation_blake3,
            &normalized.contract_blake3,
            Proposition::ReviewedCompletionDecision,
        );
        let mut ids = BTreeMap::new();
        for (key, claim) in [
            ("clippy", &clippy),
            ("tests", &tests),
            ("eligible", &eligible),
            ("reviewed", &reviewed),
            ("decision", &decision),
        ] {
            ids.insert(key, claim_identity(claim).unwrap());
        }
        let policy: Value = serde_json::from_str(POLICY).unwrap();
        let assurance_scope = policy["rules"]
            .as_array()
            .unwrap()
            .iter()
            .find(|rule| rule["id"] == "reviewed_completion_v1")
            .expect("generated policy admits reviewed completion")["assurance_scope"]
            .as_str()
            .unwrap()
            .to_string();
        let review = CorrespondenceReview {
            requirement_row_blake3: self.row.row_blake3.clone(),
            contract_blake3: normalized.contract_blake3.clone(),
            interpretation_blake3: normalized.interpretation_blake3.clone(),
            mapping_blake3: normalized.mapping_blake3.clone(),
            proposed_by: normalized.contract.proposed_by.clone(),
            reviewer: REVIEWER.into(),
            review_artifact_blake3: "d".repeat(64),
            decision: "approved".into(),
        };
        let derivation = Derivation {
            id: "derivation.completion".into(),
            rule: Rule::ReviewedCompletionV1,
            premises: vec![ids["eligible"].clone(), ids["reviewed"].clone()],
            conclusion: ids["decision"].clone(),
            applicability: self.context(),
            assurance_scope,
            residual_assumptions: std::collections::BTreeSet::from([
                "reviewed-correspondence:accepted-declaration".to_string(),
            ]),
        };
        (
            EvaluationRequest {
                profile,
                native_requirement: self.row.clone(),
                semantic_contract: contract,
                correspondence_review: Some(review),
                authorized_reviewers: vec![REVIEWER.into()],
                claims: vec![clippy, tests, eligible, reviewed, decision],
                producers: vec![self.completion(clippy_status)],
                derivations: vec![derivation],
            },
            ids,
        )
    }

    fn explain(&self, name: &str, request: &EvaluationRequest, success: bool) -> Value {
        let path = self.root.join(format!("{name}.json"));
        fs::write(&path, serde_json::to_vec(request).unwrap()).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .args([
                "claim-derivation",
                "explain",
                "--case",
                path.to_str().unwrap(),
            ])
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
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn explanation<'a>(report: &'a Value, proposition: &str, check_id: Option<&str>) -> &'a Value {
    report["claims"]
        .as_array()
        .unwrap()
        .iter()
        .find(|claim| {
            claim["claim"]["proposition"]["kind"].as_str() == Some(proposition)
                && check_id.is_none_or(|expected| {
                    claim["claim"]["proposition"]["check_id"].as_str() == Some(expected)
                })
        })
        .expect("explained claim exists")
}

#[test]
// r[verify cairn.scoped_claims.inspection]
fn advisory_explanation_covers_one_actual_native_completion_case() {
    let fixture = Fixture::new();
    let (request, ids) = fixture.case("passed");
    let before = fs::read(fixture.root.join(".cairn/specs/agent-completion/spec.md")).unwrap();
    let output = fixture.explain("positive", &request, true);
    assert_eq!(output["outcome"], "evaluated");
    assert_eq!(output["advisory"], true);
    assert_eq!(output["report"]["advisory"], true);
    assert_eq!(
        output["report"]["schema"],
        "cairn.scoped-derivation-report.v1"
    );
    assert_eq!(
        output["report"]["independence"],
        cairn_core::INDEPENDENCE_NOT_ESTABLISHED
    );
    assert_eq!(output["report"]["unresolved"], json!([]));

    let decision = explanation(&output["report"], "reviewed_completion_decision", None);
    assert_eq!(decision["state"], "supported");
    assert_eq!(decision["kind"], "bounded_property");
    assert_eq!(decision["support"][0]["rule"], "reviewed_completion_v1");
    assert_eq!(decision["support"][0]["version"], 1);
    assert_eq!(
        decision["support"][0]["premises"],
        json!([ids["eligible"], ids["reviewed"]])
    );
    assert_eq!(
        decision["support"][0]["applicability"],
        json!(request.claims[0].context)
    );

    let clippy = explanation(&output["report"], "check_passed", Some("clippy"));
    assert_eq!(clippy["state"], "supported");
    assert_eq!(clippy["kind"], "observation");
    assert_eq!(clippy["support"][0]["rule"], "native-check-observation.v1");
    assert!(clippy["residual_assumptions"]
        .as_array()
        .unwrap()
        .iter()
        .any(|assumption| assumption == "native-test-oracle:adequacy-not-established"));
    assert!(output["report"]["trust_dependencies"]
        .as_array()
        .unwrap()
        .iter()
        .any(|dependency| dependency == "native-test-oracle:shared-cargo-test-harness"));

    // Read-only: the accepted specification and lifecycle layout are unchanged.
    assert_eq!(
        before,
        fs::read(fixture.root.join(".cairn/specs/agent-completion/spec.md")).unwrap()
    );
    assert!(!fixture.root.join(".cairn/changes").exists());

    let replay = fixture.explain("positive-replay", &request, true);
    assert_eq!(
        output["report"]["report_blake3"],
        replay["report"]["report_blake3"]
    );
    assert_eq!(
        output["report"]["input_blake3"],
        replay["report"]["input_blake3"]
    );
}

#[test]
// r[verify cairn.scoped_claims.states]
fn timeout_withholds_support_and_refutation_for_the_completion_decision() {
    let fixture = Fixture::new();
    let (request, ids) = fixture.case("timed_out");
    let output = fixture.explain("timeout", &request, true);
    assert_eq!(output["outcome"], "evaluated");
    let clippy = explanation(&output["report"], "check_passed", Some("clippy"));
    assert_eq!(clippy["state"], "unresolved");
    assert_eq!(clippy["support"], json!([]));
    assert_eq!(clippy["counterevidence"], json!([]));
    let decision = explanation(&output["report"], "reviewed_completion_decision", None);
    assert_eq!(decision["state"], "unresolved");
    assert_eq!(decision["support"], json!([]));
    assert_eq!(decision["counterevidence"], json!([]));
    assert!(decision["unresolved_premises"]
        .as_array()
        .unwrap()
        .iter()
        .any(|premise| premise == &ids["eligible"]));
    assert!(output["report"]["unresolved"]
        .as_array()
        .unwrap()
        .iter()
        .any(|claim| claim == &ids["decision"]));
}

#[test]
// r[verify cairn.scoped_claims.scope]
// r[verify cairn.scoped_claims.rules]
fn rejected_scopes_and_rules_are_visible_nonaccepting_outcomes() {
    let fixture = Fixture::new();
    let (mut request, _) = fixture.case("passed");
    request.claims[0].requirement.row_blake3 = "e".repeat(64);
    let output = fixture.explain("scope", &request, false);
    assert_eq!(output["outcome"], "rejected");
    assert_eq!(output["error"]["outcome"], "scope_mismatch");

    let (mut request, _) = fixture.case("passed");
    request.derivations[0].assurance_scope = "unbounded behavior".into();
    let output = fixture.explain("rule", &request, false);
    assert_eq!(output["outcome"], "rejected");
    assert_eq!(output["error"]["outcome"], "unsupported");

    fs::write(fixture.root.join("malformed.json"), b"{").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args([
            "claim-derivation",
            "explain",
            "--case",
            fixture.root.join("malformed.json").to_str().unwrap(),
        ])
        .output()
        .unwrap();
    assert!(!output.status.success());
    let rejected: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(rejected["outcome"], "rejected");
    assert_eq!(rejected["error"]["outcome"], "invalid_input");
    assert_eq!(rejected["advisory"], true);
}

#[test]
fn cli_rejects_unknown_arguments_without_writing() {
    let fixture = Fixture::new();
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args(["claim-derivation", "explain", "--unknown"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!fixture.root.join(".cairn/changes").exists());
}
