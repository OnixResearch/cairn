//! Read-only advisory convergence inspection. No lifecycle mutation.
//!
//! The shell reads every named input bounded and without following links,
//! checks the policy-admitted schema identifier for each JSON input's role and
//! parses the facts. It never decides a state: `cairn_core::convergence`
//! receives the facts and decides everything.
use std::{
    collections::BTreeMap,
    error::Error,
    path::{Component, Path, PathBuf},
};

use cairn_core::assurance_targets::ISSUE_UNKNOWN_SCHEMA as ADMISSION_UNKNOWN_SCHEMA;
use cairn_core::{
    admit_generated_sources, admit_targets, blake3_hex, canonical_json,
    parse_assurance_targets_policy, parse_content_bound_requirement_registry_value,
    verify_content_bound_requirement_registry, AssuranceTargetIssue, AssuranceTargetsPolicy,
    BytesFacts, ContractCaseFacts, ConvergenceRequest, FileFacts, GeneratedSources, HandoffFacts,
    ProducerOutputFacts, TargetsFile, CONVERGENCE_RECEIPT_SCHEMA_V1,
};
use serde::Deserialize;
use serde_json::{json, Value};

use crate::requirement_id_registry::read_accepted_specifications;
use crate::support::{
    read_option, read_string_option, reject_positional_arguments, reject_unknown_options,
    CapabilityRoot, CapabilityRootKind,
};

/// The checked policy export embedded so the command needs no repository state.
const DEFAULT_POLICY: &str = include_str!("../../../cairn-policy/generated/assurance-targets.json");
const DEFAULT_REGISTRY_POLICY: &str = "cairn-policy/generated/cairn-policy.json";
const MAX_POLICY_BYTES: u64 = 16 * 1024 * 1024;

/// Rejection codes the shell owns. Every other rejection carries the admission
/// or registry code of the input that failed.
const ISSUE_MALFORMED_JSON: &str = "convergence.malformed_json";
const ISSUE_UNREADABLE_INPUT: &str = "convergence.unreadable_input";
const ISSUE_UNKNOWN_SCHEMA: &str = "convergence.unknown_schema";
const ISSUE_PROOF_DECK_OUTSIDE_OUTPUT: &str = "convergence.proof_deck_outside_output";

/// Registry verification issues plus the plain BLAKE3 of each accepted
/// specification, keyed by its repo-relative path.
type RegistryVerification = (Vec<String>, BTreeMap<String, String>);

/// The typed, closed convergence case file.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ConvergenceCase {
    schema_version: String,
    registry: String,
    #[serde(default)]
    registry_policy: Option<String>,
    targets: String,
    generated_sources: String,
    trusted_signers: Vec<String>,
    contracts: Vec<ContractCase>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct ContractCase {
    contract_id: String,
    contract: String,
    decision: String,
    evidence: String,
    handoff: String,
    producer_output: String,
    /// Absolute path the evidence recorded for the realized output, when the
    /// retained copy the case names lives elsewhere.
    #[serde(default)]
    producer_output_locator: Option<String>,
    #[serde(default)]
    realization_receipt: Option<String>,
    #[serde(default)]
    store_info: Option<String>,
}

fn issue(code: &str, field_path: impl ToString, message: impl ToString) -> Value {
    json!({
        "code": code,
        "field_path": field_path.to_string(),
        "message": message.to_string(),
    })
}

fn unknown_schema(path: &Path, field: &str, expected: &str) -> Value {
    issue(
        ISSUE_UNKNOWN_SCHEMA,
        path.display(),
        format!("{field} must equal the policy-admitted identifier {expected}"),
    )
}

fn admission_issues(path: &Path, issues: Vec<AssuranceTargetIssue>) -> Vec<Value> {
    issues
        .into_iter()
        .map(|issue_value| {
            if issue_value.code == ADMISSION_UNKNOWN_SCHEMA {
                issue(
                    ISSUE_UNKNOWN_SCHEMA,
                    path.display(),
                    format!("{}: {}", issue_value.field_path, issue_value.message),
                )
            } else {
                issue(
                    &issue_value.code,
                    &issue_value.field_path,
                    format!("{}: {}", path.display(), issue_value.message),
                )
            }
        })
        .collect()
}

/// Resolve a case-relative path against the case file's directory. Absolute
/// paths are used as given.
fn resolve(base: &Path, path: &str) -> PathBuf {
    let candidate = Path::new(path);
    if candidate.is_absolute() {
        candidate.to_path_buf()
    } else {
        base.join(candidate)
    }
}

