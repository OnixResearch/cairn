use super::*;

const POLICY_HASH: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
const DESTINATION: &str = ".agents/skills";
const NON_CLAIMS: &[&str] = &[
    "deterministic generation proves only a bounded transformation of admitted policy",
    "an adapter changes representation but does not add authority or lifecycle semantics",
    "generated guidance does not grant command, filesystem, network, model, gate, or release authority",
    "workflow success depends on checked project evidence and does not follow from generated text alone",
];

fn policy() -> GeneratedAgentWorkflowPolicy {
    let descriptors = INITIAL_WORKFLOW_IDS
        .iter()
        .map(|id| {
            let read_only = matches!(*id, "explore" | "verify");
            let constraints = if *id == "explore" {
                vec![GENERATED_AGENT_WORKFLOW_EXPLORE_NO_WRITE.to_string()]
            } else {
                vec!["Keep policy authoritative.".to_string()]
            };
            GeneratedWorkflowDescriptor {
                id: (*id).to_string(),
                version: 1,
                goal: format!("Complete the {id} workflow with checked evidence."),
                success_evidence: vec!["The required evidence is present.".to_string()],
                required_evidence: vec!["A checked receipt".to_string()],
                command_classes: if read_only {
                    vec!["read".to_string()]
                } else {
                    vec!["mutate".to_string(), "read".to_string()]
                },
                mutation_class: if read_only {
                    GeneratedWorkflowMutationClass::ReadOnly
                } else {
                    GeneratedWorkflowMutationClass::PreviewFirst
                },
                progress_rule: "Report each checked step.".to_string(),
                immediate_action: "Inspect the selected change.".to_string(),
                output_shape: "Return evidence and blockers.".to_string(),
                constraints,
                non_claims: NON_CLAIMS
                    .iter()
                    .map(|value| (*value).to_string())
                    .collect(),
            }
        })
        .collect();
    GeneratedAgentWorkflowPolicy {
        schema: GENERATED_AGENT_WORKFLOW_POLICY_SCHEMA_V1.to_string(),
        generator_version: GENERATED_AGENT_WORKFLOW_GENERATOR_VERSION.to_string(),
        command_classes: vec![
            GeneratedWorkflowCommandClass {
                id: "mutate".to_string(),
                permission: GeneratedWorkflowPermission::Mutation,
            },
            GeneratedWorkflowCommandClass {
                id: "read".to_string(),
                permission: GeneratedWorkflowPermission::Read,
            },
        ],
        command_catalog: vec![
            GeneratedWorkflowCommand {
                id: "change-create".to_string(),
                command_class: "mutate".to_string(),
                invocation: "cairn change create <name> --profile <profile> --root PATH".to_string(),
            },
            GeneratedWorkflowCommand {
                id: "doctor".to_string(),
                command_class: "read".to_string(),
                invocation: "cairn doctor --root PATH".to_string(),
            },
            GeneratedWorkflowCommand {
                id: "status".to_string(),
                command_class: "read".to_string(),
                invocation: "cairn status --root PATH".to_string(),
            },
            GeneratedWorkflowCommand {
                id: "workflow-instructions".to_string(),
                command_class: "read".to_string(),
                invocation: "cairn workflow instructions <artifact> --change <change> --root PATH"
                    .to_string(),
            },
            GeneratedWorkflowCommand {
                id: "workflow-status".to_string(),
                command_class: "read".to_string(),
                invocation: "cairn workflow status <change> --root PATH".to_string(),
            },
        ],
        entry: GeneratedWorkflowEntryGuide {
            id: "entry".to_string(),
            version: 1,
            goal: "Find the current project state and request one artifact's stage instructions."
                .to_string(),
            layout: GENERATED_AGENT_WORKFLOW_NATIVE_LAYOUT.to_string(),
            discovery_routes: vec!["status".to_string(), "workflow-status".to_string()],
            readiness_routes: vec!["doctor".to_string()],
            stage_route: "workflow-instructions".to_string(),
            profile_route: "change-create".to_string(),
            profiles: vec!["assurance".to_string(), "spec-driven".to_string()],
            boundaries: vec![
                "Generated guidance does not grant command, filesystem, network, model, gate, or release authority.".to_string(),
                "Report checked evidence or an exact blocker; generated text alone never proves implementation, gate passage, or release eligibility.".to_string(),
            ],
            required_evidence: vec!["Current status and workflow receipts".to_string()],
            max_bytes: 4096,
            max_stage_route_bytes: 512,
        },
        descriptors,
        adapters: vec![
            GeneratedWorkflowAdapter {
                id: "agent-skills".to_string(),
                version: 1,
                format: GeneratedWorkflowAdapterFormat::AgentSkills,
                file_prefix: "cairn-".to_string(),
                file_suffix: String::new(),
                directory_suffix: "/SKILL.md".to_string(),
                front_matter: true,
                max_files: 8,
            },
            GeneratedWorkflowAdapter {
                id: "claude".to_string(),
                version: 1,
                format: GeneratedWorkflowAdapterFormat::Claude,
                file_prefix: "cairn-".to_string(),
                file_suffix: ".md".to_string(),
                directory_suffix: String::new(),
                front_matter: false,
                max_files: 8,
            },
            GeneratedWorkflowAdapter {
                id: "codex".to_string(),
                version: 1,
                format: GeneratedWorkflowAdapterFormat::Codex,
                file_prefix: "cairn-".to_string(),
                file_suffix: ".md".to_string(),
                directory_suffix: String::new(),
                front_matter: false,
                max_files: 8,
            },
        ],
        bounds: GeneratedWorkflowBounds {
            max_descriptors: 6,
            max_adapters: 3,
            max_command_classes: 8,
            max_text_bytes: 4096,
            max_list_items: 32,
            max_files_per_adapter: 8,
            max_path_bytes: 4096,
            max_file_bytes: 65536,
            max_issues: 64,
        },
        receipt_fields: [
            "adapter",
            "destination",
            "files",
            "manifest",
            "plan_hash",
            "policy_hash",
            "non_claims",
        ]
        .into_iter()
        .map(str::to_string)
        .collect(),
        required_non_claims: NON_CLAIMS
            .iter()
            .map(|value| (*value).to_string())
            .collect(),
    }
}

