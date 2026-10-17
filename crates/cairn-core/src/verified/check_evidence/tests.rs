use std::collections::BTreeMap;

use serde_json::json;

use crate::verified::agent_receipt::{
    authoritative_command_hash, authoritative_selection_hash,
    evaluate_authoritative_agent_completion, AgentRequiredCheck, AgentVerificationContract,
};

use super::*;

fn hash(c: char) -> String {
    c.to_string().repeat(64)
}
pub(super) fn fixture() -> (AgentVerificationContract, Vec<CheckProfile>, Applicability) {
    let profile = CheckProfile {
        schema: PROFILE_SCHEMA.into(),
        check_id: "tests".into(),
        kind: CheckKind::CargoTest,
        executable: "/nix/store/reviewed-cargo/bin/cargo".into(),
        executable_hash: hash('a'),
        tool_cohort_hash: hash('b'),
        target: "x86_64-unknown-linux-gnu".into(),
        features: vec![],
        environment: BTreeMap::new(),
        working_root: ".".into(),
        bounds: ExecutionBounds {
            id: "short".into(),
            deadline_ms: 1000,
            teardown_ms: 100,
            stdout_bytes: 4096,
            stderr_bytes: 4096,
        },
    };
    let command = profile_command(&profile);
    let checks = vec![AgentRequiredCheck {
        check_id: "tests".into(),
        command_hash: authoritative_command_hash(&command),
        command,
        prerequisites: vec![],
    }];
    let contract = AgentVerificationContract {
        schema: "cairn.agent-completion.v2".into(),
        repository_id: "consumer".into(),
        registry_id: "checks".into(),
        registry_hash: hash('d'),
        selection_hash: authoritative_selection_hash(&checks),
        policy_hash: hash('e'),
        final_subject_hash: hash('f'),
        required_checks: checks,
    };
    let applicability = Applicability {
        subject_hash: hash('f'),
        policy_hash: hash('e'),
        dependency_hashes: vec![hash('a')],
        source_projection_hash: hash('c'),
    };
    (contract, vec![profile], applicability)
}
pub(super) fn nix_fixture() -> (AgentVerificationContract, Vec<CheckProfile>, Applicability) {
    let (mut contract, mut profiles, applicability) = fixture();
    profiles[0].kind = CheckKind::NixCheck;
    profiles[0].executable = "/nix/store/reviewed-nix/bin/nix".into();
    profiles[0].target = "x86_64-linux".into();
    contract.required_checks[0].command = profile_command(&profiles[0]);
    contract.required_checks[0].command_hash =
        authoritative_command_hash(&contract.required_checks[0].command);
    contract.selection_hash = authoritative_selection_hash(&contract.required_checks);
    (contract, profiles, applicability)
}

pub(super) fn native_nix_payload() -> serde_json::Value {
    json!([{
        "drvPath":"/nix/store/1n6shkyf223h8ap2g2wffrq99p0lwkiy-cairn-native-nix-observation.drv",
        "outputs":{"out":"/nix/store/3x4awq38gccmqc3d6ajhyn824j7f0hd4-cairn-native-nix-observation"}
    }])
}

pub(super) fn observation(plan: &CheckEvidencePlan) -> AttemptObservation {
    let p = &plan.profiles()[0];
    AttemptObservation {
        check_id: p.check_id.clone(),
        attempt_hash: hash('a'),
        plan_hash: plan.identity().into(),
        applicability: plan.applicability().clone(),
        source_after_hash: plan.applicability().subject_hash.clone(),
        executable_hash: p.executable_hash.clone(),
        tool_cohort_hash: p.tool_cohort_hash.clone(),
        command_hash: authoritative_command_hash(&profile_command(p)),
        producer_identity: hash('b'),
        native_status: "passed".into(),
        native_non_claims: vec!["execution observations do not prove correctness".into()],
        role: EvidenceRole::Test,
        outcome: ProcessOutcome::ExitedZero,
        payload_passed: true,
        executed_tests: 1,
        execution_observed: true,
        cache_claimed: false,
        teardown_complete: true,
        evidence_hash: hash('c'),
    }
}

