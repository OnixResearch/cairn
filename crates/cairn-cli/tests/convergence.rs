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
        self.inspect_with_policy_option(None)
    }

    /// Run the command against an explicit policy export instead of the
    /// embedded one.
    fn inspect_with_policy(&self, policy: &Path) -> Output {
        self.inspect_with_policy_option(Some(policy))
    }

    fn inspect_with_policy_option(&self, policy: Option<&Path>) -> Output {
        let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
        command
            .arg("convergence")
            .arg("inspect")
            .arg("--case")
            .arg(self.case_path())
            .arg("--root")
            .arg(&self.root);
        if let Some(policy) = policy {
            command.arg("--policy").arg(policy);
        }
        command
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
        "owner": "Octet",
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

/// Grow a file to exactly `length` bytes. JSON decoders admit trailing
/// whitespace, and the tool graph is only hashed, so space padding changes no
/// field of either record.
fn pad_file_to(path: &Path, length: usize) {
    let mut bytes = fs::read(path).unwrap();
    assert!(
        bytes.len() <= length,
        "{} is {} bytes, larger than the {length}-byte target",
        path.display(),
        bytes.len()
    );
    bytes.resize(length, b' ');
    fs::write(path, bytes).unwrap();
}

/// Rebind the bridge observation's `tool_graph_identity` to the current tool
/// graph bytes.
fn rebind_tool_graph_identity(fixture: &Fixture) {
    let tool_path = fixture
        .case_dir
        .join("handoff/tool-closure.derivation-json.json");
    let tool_bytes = fs::read(&tool_path).unwrap();
    let observation_path = fixture.case_dir.join("handoff/mantle-observation.json");
    let mut observation: Value =
        serde_json::from_slice(&fs::read(&observation_path).unwrap()).unwrap();
    observation["tool_graph_identity"] = json!(format!("b3:{}", blake3_hex(&tool_bytes)));
    fs::write(
        &observation_path,
        serde_json::to_vec_pretty(&observation).unwrap(),
    )
    .unwrap();
}

/// A copy of the exported policy with the two Mantle record bounds replaced.
fn policy_with_record_bounds(
    fixture: &Fixture,
    name: &str,
    tool_graph_bytes: u64,
    realization_receipt_bytes: u64,
) -> PathBuf {
    let mut policy: Value = serde_json::from_str(POLICY_JSON).unwrap();
    policy["bounds"]["max_tool_graph_bytes"] = json!(tool_graph_bytes);
    policy["bounds"]["max_realization_receipt_bytes"] = json!(realization_receipt_bytes);
    let path = fixture.root.join(format!("{name}-policy.json"));
    fs::write(&path, serde_json::to_vec_pretty(&policy).unwrap()).unwrap();
    path
}

/// The issue codes a receipt reports for `path`.
fn issue_codes_for(receipt: &Value, path: &Path) -> Vec<String> {
    receipt["issues"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|issue| {
            issue["field_path"]
                .as_str()
                .is_some_and(|field| field.ends_with(path.to_str().unwrap()))
        })
        .map(|issue| issue["code"].as_str().unwrap().to_string())
        .collect()
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

/// Rewrite the handoff job and rebind the observation's
/// `producer_job_identity` to the new bytes.
fn rewrite_job(fixture: &Fixture, job: Value) {
    let job_bytes = serde_json::to_vec(&job).unwrap();
    fs::write(fixture.case_dir.join("handoff/job.json"), &job_bytes).unwrap();
    let observation_path = fixture.case_dir.join("handoff/mantle-observation.json");
    let mut observation: Value =
        serde_json::from_slice(&fs::read(&observation_path).unwrap()).unwrap();
    observation["producer_job_identity"] = json!(format!("b3:{}", blake3_hex(&job_bytes)));
    fs::write(
        &observation_path,
        serde_json::to_vec_pretty(&observation).unwrap(),
    )
    .unwrap();
}

fn assert_job_rejection(fixture: &Fixture, code: &str) {
    let output = fixture.inspect();
    assert_eq!(
        output.status.code(),
        Some(1),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let rejection = receipt_json(&output);
    assert_eq!(rejection["status"], "rejected");
    assert!(
        rejection["issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| issue["code"] == code),
        "rejection: {rejection}"
    );
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
fn a_trellis_owner_job_binds_current() {
    let fixture = build_fixture("trellis-owner");
    rewrite_job(
        &fixture,
        json!({
            "schema_version": "trellis-mantle-producer-job/v1",
            "owner": "Trellis",
            "job_id": JOB_ID,
        }),
    );
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
fn an_unknown_producer_owner_is_rejected() {
    let fixture = build_fixture("unknown-owner");
    rewrite_job(
        &fixture,
        json!({
            "schema_version": "trellis-mantle-producer-job/v1",
            "owner": "Unknown",
            "job_id": JOB_ID,
        }),
    );
    assert_job_rejection(&fixture, "convergence.unknown_producer_owner");
}

#[test]
fn a_producer_schema_owner_mismatch_is_rejected() {
    let fixture = build_fixture("owner-schema-mismatch");
    rewrite_job(
        &fixture,
        json!({
            "schema_version": "trellis-mantle-producer-job/v1",
            "owner": "Octet",
            "job_id": JOB_ID,
        }),
    );
    assert_job_rejection(&fixture, "convergence.producer_schema_mismatch");
}

#[test]
fn a_missing_producer_owner_is_rejected() {
    let fixture = build_fixture("missing-owner");
    rewrite_job(
        &fixture,
        json!({
            "schema_version": "octet-mantle-producer-job/v1",
            "job_id": JOB_ID,
        }),
    );
    assert_job_rejection(&fixture, "convergence.missing_producer_owner");
}

#[test]
fn the_verified_mode_relay_value_also_binds() {
    // `octet-job` runs the bridge in verified mode, which reports
    // `externally-reported-by-octet` instead of the fixture mode's
    // `not-evaluated-by-mantle`. Neither is acceptance.
    let fixture = build_fixture("verified-relay");
    let observation_path = fixture.case_dir.join("handoff/mantle-observation.json");
    let mut observation: Value =
        serde_json::from_slice(&fs::read(&observation_path).unwrap()).unwrap();
    observation["proof_acceptance"] = json!("externally-reported-by-octet");
    fs::write(
        &observation_path,
        serde_json::to_vec_pretty(&observation).unwrap(),
    )
    .unwrap();

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
    assert_eq!(receipt["targets_current"], true);
}

#[test]
fn a_tool_graph_and_receipt_over_one_mib_are_admitted() {
    // The pilot's tool graph is 38,785,189 bytes and its realization receipt
    // 6,469,512 bytes for 11,565 units; both exceed `max_document_bytes`.
    let fixture = build_fixture("over-one-mib");
    pad_file_to(
        &fixture
            .case_dir
            .join("handoff/tool-closure.derivation-json.json"),
        1_200_000,
    );
    pad_file_to(&fixture.case_dir.join("realization.json"), 1_300_000);
    rebind_tool_graph_identity(&fixture);

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
    assert_eq!(receipt["targets_current"], true);
}

#[test]
fn a_record_of_exactly_its_bound_is_admitted_and_one_byte_more_is_rejected() {
    const BOUND: u64 = 8192;
    let fixture = build_fixture("record-bounds");
    let tool_graph = fixture
        .case_dir
        .join("handoff/tool-closure.derivation-json.json");
    let realization = fixture.case_dir.join("realization.json");
    pad_file_to(&tool_graph, BOUND as usize);
    pad_file_to(&realization, BOUND as usize);
    rebind_tool_graph_identity(&fixture);

    let exact = policy_with_record_bounds(&fixture, "exact", BOUND, BOUND);
    let output = fixture.inspect_with_policy(&exact);
    assert!(
        output.status.success(),
        "exactly-bound records must be admitted: stdout {}\nstderr {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert_eq!(receipt_json(&output)["rows"][0]["state"], "current");

    for (name, tool_bound, receipt_bound, over) in [
        ("tool graph", BOUND - 1, BOUND, &tool_graph),
        ("realization receipt", BOUND, BOUND - 1, &realization),
    ] {
        let policy =
            policy_with_record_bounds(&fixture, &name.replace(' ', "-"), tool_bound, receipt_bound);
        let output = fixture.inspect_with_policy(&policy);
        assert_eq!(
            output.status.code(),
            Some(1),
            "a {name} one byte over its bound must be rejected: stdout {}",
            String::from_utf8_lossy(&output.stdout)
        );
        let rejection = receipt_json(&output);
        assert_eq!(rejection["status"], "rejected");
        assert_eq!(
            issue_codes_for(&rejection, over),
            vec!["convergence.unreadable_input".to_string()],
            "{name} rejection: {rejection}"
        );
    }
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

fn mcp_inspect_call(fixture: &Fixture, arguments: &Value) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args(["mcp", "--call", "cairn_convergence_inspect"])
        .arg(arguments.to_string())
        .current_dir(&fixture.root)
        .output()
        .expect("run cairn mcp --call cairn_convergence_inspect")
}

fn mcp_arguments(fixture: &Fixture) -> Value {
    json!({
        "case": fixture.case_path(),
        "root": fixture.root,
    })
}

// r[verify cairn.convergence.mcp_inspect]
#[test]
fn mcp_inspect_returns_the_cli_receipt_and_repeats_identically() {
    let fixture = build_fixture("mcp-receipt");
    let cli = receipt_json(&fixture.inspect());
    let first = mcp_inspect_call(&fixture, &mcp_arguments(&fixture));
    assert!(
        first.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&first.stdout),
        String::from_utf8_lossy(&first.stderr)
    );
    let result = receipt_json(&first);
    assert_eq!(result["isError"], false);
    assert_eq!(result["structuredContent"], cli);
    assert_eq!(
        result["structuredContent"]["receipt_blake3"],
        cli["receipt_blake3"]
    );
    let second = mcp_inspect_call(&fixture, &mcp_arguments(&fixture));
    assert!(second.status.success());
    assert_eq!(first.stdout, second.stdout);
}

// r[verify cairn.convergence.mcp_inspect]
#[test]
fn mcp_inspect_rejects_missing_and_unknown_arguments() {
    let fixture = build_fixture("mcp-arguments");
    let missing = mcp_inspect_call(&fixture, &json!({"root": fixture.root}));
    assert!(!missing.status.success());
    assert!(
        String::from_utf8_lossy(&missing.stderr).contains("missing required argument case"),
        "stderr: {}",
        String::from_utf8_lossy(&missing.stderr)
    );
    let unknown = mcp_inspect_call(
        &fixture,
        &json!({"case": fixture.case_path(), "execute": true}),
    );
    assert!(!unknown.status.success());
    assert!(
        String::from_utf8_lossy(&unknown.stderr).contains("unknown argument execute"),
        "stderr: {}",
        String::from_utf8_lossy(&unknown.stderr)
    );
}

// r[verify cairn.convergence.mcp_inspect]
#[test]
fn mcp_inspect_maps_an_unadmitted_schema_to_the_rejection_receipt() {
    let fixture = build_fixture("mcp-unknown-schema");
    let observation_path = fixture.case_dir.join("handoff/mantle-observation.json");
    let mut observation: Value =
        serde_json::from_slice(&fs::read(&observation_path).unwrap()).unwrap();
    observation["schema"] = json!("synthetic-unknown-observation-v1");
    fs::write(
        &observation_path,
        serde_json::to_vec_pretty(&observation).unwrap(),
    )
    .unwrap();

    let output = mcp_inspect_call(&fixture, &mcp_arguments(&fixture));
    assert!(output.status.success());
    let receipt = receipt_json(&output)["structuredContent"].clone();
    assert_eq!(receipt["schema_version"], "cairn.convergence.v1");
    assert_eq!(receipt["status"], "rejected");
    assert!(
        receipt["issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| issue["code"] == "convergence.unknown_schema"),
        "rejection: {receipt}"
    );
}

#[cfg(unix)]
// r[verify cairn.convergence.mcp_inspect]
#[test]
fn mcp_inspect_rejects_a_symlinked_input() {
    let fixture = build_fixture("mcp-symlink");
    let decision_path = fixture.case_dir.join("decision.json");
    let outside = fixture.root.join("outside-decision.json");
    fs::rename(&decision_path, &outside).unwrap();
    std::os::unix::fs::symlink(&outside, &decision_path).unwrap();

    let output = mcp_inspect_call(&fixture, &mcp_arguments(&fixture));
    assert!(output.status.success());
    let receipt = receipt_json(&output)["structuredContent"].clone();
    assert_eq!(receipt["status"], "rejected");
    assert!(
        receipt["issues"]
            .as_array()
            .unwrap()
            .iter()
            .any(|issue| issue["code"] == "convergence.unreadable_input"),
        "symlink rejection: {receipt}"
    );
}

// r[verify cairn.convergence.mcp_inspect]
#[test]
fn mcp_catalog_registers_convergence_inspect_read_only_in_every_profile() {
    for profile in ["full", "inspect", "author", "verify"] {
        let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
            .args(["mcp", "--tool-profile", profile, "--describe"])
            .current_dir(env!("CARGO_MANIFEST_DIR"))
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "{profile}: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        let catalog = receipt_json(&output);
        let tool = catalog["tools"]
            .as_array()
            .unwrap()
            .iter()
            .find(|tool| tool["name"] == "cairn_convergence_inspect")
            .unwrap_or_else(|| panic!("{profile} omits cairn_convergence_inspect"));
        assert_eq!(tool["annotations"]["readOnlyHint"], true);
        assert_eq!(tool["annotations"]["destructiveHint"], false);
        assert_eq!(tool["inputSchema"]["additionalProperties"], false);
        assert_eq!(tool["inputSchema"]["required"], json!(["case"]));
        for optional in ["root", "policy"] {
            assert!(
                tool["inputSchema"]["properties"][optional].is_object(),
                "{profile} omits optional {optional}"
            );
        }
    }
}
