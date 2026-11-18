use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::PathBuf,
    process::Command,
    time::SystemTime,
};

use cairn_core::{
    ArgumentScope, Assurance, ChildGuarantee, Composition, ContentBoundRequirementRow,
    Coverage, DependencyContext, DependencyEndpoint, DependencyEvidence, DependencyRelation,
    EvidenceCoverage, Feasibility, FeasibilityDisposition, ImplicationRelation,
    RefinementPolicy, RefinementRequest, RequirementRevision, TypedDependency,
    DEPENDENCY_SCHEMA_V1, REFINEMENT_POLICY_SCHEMA_V1, REFINEMENT_SCHEMA_V1,
};
use serde_json::Value;

const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";
const HASH_D: &str = "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd";
const POLICY: &str = include_str!("../../../cairn-policy/generated/refinement-typed-dependencies.json");

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let root = env::temp_dir().join(format!(
            "cairn-refinement-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(root.join(".cairn/specs/refinement")).unwrap();
        fs::write(
            root.join(".cairn/specs/refinement/spec.md"),
            "# refinement fixture\n",
        )
        .unwrap();
        Self { root }
    }

    fn explain(&self, name: &str, request: &RefinementRequest, extra: &[&str]) -> (Value, bool) {
        let path = self.root.join(format!("{name}.json"));
        fs::write(&path, serde_json::to_vec(request).unwrap()).unwrap();
        let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
        command.args(["refinement", "explain", "--case", path.to_str().unwrap()]);
        command.args(extra);
        command.current_dir(&self.root);
        let output = command.output().unwrap();
        let value = serde_json::from_slice(&output.stdout).unwrap_or(Value::Null);
        (value, output.status.success())
    }

    fn explain_policy(
        &self,
        name: &str,
        request: &RefinementRequest,
        policy: &RefinementPolicy,
    ) -> (Value, bool) {
        let path = self.root.join(format!("{name}.json"));
        fs::write(&path, serde_json::to_vec(request).unwrap()).unwrap();
        let policy_path = self.root.join(format!("{name}-policy.json"));
        fs::write(&policy_path, serde_json::to_vec(policy).unwrap()).unwrap();
        let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .args([
                "refinement",
                "explain",
                "--case",
                path.to_str().unwrap(),
                "--policy",
                policy_path.to_str().unwrap(),
            ])
            .current_dir(&self.root)
            .output()
            .unwrap();
        let value = serde_json::from_slice(&output.stdout).unwrap_or(Value::Null);
        (value, output.status.success())
    }
}

impl Drop for Fixture {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.root);
    }
}

fn revision(index: u64) -> RequirementRevision {
    RequirementRevision {
        requirement: ContentBoundRequirementRow {
            requirement_id: format!("cairn.test.refinement.cli.{index}"),
            specification_path: ".cairn/specs/refinement/spec.md".into(),
            specification_blake3: HASH_A.into(),
            source_span:
                cairn_core::ContentBoundSourceSpan {
                    start_byte: 0,
                    end_byte: 32,
                    start_line: 1,
                    end_line: 1,
                },
            requirement_body_blake3: format!("{index:064x}"),
            row_blake3: format!("{:064x}", index + 10_000),
        },
        interpretation_blake3: HASH_B.into(),
        contract_blake3: HASH_C.into(),
    }
}

fn scope() -> ArgumentScope {
    ArgumentScope {
        subject_blake3: HASH_C.into(),
        policy_blake3: HASH_B.into(),
        target: "onixresearch/cairn".into(),
        environment: "linux-x86_64".into(),
        bounds: BTreeMap::from([("max_checks".to_string(), "2".to_string())]),
    }
}

fn request_with_dependency() -> RefinementRequest {
    let dependency = TypedDependency {
        schema: DEPENDENCY_SCHEMA_V1.into(),
        id: "dependency.cli".into(),
        relation: DependencyRelation::OrderingArchive,
        producer: DependencyEndpoint {
            requirement: revision(1),
            work: "producer-change".into(),
            revision: "revision-a".into(),
        },
        consumer: DependencyEndpoint {
            requirement: revision(2),
            work: "consumer-change".into(),
            revision: "revision-b".into(),
        },
        context: DependencyContext {
            producer_repository: "onixresearch/kiln".into(),
            consumer_repository: "onixresearch/cairn".into(),
            intended_target: "kiln-main".into(),
            environment: "linux-x86_64".into(),
            policy_blake3: HASH_B.into(),
            bounds: BTreeMap::from([("max_tasks".to_string(), "4".to_string())]),
        },
        coverage: Coverage::WholeChange,
        required_gates: Vec::new(),
        required_input_blake3: None,
        required_lifecycle_stage: None,
        semantic: None,
    };
    RefinementRequest {
        decompositions: Vec::new(),
        guarantee_cases: Vec::new(),
        dependencies: vec![dependency],
        dependency_evidence: vec![DependencyEvidence::Archive {
            dependency_id: "dependency.cli".into(),
            change: "producer-change".into(),
            repository: "onixresearch/kiln".into(),
            revision: "revision-a".into(),
            archive_receipt_blake3: HASH_D.into(),
            coverage: EvidenceCoverage::WholeChange,
        }],
        integration_evidence: Vec::new(),
        applicability: Vec::new(),
        previous_basis: None,
    }
}