#[test]
// r[verify cairn.check_evidence_producer.contract]
fn ordered_plan_is_stable_and_does_not_replace_selection() {
    let (mut c, mut p, a) = fixture();
    let mut next = p[0].clone();
    next.check_id = "after".into();
    let mut check = c.required_checks[0].clone();
    check.check_id = "after".into();
    check.prerequisites.push("tests".into());
    c.required_checks.push(check);
    p.push(next);
    c.selection_hash = authoritative_selection_hash(&c.required_checks);
    let plan = admit_plan(&c, &p, &a).unwrap();
    assert_eq!(plan.profiles()[0].check_id, "tests");
    p.reverse();
    c.required_checks.reverse();
    assert_eq!(admit_plan(&c, &p, &a).unwrap(), plan);
    p[0].check_id = "unknown".into();
    assert_eq!(admit_plan(&c, &p, &a), Err(ProducerIssue::Selection));
}

#[test]
fn invalid_contracts_never_create_plans() {
    let (c, p, a) = fixture();
    let mut variants = Vec::new();
    let mut changed = c.clone();
    changed.selection_hash = hash('a');
    variants.push(changed);
    let mut changed = c.clone();
    changed.required_checks[0]
        .prerequisites
        .push("absent".into());
    changed.selection_hash = authoritative_selection_hash(&changed.required_checks);
    variants.push(changed);
    let mut changed = c.clone();
    changed.required_checks[0].command.push("--ignored".into());
    variants.push(changed);
    let mut changed = c.clone();
    changed.final_subject_hash = hash('a');
    variants.push(changed);
    let mut changed = c;
    changed.policy_hash = hash('a');
    variants.push(changed);
    for changed in variants {
        assert!(admit_plan(&changed, &p, &a).is_err());
    }
}

#[test]
fn bounds_environment_grammar_and_duplicates_fail_closed() {
    let (c, p, a) = fixture();
    let mut variants = Vec::new();
    let mut q = p[0].clone();
    q.bounds.deadline_ms = 0;
    variants.push(q);
    let mut q = p[0].clone();
    q.bounds.stdout_bytes = MAX_STREAM_BYTES + 1;
    variants.push(q);
    let mut q = p[0].clone();
    q.executable = "cargo".into();
    variants.push(q);
    let mut q = p[0].clone();
    q.executable = "/nix/store/../bin/cargo".into();
    variants.push(q);
    let mut q = p[0].clone();
    q.features = vec!["same".into(), "same".into()];
    variants.push(q);
    let mut q = p[0].clone();
    q.environment.insert("PATH".into(), "secret\ntext".into());
    variants.push(q);
    let mut q = p[0].clone();
    q.working_root = "..".into();
    variants.push(q);
    for q in variants {
        assert!(admit_plan(&c, &[q], &a).is_err());
    }
    assert!(admit_plan(&c, &[p[0].clone(), p[0].clone()], &a).is_err());
    let mut a = a;
    a.dependency_hashes.push(a.dependency_hashes[0].clone());
    assert!(admit_plan(&c, &p, &a).is_err());
}

#[test]
// r[verify cairn.check_evidence_producer.local_run]
fn authority_and_executable_rechecks_precede_attempt() {
    let (c, p, a) = fixture();
    let plan = admit_plan(&c, &p, &a).unwrap();
    assert_eq!(
        authorize_attempt(&plan, "tests", false, plan.identity(), &hash('a'), &[]),
        Err(ProducerIssue::Denied)
    );
    assert_eq!(
        authorize_attempt(&plan, "tests", true, &hash('b'), &hash('a'), &[]),
        Err(ProducerIssue::PlanChanged)
    );
    assert_eq!(
        authorize_attempt(&plan, "tests", true, plan.identity(), &hash('b'), &[]),
        Err(ProducerIssue::ExecutableChanged)
    );
    assert!(authorize_attempt(&plan, "tests", true, plan.identity(), &hash('a'), &[]).is_ok());
}

#[test]
// r[verify cairn.check_evidence_producer.conformance]
fn generated_rows_reach_original_completion_validator() {
    let (c, p, a) = fixture();
    let plan = admit_plan(&c, &p, &a).unwrap();
    let observation = observation(&plan);
    let attempt = normalize_attempt(&plan, &observation).unwrap();
    let report = evaluate_authoritative_agent_completion(
        &json!(c),
        &json!([attempt.result()]),
        &a.subject_hash,
        &a.policy_hash,
    );
    assert!(report.valid, "{:?}", report.diagnostics);
    assert_eq!(attempt.observation(), &observation);
}

