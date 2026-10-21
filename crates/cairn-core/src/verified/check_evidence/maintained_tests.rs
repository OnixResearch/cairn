use super::*;
use crate::verified::agent_receipt::{
    authoritative_command_hash, authoritative_selection_hash, AgentVerificationContract,
};

pub(super) fn fixture(
    kind: CheckKind,
) -> (AgentVerificationContract, Vec<CheckProfile>, Applicability) {
    let (mut contract, mut profiles, applicability) = super::tests::fixture();
    profiles[0].kind = kind;
    if kind == CheckKind::NixFlakeCheck {
        profiles[0].executable = "/nix/store/reviewed-nix/bin/nix".into();
    }
    contract.required_checks[0].command = profile_command(&profiles[0]);
    contract.required_checks[0].command_hash =
        authoritative_command_hash(&contract.required_checks[0].command);
    contract.selection_hash = authoritative_selection_hash(&contract.required_checks);
    (contract, profiles, applicability)
}

#[test]
// r[verify cairn.check_evidence_producer.maintained_consumer_commands]
fn maintained_profiles_reject_changed_commands_features_and_executables() {
    for kind in [
        CheckKind::CargoWorkspaceTest,
        CheckKind::CargoClippy,
        CheckKind::CargoFmt,
        CheckKind::NixFlakeCheck,
    ] {
        let (contract, profiles, applicability) = fixture(kind);
        assert!(admit_plan(&contract, &profiles, &applicability).is_ok());
        let mut changed = contract.clone();
        changed.required_checks[0].command.push("--offline".into());
        changed.required_checks[0].command_hash =
            authoritative_command_hash(&changed.required_checks[0].command);
        changed.selection_hash = authoritative_selection_hash(&changed.required_checks);
        assert_eq!(
            admit_plan(&changed, &profiles, &applicability),
            Err(ProducerIssue::Command)
        );
        let mut changed = profiles.clone();
        changed[0].features.push("extra".into());
        assert_eq!(
            admit_plan(&contract, &changed, &applicability),
            Err(ProducerIssue::Profile)
        );
        changed[0].features.clear();
        changed[0].executable = "/usr/bin/sh".into();
        assert_eq!(
            admit_plan(&contract, &changed, &applicability),
            Err(ProducerIssue::Profile)
        );
    }
}

#[test]
fn only_new_flake_kind_admits_maintained_three_hour_deadline() {
    for kind in [
        CheckKind::CargoTest,
        CheckKind::NixCheck,
        CheckKind::CargoWorkspaceTest,
        CheckKind::CargoClippy,
        CheckKind::CargoFmt,
        CheckKind::NixFlakeCheck,
    ] {
        let (contract, mut profiles, applicability) = match kind {
            CheckKind::NixCheck => super::tests::nix_fixture(),
            _ => fixture(kind),
        };
        let maximum = if kind == CheckKind::NixFlakeCheck {
            MAX_FLAKE_CHECK_DEADLINE_MS
        } else {
            MAX_DEADLINE_MS
        };
        profiles[0].bounds.deadline_ms = maximum;
        assert!(admit_plan(&contract, &profiles, &applicability).is_ok());
        profiles[0].bounds.deadline_ms = maximum.checked_add(1).unwrap();
        assert_eq!(
            admit_plan(&contract, &profiles, &applicability),
            Err(ProducerIssue::Bounds)
        );
    }
}

#[test]
fn only_exact_reviewed_flake_configuration_admits_newlines() {
    let (contract, mut profiles, applicability) = fixture(CheckKind::NixFlakeCheck);
    profiles[0]
        .environment
        .insert("NIX_CONFIG".into(), FLAKE_CHECK_NIX_CONFIG.into());
    assert!(admit_plan(&contract, &profiles, &applicability).is_ok());
    for (key, value) in [
        ("NIX_CONFIG", "min-free = 0\n"),
        ("NIX_CONFIG", "builders = remote\n"),
        ("OTHER", FLAKE_CHECK_NIX_CONFIG),
        ("NIX_CONFIG", "min-free = 0\r\n"),
    ] {
        let mut changed = profiles.clone();
        changed[0].environment.clear();
        changed[0].environment.insert(key.into(), value.into());
        assert_eq!(
            admit_plan(&contract, &changed, &applicability),
            Err(ProducerIssue::Profile)
        );
    }
    for kind in [
        CheckKind::CargoTest,
        CheckKind::CargoClippy,
        CheckKind::NixCheck,
    ] {
        let (contract, mut profiles, applicability) = if kind == CheckKind::NixCheck {
            super::tests::nix_fixture()
        } else {
            fixture(kind)
        };
        profiles[0]
            .environment
            .insert("NIX_CONFIG".into(), FLAKE_CHECK_NIX_CONFIG.into());
        assert_eq!(
            admit_plan(&contract, &profiles, &applicability),
            Err(ProducerIssue::Profile)
        );
    }
}

