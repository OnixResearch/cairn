use crate::{
    requirement_id_registry::read_accepted_specifications,
    support::{
        default_policy_path, print_json, read_option, read_string_option,
        reject_positional_arguments, reject_unknown_options, CapabilityRoot, CapabilityRootKind,
    },
};
use cairn_core::{
    admit_correspondence_review, evaluate_example, normalize_contract,
    parse_content_bound_requirement_registry_value, parse_typed_requirement_inventory_value,
    verify_content_bound_requirement_registry, CorrespondenceReview, SemanticExample,
    TypedRequirementInventory, TYPED_REQUIREMENT_BOUNDARY,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::{error::Error, path::Path};

const MAX_INPUT_BYTES: u64 = 4 * 1024 * 1024;
const DEFAULT_INVENTORY: &str =
    include_str!("../../../cairn-policy/generated/typed-requirement-contracts.json");

fn load<T: DeserializeOwned>(path: &str) -> Result<T, Box<dyn Error>> {
    let (root, relative) = CapabilityRoot::open_file_parent(
        CapabilityRootKind::EvidenceBundle,
        Path::new(path),
        false,
    )?;
    Ok(serde_json::from_slice(
        &root.read_bounded(&relative, MAX_INPUT_BYTES)?,
    )?)
}

/// Read-only meaning inspection. No lifecycle mutation, external execution, or evidence admission.
pub(crate) fn run(mut args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.typed_requirements.inspection]
    if args.first().map(String::as_str) != Some("inspect") {
        return Err("usage: cairn requirement-contract inspect --requirement ID --registry FILE [--root DIR] [--policy FILE] [--inventory FILE] [--review FILE] [--example FILE]".into());
    }
    args.remove(0);
    reject_unknown_options(
        &args,
        &[
            "--requirement",
            "--registry",
            "--root",
            "--policy",
            "--inventory",
            "--review",
            "--example",
        ],
    )?;
    reject_positional_arguments(&args)?;
    let requirement =
        read_string_option(&args, "--requirement").ok_or("--requirement is required")?;
    let registry_path = read_string_option(&args, "--registry").ok_or("--registry is required")?;
    let root_path = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let registry_value = load(&registry_path)?;
    let registry = parse_content_bound_requirement_registry_value(&registry_value)
        .map_err(|issue| issue.message)?;
    let repository = CapabilityRoot::open(CapabilityRootKind::Repository, &root_path)?;
    let (policy_root, relative) = CapabilityRoot::open_file_parent(
        CapabilityRootKind::EvidenceBundle,
        Path::new(&policy_path),
        false,
    )?;
    let policy_bytes = policy_root.read_bounded(&relative, 16 * 1024 * 1024)?;
    let registry_report = verify_content_bound_requirement_registry(
        &registry,
        read_accepted_specifications(&repository)?,
        &policy_bytes,
        &registry.repository_id,
    );
    if !registry_report.valid {
        print_json(
            json!({"schema_version": "cairn.requirement-contract.inspection.v1", "well_formedness": "not_checked", "registry": "invalid_or_stale", "issues": cairn_core::content_bound_requirement_registry_issues_json(&registry_report.issues), "boundary": TYPED_REQUIREMENT_BOUNDARY}),
        )?;
        return Err("requirement registry admission failed".into());
    }
    let row = registry
        .rows
        .iter()
        .find(|row| row.requirement_id == requirement)
        .ok_or("requirement is absent from native registry")?;
    let inventory_value = match read_string_option(&args, "--inventory") {
        Some(path) => load(&path)?,
        None => serde_json::from_str(DEFAULT_INVENTORY)?,
    };
    let inventory: TypedRequirementInventory = match parse_typed_requirement_inventory_value(
        &inventory_value,
    ) {
        Ok(inventory) => inventory,
        Err(error) => {
            print_json(
                json!({"schema_version": "cairn.requirement-contract.inspection.v1", "well_formedness": "rejected", "error": error, "admitted_proof_evidence": "not_supplied", "boundary": TYPED_REQUIREMENT_BOUNDARY}),
            )?;
            return Err("typed requirement inventory rejected".into());
        }
    };
    let selected: Vec<_> = inventory
        .contracts
        .iter()
        .filter(|contract| contract.requirement.requirement_id == requirement)
        .collect();
    if selected.len() > 1 {
        return Err("duplicate contract for native requirement".into());
    }
    let Some(contract) = selected.first() else {
        if read_string_option(&args, "--example").is_some()
            || read_string_option(&args, "--review").is_some()
        {
            return Err(
                "cannot evaluate or review an informal requirement without a contract".into(),
            );
        }
        print_json(
            json!({"schema_version": "cairn.requirement-contract.inspection.v1", "requirement": row, "meaning": "informal", "well_formedness": "not_applicable", "correspondence": "not_reviewed", "example_evaluation": null, "admitted_proof_evidence": "not_supplied", "boundary": TYPED_REQUIREMENT_BOUNDARY}),
        )?;
        return Ok(());
    };
    let normalized = match normalize_contract(&inventory.profile, row, contract) {
        Ok(normalized) => normalized,
        Err(error) => {
            print_json(
                json!({"schema_version": "cairn.requirement-contract.inspection.v1", "requirement": row, "well_formedness": "rejected", "error": error, "correspondence": "not_admitted", "example_evaluation": null, "admitted_proof_evidence": "not_supplied", "boundary": TYPED_REQUIREMENT_BOUNDARY}),
            )?;
            return Err("typed requirement contract rejected".into());
        }
    };
    let mut reviews: Vec<CorrespondenceReview> = inventory
        .reviews
        .into_iter()
        .filter(|review| review.requirement_row_blake3 == row.row_blake3)
        .collect();
    if let Some(path) = read_string_option(&args, "--review") {
        reviews.push(load(&path)?);
    }
    let mut review_results = Vec::new();
    let mut reviewed = false;
    let mut rejected = false;
    for review in reviews {
        match admit_correspondence_review(&normalized, &review, &inventory.authorized_reviewers) {
            Ok(identity) => {
                reviewed = true;
                review_results.push(json!({"status": "admitted_review_declaration", "review_blake3": identity, "record": review}));
            }
            Err(error) => {
                rejected = true;
                review_results
                    .push(json!({"status": "rejected", "error": error, "record": review}));
            }
        }
    }
    let evaluation = match read_string_option(&args, "--example") {
        Some(path) => {
            let example: SemanticExample = load(&path)?;
            match evaluate_example(&inventory.profile, &normalized, &example) {
                Ok(evaluation) => json!({"status": "evaluated_example_only", "result": evaluation}),
                Err(error) => {
                    rejected = true;
                    json!({"status": "rejected", "error": error})
                }
            }
        }
        None => json!(null),
    };
    print_json(json!({
        "schema_version": "cairn.requirement-contract.inspection.v1",
        "requirement": row, "meaning": "finite_typed_contract", "well_formedness": "well_formed",
        "normalized": normalized,
        "correspondence": if reviewed { "reviewed_declaration" } else { "not_reviewed" },
        "review_authenticity": "external_retained_artifact_and_reviewer_policy_trust",
        "reviews": review_results,
        "proposed_correspondence": inventory.proposed_correspondence.iter().filter(|proposal| proposal.requirement_id == requirement).collect::<Vec<_>>(),
        "example_evaluation": evaluation, "admitted_proof_evidence": "not_supplied",
        "boundary": TYPED_REQUIREMENT_BOUNDARY,
    }))?;
    if rejected {
        return Err("review or example admission rejected".into());
    }
    Ok(())
}