#[test]
fn every_process_failure_stays_nonpassing_in_original_consumer() {
    let (c, p, a) = fixture();
    let plan = admit_plan(&c, &p, &a).unwrap();
    for (outcome, status) in [
        (ProcessOutcome::Denied, "blocked"),
        (ProcessOutcome::ExitedNonzero, "failed"),
        (ProcessOutcome::TimedOut, "timed_out"),
        (ProcessOutcome::Cancelled, "cancelled"),
        (ProcessOutcome::OutputOverflow, "infrastructure_error"),
        (ProcessOutcome::TeardownFailed, "cleanup_failed"),
        (ProcessOutcome::InfrastructureError, "infrastructure_error"),
        (ProcessOutcome::Unknown, "unknown"),
    ] {
        let mut o = observation(&plan);
        o.outcome = outcome;
        let row = normalize_attempt(&plan, &o).unwrap();
        assert_eq!(row.result().status, status);
        let report = evaluate_authoritative_agent_completion(
            &json!(c),
            &json!([row.result()]),
            &a.subject_hash,
            &a.policy_hash,
        );
        assert!(!report.valid);
    }
}

#[test]
fn zero_exit_is_not_empty_test_payload_cache_or_build_success() {
    let (c, p, a) = fixture();
    let plan = admit_plan(&c, &p, &a).unwrap();
    let o = observation(&plan);
    let mut variants = Vec::new();
    let mut x = o.clone();
    x.executed_tests = 0;
    variants.push(x);
    let mut x = o.clone();
    x.payload_passed = false;
    variants.push(x);
    let mut x = o.clone();
    x.cache_claimed = true;
    variants.push(x);
    let mut x = o.clone();
    x.execution_observed = false;
    variants.push(x);
    let mut x = o.clone();
    x.role = EvidenceRole::Build;
    variants.push(x);
    let mut x = o.clone();
    x.role = EvidenceRole::StrictLint;
    variants.push(x);
    let mut x = o.clone();
    x.teardown_complete = false;
    variants.push(x);
    for status in ["skipped", "pending", "warning_only", "failed"] {
        let mut x = o.clone();
        x.native_status = status.into();
        variants.push(x);
    }
    for x in variants {
        assert_ne!(
            normalize_attempt(&plan, &x).unwrap().result().status,
            "passed"
        );
    }
}

#[test]
// r[verify cairn.check_evidence_producer.applicability]
fn stale_and_substituted_observations_are_rejected() {
    let (c, p, a) = fixture();
    let plan = admit_plan(&c, &p, &a).unwrap();
    let o = observation(&plan);
    let mut variants = Vec::new();
    let mut x = o.clone();
    x.source_after_hash = hash('a');
    variants.push(x);
    let mut x = o.clone();
    x.applicability.policy_hash = hash('a');
    variants.push(x);
    let mut x = o.clone();
    x.applicability.dependency_hashes.clear();
    variants.push(x);
    let mut x = o.clone();
    x.applicability.source_projection_hash = hash('a');
    variants.push(x);
    let mut x = o.clone();
    x.executable_hash = hash('b');
    variants.push(x);
    let mut x = o.clone();
    x.tool_cohort_hash = hash('a');
    variants.push(x);
    let mut x = o.clone();
    x.command_hash = hash('a');
    variants.push(x);
    let mut x = o;
    x.plan_hash = hash('a');
    variants.push(x);
    for x in variants {
        assert_eq!(
            normalize_attempt(&plan, &x),
            Err(ProducerIssue::Applicability)
        );
    }
}

#[test]
// r[verify cairn.check_evidence_producer.publication]
fn only_complete_current_readback_requests_a_final_manifest() {
    let (c, p, a) = fixture();
    let plan = admit_plan(&c, &p, &a).unwrap();
    let attempt = normalize_attempt(&plan, &observation(&plan)).unwrap();
    let artifact = PublishedArtifact {
        expected_hash: hash('c'),
        observed_hash: hash('c'),
        durable: true,
    };
    let attempts = std::slice::from_ref(&attempt);
    assert_eq!(
        publication_decision(&plan, attempts, std::slice::from_ref(&artifact), &a),
        PublicationDecision::PublishFinalManifest
    );
    assert_eq!(
        publication_decision(&plan, attempts, &[], &a),
        PublicationDecision::Withhold
    );
    let mut bad = artifact.clone();
    bad.durable = false;
    assert_eq!(
        publication_decision(&plan, attempts, &[bad], &a),
        PublicationDecision::Withhold
    );
    let mut bad = artifact;
    bad.observed_hash = hash('a');
    assert_eq!(
        publication_decision(&plan, attempts, &[bad], &a),
        PublicationDecision::Withhold
    );
    let mut stale = a;
    stale.subject_hash = hash('a');
    assert_eq!(
        publication_decision(&plan, &[attempt], &[], &stale),
        PublicationDecision::Withhold
    );
}