fn empty_snapshot() -> GeneratedWorkflowDestinationSnapshot {
    GeneratedWorkflowDestinationSnapshot {
        files: Vec::new(),
        manifest: GeneratedWorkflowManifestObservation::Missing,
        pending_transactions: Vec::new(),
    }
}

fn installed_snapshot(plan: &GeneratedWorkflowInstallPlan) -> GeneratedWorkflowDestinationSnapshot {
    GeneratedWorkflowDestinationSnapshot {
        files: plan
            .files
            .iter()
            .map(|file| GeneratedWorkflowFileObservation {
                relative_path: file.relative_path.clone(),
                kind: GeneratedWorkflowObservedKind::Regular,
                content_blake3: Some(file.generated_blake3.clone()),
            })
            .collect(),
        manifest: GeneratedWorkflowManifestObservation::Present(Box::new(plan.manifest.clone())),
        pending_transactions: Vec::new(),
    }
}

#[test]
fn admits_and_renders_each_bounded_adapter() {
    let catalog = admit_generated_agent_workflow_policy(&policy()).expect("admitted policy");
    for adapter in INITIAL_ADAPTER_IDS {
        let files = render_generated_agent_workflows(&catalog, adapter).expect("rendered files");
        assert_eq!(files.len(), INITIAL_WORKFLOW_IDS.len() + 1);
        assert!(files.iter().all(|file| file.contents.contains("## Goal")));
        assert!(files
            .iter()
            .all(|file| file.contents.contains("## Non-claims")));
        let explore = files
            .iter()
            .find(|file| file.workflow_id == "explore")
            .expect("rendered explore workflow");
        assert!(explore
            .contents
            .contains(GENERATED_AGENT_WORKFLOW_EXPLORE_NO_WRITE));
        let entry = files
            .iter()
            .find(|file| file.workflow_id == GENERATED_AGENT_WORKFLOW_ENTRY_ID)
            .expect("rendered entry guide");
        assert!(!contains_legacy_layout_reference(&entry.contents));
        assert!(entry.contents.contains(".cairn/"));
    }
    let skill = render_generated_agent_workflows(&catalog, "agent-skills")
        .expect("skills")
        .remove(0);
    assert!(skill.relative_path.ends_with("/SKILL.md"));
    assert!(skill.contents.starts_with("---\nname: cairn-"));
}

#[test]
fn rejects_hidden_explore_mutation_and_unknown_adapter() {
    let mut invalid = policy();
    let explore = invalid
        .descriptors
        .iter_mut()
        .find(|descriptor| descriptor.id == "explore")
        .expect("explore");
    explore.command_classes.push("mutate".to_string());
    let issues = admit_generated_agent_workflow_policy(&invalid).expect_err("must reject");
    assert!(issues
        .iter()
        .any(|issue| issue.field_path.contains("explore") || issue.message.contains("read-only")));

    let catalog = admit_generated_agent_workflow_policy(&policy()).expect("admitted policy");
    let issues = render_generated_agent_workflows(&catalog, "remote-agent")
        .expect_err("unknown adapter must fail");
    assert_eq!(issues[0].code, GENERATED_AGENT_WORKFLOW_ISSUE_ADAPTER);
}

#[test]
fn adapter_projection_rejects_authority_and_runtime_hooks() {
    for field in ["authority", "network_hook", "model_hook"] {
        let mut adapter = serde_json::to_value(&policy().adapters[0]).expect("adapter value");
        adapter[field] = serde_json::json!("forbidden");
        let error = serde_json::from_value::<GeneratedWorkflowAdapter>(adapter)
            .expect_err("adapter must not add authority or runtime hooks");
        assert!(error.to_string().contains("unknown field"));
    }
}

#[test]
fn rejects_unsafe_destination_before_planning() {
    let catalog = admit_generated_agent_workflow_policy(&policy()).expect("admitted policy");
    for destination in ["../home", "/tmp/skills", "~/.agents", "skills/*"] {
        let issues = plan_generated_agent_workflow_install(
            &catalog,
            "agent-skills",
            destination,
            POLICY_HASH,
            &empty_snapshot(),
        )
        .expect_err("unsafe destination must fail");
        assert_eq!(issues[0].code, GENERATED_AGENT_WORKFLOW_ISSUE_DESTINATION);
    }
}

