use super::*;

fn id(label: &str) -> BootstrapIdentity {
    BootstrapIdentity(blake3::hash(label.as_bytes()).to_hex().to_string())
}

fn cohort() -> BootstrapCohort {
    BootstrapCohort {
        schema: BOOTSTRAP_SCHEMA.into(),
        tool: id("tool"),
        policy: id("policy"),
        export: id("export"),
        evaluator: id("evaluator"),
        source: id("source"),
        workflow: id("workflow"),
        profile: "spec-driven".into(),
        managed_locators: vec![id("policy-locator")],
        upgrade_from: vec![],
        bounds: BootstrapBounds {
            max_artifacts: 4,
            max_artifact_bytes: 1024,
            max_total_bytes: 4096,
        },
    }
}

fn observations(cohort: &BootstrapCohort) -> CohortObservations {
    CohortObservations {
        tool: cohort.tool.clone(),
        policy: cohort.policy.clone(),
        export: cohort.export.clone(),
        evaluator: cohort.evaluator.clone(),
        source: cohort.source.clone(),
        workflow: cohort.workflow.clone(),
        export_current: true,
    }
}

fn artifact() -> ManagedArtifact {
    ManagedArtifact {
        locator: id("policy-locator"),
        desired: id("policy"),
        desired_bytes: 10,
        previously_owned: None,
        observed: FileObservation::Missing,
    }
}

fn plan() -> BootstrapPlan {
    let cohort = cohort();
    plan_bootstrap(
        &cohort,
        &observations(&cohort),
        None,
        None,
        false,
        &[artifact()],
    )
    .unwrap()
}

// r[verify cairn.consumer_bootstrap.policy]
#[test]
fn policy_precedence_distinguishes_absence_from_failure() {
    let valid = PolicyObservation::Valid(id("policy"));
    let invalid = PolicyObservation::Invalid;
    let absent = PolicyObservation::Absent;
    assert_eq!(
        select_policy(&valid, &invalid, &invalid),
        Ok((PolicySource::Explicit, id("policy")))
    );
    assert_eq!(
        select_policy(&absent, &valid, &invalid),
        Ok((PolicySource::Project, id("policy")))
    );
    assert_eq!(
        select_policy(&absent, &absent, &valid),
        Ok((PolicySource::LegacyDefault, id("policy")))
    );
    for inputs in [
        (&invalid, &valid, &valid),
        (&absent, &invalid, &valid),
        (&absent, &absent, &absent),
    ] {
        assert_eq!(
            select_policy(inputs.0, inputs.1, inputs.2),
            Err(BootstrapError::InvalidSelection)
        );
    }
}

#[test]
fn identity_decode_rejects_noncanonical_values() {
    assert!(BootstrapIdentity::try_from("a".repeat(DIGEST_HEX_BYTES)).is_ok());
    for invalid in [
        "A".repeat(DIGEST_HEX_BYTES),
        "g".repeat(DIGEST_HEX_BYTES),
        "a".repeat(DIGEST_HEX_BYTES - 1),
    ] {
        assert_eq!(
            BootstrapIdentity::try_from(invalid),
            Err(BootstrapError::InvalidIdentity)
        );
    }
}

// r[verify cairn.consumer_bootstrap.policy]
#[test]
fn project_declaration_is_closed_and_does_not_grant_path_authority() {
    let mut declaration = BootstrapProjectDeclaration {
        schema: "cairn.bootstrap-project.v1".into(),
        policy_locator: ".cairn/bootstrap/policy.json".into(),
        policy: id("policy"),
        cohort: id("cohort"),
        complete_manifest: id("manifest"),
    };
    assert_eq!(declaration.validate(), Ok(()));
    for locator in [
        "",
        "/policy",
        "../policy",
        "a/../policy",
        "a//b",
        ".",
        "a/./b",
        "a\\b",
    ] {
        declaration.policy_locator = locator.into();
        assert_eq!(
            declaration.validate(),
            Err(BootstrapError::InvalidSelection)
        );
    }
    declaration.policy_locator = "policy.json".into();
    declaration.schema = "future".into();
    assert_eq!(
        declaration.validate(),
        Err(BootstrapError::InvalidSelection)
    );
}