/// Read one bounded, link-refusing file. The capability helper opens a
/// directory root without following the final component.
fn read_file(path: &Path, maximum: u64) -> Result<Vec<u8>, Vec<Value>> {
    let (root, relative) =
        CapabilityRoot::open_file_parent(CapabilityRootKind::EvidenceBundle, path, false)
            .map_err(|error| vec![issue(ISSUE_UNREADABLE_INPUT, path.display(), error)])?;
    root.read_bounded(&relative, maximum)
        .map_err(|error| vec![issue(ISSUE_UNREADABLE_INPUT, path.display(), error)])
}

fn parse_json(bytes: &[u8], path: &Path) -> Result<Value, Vec<Value>> {
    serde_json::from_slice(bytes)
        .map_err(|error| vec![issue(ISSUE_MALFORMED_JSON, path.display(), error)])
}

fn read_json(path: &Path, maximum: u64) -> Result<FileFacts, Vec<Value>> {
    let bytes = read_file(path, maximum)?;
    let value = parse_json(&bytes, path)?;
    Ok(FileFacts {
        blake3: blake3_hex(&bytes),
        value,
    })
}

fn bytes_facts(bytes: Vec<u8>) -> BytesFacts {
    let blake3 = blake3_hex(&bytes);
    let text = String::from_utf8(bytes).ok();
    BytesFacts { blake3, text }
}

fn require_schema(
    facts: &FileFacts,
    field: &str,
    expected: &str,
    path: &Path,
) -> Result<(), Vec<Value>> {
    if facts.value.get(field).and_then(Value::as_str) == Some(expected) {
        Ok(())
    } else {
        Err(vec![unknown_schema(path, field, expected)])
    }
}

fn read_in_dir(
    root: &CapabilityRoot,
    directory: &Path,
    name: &str,
    maximum: u64,
) -> Result<Vec<u8>, Vec<Value>> {
    root.read_bounded(Path::new(name), maximum)
        .map_err(|error| {
            vec![issue(
                ISSUE_UNREADABLE_INPUT,
                directory.join(name).display(),
                error,
            )]
        })
}

fn read_json_in_dir(
    root: &CapabilityRoot,
    directory: &Path,
    name: &str,
    maximum: u64,
) -> Result<FileFacts, Vec<Value>> {
    let bytes = read_in_dir(root, directory, name, maximum)?;
    let path = directory.join(name);
    let value = parse_json(&bytes, &path)?;
    Ok(FileFacts {
        blake3: blake3_hex(&bytes),
        value,
    })
}

fn open_directory(path: &Path) -> Result<CapabilityRoot, Vec<Value>> {
    CapabilityRoot::open(CapabilityRootKind::EvidenceBundle, path)
        .map_err(|error| vec![issue(ISSUE_UNREADABLE_INPUT, path.display(), error)])
}

/// Resolve a proof-deck locator to the relative path read inside the producer
/// output directory.
///
/// Octet names the deck it judged by the locator it resolved against its own
/// source root, which may be absolute. A relative locator is used as given; an
/// absolute locator must lexically start with `absolute_base` followed by `/`
/// (the case's resolved `producer_output`, or its `producer_output_locator`
/// when the retained copy lives elsewhere). Nothing is canonicalized, so a link
/// cannot widen the directory, and no `..` component may step outside it.
fn deck_relative_path<'a>(absolute_base: &str, locator: &'a str) -> Option<&'a str> {
    if locator.is_empty() {
        return None;
    }
    let relative = if Path::new(locator).is_absolute() {
        let prefix = format!("{}/", absolute_base.trim_end_matches('/'));
        locator.strip_prefix(&prefix)?
    } else {
        locator
    };
    let deck = Path::new(relative);
    if relative.is_empty() || deck.is_absolute() {
        return None;
    }
    deck.components()
        .all(|component| matches!(component, Component::Normal(_)))
        .then_some(relative)
}

