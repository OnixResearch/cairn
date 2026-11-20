//! SYNTHETIC convergence fixtures: every input below is built at test time in
//! a fresh temporary root. Nothing here is an exercised Mantle or Octet run.
// r[verify cairn.convergence.inputs]
// r[verify cairn.convergence.inspection]
// r[verify cairn.convergence.authority]
// r[verify cairn.convergence.determinism]
// r[verify cairn.convergence.completeness]
// r[verify cairn.convergence.realization]
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Output},
    sync::atomic::{AtomicUsize, Ordering},
};

use cairn_core::{
    admit_generated_sources, admit_targets, blake3_hex, parse_assurance_targets_policy,
    project_contract, ContentBoundRequirementRegistry, TargetsFile,
};
use serde_json::{json, Value};

const POLICY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../cairn-policy/generated/assurance-targets.json"
));
const REGISTRY_POLICY_JSON: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../cairn-policy/generated/cairn-policy.json"
));

const REVISION: &str = "0123456789abcdef0123456789abcdef01234567";
const REPOSITORY_ID: &str = "example/synthetic";
const CONTRACT_ID: &str = "example.synthetic-contract";
const JOB_ID: &str = "example.synthetic.job.v1";
const EVIDENCE_ID: &str = "example.synthetic.evidence";
const DECK_NAME: &str = "synthetic-deck.json";
const RECEIPT_BYTES: &[u8] = b"synthetic octet receipt bytes\n";
const PROFILE_BYTES: &[u8] = b"synthetic octet profile bytes\n";
const TOOL_BYTES: &[u8] = b"synthetic tool closure bytes\n";
const ROOT_DERIVATION: &str = "synthetic-root-derivation";
const TRANSLATED_ROOT: &str =
    "/nix/store/cccccccccccccccccccccccccccccccc-synthetic-translated.drv";
const ROOT_BASENAME: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa-synthetic-output";
const ROOT_PATH: &str = concat!(
    "/nix/store/",
    "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa-synthetic-output"
);
const NAR_SHA256: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
const SIGNER: &str = "synthetic-key";

static NEXT_DIRECTORY: AtomicUsize = AtomicUsize::new(0);

fn synthetic_specification() -> &'static str {
    r#"# Synthetic Convergence Specification

## Requirements

### Requirement: The first synthetic requirement

r[example.synthetic.first] Cairn MUST report the first synthetic requirement.

#### Scenario: The first synthetic scenario

- GIVEN a synthetic input
- WHEN the command runs
- THEN the first row MUST be reported.

### Requirement: The second synthetic requirement

r[example.synthetic.second] Cairn MUST report the second synthetic requirement.

#### Scenario: The second synthetic scenario

- GIVEN a synthetic input
- WHEN the command runs
- THEN the second row MUST be reported.
"#
}

struct Fixture {
    root: PathBuf,
    case_dir: PathBuf,
    registry: ContentBoundRequirementRegistry,
    targets: TargetsFile,
    contract_bytes: Vec<u8>,
    evidence_bytes: Vec<u8>,
}

impl Fixture {
    fn case_path(&self) -> PathBuf {
        self.case_dir.join("case.json")
    }

    fn inspect(&self) -> Output {
        Command::new(env!("CARGO_BIN_EXE_cairn"))
            .arg("convergence")
            .arg("inspect")
            .arg("--case")
            .arg(self.case_path())
            .arg("--root")
            .arg(&self.root)
            .current_dir(&self.root)
            .output()
            .expect("run cairn convergence inspect")
    }

    fn rewrite_case(&self, mutate: impl FnOnce(&mut Value)) {
        let path = self.case_path();
        let mut case: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        mutate(&mut case);
        fs::write(&path, serde_json::to_vec_pretty(&case).unwrap()).unwrap();
    }
}

fn fresh_root(label: &str) -> PathBuf {
    let counter = NEXT_DIRECTORY.fetch_add(1, Ordering::Relaxed);
    let root = env::temp_dir().join(format!(
        "cairn-convergence-{label}-{}-{counter}",
        std::process::id()
    ));
    if root.exists() {
        fs::remove_dir_all(&root).unwrap();
    }
    fs::create_dir_all(&root).unwrap();
    root
}