// r[verify cairn.consumer_bootstrap.compatibility]
#[test]
fn exact_cohort_and_explicit_upgrade_are_required() {
    let initial = cohort();
    assert_eq!(
        admit_cohort(&initial, &observations(&initial), None),
        Ok(())
    );
    let prior = cohort_identity(&initial);
    let mut upgrade = initial.clone();
    upgrade.policy = id("new-policy");
    assert_eq!(
        admit_cohort(&upgrade, &observations(&upgrade), Some(&prior)),
        Err(BootstrapError::UnsupportedUpgrade)
    );
    upgrade.upgrade_from.push(prior.clone());
    assert_eq!(
        admit_cohort(&upgrade, &observations(&upgrade), Some(&prior)),
        Ok(())
    );
    assert_ne!(cohort_identity(&initial), cohort_identity(&upgrade));
    let mut observed = observations(&upgrade);
    observed.export_current = false;
    assert_eq!(
        admit_cohort(&upgrade, &observed, Some(&prior)),
        Err(BootstrapError::StaleExport)
    );
    observed = observations(&upgrade);
    observed.tool = id("other-tool");
    assert_eq!(
        admit_cohort(&upgrade, &observed, Some(&prior)),
        Err(BootstrapError::UnsupportedCohort)
    );
}

#[test]
fn cohort_bounds_and_duplicate_declarations_fail_closed() {
    let mut invalid = cohort();
    invalid.bounds.max_artifacts = MAX_MANAGED_ARTIFACTS + 1;
    assert_eq!(
        admit_cohort(&invalid, &observations(&invalid), None),
        Err(BootstrapError::Bounds)
    );
    invalid = cohort();
    invalid
        .managed_locators
        .push(invalid.managed_locators[0].clone());
    assert_eq!(
        admit_cohort(&invalid, &observations(&invalid), None),
        Err(BootstrapError::UnsupportedCohort)
    );
    invalid = cohort();
    invalid.schema = "future".into();
    assert_eq!(
        admit_cohort(&invalid, &observations(&invalid), None),
        Err(BootstrapError::UnsupportedCohort)
    );
}

// r[verify cairn.consumer_bootstrap.publication]
#[test]
fn ownership_does_not_follow_byte_equivalence() {
    let mut value = artifact();
    assert_eq!(classify_artifact(&value), Ok(PublicationAction::Create));
    value.observed = FileObservation::Regular {
        identity: value.desired.clone(),
        bytes: value.desired_bytes,
    };
    assert_eq!(classify_artifact(&value), Err(BootstrapError::Unmanaged));
    value.previously_owned = Some(value.desired.clone());
    assert_eq!(classify_artifact(&value), Ok(PublicationAction::Preserve));
    value.desired = id("upgrade");
    assert_eq!(
        classify_artifact(&value),
        Ok(PublicationAction::ReplaceOwned)
    );
    for (observation, error) in [
        (
            FileObservation::Regular {
                identity: id("operator-edit"),
                bytes: 10,
            },
            BootstrapError::Modified,
        ),
        (FileObservation::Missing, BootstrapError::MissingManaged),
        (FileObservation::UnsafePath, BootstrapError::UnsafePath),
        (
            FileObservation::Unreadable,
            BootstrapError::ObservationFailed,
        ),
    ] {
        value.observed = observation;
        assert_eq!(classify_artifact(&value), Err(error));
    }
}

#[test]
fn plan_binds_cohort_manifest_and_destination_without_effects() {
    let first = plan();
    assert_eq!(first, plan());
    assert_eq!(first.actions(), &[PublicationAction::Create]);
    assert_eq!(recheck_plan(first.identity(), &first), Ok(()));
    let mut cohort = cohort();
    cohort.source = id("changed-source");
    let changed = plan_bootstrap(
        &cohort,
        &observations(&cohort),
        None,
        None,
        false,
        &[artifact()],
    )
    .unwrap();
    assert_eq!(
        recheck_plan(first.identity(), &changed),
        Err(BootstrapError::StalePlan)
    );
    assert_eq!(
        plan_bootstrap(
            &cohort,
            &observations(&cohort),
            None,
            None,
            true,
            &[artifact()]
        ),
        Err(BootstrapError::RecoveryRequired)
    );
    let mut value = artifact();
    value.desired_bytes = cohort.bounds.max_artifact_bytes + 1;
    assert_eq!(
        plan_bootstrap(&cohort, &observations(&cohort), None, None, false, &[value]),
        Err(BootstrapError::Bounds)
    );
}