#[test]
fn plans_fresh_install_and_classifies_current_install() {
    let catalog = admit_generated_agent_workflow_policy(&policy()).expect("admitted policy");
    let fresh = plan_generated_agent_workflow_install(
        &catalog,
        "agent-skills",
        DESTINATION,
        POLICY_HASH,
        &empty_snapshot(),
    )
    .expect("fresh plan");
    assert!(!fresh.blocked());
    assert!(!fresh.current());
    assert!(fresh
        .files
        .iter()
        .all(|file| file.freshness == GeneratedWorkflowFreshness::Missing));
    assert!(validate_generated_workflow_manifest(&fresh.manifest));

    let current = plan_generated_agent_workflow_install(
        &catalog,
        "agent-skills",
        DESTINATION,
        POLICY_HASH,
        &installed_snapshot(&fresh),
    )
    .expect("current plan");
    assert!(current.current());
    assert_eq!(
        fresh.manifest.manifest_identity,
        current.manifest.manifest_identity
    );
}

#[test]
fn blocks_unmanaged_modified_missing_and_symlink_destinations() {
    let catalog = admit_generated_agent_workflow_policy(&policy()).expect("admitted policy");
    let fresh = plan_generated_agent_workflow_install(
        &catalog,
        "claude",
        ".claude/commands",
        POLICY_HASH,
        &empty_snapshot(),
    )
    .expect("fresh plan");

    let entry_path = fresh
        .files
        .iter()
        .find(|file| file.workflow_id == GENERATED_AGENT_WORKFLOW_ENTRY_ID)
        .expect("entry file")
        .relative_path
        .clone();
    let unmanaged = GeneratedWorkflowDestinationSnapshot {
        files: vec![GeneratedWorkflowFileObservation {
            relative_path: entry_path.clone(),
            kind: GeneratedWorkflowObservedKind::Regular,
            content_blake3: Some(
                "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string(),
            ),
        }],
        manifest: GeneratedWorkflowManifestObservation::Missing,
        pending_transactions: Vec::new(),
    };
    let plan = plan_generated_agent_workflow_install(
        &catalog,
        "claude",
        ".claude/commands",
        POLICY_HASH,
        &unmanaged,
    )
    .expect("conflict plan");
    assert!(plan.blocked());
    assert!(plan.files.iter().any(|file| {
        file.relative_path == entry_path
            && file.freshness == GeneratedWorkflowFreshness::UnmanagedConflict
    }));

    let mut installed = installed_snapshot(&fresh);
    let entry_index = installed
        .files
        .iter()
        .position(|file| file.relative_path == entry_path)
        .expect("entry observation");
    installed.files[entry_index].kind = GeneratedWorkflowObservedKind::Symlink;
    installed.files[1].content_blake3 =
        Some("cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc".to_string());
    installed.files.remove(2);
    let blocked = plan_generated_agent_workflow_install(
        &catalog,
        "claude",
        ".claude/commands",
        POLICY_HASH,
        &installed,
    )
    .expect("blocked plan");
    assert!(blocked.blocked());
    assert!(blocked
        .files
        .iter()
        .any(|file| file.freshness == GeneratedWorkflowFreshness::SymlinkConflict));
    assert!(blocked
        .files
        .iter()
        .any(|file| file.freshness == GeneratedWorkflowFreshness::Modified));
    assert!(blocked
        .files
        .iter()
        .any(|file| file.freshness == GeneratedWorkflowFreshness::Missing));

    let mut missing_entry = installed_snapshot(&fresh);
    missing_entry
        .files
        .retain(|file| file.relative_path != entry_path);
    let missing_entry_plan = plan_generated_agent_workflow_install(
        &catalog,
        "claude",
        ".claude/commands",
        POLICY_HASH,
        &missing_entry,
    )
    .expect("missing entry plan");
    assert!(missing_entry_plan.blocked());
    assert!(missing_entry_plan.files.iter().any(|file| {
        file.workflow_id == GENERATED_AGENT_WORKFLOW_ENTRY_ID
            && file.freshness == GeneratedWorkflowFreshness::Missing
    }));
}

#[test]
fn obsolete_manifest_path_is_reported_without_a_delete_effect() {
    let catalog = admit_generated_agent_workflow_policy(&policy()).expect("admitted policy");
    let fresh = plan_generated_agent_workflow_install(
        &catalog,
        "codex",
        ".codex/prompts",
        POLICY_HASH,
        &empty_snapshot(),
    )
    .expect("fresh plan");
    let mut snapshot = installed_snapshot(&fresh);
    let prior = match &mut snapshot.manifest {
        GeneratedWorkflowManifestObservation::Present(manifest) => manifest,
        _ => panic!("installed manifest"),
    };
    let mut obsolete = prior.files[0].clone();
    obsolete.workflow_id = "obsolete".to_string();
    obsolete.relative_path = ".codex/prompts/cairn-obsolete.md".to_string();
    prior.files.push(obsolete);
    prior.manifest_identity = generated_workflow_manifest_identity(prior);

    let plan = plan_generated_agent_workflow_install(
        &catalog,
        "codex",
        ".codex/prompts",
        POLICY_HASH,
        &snapshot,
    )
    .expect("obsolete path plan");
    assert_eq!(
        plan.removal_candidates,
        vec![".codex/prompts/cairn-obsolete.md".to_string()]
    );
    assert!(plan
        .files
        .iter()
        .all(|file| { file.relative_path != ".codex/prompts/cairn-obsolete.md" }));
    let repeated = plan_generated_agent_workflow_install(
        &catalog,
        "codex",
        ".codex/prompts",
        POLICY_HASH,
        &installed_snapshot(&plan),
    )
    .expect("repeated obsolete path plan");
    assert_eq!(repeated.removal_candidates, plan.removal_candidates);
}