fn handoff_facts(
    directory: &Path,
    policy: &AssuranceTargetsPolicy,
) -> Result<HandoffFacts, Vec<Value>> {
    let maximum = policy.bounds.max_document_bytes as u64;
    let root = open_directory(directory)?;
    let observation = read_json_in_dir(&root, directory, "mantle-observation.json", maximum)?;
    require_schema(
        &observation,
        "schema",
        &policy.schemas.mantle_observation,
        &directory.join("mantle-observation.json"),
    )?;
    let job = read_json_in_dir(&root, directory, "job.json", maximum)?;
    require_schema(
        &job,
        "schema_version",
        &policy.schemas.octet_producer_job,
        &directory.join("job.json"),
    )?;
    let receipt = bytes_facts(read_in_dir(
        &root,
        directory,
        "external-receipt.json",
        maximum,
    )?);
    let profile = bytes_facts(read_in_dir(
        &root,
        directory,
        "external-profile.json",
        maximum,
    )?);
    let tool_graph = bytes_facts(read_in_dir(
        &root,
        directory,
        "tool-closure.derivation-json.json",
        policy.bounds.max_tool_graph_bytes as u64,
    )?);
    let root_derivation = bytes_facts(read_in_dir(&root, directory, "root-derivation", maximum)?);
    let source_revision = bytes_facts(read_in_dir(&root, directory, "source-revision", maximum)?);
    Ok(HandoffFacts {
        observation,
        job,
        receipt,
        profile,
        tool_graph,
        root_derivation,
        source_revision,
    })
}

fn producer_output_facts(
    directory: &Path,
    locator: Option<&str>,
    evidence: &Value,
    policy: &AssuranceTargetsPolicy,
) -> Result<ProducerOutputFacts, Vec<Value>> {
    let maximum = policy.bounds.max_document_bytes as u64;
    let dir_name = directory
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| {
            vec![issue(
                ISSUE_MALFORMED_JSON,
                directory.display(),
                "producer_output must have a final path component",
            )]
        })?
        .to_string();
    // Absolute deck locators are matched against the path the evidence
    // recorded; the bytes read still come from the case's retained directory.
    let absolute_base = match locator {
        Some(locator) => {
            let locator_name = Path::new(locator)
                .file_name()
                .and_then(|name| name.to_str());
            if locator_name != Some(dir_name.as_str()) {
                return Err(vec![issue(
                    ISSUE_PROOF_DECK_OUTSIDE_OUTPUT,
                    directory.display(),
                    format!(
                        "producer_output_locator must end with the producer output directory name: {locator}"
                    ),
                )]);
            }
            locator.to_string()
        }
        None => directory.to_string_lossy().into_owned(),
    };
    let root = open_directory(directory)?;
    let receipt = bytes_facts(read_in_dir(
        &root,
        directory,
        "external-receipt.json",
        maximum,
    )?);
    let profile = bytes_facts(read_in_dir(
        &root,
        directory,
        "external-profile.json",
        maximum,
    )?);
    let mut decks = BTreeMap::new();
    if let Some(proof_decks) = evidence.get("proof_decks").and_then(Value::as_array) {
        for deck in proof_decks {
            let Some(locator) = deck.get("deck").and_then(Value::as_str) else {
                continue;
            };
            let Some(relative) = deck_relative_path(&absolute_base, locator) else {
                return Err(vec![issue(
                    ISSUE_PROOF_DECK_OUTSIDE_OUTPUT,
                    directory.display(),
                    format!(
                        "proof deck locator is outside the producer output directory: {locator}"
                    ),
                )]);
            };
            let path = directory.join(relative);
            match root.read_bounded(Path::new(relative), maximum) {
                Ok(bytes) => {
                    let value = parse_json(&bytes, &path)?;
                    let facts = FileFacts {
                        blake3: blake3_hex(&bytes),
                        value,
                    };
                    require_schema(
                        &facts,
                        "schema_version",
                        &policy.schemas.octet_proof_cards,
                        &path,
                    )?;
                    // Keyed by the locator the evidence bundle named, so the
                    // core finds the deck whether it is relative or absolute.
                    decks.insert(locator.to_string(), facts);
                }
                // A missing deck is a decision for the core (`incomplete`);
                // any other read failure is an unreadable input.
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => {
                    return Err(vec![issue(ISSUE_UNREADABLE_INPUT, path.display(), error)])
                }
            }
        }
    }
    Ok(ProducerOutputFacts {
        dir_name,
        receipt,
        profile,
        decks,
    })
}

fn admission_of_targets(
    value: &Value,
    policy: &AssuranceTargetsPolicy,
    path: &Path,
) -> Result<TargetsFile, Vec<Value>> {
    admit_targets(value, policy).map_err(|issues| admission_issues(path, issues))
}

fn admission_of_generated(
    value: &Value,
    targets: &TargetsFile,
    policy: &AssuranceTargetsPolicy,
    path: &Path,
) -> Result<GeneratedSources, Vec<Value>> {
    admit_generated_sources(value, targets, policy).map_err(|issues| admission_issues(path, issues))
}