#[test]
fn successful_upgrade_requires_owned_manifest_and_complete_locator_set() {
    let previous = cohort();
    let previous_id = cohort_identity(&previous);
    let mut upgrade = previous.clone();
    upgrade.policy = id("new-policy");
    upgrade.upgrade_from.push(previous_id.clone());
    let mut value = artifact();
    value.previously_owned = Some(value.desired.clone());
    value.observed = FileObservation::Regular {
        identity: value.desired.clone(),
        bytes: value.desired_bytes,
    };
    value.desired = upgrade.policy.clone();
    let manifest = id("previous-complete-manifest");
    let result = plan_bootstrap(
        &upgrade,
        &observations(&upgrade),
        Some(&previous_id),
        Some(&manifest),
        false,
        &[value.clone()],
    )
    .unwrap();
    assert_eq!(result.actions(), &[PublicationAction::ReplaceOwned]);
    let changed_manifest = plan_bootstrap(
        &upgrade,
        &observations(&upgrade),
        Some(&previous_id),
        Some(&id("other-manifest")),
        false,
        &[value.clone()],
    )
    .unwrap();
    assert_ne!(result.identity(), changed_manifest.identity());
    value.locator = id("undeclared-locator");
    assert_eq!(
        plan_bootstrap(
            &upgrade,
            &observations(&upgrade),
            Some(&previous_id),
            Some(&manifest),
            false,
            &[value]
        ),
        Err(BootstrapError::InvalidSelection)
    );
}

// r[verify cairn.consumer_bootstrap.publication]
#[test]
fn interruption_and_unknown_durability_never_imply_completion() {
    let plan = plan();
    let desired = FileObservation::Regular {
        identity: artifact().desired,
        bytes: artifact().desired_bytes,
    };
    assert_eq!(
        recover_bootstrap(
            &plan,
            plan.identity(),
            &[FileObservation::Missing],
            &[false]
        ),
        RecoveryDecision::ResumeUnchanged
    );
    assert_eq!(
        recover_bootstrap(
            &plan,
            plan.identity(),
            std::slice::from_ref(&desired),
            &[false]
        ),
        RecoveryDecision::RequireSynchronization
    );
    assert_eq!(
        recover_bootstrap(&plan, plan.identity(), &[desired], &[true]),
        RecoveryDecision::PublishCompleteManifest
    );
    assert_eq!(
        recover_bootstrap(
            &plan,
            &id("other-plan"),
            &[FileObservation::Missing],
            &[true]
        ),
        RecoveryDecision::ManualRepair
    );
    for observation in [
        FileObservation::UnsafePath,
        FileObservation::Unreadable,
        FileObservation::Regular {
            identity: id("unowned"),
            bytes: 10,
        },
    ] {
        assert_eq!(
            recover_bootstrap(&plan, plan.identity(), &[observation], &[true]),
            RecoveryDecision::ManualRepair
        );
    }
    assert_eq!(
        recover_bootstrap(&plan, plan.identity(), &[], &[]),
        RecoveryDecision::ManualRepair
    );
}

#[test]
fn recovery_observes_every_member_before_manifest_publication() {
    let mut cohort = cohort();
    cohort.managed_locators.push(id("guidance-locator"));
    let mut second = artifact();
    second.locator = id("guidance-locator");
    let plan = plan_bootstrap(
        &cohort,
        &observations(&cohort),
        None,
        None,
        false,
        &[artifact(), second],
    )
    .unwrap();
    let desired = FileObservation::Regular {
        identity: artifact().desired,
        bytes: artifact().desired_bytes,
    };
    assert_eq!(
        recover_bootstrap(
            &plan,
            plan.identity(),
            &[desired.clone(), FileObservation::Missing],
            &[true, false]
        ),
        RecoveryDecision::ResumeUnchanged
    );
    assert_eq!(
        recover_bootstrap(
            &plan,
            plan.identity(),
            &[desired, FileObservation::Unreadable],
            &[true, false]
        ),
        RecoveryDecision::ManualRepair
    );
}