#[test]
fn unsupported_manifest_version_blocks_installation() {
    let catalog = admit_generated_agent_workflow_policy(&policy()).expect("admitted policy");
    let fresh = plan_generated_agent_workflow_install(
        &catalog,
        "agent-skills",
        DESTINATION,
        POLICY_HASH,
        &empty_snapshot(),
    )
    .expect("fresh plan");
    let mut snapshot = installed_snapshot(&fresh);
    let prior = match &mut snapshot.manifest {
        GeneratedWorkflowManifestObservation::Present(manifest) => manifest,
        _ => panic!("installed manifest"),
    };
    prior.adapter_version += 1;
    prior.manifest_identity = generated_workflow_manifest_identity(prior);
    let plan = plan_generated_agent_workflow_install(
        &catalog,
        "agent-skills",
        DESTINATION,
        POLICY_HASH,
        &snapshot,
    )
    .expect("unsupported plan");
    assert!(plan.blocked());
    assert_eq!(
        plan.manifest_freshness,
        GeneratedWorkflowFreshness::UnsupportedVersion
    );
}

#[test]
fn source_change_is_stale_but_preserves_modified_file_protection() {
    let first_catalog = admit_generated_agent_workflow_policy(&policy()).expect("first policy");
    let first = plan_generated_agent_workflow_install(
        &first_catalog,
        "codex",
        ".codex/prompts",
        POLICY_HASH,
        &empty_snapshot(),
    )
    .expect("first plan");
    let snapshot = installed_snapshot(&first);
    let mut changed_policy = policy();
    changed_policy.descriptors[0]
        .goal
        .push_str(" Keep scope bounded.");
    let changed_catalog =
        admit_generated_agent_workflow_policy(&changed_policy).expect("changed policy");
    let changed = plan_generated_agent_workflow_install(
        &changed_catalog,
        "codex",
        ".codex/prompts",
        "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd",
        &snapshot,
    )
    .expect("changed plan");
    assert!(!changed.blocked());
    assert!(changed
        .files
        .iter()
        .any(|file| file.freshness == GeneratedWorkflowFreshness::StaleSource));
    assert_eq!(
        changed.manifest_freshness,
        GeneratedWorkflowFreshness::StaleSource
    );
}

fn entry_file(
    catalog: &AdmittedGeneratedWorkflowCatalog,
    adapter: &str,
) -> RenderedGeneratedWorkflowFile {
    render_generated_agent_workflows(catalog, adapter)
        .expect("rendered files")
        .into_iter()
        .find(|file| file.workflow_id == GENERATED_AGENT_WORKFLOW_ENTRY_ID)
        .expect("entry guide")
}

#[test]
fn entry_guide_routes_are_semantically_equal_across_adapters() {
    let catalog = admit_generated_agent_workflow_policy(&policy()).expect("admitted policy");
    for adapter in INITIAL_ADAPTER_IDS {
        let entry = entry_file(&catalog, adapter);
        for command in &catalog.policy.command_catalog {
            if catalog.policy.entry.discovery_routes.contains(&command.id)
                || catalog.policy.entry.readiness_routes.contains(&command.id)
                || catalog.policy.entry.stage_route == command.id
                || catalog.policy.entry.profile_route == command.id
            {
                assert!(
                    entry.contents.contains(&command.invocation),
                    "{} misses {}",
                    adapter,
                    command.id
                );
            }
        }
        for profile in &catalog.policy.entry.profiles {
            assert!(
                entry.contents.contains(profile),
                "{adapter} misses {profile}"
            );
        }
        for boundary in &catalog.policy.entry.boundaries {
            assert!(entry.contents.contains(boundary));
        }
        assert!(entry
            .contents
            .contains("Load only that selected stage guide"));
        assert!(!entry.contents.contains("## Immediate action"));
        for descriptor in &catalog.policy.descriptors {
            assert!(!entry.contents.contains(&descriptor.goal));
        }
    }
}

#[test]
fn entry_guide_rejects_legacy_layouts_unknown_routes_and_profiles() {
    let mut legacy = policy();
    legacy.entry.layout = "cairn".to_string();
    let issues = admit_generated_agent_workflow_policy(&legacy).expect_err("legacy layout");
    assert!(issues.iter().any(|issue| {
        issue.code == GENERATED_AGENT_WORKFLOW_ISSUE_ENTRY && issue.field_path.contains("entry")
    }));

    let mut unknown_route = policy();
    unknown_route.entry.discovery_routes = vec!["remote-command".to_string()];
    let issues = admit_generated_agent_workflow_policy(&unknown_route).expect_err("unknown route");
    assert!(issues
        .iter()
        .any(|issue| issue.field_path.contains("discovery_routes")));

    let mut legacy_text = policy();
    legacy_text.entry.boundaries[0] = "Read cairn/changes for current work.".to_string();
    let issues = admit_generated_agent_workflow_policy(&legacy_text).expect_err("legacy text");
    assert!(issues
        .iter()
        .any(|issue| issue.field_path.contains("boundaries")));

    let mut legacy_command = policy();
    legacy_command.command_catalog[2].invocation = "cairn status --root cairn/specs".to_string();
    let issues =
        admit_generated_agent_workflow_policy(&legacy_command).expect_err("legacy command");
    assert!(issues
        .iter()
        .any(|issue| issue.field_path.contains("invocation")));

    let catalog = admit_generated_agent_workflow_policy(&policy()).expect("admitted policy");
    assert!(find_entry_guide_profile(&catalog, "spec-driven").is_ok());
    let issue = find_entry_guide_profile(&catalog, "remote-profile").expect_err("unknown profile");
    assert!(issue.message.contains("spec-driven"));

    assert!(contains_legacy_layout_reference("read cairn/changes"));
    assert!(contains_legacy_layout_reference("cairn/specs"));
    assert!(!contains_legacy_layout_reference("read .cairn/changes"));
    assert!(!contains_legacy_layout_reference(
        "read spec-driven/assurance"
    ));
}