fn build_fixture(label: &str) -> Fixture {
    let root = fresh_root(label);
    let specification_dir = root.join(".cairn/specs/synthetic");
    fs::create_dir_all(&specification_dir).unwrap();
    fs::write(specification_dir.join("spec.md"), synthetic_specification()).unwrap();
    let policy_dir = root.join("cairn-policy/generated");
    fs::create_dir_all(&policy_dir).unwrap();
    fs::write(policy_dir.join("cairn-policy.json"), REGISTRY_POLICY_JSON).unwrap();

    let case_dir = root.join("case");
    fs::create_dir_all(&case_dir).unwrap();
    let registry_path = case_dir.join("registry.json");
    let export = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args([
            "requirement-registry",
            "export",
            "--root",
            root.to_str().unwrap(),
            "--repository-id",
            REPOSITORY_ID,
            "--revision-kind",
            "git",
            "--revision",
            REVISION,
            "--out",
            registry_path.to_str().unwrap(),
        ])
        .output()
        .expect("run cairn requirement-registry export");
    assert!(
        export.status.success(),
        "registry export failed: {}",
        String::from_utf8_lossy(&export.stdout)
    );
    let registry: ContentBoundRequirementRegistry =
        serde_json::from_slice(&fs::read(&registry_path).unwrap()).unwrap();
    assert_eq!(registry.rows.len(), 2);

    let policy_value: Value = serde_json::from_str(POLICY_JSON).unwrap();
    let policy = parse_assurance_targets_policy(&policy_value).unwrap();
    let row = &registry.rows[0];

    let targets_value = json!({
        "schema_version": "cairn.assurance-targets.v1",
        "contracts": [{
            "contract_id": CONTRACT_ID,
            "specification": {
                "path": row.specification_path,
                "blake3": row.specification_blake3,
            },
            "targets": [{
                "requirement": row,
                "assurance": "checked_proof",
                "claim": "synthetic claim",
                "informal_remainder": "synthetic remainder",
                "evidence_job": {"job_id": JOB_ID, "definition": "jobs/synthetic.nix"},
                "checked_scope": ["synthetic checked scope"],
                "evidence_ids": [EVIDENCE_ID],
                "evidence_classes": ["formal_proof"],
                "negative_controls": [],
            }],
            "assumptions": [],
            "exclusions": [],
            "non_claims": [],
        }],
    });
    let targets = admit_targets(&targets_value, &policy).unwrap();
    let generated_value = json!({
        "schema_version": "cairn.assurance-target-sources.v1",
        "specifications": [{
            "path": row.specification_path,
            "blake3": blake3_hex(synthetic_specification().as_bytes()),
        }],
        "entries": [{
            "requirement_id": row.requirement_id,
            "sources": [{
                "locator": "synthetic/source.nix",
                "blake3": row.requirement_body_blake3,
            }],
            "source_ids": [],
            "policy_ids": [],
            "tool_identity_hashes": [],
            "artifact_hashes": [],
        }],
    });
    let generated = admit_generated_sources(&generated_value, &targets, &policy).unwrap();
    let contract = project_contract(&targets.contracts[0], &generated, &policy).unwrap();
    let contract_bytes = serde_json::to_vec_pretty(&contract).unwrap();
    fs::write(case_dir.join("contract.json"), &contract_bytes).unwrap();
    fs::write(
        case_dir.join("targets.json"),
        serde_json::to_vec_pretty(&targets_value).unwrap(),
    )
    .unwrap();
    fs::write(
        case_dir.join("generated.json"),
        serde_json::to_vec_pretty(&generated_value).unwrap(),
    )
    .unwrap();

    let evidence_value = json!({
        "schema_version": "octet-requirement-evidence/v1",
        "contract_blake3": blake3_hex(&contract_bytes),
        "proof_decks": [{"evidence_id": EVIDENCE_ID, "deck": DECK_NAME}],
    });
    let evidence_bytes = serde_json::to_vec_pretty(&evidence_value).unwrap();
    fs::write(case_dir.join("evidence.json"), &evidence_bytes).unwrap();

    let decision_value = json!({
        "schema_version": "octet-requirement-decision/v1",
        "contract_blake3": blake3_hex(&contract_bytes),
        "evidence_blake3": blake3_hex(&evidence_bytes),
        "reviewed_specification": contract["reviewed_specification"],
        "aggregate_result": "passed",
        "clauses": [{"clause_id": row.requirement_id, "status": "passed"}],
    });
    fs::write(
        case_dir.join("decision.json"),
        serde_json::to_vec_pretty(&decision_value).unwrap(),
    )
    .unwrap();

    let job_bytes = serde_json::to_vec(&json!({
        "schema_version": "octet-mantle-producer-job/v1",
        "job_id": JOB_ID,
    }))
    .unwrap();
    let handoff = case_dir.join("handoff");
    fs::create_dir_all(&handoff).unwrap();
    fs::write(handoff.join("job.json"), &job_bytes).unwrap();
    fs::write(handoff.join("external-receipt.json"), RECEIPT_BYTES).unwrap();
    fs::write(handoff.join("external-profile.json"), PROFILE_BYTES).unwrap();
    fs::write(
        handoff.join("tool-closure.derivation-json.json"),
        TOOL_BYTES,
    )
    .unwrap();
    fs::write(handoff.join("root-derivation"), ROOT_DERIVATION).unwrap();
    fs::write(handoff.join("source-revision"), format!("{REVISION}\n")).unwrap();
    let observation = json!({
        "schema": "mantle-external-evidence-observation-v1",
        "proof_acceptance": "not-evaluated-by-mantle",
        "packaging": {"source_revision": REVISION},
        "producer_job_identity": format!("b3:{}", blake3_hex(&job_bytes)),
        "tool_graph_identity": format!("b3:{}", blake3_hex(TOOL_BYTES)),
        "receipt_identity": format!("b3:{}", blake3_hex(RECEIPT_BYTES)),
    });
    fs::write(
        handoff.join("mantle-observation.json"),
        serde_json::to_vec_pretty(&observation).unwrap(),
    )
    .unwrap();

    let producer = case_dir.join(ROOT_BASENAME);
    fs::create_dir_all(&producer).unwrap();
    fs::write(producer.join("external-receipt.json"), RECEIPT_BYTES).unwrap();
    fs::write(producer.join("external-profile.json"), PROFILE_BYTES).unwrap();
    let deck = json!({
        "schema_version": "octet-proof-cards/v1",
        "cards": [{
            "proof_or_property_id": EVIDENCE_ID,
            "receipt_path": "proofs/synthetic-receipt.txt",
            "artifact_blake3_identities": [{
                "artifact": "proofs/synthetic-receipt.txt",
                "blake3": blake3_hex(RECEIPT_BYTES),
            }],
        }],
    });
    fs::write(
        producer.join(DECK_NAME),
        serde_json::to_vec_pretty(&deck).unwrap(),
    )
    .unwrap();

    let realization = json!({
        "schema": "mantle-foreign-realization-receipt-v1",
        "status": "complete",
        "strongest_state": "realized",
        "selected_root_paths": [ROOT_PATH],
        "units": [{
            "node_id": format!("nix:{ROOT_DERIVATION}"),
            "target_derivation": TRANSLATED_ROOT,
            "outputs": [{
                "target_path": ROOT_PATH,
                "nar_sha256": NAR_SHA256,
                "nar_size": 1,
            }],
        }],
    });
    fs::write(
        case_dir.join("realization.json"),
        serde_json::to_vec_pretty(&realization).unwrap(),
    )
    .unwrap();
    let store_info = json!({
        "schema": "mantle-store-info-v3",
        "paths": [{
            "store_path": ROOT_PATH,
            "nar_sha256": NAR_SHA256,
            "signatures": ["synthetic-key:AAAA"],
        }],
    });
    fs::write(
        case_dir.join("store-info.json"),
        serde_json::to_vec_pretty(&store_info).unwrap(),
    )
    .unwrap();

    let case = json!({
        "schema_version": "cairn.convergence-case.v1",
        "registry": "registry.json",
        "registry_policy": "cairn-policy/generated/cairn-policy.json",
        "targets": "targets.json",
        "generated_sources": "generated.json",
        "trusted_signers": [SIGNER],
        "contracts": [{
            "contract_id": CONTRACT_ID,
            "contract": "contract.json",
            "decision": "decision.json",
            "evidence": "evidence.json",
            "handoff": "handoff",
            "producer_output": ROOT_BASENAME,
            "realization_receipt": "realization.json",
            "store_info": "store-info.json",
        }],
    });
    fs::write(
        case_dir.join("case.json"),
        serde_json::to_vec_pretty(&case).unwrap(),
    )
    .unwrap();

    Fixture {
        root,
        case_dir,
        registry,
        targets,
        contract_bytes,
        evidence_bytes,
    }
}