#[test]
fn process_checks_never_admit_test_coverage_or_native_build_claims() {
    for kind in [
        CheckKind::CargoFmt,
        CheckKind::CargoClippy,
        CheckKind::NixFlakeCheck,
    ] {
        let (contract, profiles, applicability) = fixture(kind);
        let plan = admit_plan(&contract, &profiles, &applicability).unwrap();
        let mut observation = super::tests::observation(&plan);
        observation.role = EvidenceRole::ProcessCheck;
        observation.executed_tests = 0;
        observation.payload_passed = false;
        assert_eq!(
            normalize_attempt(&plan, &observation).unwrap().result().status,
            "passed"
        );
        let mut changed = observation.clone();
        changed.executed_tests = 1;
        assert_ne!(
            normalize_attempt(&plan, &changed).unwrap().result().status,
            "passed"
        );
        changed = observation.clone();
        changed.payload_passed = true;
        assert_ne!(
            normalize_attempt(&plan, &changed).unwrap().result().status,
            "passed"
        );
        changed = observation.clone();
        changed.role = EvidenceRole::Build;
        assert_ne!(
            normalize_attempt(&plan, &changed).unwrap().result().status,
            "passed"
        );
        for outcome in [
            ProcessOutcome::ExitedNonzero,
            ProcessOutcome::TimedOut,
            ProcessOutcome::Cancelled,
            ProcessOutcome::OutputOverflow,
            ProcessOutcome::Unknown,
        ] {
            changed = observation.clone();
            changed.outcome = outcome;
            assert_ne!(
                normalize_attempt(&plan, &changed).unwrap().result().status,
                "passed"
            );
        }
    }
}

#[test]
fn maintained_workspace_tests_still_require_executed_passing_summaries() {
    let (contract, profiles, applicability) = fixture(CheckKind::CargoWorkspaceTest);
    let plan = admit_plan(&contract, &profiles, &applicability).unwrap();
    let observation = super::tests::observation(&plan);
    assert_eq!(
        normalize_attempt(&plan, &observation).unwrap().result().status,
        "passed"
    );
    let mut changed = observation.clone();
    changed.executed_tests = 0;
    assert_ne!(
        normalize_attempt(&plan, &changed).unwrap().result().status,
        "passed"
    );
    changed = observation;
    changed.payload_passed = false;
    assert_ne!(
        normalize_attempt(&plan, &changed).unwrap().result().status,
        "passed"
    );
}

#[test]
fn measured_new_kind_stream_limits_do_not_expand_legacy_profiles() {
    for (kind, stdout, stderr) in [
        (CheckKind::CargoTest, MAX_STREAM_BYTES, MAX_STREAM_BYTES),
        (CheckKind::NixCheck, MAX_STREAM_BYTES, MAX_STREAM_BYTES),
        (
            CheckKind::CargoWorkspaceTest,
            MAX_WORKSPACE_TEST_STREAM_BYTES,
            MAX_WORKSPACE_TEST_STREAM_BYTES,
        ),
        (
            CheckKind::CargoClippy,
            MAX_STREAM_BYTES,
            MAX_CLIPPY_STDERR_BYTES,
        ),
        (CheckKind::CargoFmt, MAX_STREAM_BYTES, MAX_STREAM_BYTES),
        (
            CheckKind::NixFlakeCheck,
            MAX_STREAM_BYTES,
            MAX_FLAKE_CHECK_STDERR_BYTES,
        ),
    ] {
        let (contract, mut profiles, applicability) = if kind == CheckKind::NixCheck {
            super::tests::nix_fixture()
        } else {
            fixture(kind)
        };
        profiles[0].bounds.stdout_bytes = stdout;
        profiles[0].bounds.stderr_bytes = stderr;
        assert!(admit_plan(&contract, &profiles, &applicability).is_ok());
        for overflow_stdout in [true, false] {
            let mut changed = profiles.clone();
            if overflow_stdout {
                changed[0].bounds.stdout_bytes = stdout.checked_add(1).unwrap();
            } else {
                changed[0].bounds.stderr_bytes = stderr.checked_add(1).unwrap();
            }
            assert_eq!(
                admit_plan(&contract, &changed, &applicability),
                Err(ProducerIssue::Bounds)
            );
        }
    }
}