#[test]
fn entry_guide_budget_must_carry_mandatory_constraints() {
    let mut too_small = policy();
    too_small.entry.max_bytes = 64;
    let issues = admit_generated_agent_workflow_policy(&too_small).expect_err("small entry budget");
    assert!(issues
        .iter()
        .any(|issue| issue.field_path.contains("entry.max_bytes")));

    let mut file_bound = policy();
    file_bound.bounds.max_file_bytes = 512;
    let issues = admit_generated_agent_workflow_policy(&file_bound).expect_err("small file budget");
    assert!(issues
        .iter()
        .any(|issue| issue.field_path.contains("entry.max_bytes")));

    let mut stage_bound = policy();
    stage_bound.entry.max_stage_route_bytes = 8;
    let issues =
        admit_generated_agent_workflow_policy(&stage_bound).expect_err("small stage budget");
    assert!(issues
        .iter()
        .any(|issue| issue.field_path.contains("max_stage_route_bytes")));
}

#[test]
fn command_catalog_drift_changes_entry_identity_and_reports_stale_source() {
    let first_catalog = admit_generated_agent_workflow_policy(&policy()).expect("first policy");
    let first = plan_generated_agent_workflow_install(
        &first_catalog,
        "codex",
        ".codex/prompts",
        POLICY_HASH,
        &empty_snapshot(),
    )
    .expect("first plan");
    let first_entry = first
        .files
        .iter()
        .find(|file| file.workflow_id == GENERATED_AGENT_WORKFLOW_ENTRY_ID)
        .expect("first entry");

    let mut changed = policy();
    changed
        .command_catalog
        .iter_mut()
        .find(|command| command.id == "status")
        .expect("status command")
        .invocation = "cairn status --root . --format json".to_string();
    let second_catalog = admit_generated_agent_workflow_policy(&changed).expect("second policy");
    assert_ne!(
        first_catalog.command_catalog_identity,
        second_catalog.command_catalog_identity
    );
    assert_ne!(
        first_catalog.catalog_identity,
        second_catalog.catalog_identity
    );
    let second = plan_generated_agent_workflow_install(
        &second_catalog,
        "codex",
        ".codex/prompts",
        POLICY_HASH,
        &installed_snapshot(&first),
    )
    .expect("second plan");
    let second_entry = second
        .files
        .iter()
        .find(|file| file.workflow_id == GENERATED_AGENT_WORKFLOW_ENTRY_ID)
        .expect("second entry");
    assert_ne!(
        first_entry.descriptor_identity,
        second_entry.descriptor_identity
    );
    assert_eq!(
        second_entry.freshness,
        GeneratedWorkflowFreshness::StaleSource
    );
    assert_eq!(
        second.manifest_freshness,
        GeneratedWorkflowFreshness::StaleSource
    );
}

#[test]
fn copied_guide_inspection_is_report_only_and_identity_bound() {
    let catalog = admit_generated_agent_workflow_policy(&policy()).expect("admitted policy");
    let entry = entry_file(&catalog, "claude");
    let plan = plan_generated_agent_workflow_install(
        &catalog,
        "claude",
        ".claude/commands",
        POLICY_HASH,
        &empty_snapshot(),
    )
    .expect("plan");
    let manifest = &plan.manifest;
    let current = inspect_copied_generated_workflow_guide(
        &entry.contents,
        GENERATED_AGENT_WORKFLOW_ENTRY_ID,
        Some(manifest),
        Some(&manifest.manifest_identity),
    );
    assert_eq!(
        current.classification,
        GeneratedWorkflowCopiedGuideClassification::Current
    );
    assert!(current.native_layout_current);
    assert!(current.issues.is_empty());

    let stale = inspect_copied_generated_workflow_guide(
        "previous generated guide",
        GENERATED_AGENT_WORKFLOW_ENTRY_ID,
        Some(manifest),
        Some(&manifest.manifest_identity),
    );
    assert_eq!(
        stale.classification,
        GeneratedWorkflowCopiedGuideClassification::Modified
    );

    let unclassified = inspect_copied_generated_workflow_guide(
        &entry.contents,
        GENERATED_AGENT_WORKFLOW_ENTRY_ID,
        Some(manifest),
        Some(POLICY_HASH),
    );
    assert_eq!(
        unclassified.classification,
        GeneratedWorkflowCopiedGuideClassification::Unclassified
    );
    assert!(unclassified
        .issues
        .iter()
        .any(|issue| issue.code == GENERATED_AGENT_WORKFLOW_ISSUE_COPIED_GUIDE));

    let without_identity = inspect_copied_generated_workflow_guide(
        "arbitrary prose",
        GENERATED_AGENT_WORKFLOW_ENTRY_ID,
        None,
        None,
    );
    assert_eq!(
        without_identity.classification,
        GeneratedWorkflowCopiedGuideClassification::Unclassified
    );

    let legacy = inspect_copied_generated_workflow_guide(
        "Read cairn/changes first.",
        GENERATED_AGENT_WORKFLOW_ENTRY_ID,
        Some(manifest),
        Some(&manifest.manifest_identity),
    );
    assert!(!legacy.native_layout_current);
    assert!(legacy
        .issues
        .iter()
        .any(|issue| issue.code == GENERATED_AGENT_WORKFLOW_ISSUE_GUIDE_LAYOUT));
}

