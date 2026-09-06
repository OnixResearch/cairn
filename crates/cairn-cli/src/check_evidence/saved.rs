//! Saved producer admission calls the producer-owned contract, never a guessed decoder.
use super::{named_options, read_input};
use crate::support::{print_json, CapabilityRoot, CapabilityRootKind};
use mantle_build_contract::{validate_observation, BuildObservation, BuildRequest, Identity};
use serde_json::json;
use std::{error::Error, path::Path};

pub(super) fn import(args: &[String]) -> Result<(), Box<dyn Error>> {
    let Some((producer, pairs)) = args.split_first() else {
        return Err("import requires an installed producer adapter".into());
    };
    if producer == "octet" {
        return import_octet(pairs);
    }
    if producer == "valence" {
        return import_valence(pairs);
    }
    if producer == "lattice" {
        return import_lattice(pairs);
    }
    if producer == "kiln" {
        return import_kiln(pairs);
    }
    if producer != "mantle" {
        return Err(
            "requested native producer admission adapter is unavailable; no results admitted"
                .into(),
        );
    }
    let options = named_options(
        pairs,
        &["--root", "--request", "--builder", "--observation"],
    )?;
    let root = CapabilityRoot::open(
        CapabilityRootKind::EvidenceBundle,
        Path::new(options.get("--root").copied().unwrap_or(".")),
    )?;
    let request: BuildRequest =
        read_input(&root, options.get("--request").ok_or("missing --request")?)?;
    let builder: Identity =
        read_input(&root, options.get("--builder").ok_or("missing --builder")?)?;
    let observation: BuildObservation = read_input(
        &root,
        options
            .get("--observation")
            .ok_or("missing --observation")?,
    )?;
    validate_observation(&request, &builder, &observation)
        .map_err(|_| "native Mantle contract rejected the observation")?;
    print_json(
        json!({"schema":"cairn.saved-check-admission.v1", "producer":"mantle-build-contract",
        "producer_revision":"0bbab77e23ab95caa4e4184b9945cfaa5c8040e2", "native_contract_valid":true,
        "native_schema":observation.schema, "native_identity":observation.identity,
        "native_status":observation.outcome, "native_cache":observation.cache,
        "native_non_claims":observation.non_claims, "scope":"build_observation",
        "checks_executed":false, "completion_results":[], "completion_eligible":false,
        "blockers":["native build observations contain no executed-test coverage", "Cairn check applicability and command binding are not supplied by this native contract"],
        "non_claims":["saved identity admission does not grant execution authority", "a build observation does not satisfy a test or strict-lint obligation"]}),
    )
}

fn import_kiln(pairs: &[String]) -> Result<(), Box<dyn Error>> {
    let options = named_options(pairs, &["--root", "--plan", "--observation"])?;
    let root = CapabilityRoot::open(
        CapabilityRootKind::EvidenceBundle,
        Path::new(options.get("--root").copied().unwrap_or(".")),
    )?;
    let plan: serde_json::Value =
        read_input(&root, options.get("--plan").ok_or("missing --plan")?)?;
    let observation: serde_json::Value = read_input(
        &root,
        options
            .get("--observation")
            .ok_or("missing --observation")?,
    )?;
    let admission = cairn_core::verified::check_evidence::admit_kiln_json(&plan, &observation)?;
    print_json(
        json!({"schema":"cairn.saved-check-admission.v1", "producer":"kiln-workflow-observation",
        "native_contract_valid":true,"admission":admission,"checks_executed":false,
        "completion_results":[],"completion_eligible":false,
        "blockers":["native CI workflow observations supply no executed-test coverage or current Cairn command binding"]}),
    )
}

fn import_lattice(pairs: &[String]) -> Result<(), Box<dyn Error>> {
    let options = named_options(
        pairs,
        &[
            "--root",
            "--profile",
            "--request",
            "--accepted",
            "--observation",
        ],
    )?;
    let root = CapabilityRoot::open(
        CapabilityRootKind::EvidenceBundle,
        Path::new(options.get("--root").copied().unwrap_or(".")),
    )?;
    let mut bundle = serde_json::Map::new();
    for (option, field) in [
        ("--profile", "profile"),
        ("--request", "request"),
        ("--accepted", "accepted"),
        ("--observation", "observation"),
    ] {
        let value: serde_json::Value =
            read_input(&root, options.get(option).ok_or("missing native input")?)?;
        bundle.insert(field.into(), value);
    }
    let admission = cairn_core::verified::check_evidence::admit_lattice_json(
        &serde_json::Value::Object(bundle),
    )?;
    print_json(
        json!({"schema":"cairn.saved-check-admission.v1", "producer":"lattice-workflow-contract",
        "native_contract_valid":true, "admission":admission, "checks_executed":false,
        "completion_results":[], "completion_eligible":false,
        "blockers":["workflow states and receipt identities supply no executed-test coverage or current Cairn applicability"]}),
    )
}

fn import_valence(pairs: &[String]) -> Result<(), Box<dyn Error>> {
    let options = named_options(pairs, &["--root", "--observation"])?;
    let root = CapabilityRoot::open(
        CapabilityRootKind::EvidenceBundle,
        Path::new(options.get("--root").copied().unwrap_or(".")),
    )?;
    let value: serde_json::Value = read_input(
        &root,
        options
            .get("--observation")
            .ok_or("missing --observation")?,
    )?;
    let admission = cairn_core::verified::check_evidence::admit_valence_json(&value)?;
    print_json(json!({
        "schema":"cairn.saved-check-admission.v1", "producer":"valence-stack-provenance-sidecar",
        "native_contract_valid":true, "admission":admission, "checks_executed":false,
        "completion_results":[], "completion_eligible":false,
        "blockers":["sidecar metadata supplies no execution status or executed-test coverage",
            "measured bytes, execution truth and current Cairn applicability are not verified"]
    }))
}

fn import_octet(pairs: &[String]) -> Result<(), Box<dyn Error>> {
    let options = named_options(pairs, &["--root", "--observation"])?;
    let root = CapabilityRoot::open(
        CapabilityRootKind::EvidenceBundle,
        Path::new(options.get("--root").copied().unwrap_or(".")),
    )?;
    let value: serde_json::Value = read_input(
        &root,
        options
            .get("--observation")
            .ok_or("missing --observation")?,
    )?;
    let admission = cairn_core::verified::check_evidence::admit_octet_json(&value)?;
    print_json(
        json!({"schema":"cairn.saved-check-admission.v1", "producer":"octet-acceptance-receipt",
        "native_contract_valid":true, "admission":admission, "checks_executed":false,
        "completion_results":[], "completion_eligible":false,
        "blockers":["published acceptance JSON lacks an exact full-catalog deny-all obligation binding"]}),
    )
}