fn contract_facts(
    case: &ContractCase,
    base: &Path,
    policy: &AssuranceTargetsPolicy,
) -> Result<ContractCaseFacts, Vec<Value>> {
    let maximum = policy.bounds.max_document_bytes as u64;

    let contract_path = resolve(base, &case.contract);
    let contract = read_json(&contract_path, maximum)?;
    require_schema(
        &contract,
        "schema_version",
        &policy.schemas.octet_contract,
        &contract_path,
    )?;

    let decision_path = resolve(base, &case.decision);
    let decision = read_json(&decision_path, maximum)?;
    require_schema(
        &decision,
        "schema_version",
        &policy.schemas.octet_decision,
        &decision_path,
    )?;

    let evidence_path = resolve(base, &case.evidence);
    let evidence = read_json(&evidence_path, maximum)?;
    require_schema(
        &evidence,
        "schema_version",
        &policy.schemas.octet_evidence,
        &evidence_path,
    )?;

    let handoff = handoff_facts(&resolve(base, &case.handoff), policy)?;
    let producer_output = producer_output_facts(
        &resolve(base, &case.producer_output),
        case.producer_output_locator.as_deref(),
        &evidence.value,
        policy,
    )?;

    let realization = match &case.realization_receipt {
        Some(path) => {
            let path = resolve(base, path);
            let facts = read_json(&path, policy.bounds.max_realization_receipt_bytes as u64)?;
            require_schema(
                &facts,
                "schema",
                &policy.schemas.mantle_realization_receipt,
                &path,
            )?;
            Some(facts)
        }
        None => None,
    };
    let store_info = match &case.store_info {
        Some(path) => {
            let path = resolve(base, path);
            let facts = read_json(&path, maximum)?;
            require_schema(&facts, "schema", &policy.schemas.mantle_store_info, &path)?;
            Some(facts)
        }
        None => None,
    };

    Ok(ContractCaseFacts {
        contract_id: case.contract_id.clone(),
        producer_output_locator: case.producer_output_locator.clone(),
        contract,
        decision,
        evidence,
        handoff,
        producer_output,
        realization,
        store_info,
    })
}

fn registry_verification_issues(
    root_path: &Path,
    case: &ConvergenceCase,
    registry: &cairn_core::ContentBoundRequirementRegistry,
) -> Result<RegistryVerification, Vec<Value>> {
    let repository = CapabilityRoot::open(CapabilityRootKind::Repository, root_path)
        .map_err(|error| vec![issue(ISSUE_UNREADABLE_INPUT, root_path.display(), error)])?;
    let specifications = read_accepted_specifications(&repository)
        .map_err(|error| vec![issue(ISSUE_UNREADABLE_INPUT, root_path.display(), error)])?;
    // Octet verifies the plain BLAKE3 of the reviewed specification bytes, so
    // the projection must carry this digest, not the registry's
    // domain-separated identity. `read_accepted_specifications` returns the
    // exact UTF-8 file bytes under the repo-relative accepted path.
    let root_specification_digests = specifications
        .iter()
        .map(|specification| {
            (
                specification.path.clone(),
                blake3_hex(specification.markdown.as_bytes()),
            )
        })
        .collect();
    let policy_path = case
        .registry_policy
        .as_deref()
        .unwrap_or(DEFAULT_REGISTRY_POLICY);
    let policy_bytes = repository
        .read_bounded(Path::new(policy_path), MAX_POLICY_BYTES)
        .map_err(|error| {
            vec![issue(
                ISSUE_UNREADABLE_INPUT,
                root_path.join(policy_path).display(),
                error,
            )]
        })?;
    let report = verify_content_bound_requirement_registry(
        registry,
        specifications,
        &policy_bytes,
        &registry.repository_id,
    );
    let mut codes = report
        .issues
        .iter()
        .map(|issue_value| issue_value.code.clone())
        .collect::<Vec<_>>();
    codes.sort();
    codes.dedup();
    Ok((codes, root_specification_digests))
}