#[test]
fn pending_transactions_block_installation_for_deliberate_recovery() {
    let catalog = admit_generated_agent_workflow_policy(&policy()).expect("admitted policy");
    let mut snapshot = empty_snapshot();
    snapshot.pending_transactions =
        vec![".cairn/generated-agent-workflow-tmp-abc.pending".to_string()];
    let plan = plan_generated_agent_workflow_install(
        &catalog,
        "agent-skills",
        DESTINATION,
        POLICY_HASH,
        &snapshot,
    )
    .expect("recovery plan");
    assert!(plan.blocked());
    assert_eq!(plan.recovery_required, snapshot.pending_transactions);
    assert!(plan
        .blockers
        .iter()
        .any(|blocker| blocker.contains(GENERATED_AGENT_WORKFLOW_ISSUE_RECOVERY)));
    assert!(!plan.current());
}

fn descriptor_mut<'a>(
    policy: &'a mut GeneratedAgentWorkflowPolicy,
    id: &str,
) -> &'a mut GeneratedWorkflowDescriptor {
    policy
        .descriptors
        .iter_mut()
        .find(|descriptor| descriptor.id == id)
        .expect("descriptor")
}

fn command_mention_issues(policy: &GeneratedAgentWorkflowPolicy) -> Vec<GeneratedWorkflowIssue> {
    match admit_generated_agent_workflow_policy(policy) {
        Ok(_) => Vec::new(),
        Err(issues) => issues,
    }
}

// r[verify cairn.generated_agent_workflows.command_routes]
// r[verify cairn.generated_agent_workflows.command_routes.permitted]
#[test]
fn named_commands_resolve_to_permitted_catalog_classes() {
    let mut valid = policy();
    descriptor_mut(&mut valid, "explore").immediate_action =
        "Run cairn status, then `cairn workflow status CHANGE` and cairn workflow instructions ARTIFACT."
            .to_string();
    descriptor_mut(&mut valid, "apply").immediate_action =
        "Run cairn change create NAME, then cairn doctor.".to_string();
    descriptor_mut(&mut valid, "verify").constraints = vec![
        "Use only policy-known Cairn command classes; cairn, not a shell, decides.".to_string(),
    ];
    assert!(command_mention_issues(&valid).is_empty());

    // The longest command-word prefix decides: `cairn workflow status` is the
    // workflow-status entry, never the shorter `cairn status`.
    let mut longest = valid.clone();
    longest
        .command_catalog
        .iter_mut()
        .find(|command| command.id == "workflow-status")
        .expect("workflow-status")
        .command_class = "mutate".to_string();
    descriptor_mut(&mut longest, "explore").immediate_action =
        "Run cairn status for the explicit root.".to_string();
    assert!(command_mention_issues(&longest).is_empty());
}

// r[verify cairn.generated_agent_workflows.command_routes]
// r[verify cairn.generated_agent_workflows.command_routes.refused]
#[test]
fn named_commands_outside_permitted_classes_fail_admission() {
    let mut unpermitted = policy();
    descriptor_mut(&mut unpermitted, "verify").immediate_action =
        "Run cairn change create NAME before checking.".to_string();
    let issues = command_mention_issues(&unpermitted);
    assert_eq!(issues.len(), 1, "{issues:?}");
    assert_eq!(issues[0].code, GENERATED_AGENT_WORKFLOW_ISSUE_DESCRIPTOR);
    assert_eq!(
        issues[0].field_path,
        "generated_agent_workflow_policy.descriptors[5].immediate_action"
    );
    assert!(issues[0]
        .message
        .contains("cairn change create, catalog command change-create of class mutate"));

    let mut longest = policy();
    longest
        .command_catalog
        .iter_mut()
        .find(|command| command.id == "workflow-status")
        .expect("workflow-status")
        .command_class = "mutate".to_string();
    descriptor_mut(&mut longest, "explore").immediate_action =
        "Run cairn workflow status CHANGE.".to_string();
    let issues = command_mention_issues(&longest);
    assert!(
        issues.iter().any(|issue| issue.field_path
            == "generated_agent_workflow_policy.descriptors[3].immediate_action"
            && issue.message.contains("catalog command workflow-status")),
        "{issues:?}"
    );

    let mut uncatalogued = policy();
    descriptor_mut(&mut uncatalogued, "drain").constraints =
        vec!["Inspect (cairn drain-plan --view readiness) first.".to_string()];
    let issues = command_mention_issues(&uncatalogued);
    assert_eq!(issues.len(), 1, "{issues:?}");
    assert_eq!(
        issues[0].field_path,
        "generated_agent_workflow_policy.descriptors[2].constraints[0]"
    );
    assert!(issues[0]
        .message
        .contains("names cairn drain-plan, which matches no command catalog invocation"));

    let mut every_field = policy();
    let apply = descriptor_mut(&mut every_field, "apply");
    apply.command_classes = vec!["mutate".to_string()];
    apply.goal = "Use cairn status.".to_string();
    apply.success_evidence = vec!["cairn doctor reports no issue.".to_string()];
    apply.required_evidence = vec!["cairn workflow status output".to_string()];
    apply.progress_rule = "Rerun cairn status.".to_string();
    apply.output_shape = "Quote cairn doctor.".to_string();
    apply
        .non_claims
        .push("cairn status proves nothing.".to_string());
    let fields = command_mention_issues(&every_field)
        .into_iter()
        .map(|issue| issue.field_path)
        .collect::<std::collections::BTreeSet<_>>();
    let prefix = "generated_agent_workflow_policy.descriptors[0]";
    for field in [
        "goal",
        "success_evidence[0]",
        "required_evidence[0]",
        "progress_rule",
        "output_shape",
    ] {
        assert!(
            fields.contains(&format!("{prefix}.{field}")),
            "{field}: {fields:?}"
        );
    }
    assert!(fields.contains(&format!("{prefix}.non_claims[{}]", NON_CLAIMS.len())));
}

