//! Read-only workflow obligation inspection and export.
//!
//! The check composes the current generated workflow catalog, the checked
//! obligation policy and an explicit case. It never executes actions, observes
//! clocks or starts monitoring. An unsatisfied outcome is a result, not an
//! error; only admission and input failures are rejected.
use std::{collections::BTreeMap, error::Error, path::Path};

use cairn_core::{
    admit_generated_agent_workflow_policy, admit_workflow_obligation_policy, canonical_json_hash,
    evaluate_workflow_obligations, export_workflow_obligation, find_workflow_obligation,
    AgentClaims, EvidenceObservation, ExceptionSnapshot, ObligationRequest, ProgressObservation,
    TemporalObservation, TerminalObservation, WorkflowObligationError, WorkflowObligationPolicy,
    WORKFLOW_OBLIGATION_BOUNDARY,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::support::{
    default_policy_path, load_policy_file, print_json, read_string_option,
    reject_positional_arguments, reject_unknown_options, CapabilityRoot, CapabilityRootKind,
};

const EXPLANATION_SCHEMA: &str = "cairn.workflow-obligation.inspection.v1";
const EXPORT_SCHEMA: &str = "cairn.workflow-obligation.consumer-export.v1";
const MAX_CASE_BYTES: u64 = 8 * 1024 * 1024;
const MAX_POLICY_BYTES: u64 = 16 * 1024 * 1024;
const DEFAULT_OBLIGATION_POLICY: &str =
    include_str!("../../../cairn-policy/generated/workflow-obligations.json");

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ObligationCase {
    definition: String,
    subject_blake3: String,
    context_blake3: String,
    #[serde(default)]
    selected_alternatives: BTreeMap<String, String>,
    #[serde(default)]
    evidence: Vec<EvidenceObservation>,
    #[serde(default)]
    progress: Vec<ProgressObservation>,
    #[serde(default)]
    agent_claims: Option<AgentClaims>,
    #[serde(default)]
    exceptions: Vec<ExceptionSnapshot>,
    #[serde(default)]
    temporal: Option<TemporalObservation>,
    #[serde(default)]
    terminal: Option<TerminalObservation>,
}

fn read_bounded_file(path: &str, maximum: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    let (root, relative) = CapabilityRoot::open_file_parent(
        CapabilityRootKind::EvidenceBundle,
        Path::new(path),
        false,
    )?;
    Ok(root.read_bounded(&relative, maximum)?)
}

fn rejection(outcome: &str, detail: String) -> Value {
    json!({
        "schema_version": EXPLANATION_SCHEMA,
        "outcome": "rejected",
        "advisory": true,
        "report": null,
        "error": {"outcome": outcome, "detail": detail},
        "boundary": WORKFLOW_OBLIGATION_BOUNDARY,
    })
}

fn rejected(error: WorkflowObligationError) -> Result<(), Box<dyn Error>> {
    print_json(rejection(error.outcome(), format!("{error:?}")))?;
    Err("workflow obligation admission rejected".into())
}

fn load_admitted(
    policy_path: &Path,
    obligation_policy_path: Option<&str>,
) -> Result<
    (
        cairn_core::AdmittedWorkflowObligationCatalog,
        String,
    ),
    Box<dyn Error>,
> {
    let policy = load_policy_file(policy_path.to_path_buf())?;
    let policy_hash = canonical_json_hash(&policy.raw);
    let workflow_catalog =
        admit_generated_agent_workflow_policy(&policy.generated_agent_workflow_policy)
            .map_err(|issues| format!("generated workflow policy is invalid: {issues:?}"))?;
    let obligation_policy: WorkflowObligationPolicy = match obligation_policy_path {
        Some(path) => serde_json::from_slice(&read_bounded_file(path, MAX_POLICY_BYTES)?)
            .map_err(|error| format!("workflow obligation policy is invalid: {error}"))?,
        None => serde_json::from_str(DEFAULT_OBLIGATION_POLICY)
            .map_err(|error| format!("workflow obligation policy is invalid: {error}"))?,
    };
    let catalog = match admit_workflow_obligation_policy(
        &obligation_policy,
        &workflow_catalog,
        &policy_hash,
    ) {
        Ok(catalog) => catalog,
        Err(issues) => {
            print_json(json!({
                "schema_version": EXPLANATION_SCHEMA,
                "outcome": "rejected",
                "advisory": true,
                "report": null,
                "error": {"outcome": "policy_rejected", "detail": format!("{issues:?}")},
                "boundary": WORKFLOW_OBLIGATION_BOUNDARY,
            }))?;
            return Err("workflow obligation policy admission rejected".into());
        }
    };
    Ok((catalog, policy_hash))
}

fn load_case(path: &str) -> Result<ObligationCase, Box<dyn Error>> {
    let bytes = read_bounded_file(path, MAX_CASE_BYTES)?;
    serde_json::from_slice(&bytes).map_err(|error| -> Box<dyn Error> {
        format!("workflow obligation case is invalid: {error}").into()
    })
}

fn case_and_definition<'a>(
    catalog: &'a cairn_core::AdmittedWorkflowObligationCatalog,
    case: &ObligationCase,
) -> Result<&'a cairn_core::AdmittedWorkflowObligation, Box<dyn Error>> {
    find_workflow_obligation(catalog, &case.definition).ok_or_else(|| -> Box<dyn Error> {
        format!(
            "workflow obligation definition {} is not admitted",
            case.definition
        )
        .into()
    })
}