fn request_with_unobserved_condition() -> RefinementRequest {
    let parent = revision(10);
    let child_revision = revision(11);
    let decomposition = cairn_core::Decomposition {
        schema: REFINEMENT_SCHEMA_V1.into(),
        id: "decomposition.cli".into(),
        parent,
        scope: scope(),
        children: vec![ChildGuarantee {
            id: "child.cli".into(),
            requirement: child_revision,
            scope: scope(),
            guarantee_case: "guarantee.missing".into(),
            claim_id: "claim.missing".into(),
        }],
        environment: Vec::new(),
        integration: Vec::new(),
        composition: Composition::Conditional {
            condition: "integration.ready".into(),
            when_satisfied: vec![Composition::AllOf {
                obligations: vec![Composition::Child {
                    child: "child.cli".into(),
                }],
            }],
            when_unsatisfied: vec![Composition::AllOf {
                obligations: vec![Composition::Child {
                    child: "child.cli".into(),
                }],
            }],
        },
        relation: ImplicationRelation {
            assurance: Assurance::Reviewed,
            witness_blake3: HASH_D.into(),
            statement: "reviewed CLI argument".into(),
            proof_checker: None,
        },
        feasibility: FeasibilityDisposition {
            disposition: Feasibility::Feasible,
            method: "reviewed-argument.v1".into(),
            assurance: Assurance::Reviewed,
            justification_blake3: HASH_D.into(),
            covered: BTreeSet::from(["child.cli".into()]),
            proof_checker: None,
        },
    };
    RefinementRequest {
        decompositions: vec![decomposition],
        guarantee_cases: Vec::new(),
        dependencies: Vec::new(),
        dependency_evidence: Vec::new(),
        integration_evidence: Vec::new(),
        applicability: Vec::new(),
        previous_basis: None,
    }
}

#[test]
// r[verify cairn.refinement.inspection]
fn read_only_explanation_of_a_supported_dependency() {
    let fixture = Fixture::new();
    let before = fs::read(fixture.root.join(".cairn/specs/refinement/spec.md")).unwrap();
    let (output, success) = fixture.explain("positive", &request_with_dependency(), &[]);
    assert!(success);
    assert_eq!(output["outcome"], "evaluated");
    assert_eq!(output["schema_version"], "cairn.refinement.explanation.v1");
    assert_eq!(output["advisory"], true);
    assert_eq!(
        output["report"]["schema"],
        "cairn.refinement-argument-report.v1"
    );
    assert_eq!(
        output["report"]["dependencies"][0]["state"],
        "supported"
    );
    assert_eq!(output["boundary"], cairn_core::REFINEMENT_BOUNDARY);
    let after = fs::read(fixture.root.join(".cairn/specs/refinement/spec.md")).unwrap();
    assert_eq!(before, after);
    let lifecycle_entries = fs::read_dir(fixture.root.join(".cairn")).unwrap().count();
    assert_eq!(lifecycle_entries, 1);
}

#[test]
// r[verify cairn.refinement.inspection]
fn pending_argument_is_an_explicit_successful_explanation() {
    let fixture = Fixture::new();
    let (output, success) = fixture.explain("pending", &request_with_unobserved_condition(), &[]);
    assert!(success);
    assert_eq!(output["outcome"], "evaluated");
    assert_eq!(
        output["report"]["decompositions"][0]["state"],
        "pending"
    );
    assert!(output["report"]["decompositions"][0]["reason"]
        .as_str()
        .unwrap()
        .contains("condition_unobserved"));
}

#[test]
// r[verify cairn.refinement.inspection]
fn malformed_case_and_policy_are_rejected() {
    let fixture = Fixture::new();
    let path = fixture.root.join("malformed.json");
    fs::write(&path, b"{\"decompositions\": []}").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args(["refinement", "explain", "--case", path.to_str().unwrap()])
        .current_dir(&fixture.root)
        .output()
        .unwrap();
    assert!(!output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout).unwrap();
    assert_eq!(value["outcome"], "rejected");
    assert_eq!(value["error"]["outcome"], "invalid_input");

    let mut policy: RefinementPolicy = serde_json::from_str(POLICY).unwrap();
    policy.schema = "cairn.refinement-policy.v0".into();
    let (value, success) =
        fixture.explain_policy("policy", &request_with_dependency(), &policy);
    assert!(!success);
    assert_eq!(value["outcome"], "rejected");
    assert_eq!(value["error"]["outcome"], "unsupported");
}

#[test]
// r[verify cairn.refinement.inspection]
fn unknown_arguments_fail_without_writing_lifecycle_state() {
    let fixture = Fixture::new();
    let path = fixture.root.join("case.json");
    fs::write(
        &path,
        serde_json::to_vec(&request_with_dependency()).unwrap(),
    )
    .unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args([
            "refinement",
            "explain",
            "--case",
            path.to_str().unwrap(),
            "--unknown",
        ])
        .current_dir(&fixture.root)
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(output.stdout.is_empty());
    assert_eq!(
        fs::read_dir(fixture.root.join(".cairn")).unwrap().count(),
        1
    );
}

#[test]
fn generated_policy_admits_the_default_vocabulary() {
    let policy: RefinementPolicy = serde_json::from_str(POLICY).unwrap();
    assert_eq!(policy.schema, REFINEMENT_POLICY_SCHEMA_V1);
    cairn_core::admit_refinement_policy(&policy).unwrap();
    assert_eq!(policy.relations.len(), 7);
    assert_eq!(policy.relations[0].tag(), "ordering_archive");
}
