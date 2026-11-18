use std::{
    collections::{BTreeMap, BTreeSet},
    env, fs,
    path::PathBuf,
    process::Command,
    time::SystemTime,
};

use cairn_core::{
    claim_identity, BranchPolarity, Change, ChangeEvent, ClaimContext, ContentBoundRequirementRow,
    ContentBoundSourceSpan, EvidencePolarity, EvidenceRecord, LifecycleObligation, Proposition,
    ScopedClaim, SupportBranch, SupportCase, CLAIM_SCHEMA_V1, SUPPORT_BOUNDARY,
    SUPPORT_CASE_SCHEMA_V1,
};
use serde_json::Value;

const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

struct Fixture {
    root: PathBuf,
}

impl Fixture {
    fn new() -> Self {
        let root = env::temp_dir().join(format!(
            "cairn-support-maintenance-{}-{}",
            std::process::id(),
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&root).unwrap();
        Self { root }
    }

    fn row(tag: &str) -> ContentBoundRequirementRow {
        ContentBoundRequirementRow {
            requirement_id: format!("cairn.test.support.{tag}"),
            specification_path: format!(".cairn/specs/support/{tag}.md"),
            specification_blake3: HASH_A.into(),
            source_span: ContentBoundSourceSpan {
                start_byte: 0,
                end_byte: 64,
                start_line: 1,
                end_line: 1,
            },
            requirement_body_blake3: HASH_B.into(),
            row_blake3: HASH_C.into(),
        }
    }

    fn context() -> ClaimContext {
        ClaimContext {
            subject_blake3: HASH_A.into(),
            policy_blake3: HASH_B.into(),
            target: "onixresearch/cairn".into(),
            environment: "linux-x86_64".into(),
            bounds: BTreeMap::from([("max_checks".to_string(), "4".to_string())]),
        }
    }

    fn claim(tag: &str, proposition: Proposition) -> ScopedClaim {
        ScopedClaim {
            schema: CLAIM_SCHEMA_V1.into(),
            kind: proposition.claim_kind(),
            requirement: Self::row(tag),
            interpretation_blake3: HASH_B.into(),
            contract_blake3: HASH_C.into(),
            context: Self::context(),
            proposition,
        }
    }

    fn evidence(id: &str, claim: &str, polarity: EvidencePolarity) -> EvidenceRecord {
        EvidenceRecord {
            id: id.into(),
            claim: claim.into(),
            polarity,
            rule: "native-check-observation.v1".into(),
            version: 1,
            assurance_scope: "exact saved observation".into(),
            assumptions: BTreeSet::new(),
        }
    }

    fn branch(id: &str, conclusion: &str, premises: &[&str]) -> SupportBranch {
        SupportBranch {
            id: id.into(),
            conclusion: conclusion.into(),
            polarity: BranchPolarity::Support,
            rule: "reviewed_completion_v1".into(),
            version: 1,
            premises: premises.iter().map(|premise| (*premise).to_string()).collect(),
            assumptions: BTreeSet::new(),
            assurance_scope: "bounded derivation".into(),
            applicability: Self::context(),
            cross_revision_relation: None,
        }
    }

    fn positive_case(&self) -> SupportCase {
        let a = Self::claim(
            "shared",
            Proposition::CheckPassed {
                check_id: "a".into(),
            },
        );
        let b = Self::claim(
            "shared",
            Proposition::CheckPassed {
                check_id: "b".into(),
            },
        );
        let c = Self::claim(
            "shared",
            Proposition::CheckPassed {
                check_id: "c".into(),
            },
        );
        let d = Self::claim("shared", Proposition::NativeCompletionEligible);
        let ids = BTreeMap::from([
            ("a", claim_identity(&a).unwrap()),
            ("b", claim_identity(&b).unwrap()),
            ("c", claim_identity(&c).unwrap()),
            ("d", claim_identity(&d).unwrap()),
        ]);
        SupportCase {
            schema: SUPPORT_CASE_SCHEMA_V1.into(),
            claims: vec![a, b, c, d],
            branches: vec![
                Self::branch("branch.ab", &ids["d"], &[&ids["a"], &ids["b"]]),
                Self::branch("branch.c", &ids["d"], &[&ids["c"]]),
            ],
            evidence: vec![
                Self::evidence("evidence.a", &ids["a"], EvidencePolarity::Support),
                Self::evidence("evidence.b", &ids["b"], EvidencePolarity::Support),
                Self::evidence("evidence.c", &ids["c"], EvidencePolarity::Support),
            ],
            changes: vec![ChangeEvent {
                id: "stale-b".into(),
                sequence: 1,
                reason: "the saved b receipt is stale".into(),
                change: Change::EvidenceStale {
                    evidence: "evidence.b".into(),
                },
            }],
            obligations: vec![LifecycleObligation {
                id: "obligation.d".into(),
                claim: ids["d"].clone(),
                description: "completion obligation".into(),
            }],
        }
    }

    fn write_case(&self, name: &str, case: &SupportCase) -> PathBuf {
        let path = self.root.join(name);
        fs::write(&path, serde_json::to_vec(case).unwrap()).unwrap();
        path
    }

    fn explain(&self, name: &str, case: &SupportCase, success: bool) -> Value {
        let path = self.write_case(name, case);
        let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .args([
                "support-maintenance",
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

#[test]
// r[verify cairn.support_maintenance.inspection]
fn cli_explains_surviving_and_withdrawn_alternatives_without_mutation() {
    let fixture = Fixture::new();
    let case = fixture.positive_case();
    let path = fixture.write_case("positive.json", &case);
    let before = fs::read(&path).unwrap();

    let output = fixture.explain("positive.json", &case, true);
    assert_eq!(output["outcome"], "evaluated");
    assert_eq!(output["advisory"], true);
    assert_eq!(output["boundary"], SUPPORT_BOUNDARY);
    assert_eq!(
        output["report"]["schema"],
        "cairn.support-maintenance-report.v1"
    );
    assert_eq!(output["report"]["analysis"]["mode"], "full_recomputation");
    assert_eq!(output["report"]["analysis"]["incremental"], false);

    let branches = output["report"]["branches"].as_array().unwrap();
    let withdrawn = branches
        .iter()
        .find(|branch| branch["id"] == "branch.ab")
        .unwrap();
    assert_eq!(withdrawn["status"], "withdrawn");
    assert_eq!(withdrawn["cause"]["origin"]["kind"], "evidence_stale");
    assert_eq!(
        withdrawn["cause"]["origin"]["detail"],
        "the saved b receipt is stale: evidence loses current applicability but remains a historical observation"
    );
    let survived = branches
        .iter()
        .find(|branch| branch["id"] == "branch.c")
        .unwrap();
    assert_eq!(survived["status"], "survived");
    assert!(survived["discharge_paths"]
        .as_array()
        .unwrap()
        .iter()
        .any(|path| path.as_str().unwrap().ends_with("via evidence.c")));

    let claims = output["report"]["claims"].as_array().unwrap();
    let d = claims
        .iter()
        .find(|claim| claim["claim"]["proposition"]["kind"] == "native_completion_eligible")
        .unwrap();
    assert_eq!(d["state"], "supported");
    assert_eq!(d["surviving_branches"][0], "branch.c");
    assert_eq!(d["withdrawn_branches"][0], "branch.ab");
    assert_eq!(output["report"]["affected_obligations"][0]["status"], "alternative_withdrawn");

    assert_eq!(before, fs::read(&path).unwrap());
    assert!(!fixture.root.join(".cairn/changes").exists());
    assert!(!fixture.root.join(".cairn/specs").exists());

    let replay = fixture.explain("positive-replay.json", &case, true);
    assert_eq!(
        replay["report"]["report_blake3"],
        output["report"]["report_blake3"]
    );
    assert_eq!(
        replay["report"]["input_blake3"],
        output["report"]["input_blake3"]
    );
}

#[test]
// r[verify cairn.support_maintenance.invalidation]
fn cli_rejects_scope_and_input_errors_without_writing() {
    let fixture = Fixture::new();
    let mut case = fixture.positive_case();
    let old = Fixture::claim("old", Proposition::NativeCompletionEligible);
    let new = Fixture::claim("new", Proposition::NativeCompletionEligible);
    let ids = BTreeMap::from([
        ("old", claim_identity(&old).unwrap()),
        ("new", claim_identity(&new).unwrap()),
    ]);
    case.claims = vec![old, new];
    case.branches = vec![Fixture::branch("branch.relation", &ids["new"], &[&ids["old"]])];
    case.evidence.clear();
    case.changes.clear();
    case.obligations.clear();
    let output = fixture.explain("scope.json", &case, false);
    assert_eq!(output["outcome"], "rejected");
    assert_eq!(output["error"]["outcome"], "scope_mismatch");
    assert_eq!(output["advisory"], true);

    fs::write(fixture.root.join("malformed.json"), b"{").unwrap();
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args([
            "support-maintenance",
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

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args(["support-maintenance", "explain", "--unknown"])
        .output()
        .unwrap();
    assert!(!output.status.success());
    assert!(!fixture.root.join(".cairn/changes").exists());
}