/// Read-only inspection of admitted obligations. No lifecycle mutation.
// r[impl cairn.workflow_obligations.inspection]
pub(crate) fn run(mut args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let Some(command) = args.first().map(String::as_str) else {
        return Err(usage());
    };
    let command = command.to_string();
    args.remove(0);
    reject_unknown_options(
        &args,
        &["--case", "--policy", "--obligations"],
    )?;
    reject_positional_arguments(&args)?;
    let policy_path = read_string_option(&args, "--policy")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|| std::path::PathBuf::from(default_policy_path()));
    let obligations_path = read_string_option(&args, "--obligations");
    let (catalog, policy_hash) =
        load_admitted(&policy_path, obligations_path.as_deref())?;
    match command.as_str() {
        "list" => {
            let definitions = catalog
                .definitions
                .iter()
                .map(|admitted| {
                    json!({
                        "definition": admitted.definition.id,
                        "workflow": admitted.definition.workflow_id,
                        "workflow_version": admitted.definition.workflow_version,
                        "binding_blake3": admitted.binding.binding_blake3,
                        "command_classes": admitted.command_classes,
                        "temporal": admitted.definition.temporal.is_some(),
                        "exceptions": admitted.definition.exceptions.len(),
                    })
                })
                .collect::<Vec<_>>();
            print_json(json!({
                "schema_version": EXPORT_SCHEMA,
                "outcome": "listed",
                "advisory": true,
                "workflow_policy_blake3": policy_hash,
                "catalog_identity": catalog.catalog_identity,
                "definitions": definitions,
                "boundary": WORKFLOW_OBLIGATION_BOUNDARY,
            }))
        }
        "explain" => {
            let case = load_case(
                read_string_option(&args, "--case").ok_or("--case is required")?,
            )?;
            let admitted = case_and_definition(&catalog, &case)?;
            let ObligationCase {
                subject_blake3,
                context_blake3,
                selected_alternatives,
                evidence,
                progress,
                agent_claims,
                exceptions,
                temporal,
                terminal,
                ..
            } = case;
            let request = ObligationRequest {
                binding: admitted.binding.clone(),
                subject_blake3,
                context_blake3,
                selected_alternatives,
                evidence,
                progress,
                agent_claims,
                exceptions,
                temporal,
                terminal,
            };
            match evaluate_workflow_obligations(admitted, &catalog.policy, &request) {
                Ok(report) => print_json(json!({
                    "schema_version": EXPLANATION_SCHEMA,
                    "outcome": "evaluated",
                    "advisory": true,
                    "report": report,
                    "error": null,
                    "boundary": WORKFLOW_OBLIGATION_BOUNDARY,
                })),
                Err(error) => rejected(error),
            }
        }
        "export" => {
            let case = load_case(
                read_string_option(&args, "--case").ok_or("--case is required")?,
            )?;
            let admitted = case_and_definition(&catalog, &case)?;
            match export_workflow_obligation(
                admitted,
                &catalog.policy,
                &case.selected_alternatives,
                &case.subject_blake3,
                &case.context_blake3,
            ) {
                Ok(export) => print_json(json!({
                    "schema_version": EXPORT_SCHEMA,
                    "outcome": "exported",
                    "advisory": true,
                    "workflow_policy_blake3": policy_hash,
                    "catalog_identity": catalog.catalog_identity,
                    "export": export,
                    "error": null,
                    "boundary": WORKFLOW_OBLIGATION_BOUNDARY,
                })),
                Err(error) => rejected(error),
            }
        }
        _ => Err(usage()),
    }
}

fn usage() -> Box<dyn Error> {
    "usage: cairn obligation <list|explain|export> [--case FILE] [--policy FILE] [--obligations FILE]".into()
}