/// The test policy with a read-only `gate` class and the `gate-tasks` entry.
fn policy_with_gate() -> GeneratedAgentWorkflowPolicy {
    let mut policy = policy();
    policy.command_classes.push(GeneratedWorkflowCommandClass {
        id: "gate".to_string(),
        permission: GeneratedWorkflowPermission::Read,
    });
    policy.command_catalog.push(GeneratedWorkflowCommand {
        id: "gate-tasks".to_string(),
        command_class: "gate".to_string(),
        invocation: "cairn gate tasks <change> --root PATH".to_string(),
    });
    policy
}

// r[verify cairn.generated_agent_workflows.command_routes.permitted]
#[test]
fn fence_info_strings_are_not_commands() {
    let mut fenced = policy();
    descriptor_mut(&mut fenced, "explore").immediate_action =
        "Run:\n```cairn\ncairn status\n```\nthen:\n~~~cairn change create\ncairn doctor\n~~~"
            .to_string();
    assert!(command_mention_issues(&fenced).is_empty());

    // An inline triple-backtick span is not a fence; its command is read.
    let mut inline = policy();
    descriptor_mut(&mut inline, "explore").immediate_action =
        "Run ```cairn status``` first.".to_string();
    assert!(command_mention_issues(&inline).is_empty());
}

// r[verify cairn.generated_agent_workflows.command_routes.refused]
#[test]
fn fenced_body_commands_are_still_checked() {
    let mut fenced = policy();
    descriptor_mut(&mut fenced, "verify").immediate_action =
        "Run:\n```cairn\ncairn change create NAME\n```".to_string();
    let issues = command_mention_issues(&fenced);
    assert_eq!(issues.len(), 1, "{issues:?}");
    assert_eq!(
        issues[0].field_path,
        "generated_agent_workflow_policy.descriptors[5].immediate_action"
    );
    assert!(
        issues[0]
            .message
            .contains("names cairn change create, catalog command change-create of class mutate"),
        "{issues:?}"
    );
    assert!(!issues[0].message.contains("cairn cairn"), "{issues:?}");
}

const WRAPPED_GATE_MENTIONS: &[&str] = &[
    "Run **cairn gate tasks CHANGE**",
    "Run `cairn gate tasks CHANGE`.",
    "Then (cairn gate tasks CHANGE).",
    "Run _cairn gate tasks_; then report.",
    "Run \"cairn gate tasks\" and ['cairn gate tasks'].",
];

// r[verify cairn.generated_agent_workflows.command_routes.permitted]
#[test]
fn markdown_wrapped_commands_resolve_when_permitted() {
    for text in WRAPPED_GATE_MENTIONS {
        let mut permitted = policy_with_gate();
        let apply = descriptor_mut(&mut permitted, "apply");
        apply.command_classes.push("gate".to_string());
        apply.immediate_action = (*text).to_string();
        let issues = command_mention_issues(&permitted);
        assert!(issues.is_empty(), "{text}: {issues:?}");
    }
    // Capitalized prose stays prose, wrapped or not.
    let mut prose = policy_with_gate();
    descriptor_mut(&mut prose, "verify").immediate_action =
        "**Cairn** decides; Cairn gate tasks are `Cairn`'s, (Cairn) gate tasks.".to_string();
    assert!(command_mention_issues(&prose).is_empty());
}

// r[verify cairn.generated_agent_workflows.command_routes.refused]
#[test]
fn markdown_wrapped_commands_outside_permitted_classes_fail_admission() {
    for text in WRAPPED_GATE_MENTIONS {
        let mut refused = policy_with_gate();
        descriptor_mut(&mut refused, "apply").immediate_action = (*text).to_string();
        let issues = command_mention_issues(&refused);
        assert_eq!(issues.len(), 1, "{text}: {issues:?}");
        assert_eq!(
            issues[0].field_path,
            "generated_agent_workflow_policy.descriptors[0].immediate_action"
        );
        assert!(
            issues[0]
                .message
                .contains("names cairn gate tasks, catalog command gate-tasks of class gate"),
            "{text}: {issues:?}"
        );
    }
}

/// Repeats `unit` as often as fits in `max_bytes` and pads the rest with
/// spaces, so the text is exactly `max_bytes`; returns the text and count.
fn repeated_to_bound(unit: &str, max_bytes: usize) -> (String, usize) {
    let count = max_bytes / unit.len();
    let mut text = unit.repeat(count);
    text.push_str(&" ".repeat(max_bytes - text.len()));
    assert_eq!(text.len(), max_bytes);
    (text, count)
}