#[test]
fn prerequisites_require_exact_prior_success_and_never_retry() {
    let (mut c, mut p, a) = fixture();
    let mut next = p[0].clone();
    next.check_id = "after".into();
    let mut check = c.required_checks[0].clone();
    check.check_id = "after".into();
    check.prerequisites.push("tests".into());
    c.required_checks.push(check);
    p.push(next);
    c.selection_hash = authoritative_selection_hash(&c.required_checks);
    let plan = admit_plan(&c, &p, &a).unwrap();
    assert_eq!(
        authorize_attempt(&plan, "after", true, plan.identity(), &hash('a'), &[]),
        Err(ProducerIssue::Prerequisite)
    );
    let passed = normalize_attempt(&plan, &observation(&plan)).unwrap();
    assert!(authorize_attempt(
        &plan,
        "after",
        true,
        plan.identity(),
        &hash('a'),
        std::slice::from_ref(&passed)
    )
    .is_ok());
    assert_eq!(
        authorize_attempt(
            &plan,
            "tests",
            true,
            plan.identity(),
            &hash('a'),
            std::slice::from_ref(&passed)
        ),
        Err(ProducerIssue::Prerequisite)
    );
    assert_eq!(
        authorize_attempt(
            &plan,
            "after",
            true,
            plan.identity(),
            &hash('a'),
            &[passed.clone(), passed]
        ),
        Err(ProducerIssue::Prerequisite)
    );
    let mut failed = observation(&plan);
    failed.outcome = ProcessOutcome::TimedOut;
    let failed = normalize_attempt(&plan, &failed).unwrap();
    assert_eq!(
        authorize_attempt(&plan, "after", true, plan.identity(), &hash('a'), &[failed]),
        Err(ProducerIssue::Prerequisite)
    );
}

#[test]
fn nix_check_is_only_a_build_obligation() {
    // r[verify cairn.check_evidence_producer.native_nix_build]
    let (c, p, a) = nix_fixture();
    let plan = admit_plan(&c, &p, &a).unwrap();
    let mut o = observation(&plan);
    assert_eq!(
        normalize_attempt(&plan, &o).unwrap().result().status,
        "blocked"
    );
    o.role = EvidenceRole::Build;
    o.executed_tests = 0;
    assert_eq!(
        normalize_attempt(&plan, &o).unwrap().result().status,
        "passed"
    );
    o.executed_tests = 1;
    assert_eq!(
        normalize_attempt(&plan, &o).unwrap().result().status,
        "blocked"
    );
    o.executed_tests = 0;
    o.cache_claimed = true;
    assert_eq!(
        normalize_attempt(&plan, &o).unwrap().result().status,
        "unknown"
    );
}

#[test]
fn publication_rejects_mixed_attempts_and_failed_or_reordered_rows() {
    let (mut c, mut p, a) = fixture();
    let mut next = p[0].clone();
    next.check_id = "after".into();
    let mut check = c.required_checks[0].clone();
    check.check_id = "after".into();
    check.prerequisites.push("tests".into());
    c.required_checks.push(check);
    p.push(next);
    c.selection_hash = authoritative_selection_hash(&c.required_checks);
    let plan = admit_plan(&c, &p, &a).unwrap();
    let first = observation(&plan);
    let mut second = first.clone();
    second.check_id = "after".into();
    let artifacts = vec![
        PublishedArtifact {
            expected_hash: hash('c'),
            observed_hash: hash('c'),
            durable: true,
        };
        2
    ];
    let rows = vec![
        normalize_attempt(&plan, &first).unwrap(),
        normalize_attempt(&plan, &second).unwrap(),
    ];
    assert_eq!(
        publication_decision(&plan, &rows, &artifacts, &a),
        PublicationDecision::PublishFinalManifest
    );
    second.attempt_hash = hash('b');
    let mixed = vec![rows[0].clone(), normalize_attempt(&plan, &second).unwrap()];
    assert_eq!(
        publication_decision(&plan, &mixed, &artifacts, &a),
        PublicationDecision::Withhold
    );
    let mut reversed = rows.clone();
    reversed.reverse();
    assert_eq!(
        publication_decision(&plan, &reversed, &artifacts, &a),
        PublicationDecision::Withhold
    );
    second.attempt_hash = first.attempt_hash;
    second.outcome = ProcessOutcome::ExitedNonzero;
    let failed = vec![rows[0].clone(), normalize_attempt(&plan, &second).unwrap()];
    assert_eq!(
        publication_decision(&plan, &failed, &artifacts, &a),
        PublicationDecision::Withhold
    );
    let duplicate = vec![rows[0].clone(), rows[0].clone()];
    assert_eq!(
        publication_decision(&plan, &duplicate, &artifacts, &a),
        PublicationDecision::Withhold
    );
}