fn receipt_json(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "stdout is not JSON: {error}\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

/// Point the evidence bundle at a new deck locator and rebind the decision to
/// the rewritten evidence bytes.
fn rewrite_evidence_deck(fixture: &Fixture, locator: &str) {
    let evidence_path = fixture.case_dir.join("evidence.json");
    let mut evidence: Value = serde_json::from_slice(&fs::read(&evidence_path).unwrap()).unwrap();
    evidence["proof_decks"][0]["deck"] = json!(locator);
    let evidence_bytes = serde_json::to_vec_pretty(&evidence).unwrap();
    fs::write(&evidence_path, &evidence_bytes).unwrap();
    let decision_path = fixture.case_dir.join("decision.json");
    let mut decision: Value = serde_json::from_slice(&fs::read(&decision_path).unwrap()).unwrap();
    decision["evidence_blake3"] = json!(blake3_hex(&evidence_bytes));
    fs::write(
        &decision_path,
        serde_json::to_vec_pretty(&decision).unwrap(),
    )
    .unwrap();
}

fn hash_tree(root: &Path) -> BTreeMap<String, String> {
    fn walk(base: &Path, directory: &Path, entries: &mut BTreeMap<String, String>) {
        for entry in fs::read_dir(directory).unwrap() {
            let path = entry.unwrap().path();
            let metadata = fs::symlink_metadata(&path).unwrap();
            let relative = path
                .strip_prefix(base)
                .unwrap()
                .to_string_lossy()
                .into_owned();
            if metadata.file_type().is_symlink() {
                entries.insert(relative, "symlink".into());
            } else if metadata.is_dir() {
                walk(base, &path, entries);
            } else {
                entries.insert(relative, blake3_hex(&fs::read(&path).unwrap()));
            }
        }
    }
    let mut entries = BTreeMap::new();
    walk(root, root, &mut entries);
    entries
}

#[test]
fn positive_case_reports_the_target_current_and_the_rest_untargeted() {
    let fixture = build_fixture("positive");
    let output = fixture.inspect();
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt = receipt_json(&output);
    let rows = receipt["rows"].as_array().unwrap();
    assert_eq!(rows.len(), 2);
    assert_eq!(rows[0]["requirement_id"], "example.synthetic.first");
    assert_eq!(rows[0]["state"], "current");
    assert_eq!(rows[1]["requirement_id"], "example.synthetic.second");
    assert_eq!(rows[1]["state"], "untargeted");
    assert_eq!(receipt["targets_current"], true);
    assert_eq!(receipt["coverage_complete"], false);
    assert_eq!(receipt["complete"], false);
    assert_eq!(rows[0]["bound"]["signer"], SIGNER);
    assert_eq!(rows[0]["bound"]["target_path"], ROOT_PATH);
}

#[test]
fn an_absolute_deck_locator_inside_the_producer_output_is_current() {
    let fixture = build_fixture("absolute-deck");
    let producer = fixture.case_dir.join(ROOT_BASENAME);
    let locator = producer.join(DECK_NAME);
    rewrite_evidence_deck(&fixture, locator.to_str().unwrap());
    let output = fixture.inspect();
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt = receipt_json(&output);
    assert_eq!(receipt["rows"][0]["state"], "current");
    assert!(receipt["rows"][0]["issues"].as_array().unwrap().is_empty());
}

#[test]
fn a_deck_locator_outside_the_producer_output_is_rejected() {
    let fixture = build_fixture("outside-deck");
    let producer_deck = fixture.case_dir.join(ROOT_BASENAME).join(DECK_NAME);
    let outside = fixture.root.join("outside-deck.json");
    fs::copy(&producer_deck, &outside).unwrap();

    for locator in [
        outside.to_str().unwrap().to_string(),
        format!("../{DECK_NAME}"),
    ] {
        rewrite_evidence_deck(&fixture, &locator);
        let output = fixture.inspect();
        assert_eq!(
            output.status.code(),
            Some(1),
            "locator {locator} must be rejected: stdout {}",
            String::from_utf8_lossy(&output.stdout)
        );
        let rejection = receipt_json(&output);
        assert_eq!(rejection["status"], "rejected");
        assert!(
            rejection["issues"]
                .as_array()
                .unwrap()
                .iter()
                .any(|issue| issue["code"] == "convergence.proof_deck_outside_output"),
            "locator {locator} rejection: {rejection}"
        );
    }
}

#[test]
fn a_producer_output_locator_resolves_absolute_decks_in_a_retained_copy() {
    let fixture = build_fixture("retained-output");
    let locator =
        format!("/datapool/mantle-store/convergence-pilot-20261119/store/{ROOT_BASENAME}");
    let retained = fixture.case_dir.join("retained").join(ROOT_BASENAME);
    fs::create_dir_all(retained.parent().unwrap()).unwrap();
    fs::rename(fixture.case_dir.join(ROOT_BASENAME), &retained).unwrap();
    fixture.rewrite_case(|case| {
        case["contracts"][0]["producer_output"] = json!(format!("retained/{ROOT_BASENAME}"));
        case["contracts"][0]["producer_output_locator"] = json!(locator);
    });
    rewrite_evidence_deck(&fixture, &format!("{locator}/{DECK_NAME}"));

    let output = fixture.inspect();
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let receipt = receipt_json(&output);
    assert_eq!(receipt["rows"][0]["state"], "current");
    assert_eq!(
        receipt["inputs"]["contracts"][0]["producer_output_locator"],
        json!(locator)
    );
}

#[test]
fn a_producer_output_locator_with_a_different_basename_is_rejected() {
    let fixture = build_fixture("locator-basename");
    let locator = "/datapool/mantle-store/pilot/other-output".to_string();
    fixture.rewrite_case(|case| {
        case["contracts"][0]["producer_output_locator"] = json!(locator);
    });
    rewrite_evidence_deck(&fixture, &format!("{locator}/{DECK_NAME}"));

    let output = fixture.inspect();
    assert_eq!(output.status.code(), Some(1));
    let rejection = receipt_json(&output);
    assert!(
        rejection["issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| issue["code"] == "convergence.proof_deck_outside_output"),
        "rejection: {rejection}"
    );
}

#[test]
fn an_absolute_deck_outside_the_producer_output_locator_is_rejected() {
    let fixture = build_fixture("locator-outside");
    let locator = format!("/datapool/mantle-store/pilot/{ROOT_BASENAME}");
    fixture.rewrite_case(|case| {
        case["contracts"][0]["producer_output_locator"] = json!(locator);
    });
    rewrite_evidence_deck(&fixture, "/datapool/mantle-store/pilot/outside-deck.json");

    let output = fixture.inspect();
    assert_eq!(output.status.code(), Some(1));
    let rejection = receipt_json(&output);
    assert!(
        rejection["issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| issue["code"] == "convergence.proof_deck_outside_output"),
        "rejection: {rejection}"
    );
}

#[test]
fn omitted_realization_and_store_info_report_unrealized() {
    let fixture = build_fixture("unrealized");
    fixture.rewrite_case(|case| {
        let entry = case["contracts"][0].as_object_mut().unwrap();
        entry.remove("realization_receipt");
        entry.remove("store_info");
    });
    let output = fixture.inspect();
    assert!(output.status.success());
    let receipt = receipt_json(&output);
    assert_eq!(receipt["rows"][0]["state"], "unrealized");
    assert_eq!(receipt["targets_current"], false);
}

#[test]
fn an_untrusted_signer_reports_unrealized() {
    let fixture = build_fixture("untrusted");
    fixture.rewrite_case(|case| {
        case["trusted_signers"] = json!(["another-key"]);
    });
    let output = fixture.inspect();
    assert!(output.status.success());
    let receipt = receipt_json(&output);
    assert_eq!(receipt["rows"][0]["state"], "unrealized");
    assert!(receipt["rows"][0]["issues"]
        .as_array()
        .unwrap()
        .iter()
        .any(|issue| issue == "convergence.untrusted_signer"));
}

#[test]
fn a_weakened_contract_reports_stale() {
    let fixture = build_fixture("weakened");
    let contract_path = fixture.case_dir.join("contract.json");
    let mut contract: Value = serde_json::from_slice(&fixture.contract_bytes).unwrap();
    contract["clauses"][0]["disposition"] = json!("empirical_check");
    let contract_bytes = serde_json::to_vec_pretty(&contract).unwrap();
    fs::write(&contract_path, &contract_bytes).unwrap();

    let evidence_path = fixture.case_dir.join("evidence.json");
    let mut evidence: Value = serde_json::from_slice(&fixture.evidence_bytes).unwrap();
    evidence["contract_blake3"] = json!(blake3_hex(&contract_bytes));
    let evidence_bytes = serde_json::to_vec_pretty(&evidence).unwrap();
    fs::write(&evidence_path, &evidence_bytes).unwrap();

    let decision_path = fixture.case_dir.join("decision.json");
    let mut decision: Value = serde_json::from_slice(&fs::read(&decision_path).unwrap()).unwrap();
    decision["contract_blake3"] = json!(blake3_hex(&contract_bytes));
    decision["evidence_blake3"] = json!(blake3_hex(&evidence_bytes));
    fs::write(
        &decision_path,
        serde_json::to_vec_pretty(&decision).unwrap(),
    )
    .unwrap();

    let output = fixture.inspect();
    assert!(output.status.success());
    let receipt = receipt_json(&output);
    assert_eq!(receipt["rows"][0]["state"], "stale");
    assert!(receipt["rows"][0]["issues"]
        .as_array()
        .unwrap()
        .iter()
        .any(|issue| issue == "convergence.projection_mismatch"));
}

#[test]
fn unknown_options_and_missing_case_fail() {
    let fixture = build_fixture("usage");
    let unknown = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args(["convergence", "inspect", "--case"])
        .arg(fixture.case_path())
        .arg("--unknown")
        .output()
        .unwrap();
    assert_eq!(unknown.status.code(), Some(1));

    let missing = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args(["convergence", "inspect", "--root"])
        .arg(&fixture.root)
        .output()
        .unwrap();
    assert_eq!(missing.status.code(), Some(1));
}

#[test]
fn an_unknown_observation_schema_is_rejected() {
    let fixture = build_fixture("unknown-schema");
    let observation_path = fixture.case_dir.join("handoff/mantle-observation.json");
    let mut observation: Value =
        serde_json::from_slice(&fs::read(&observation_path).unwrap()).unwrap();
    observation["schema"] = json!("synthetic-unknown-observation-v1");
    fs::write(
        &observation_path,
        serde_json::to_vec_pretty(&observation).unwrap(),
    )
    .unwrap();

    let output = fixture.inspect();
    assert_eq!(output.status.code(), Some(1));
    let rejection = receipt_json(&output);
    assert_eq!(rejection["status"], "rejected");
    assert!(
        rejection["issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| issue["code"] == "convergence.unknown_schema"),
        "rejection: {rejection}"
    );
}

#[cfg(unix)]
#[test]
fn a_symlinked_decision_file_is_rejected() {
    let fixture = build_fixture("symlink");
    let decision_path = fixture.case_dir.join("decision.json");
    let outside = fixture.root.join("outside-decision.json");
    fs::rename(&decision_path, &outside).unwrap();
    std::os::unix::fs::symlink(&outside, &decision_path).unwrap();

    let output = fixture.inspect();
    assert_eq!(output.status.code(), Some(1));
    let rejection = receipt_json(&output);
    assert_eq!(rejection["status"], "rejected");
    assert!(
        rejection["issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| issue["code"] == "convergence.unreadable_input"),
        "symlink rejection: {rejection}"
    );
}

#[test]
fn receipts_are_deterministic_and_the_root_is_unchanged() {
    let fixture = build_fixture("determinism");
    let before = hash_tree(&fixture.root);
    let first = fixture.inspect();
    let second = fixture.inspect();
    assert!(first.status.success());
    assert!(second.status.success());
    assert_eq!(first.stdout, second.stdout);
    let receipt = receipt_json(&first);
    let receipt_blake3 = receipt["receipt_blake3"].as_str().unwrap();
    assert_eq!(receipt_blake3.len(), 64);
    let after = hash_tree(&fixture.root);
    assert_eq!(before, after);
    // The targets admission and registry rows the fixture asserted survive the
    // two reads untouched.
    assert_eq!(fixture.targets.contracts.len(), 1);
    assert_eq!(fixture.registry.rows.len(), 2);
}