/// Build the pure request from already-read facts; every decision stays in the
/// core evaluator.
fn inspect(case_path: &Path, root_path: &Path, policy_bytes: &[u8]) -> Result<Value, Vec<Value>> {
    let base = case_path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
        .unwrap_or_else(|| Path::new("."));

    let policy_value = parse_json(policy_bytes, Path::new("<policy>"))?;
    let policy = parse_assurance_targets_policy(&policy_value)
        .map_err(|issues| admission_issues(Path::new("<policy>"), issues))?;
    let policy_blake3 = blake3_hex(policy_bytes);
    let document_maximum = policy.bounds.max_document_bytes as u64;
    let input_maximum = policy.bounds.max_input_bytes as u64;

    let case_bytes = read_file(case_path, document_maximum)?;
    let case: ConvergenceCase = serde_json::from_slice(&case_bytes)
        .map_err(|error| vec![issue(ISSUE_MALFORMED_JSON, case_path.display(), error)])?;
    if case.schema_version != policy.case_schema {
        return Err(vec![unknown_schema(
            case_path,
            "schema_version",
            &policy.case_schema,
        )]);
    }
    let case_blake3 = blake3_hex(&case_bytes);

    let registry_path = resolve(base, &case.registry);
    let registry_bytes = read_file(&registry_path, input_maximum)?;
    let registry_value = parse_json(&registry_bytes, &registry_path)?;
    let registry =
        parse_content_bound_requirement_registry_value(&registry_value).map_err(|issue_value| {
            vec![issue(
                &issue_value.code,
                &issue_value.field_path,
                format!("{}: {}", registry_path.display(), issue_value.message),
            )]
        })?;
    if registry.schema_version != policy.schemas.registry {
        return Err(vec![unknown_schema(
            &registry_path,
            "schema_version",
            &policy.schemas.registry,
        )]);
    }
    let registry_blake3 = blake3_hex(&registry_bytes);

    let targets_path = resolve(base, &case.targets);
    let targets_bytes = read_file(&targets_path, document_maximum)?;
    let targets_value = parse_json(&targets_bytes, &targets_path)?;
    let targets = admission_of_targets(&targets_value, &policy, &targets_path)?;
    let targets_blake3 = blake3_hex(&targets_bytes);

    let generated_path = resolve(base, &case.generated_sources);
    let generated_bytes = read_file(&generated_path, document_maximum)?;
    let generated_value = parse_json(&generated_bytes, &generated_path)?;
    let generated = admission_of_generated(&generated_value, &targets, &policy, &generated_path)?;
    let generated_blake3 = blake3_hex(&generated_bytes);

    let mut contracts = Vec::with_capacity(case.contracts.len());
    for entry in &case.contracts {
        contracts.push(contract_facts(entry, base, &policy)?);
    }

    let (registry_verification, root_specification_digests) =
        registry_verification_issues(root_path, &case, &registry)?;

    let request = ConvergenceRequest {
        policy,
        policy_blake3,
        case_blake3,
        registry,
        registry_blake3,
        registry_verification_issues: registry_verification,
        targets,
        targets_blake3,
        generated,
        generated_blake3,
        root_specification_digests,
        trusted_signers: case.trusted_signers.clone(),
        contracts,
    };
    Ok(cairn_core::convergence::evaluate_convergence(&request))
}

/// Read-only advisory inspection of one convergence case.
///
/// It reads bounded, link-refusing inputs, prints one canonical receipt on
/// standard output and never writes a file, spawns a process or evaluates
/// Nickel.
// r[impl cairn.convergence.inputs]
// r[impl cairn.convergence.inspection]
// r[impl cairn.convergence.authority]
pub(crate) fn run(mut args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.first().map(String::as_str) != Some("inspect") {
        return Err(
            "usage: cairn convergence inspect --case FILE [--policy FILE] [--root DIR]".into(),
        );
    }
    args.remove(0);
    reject_unknown_options(&args, &["--case", "--policy", "--root"])?;
    reject_positional_arguments(&args)?;
    let case_path = read_string_option(&args, "--case").ok_or("--case is required")?;
    let root_path = read_option(args.clone(), "--root", ".");
    let policy_bytes = match read_string_option(&args, "--policy") {
        Some(path) => read_file(Path::new(&path), MAX_POLICY_BYTES).map_err(|issues| {
            let message = issues
                .first()
                .and_then(|issue| issue.get("message"))
                .and_then(Value::as_str)
                .unwrap_or("policy is unreadable");
            format!("convergence policy unreadable: {message}")
        })?,
        None => DEFAULT_POLICY.as_bytes().to_vec(),
    };

    match inspect(Path::new(&case_path), &root_path, &policy_bytes) {
        Ok(receipt) => {
            println!("{}", canonical_json(&receipt));
            Ok(())
        }
        Err(issues) => {
            let rejection = json!({
                "schema_version": CONVERGENCE_RECEIPT_SCHEMA_V1,
                "status": "rejected",
                "issues": issues,
            });
            println!("{}", canonical_json(&rejection));
            std::process::exit(1);
        }
    }
}