#[test]
fn exported_nickel_profile_matches_rust_and_rejects_extra_fields() {
    let value = include_str!("../../../../../fixtures/check-evidence/profiles.json");
    let parsed: Vec<CheckProfile> = serde_json::from_str(value).unwrap();
    assert_eq!(parsed, fixture().1);
    let mut extra = json!(parsed[0]);
    extra["zero_test_waiver"] = json!(true);
    assert!(serde_json::from_value::<CheckProfile>(extra).is_err());
}

#[test]
fn target_features_and_environment_change_review_identity() {
    let (mut c, mut p, a) = fixture();
    let plan = admit_plan(&c, &p, &a).unwrap();
    p[0].environment.insert("LANG".into(), "C".into());
    assert_ne!(admit_plan(&c, &p, &a).unwrap().identity(), plan.identity());
    p[0].features.push("feature".into());
    assert_eq!(admit_plan(&c, &p, &a), Err(ProducerIssue::Command));
    c.required_checks[0].command = profile_command(&p[0]);
    c.required_checks[0].command_hash = authoritative_command_hash(&c.required_checks[0].command);
    c.selection_hash = authoritative_selection_hash(&c.required_checks);
    assert_ne!(admit_plan(&c, &p, &a).unwrap().identity(), plan.identity());
}