// r[verify cairn.generated_agent_workflows.command_routes.permitted]
#[test]
fn command_scan_reads_each_token_a_bounded_number_of_times() {
    let policy = policy_with_gate();
    let max_bytes = policy.bounds.max_text_bytes;
    // The deepest catalog invocations (`change create`, `gate tasks`,
    // `workflow status`) have two command words.
    let max_depth = 2;

    let (text, count) = repeated_to_bound("cairn status ", max_bytes);
    let mut mentions = 0;
    let inspected = super::admission::scan_command_mentions(&text, max_depth, |words| {
        // The lookahead stops at `max_depth` words: `status cairn`, or
        // `status` at the end of the text.
        assert_eq!(words.first(), Some(&"status"));
        assert!(words.len() <= max_depth);
        mentions += 1;
    });
    assert_eq!(mentions, count);
    let tokens = 2 * count;
    assert_eq!(inspected, tokens + 2 * (count - 1) + 1);
    assert!(inspected <= tokens * (1 + max_depth), "{inspected}");

    let mut admitted = policy.clone();
    descriptor_mut(&mut admitted, "explore").goal = text;
    assert!(command_mention_issues(&admitted).is_empty());
}

// r[verify cairn.generated_agent_workflows.command_routes.refused]
#[test]
fn repeated_cairn_tokens_fail_admission_in_linear_work() {
    let policy = policy_with_gate();
    let max_bytes = policy.bounds.max_text_bytes;
    let max_depth = 2;

    let (text, tokens) = repeated_to_bound("cairn ", max_bytes);
    assert_eq!(text.len(), max_bytes);
    let mut mentions = 0;
    let inspected = super::admission::scan_command_mentions(&text, max_depth, |words| {
        assert!(words.len() <= max_depth && words.iter().all(|word| *word == "cairn"));
        mentions += 1;
    });
    // Every token but the last starts a mention, and each mention reads at
    // most `max_depth` following tokens: linear, not quadratic, in the text.
    assert_eq!(mentions, tokens - 1);
    assert_eq!(inspected, tokens + 2 * (tokens - 2) + 1);
    assert!(inspected <= tokens * (1 + max_depth));

    let mut refused = policy.clone();
    descriptor_mut(&mut refused, "explore").goal = text;
    let issues = command_mention_issues(&refused);
    assert_eq!(issues.len(), 1, "{issues:?}");
    assert_eq!(
        issues[0].field_path,
        "generated_agent_workflow_policy.descriptors[3].goal"
    );
    assert!(issues[0]
        .message
        .contains("names cairn cairn, which matches no command catalog invocation"));
}

const DIAGNOSTIC_BOUND_BYTES: usize = 512;

fn issue_bytes(issue: &GeneratedWorkflowIssue) -> usize {
    issue.code.len() + issue.field_path.len() + issue.message.len()
}

// r[verify cairn.generated_agent_workflows.command_routes.refused]
#[test]
fn diagnostics_copy_short_identifiers_verbatim() {
    let id = "r".repeat(64);
    let mut refused = policy();
    refused.command_catalog.push(GeneratedWorkflowCommand {
        id: id.clone(),
        command_class: "mutate".to_string(),
        invocation: "cairn rebuild <change> --root PATH".to_string(),
    });
    descriptor_mut(&mut refused, "verify").immediate_action =
        "Run cairn rebuild CHANGE.".to_string();
    let issues = command_mention_issues(&refused);
    assert_eq!(issues.len(), 1, "{issues:?}");
    assert!(issues[0]
        .message
        .contains(&format!("catalog command {id} of class mutate,")));
    assert!(!issues[0].message.contains("truncated"));
}

// r[verify cairn.generated_agent_workflows.command_routes.refused]
#[test]
fn diagnostics_bound_megabyte_identifiers() {
    let huge = "a".repeat(1 << 20);
    let mut refused = policy();
    refused.command_catalog.push(GeneratedWorkflowCommand {
        id: huge.clone(),
        command_class: "mutate".to_string(),
        invocation: "cairn rebuild <change> --root PATH".to_string(),
    });
    refused.command_catalog.push(GeneratedWorkflowCommand {
        id: "huge-class".to_string(),
        command_class: huge.clone(),
        invocation: "cairn doctor --root PATH".to_string(),
    });
    let verify = descriptor_mut(&mut refused, "verify");
    verify.immediate_action = "Run cairn rebuild CHANGE.".to_string();
    verify.command_classes.push(huge.clone());
    let issues = command_mention_issues(&refused);
    assert!(issues.len() >= 3, "{}", issues.len());
    for issue in &issues {
        assert!(
            issue_bytes(issue) < DIAGNOSTIC_BOUND_BYTES,
            "{} bytes at {}",
            issue_bytes(issue),
            issue.field_path
        );
    }
    let marker = format!("{}...(truncated, {} bytes)", &huge[..64], huge.len());
    let message = |path: &str| {
        issues
            .iter()
            .find(|issue| issue.field_path == path)
            .map(|issue| issue.message.as_str())
            .unwrap_or_else(|| panic!("{path}: {issues:?}"))
    };
    assert!(
        message("generated_agent_workflow_policy.descriptors[5].immediate_action")
            .contains(&format!("catalog command {marker} of class mutate,"))
    );
    assert_eq!(
        message("generated_agent_workflow_policy.command_catalog[6].command_class"),
        format!("unknown command class: {marker}")
    );
    assert_eq!(
        message("generated_agent_workflow_policy.descriptors[5].command_classes"),
        format!("unknown command class: {marker}")
    );
}