#[test]
// r[verify cairn.check_evidence_producer.native_nix_build]
fn native_nix_build_contract_rejects_legacy_evaluation_selection() {
    let (mut contract, profiles, applicability) = nix_fixture();
    contract.required_checks[0].command = [
        profiles[0].executable.as_str(),
        "build",
        "--offline",
        "--no-link",
        "--json",
        "--file",
        "./check.nix",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    contract.required_checks[0].command_hash =
        authoritative_command_hash(&contract.required_checks[0].command);
    contract.selection_hash = authoritative_selection_hash(&contract.required_checks);
    assert!(admit_plan(&contract, &profiles, &applicability).is_ok());
    contract.required_checks[0].command = [
        profiles[0].executable.as_str(),
        "eval",
        "--offline",
        "--json",
        "--file",
        "./check.nix",
    ]
    .into_iter()
    .map(String::from)
    .collect();
    contract.required_checks[0].command_hash =
        authoritative_command_hash(&contract.required_checks[0].command);
    contract.selection_hash = authoritative_selection_hash(&contract.required_checks);
    assert!(admit_plan(&contract, &profiles, &applicability).is_err());
}

#[test]
// r[verify cairn.check_evidence_producer.native_nix_build]
fn native_nix_derivation_outputs_admit_supported_timings_and_multiple_outputs() {
    let payload = native_nix_payload();
    assert!(nix_build_payload_passed(&payload));
    for (field, value) in [
        ("startTime", json!(1720000000)),
        ("stopTime", json!(1720000001)),
        ("cpuUser", json!(0.001)),
        ("cpuSystem", json!(0)),
    ] {
        let mut timed = payload.clone();
        timed[0][field] = value;
        assert!(nix_build_payload_passed(&timed));
    }
    let mut multiple = payload.clone();
    multiple[0]["outputs"]["dev"] =
        json!("/nix/store/3x4awq38gccmqc3d6ajhyn824j7f0hd4-cairn-native-nix-observation-dev");
    let mut second = payload[0].clone();
    second["drvPath"] = json!("/nix/store/1n6shkyf223h8ap2g2wffrq99p0lwkiy-second.drv");
    second["outputs"]["out"] = json!("/nix/store/3x4awq38gccmqc3d6ajhyn824j7f0hd4-second");
    multiple.as_array_mut().unwrap().push(second);
    assert!(nix_build_payload_passed(&multiple));
}

#[test]
// r[verify cairn.check_evidence_producer.native_nix_build]
fn native_nix_rejects_nonbuild_partial_unsupported_and_duplicate_payloads() {
    let payload = native_nix_payload();
    for invalid in [
        json!(null),
        json!(true),
        json!(1),
        json!({"success":true}),
        json!([]),
        json!([null]),
        json!([{}]),
        json!([payload[0], payload[0]]),
        json!([{"path":payload[0]["outputs"]["out"]}]),
        json!([{"drvPath":payload[0]["drvPath"]}]),
        json!([{"outputs":payload[0]["outputs"]}]),
    ] {
        assert!(!nix_build_payload_passed(&invalid), "admitted {invalid}");
    }
    for (field, value) in [
        ("outputs", json!({})),
        ("outputs", json!([])),
        ("outputs", json!({"out":null})),
        ("outputs", json!({"out":false})),
        ("outputs", json!({"":payload[0]["outputs"]["out"]})),
        (
            "outputs",
            json!({"out":payload[0]["outputs"]["out"], "dev":payload[0]["outputs"]["out"]}),
        ),
        ("drvPath", json!(null)),
        ("drvPath", json!({"output":"out"})),
        ("drvPath", payload[0]["outputs"]["out"].clone()),
        ("success", json!(true)),
        ("status", json!("failed")),
        ("startTime", json!(null)),
        ("startTime", json!(0)),
        ("startTime", json!(-1)),
        ("stopTime", json!(1.5)),
        ("cpuUser", json!(-0.1)),
        ("cpuSystem", json!("0.1")),
    ] {
        let mut invalid = payload.clone();
        invalid[0][field] = value;
        assert!(!nix_build_payload_passed(&invalid), "admitted {invalid}");
    }
    let mut reversed = payload.clone();
    reversed[0]["startTime"] = json!(2);
    reversed[0]["stopTime"] = json!(1);
    assert!(!nix_build_payload_passed(&reversed));
}

#[test]
// r[verify cairn.check_evidence_producer.native_nix_build]
fn native_nix_rejects_invalid_store_leaves_and_exceeded_bounds() {
    let payload = native_nix_payload();
    for path in [
        "/nix/store/short-name",
        "/tmp/3x4awq38gccmqc3d6ajhyn824j7f0hd4-name",
        "/nix/store/ex4awq38gccmqc3d6ajhyn824j7f0hd4-name",
        "/nix/store/3x4awq38gccmqc3d6ajhyn824j7f0hd4/name",
        "/nix/store/3x4awq38gccmqc3d6ajhyn824j7f0hd4-",
        "/nix/store/3x4awq38gccmqc3d6ajhyn824j7f0hd4-..",
        "/nix/store/3x4awq38gccmqc3d6ajhyn824j7f0hd4-.-name",
        "/nix/store/3x4awq38gccmqc3d6ajhyn824j7f0hd4-name/child",
        "/nix/store/3x4awq38gccmqc3d6ajhyn824j7f0hd4-name with space",
    ] {
        let mut invalid = payload.clone();
        invalid[0]["outputs"]["out"] = json!(path);
        assert!(!nix_build_payload_passed(&invalid), "admitted {path}");
    }
    let mut rows = Vec::new();
    for index in 0..=MAX_CHECKS {
        let mut row = payload[0].clone();
        row["drvPath"] = json!(format!(
            "/nix/store/1n6shkyf223h8ap2g2wffrq99p0lwkiy-{index}.drv"
        ));
        rows.push(row);
    }
    assert!(nix_build_payload_passed(&json!(&rows[..MAX_CHECKS])));
    assert!(!nix_build_payload_passed(&json!(rows)));
    let mut outputs = serde_json::Map::new();
    for index in 0..=MAX_CHECKS {
        outputs.insert(
            format!("out{index}"),
            json!(format!(
                "/nix/store/3x4awq38gccmqc3d6ajhyn824j7f0hd4-{index}"
            )),
        );
    }
    let mut excessive = payload;
    excessive[0]["outputs"] = json!(outputs);
    assert!(!nix_build_payload_passed(&excessive));
    excessive[0]["outputs"]
        .as_object_mut()
        .unwrap()
        .remove("out64");
    assert!(nix_build_payload_passed(&excessive));
}
