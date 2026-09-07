use cairn_core::{
    canonical_json_hash, derive_verification_impact, evaluate_content_bound_requirement_registry,
    evaluate_content_bound_trace_index, ContentBoundRevision, ContentBoundSpecificationInput,
    ContentBoundTraceIndex, TraceabilityFile, TraceabilityProfile, TraceabilitySource,
    VerificationImpactInput,
};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

const REQUIREMENT: &str = "cairn.verification.demo";
const SOURCE: &str = "crates/demo/src/lib.rs";
const COPY: &str = "crates/demo/src/copy.rs";
const CONTRACT: &str = "contract with spaces; not-executed.json";
const PROFILES: &str = "profiles.json";
const APPLICABILITY: &str = "applicability.json";
const MUTANT: &str = "unsafe-omission";

fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn index(files: Vec<TraceabilityFile>) -> ContentBoundTraceIndex {
    let registry = evaluate_content_bound_requirement_registry(
        vec![ContentBoundSpecificationInput {
            path: ".cairn/specs/demo/spec.md".into(),
            markdown: format!("# Demo\n\n## Requirements\n\n### Requirement: Demo\nr[{REQUIREMENT}] The system MUST preserve required checks.\n\n#### Scenario: Changed\n- GIVEN changed source\n- WHEN guidance runs\n- THEN required checks remain\n"),
        }, ContentBoundSpecificationInput {
            path: ".cairn/specs/change-verification-guidance/spec.md".into(),
            markdown: "# Guidance\n\n## Requirements\n\n### Requirement: Interfaces\nr[cairn.verification_impact.interfaces] The system MUST expose read-only guidance.\n\n#### Scenario: Parity\n- GIVEN source inputs\n- WHEN guidance runs\n- THEN both transports agree\n".into(),
        }], b"verification-impact-controls", "onixresearch/cairn",
        ContentBoundRevision { kind: "opaque".into(), value: "fresh-controls".into() },
    ).registry.expect("valid registry");
    evaluate_content_bound_trace_index(
        &registry,
        &TraceabilityProfile {
            id: "guidance-controls".into(),
            requirement_sources: vec![TraceabilitySource {
                root: ".cairn/specs".into(),
                source_kind: "accepted_specs".into(),
                extensions: vec!["md".into()],
            }],
            evidence_sources: vec![TraceabilitySource {
                root: "crates".into(),
                source_kind: "rust_source".into(),
                extensions: vec!["rs".into()],
            }],
            marker_verbs: vec!["impl".into(), "verify".into()],
            assurance_level: "declared".into(),
            anchor_kind: "exact_marker_bytes".into(),
        },
        files,
    )
    .index
    .expect("fresh index")
}

fn manifest(index: &ContentBoundTraceIndex) -> Value {
    json!({"identity":canonical_json_hash(&json!(index.sources)),
        "scope":{"repository_id":"onixresearch/cairn","target":"x86_64-linux","features":["default"]},
        "members":index.sources.iter().map(|source| json!({"path":source.path,"content_blake3":source.content_blake3,"generated":false})).collect::<Vec<_>>()})
}

fn comparison(before: &[(&str, &str)], after: &[(&str, &str)]) -> Value {
    let make = |files: &[(&str, &str)]| {
        index(
            files
                .iter()
                .map(|(path, text)| TraceabilityFile {
                    path: (*path).into(),
                    text: (*text).into(),
                })
                .collect(),
        )
    };
    let before = make(before);
    let after = make(after);
    let mut paths = BTreeMap::new();
    for source in &before.sources {
        paths.insert(
            source.path.clone(),
            (Some(source.content_blake3.clone()), None),
        );
    }
    for source in &after.sources {
        paths.entry(source.path.clone()).or_default().1 = Some(source.content_blake3.clone());
    }
    let changed: Vec<_> = paths
        .into_iter()
        .filter_map(|(path, (a, b))| (a != b).then_some(path))
        .collect();
    let check = |id: &str, mandatory| json!({"check_id":id,"target":"x86_64-linux","features":["default"],"mandatory":mandatory});
    json!({"schema_version":"cairn.verification-impact.v1", "before_manifest":manifest(&before), "after_manifest":manifest(&after),
        "asserted_changed_paths":changed, "before_trace_index":before,"after_trace_index":after,
        "check_map":{"schema_version":"cairn.verification-check-map.v1","map_id":"cairn-local","coverage_complete":true,
            "default_check_ids":["workspace-tests"],"checks":[check("focused",false),check("policy-fresh",true),check("workspace-tests",false)],
            "requirement_checks":[{"requirement_id":REQUIREMENT,"check_id":"focused"}],"dependency_edges":[],"dependency_hashes":[],
            "macro_opaque_paths":[],"generated_source_paths":[],"incomplete_source_paths":[]},"prior_evidence":[]})
}

fn linked() -> String {
    format!("// r[impl {REQUIREMENT}]\nfn demo() {{}}\n")
}

fn run(args: &[&str]) -> Value {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .args(args)
        .output()
        .expect("command");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    serde_json::from_slice(&output.stdout).expect("JSON output")
}

fn snapshot(root: &Path) -> BTreeMap<PathBuf, Vec<u8>> {
    fs::read_dir(root)
        .unwrap()
        .map(|entry| {
            let path = entry.unwrap().path();
            let bytes = fs::read(&path).unwrap();
            (path, bytes)
        })
        .collect()
}

fn parity(name: &str, input: &Value, evidence: Option<&Value>) -> Value {
    let root = std::env::temp_dir().join(format!("cairn-guidance-{name}-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("input.json"), serde_json::to_vec(input).unwrap()).unwrap();
    for path in [CONTRACT, PROFILES, APPLICABILITY] {
        fs::write(
            root.join(path),
            "not producer input; guidance must not read or execute this",
        )
        .unwrap();
    }
    let root_text = root.to_str().unwrap();
    let mut args = vec![
        "verification-impact",
        "guidance",
        "--root",
        root_text,
        "--input",
        "input.json",
        "--change",
        "demo",
        "--contract",
        CONTRACT,
        "--profiles",
        PROFILES,
        "--applicability",
        APPLICABILITY,
    ];
    let mut request = json!({"root":root_text,"input":input,"change":"demo","contract":CONTRACT,"profiles":PROFILES,"applicability":APPLICABILITY});
    if let Some(evidence) = evidence {
        fs::write(
            root.join("evidence.json"),
            serde_json::to_vec(evidence).unwrap(),
        )
        .unwrap();
        args.extend(["--evidence", "evidence.json"]);
        request["evidence"] = evidence.clone();
    }
    let before = snapshot(&root);
    let cli = run(&args);
    let mcp = run(&[
        "mcp",
        "--call",
        "cairn_verification_impact",
        &request.to_string(),
    ]);
    assert_eq!(
        mcp["structuredContent"], cli,
        "{name}: transport divergence"
    );
    assert_eq!(snapshot(&root), before, "{name}: guidance mutated inputs");
    assert_eq!(cli["checks_executed"], false);
    assert_eq!(cli["mutation_authorized"], false);
    assert_eq!(cli["structural_evidence"]["completion_eligible"], false);
    assert_eq!(
        cli["structural_evidence"]["selection_scope"],
        "declared_links_only"
    );
    if cli["valid"] == true {
        assert_eq!(cli["actions"][0]["tool"], "cairn_readiness");
        assert_eq!(
            cli["actions"][1]["argv"],
            json!([
                "cairn",
                "check-evidence",
                "plan",
                "--root",
                root_text,
                "--contract",
                CONTRACT,
                "--profiles",
                PROFILES,
                "--applicability",
                APPLICABILITY
            ])
        );
        assert_eq!(
            cli["actions"][1]["arguments"],
            json!({"root":root_text,"contract":CONTRACT,"profiles":PROFILES,"applicability":APPLICABILITY})
        );
        assert_eq!(cli["actions"][1]["checks_executed"], false);
        assert!(
            cli["report"]["selected_checks"]
                .as_array()
                .unwrap()
                .contains(&json!("policy-fresh")),
            "{name}: mandatory check omitted"
        );
    } else {
        assert_eq!(cli["actions"], json!([]));
    }
    fs::remove_dir_all(root).unwrap();
    cli
}

#[test]
// r[verify cairn.verification_impact.interfaces]
// r[verify cairn.verification_impact.selection]
// r[verify cairn.verification_impact.uncertainty]
// r[verify cairn.verification_impact.conformance]
fn verification_impact_transports_preserve_controls_and_exact_plan_arguments() {
    let body = linked();
    let changed = format!("{body}// changed\n");
    let base = comparison(&[(SOURCE, &body)], &[(SOURCE, &changed)]);
    let mut cases = vec![
        ("linked", base.clone(), true, "focused"),
        (
            "deletion",
            comparison(&[(SOURCE, &body)], &[]),
            true,
            "focused",
        ),
        (
            "rename",
            comparison(&[(SOURCE, &body)], &[(COPY, &body)]),
            true,
            "focused",
        ),
        (
            "copied-marker",
            comparison(&[(SOURCE, &body)], &[(SOURCE, &body), (COPY, &body)]),
            true,
            "focused",
        ),
        (
            "unlinked",
            comparison(&[(SOURCE, "fn before() {}")], &[(SOURCE, "fn after() {}")]),
            true,
            "workspace-tests",
        ),
    ];
    for (name, pointer, replacement, valid, check) in [
        (
            "missing-edge",
            "/check_map/coverage_complete",
            json!(false),
            true,
            "workspace-tests",
        ),
        (
            "missing-mapping",
            "/check_map/requirement_checks",
            json!([]),
            true,
            "workspace-tests",
        ),
        (
            "stale-index",
            "/after_trace_index/index_blake3",
            json!("a".repeat(64)),
            false,
            "",
        ),
        (
            "cycle",
            "/check_map/dependency_edges",
            json!([{"from_requirement_id":REQUIREMENT,"to_requirement_id":REQUIREMENT}]),
            false,
            "",
        ),
        (
            "macro",
            "/check_map/macro_opaque_paths",
            json!([SOURCE]),
            true,
            "workspace-tests",
        ),
        (
            "generated",
            "/after_manifest/members/0/generated",
            json!(true),
            true,
            "workspace-tests",
        ),
        (
            "target",
            "/after_manifest/scope/target",
            json!("wasm32-unknown-unknown"),
            false,
            "",
        ),
        (
            "feature",
            "/after_manifest/scope/features",
            json!(["other"]),
            false,
            "",
        ),
    ] {
        let mut input = base.clone();
        *input.pointer_mut(pointer).unwrap() = replacement;
        cases.push((name, input, valid, check));
    }
    for (name, input, valid, check) in cases {
        let report = parity(name, &input, None);
        assert_eq!(report["valid"], valid, "{name}: {report:#}");
        if valid {
            assert!(
                report["report"]["selected_checks"]
                    .as_array()
                    .unwrap()
                    .contains(&json!(check)),
                "{name}: missing {check}"
            );
        } else {
            assert!(!report["report"]["blockers"].as_array().unwrap().is_empty());
        }
    }
}

#[test]
// r[verify cairn.verification_impact.uncertainty]
fn verification_impact_transports_keep_octet_ineligible_and_reject_substitution() {
    use cairn_core::verified::check_evidence::OCTET_REVISION;
    let body = linked();
    let input = comparison(&[(SOURCE, &body)], &[(SOURCE, &format!("{body}// edit"))]);
    let receipt: Value = serde_json::from_slice(
        &fs::read(repository().join("fixtures/check-evidence/octet-producer-fixture.json"))
            .unwrap(),
    )
    .unwrap();
    let evidence = json!({"schema_version":"cairn.verification-impact-evidence.v1", "source_identity":input["after_manifest"]["identity"],
        "scope":input["after_manifest"]["scope"],"octet":{"producer_revision":OCTET_REVISION,"receipt_hash":canonical_json_hash(&receipt),"receipt":receipt},"valence":null});
    let positive = parity("octet", &input, Some(&evidence));
    assert_eq!(positive["valid"], true);
    assert_eq!(
        positive["structural_evidence"]["octet"]["completion_eligible"],
        false
    );
    for (name, pointer, value) in [
        ("source", "/source_identity", json!("f".repeat(64))),
        ("cohort", "/octet/producer_revision", json!("substituted")),
        ("receipt", "/octet/receipt/status", json!("failed")),
        (
            "valence-span-unavailable",
            "/valence",
            json!({"schema":"valence.evidence-admission.v1","valid":true,"spans":[]}),
        ),
    ] {
        let mut substituted = evidence.clone();
        *substituted.pointer_mut(pointer).unwrap() = value;
        assert_eq!(parity(name, &input, Some(&substituted))["valid"], false);
    }
}

#[test]
// r[verify cairn.verification_impact.interfaces]
fn verification_impact_rejects_partial_plans_and_execution_through_both_transports() {
    let input = comparison(&[(SOURCE, "before")], &[(SOURCE, "after")]);
    let root = std::env::temp_dir().join(format!("cairn-guidance-rejected-{}", std::process::id()));
    fs::create_dir_all(&root).unwrap();
    fs::write(root.join("input.json"), serde_json::to_vec(&input).unwrap()).unwrap();
    for (cli_extra, mcp_extra) in [
        (vec!["--contract", "c.json"], json!({"contract":"c.json"})),
        (vec!["--execute", "true"], json!({"execute":true})),
    ] {
        let mut args = vec![
            "verification-impact",
            "guidance",
            "--root",
            root.to_str().unwrap(),
            "--input",
            "input.json",
        ];
        args.extend(cli_extra);
        assert!(!Command::new(env!("CARGO_BIN_EXE_cairn"))
            .args(args)
            .output()
            .unwrap()
            .status
            .success());
        let mut request = mcp_extra;
        request["input"] = input.clone();
        assert!(!Command::new(env!("CARGO_BIN_EXE_cairn"))
            .args([
                "mcp",
                "--call",
                "cairn_verification_impact",
                &request.to_string()
            ])
            .output()
            .unwrap()
            .status
            .success());
    }
    fs::remove_dir_all(root).unwrap();
}

fn preserve_defaults(report: &Value) {
    assert!(
        report["selected_checks"]
            .as_array()
            .unwrap()
            .contains(&json!("workspace-tests")),
        "unsafe omission: incomplete coverage must preserve broader defaults"
    );
    assert!(
        report["selected_checks"]
            .as_array()
            .unwrap()
            .contains(&json!("policy-fresh")),
        "unsafe omission: mandatory check removed"
    );
}

#[test]
// r[verify cairn.verification_impact.conformance]
fn verification_impact_required_check_preservation_rejects_unsafe_omission_mutant() {
    let body = linked();
    let mut input = comparison(
        &[(SOURCE, &body)],
        &[(SOURCE, &format!("{body}// changed"))],
    );
    input["check_map"]["coverage_complete"] = json!(false);
    let typed: VerificationImpactInput = serde_json::from_value(input).unwrap();
    let valid = json!(derive_verification_impact(&typed));
    preserve_defaults(&valid);
    let mut mutant = valid.clone();
    mutant["selected_checks"] = json!(["focused", "policy-fresh"]);
    assert!(
        std::panic::catch_unwind(|| preserve_defaults(&mutant)).is_err(),
        "{MUTANT} survived"
    );
    let mut mandatory_mutant = valid;
    mandatory_mutant["selected_checks"] = json!(["focused", "workspace-tests"]);
    assert!(
        std::panic::catch_unwind(|| preserve_defaults(&mandatory_mutant)).is_err(),
        "mandatory omission survived"
    );
}

#[test]
// r[verify cairn.verification_impact.conformance]
// r[verify cairn.verification_impact.applicability]
fn verification_impact_fresh_cairn_source_replay_keeps_full_gates_outstanding() {
    // Read complete current Cairn source bytes in a one-file projection.
    // Append a comment only in memory, then regenerate both indexes.
    let path = "crates/cairn-cli/src/verification_impact.rs";
    let current = fs::read_to_string(repository().join(path)).unwrap();
    let changed = format!("{current}\n// fresh consumer comparison\n");
    let mut input = comparison(&[(path, &current)], &[(path, &changed)]);
    let typed: VerificationImpactInput = serde_json::from_value(input.clone()).unwrap();
    let mut prior = json!(derive_verification_impact(&typed)
        .current_applicability
        .unwrap());
    prior["subject_hash"] = input["before_manifest"]["identity"].clone();
    input["prior_evidence"] =
        json!([{"evidence_id":"before","check_ids":["workspace-tests"],"applicability":prior}]);
    let result = parity("fresh-consumer", &input, None);
    assert_eq!(result["valid"], true);
    assert_eq!(
        result["report"]["selected_checks"],
        json!(["policy-fresh", "workspace-tests"])
    );
    assert_eq!(result["report"]["evidence_refresh"][0]["current"], false);
    assert_eq!(
        result["report"]["evidence_refresh"][0]["stale_reasons"],
        json!(["subject_changed"])
    );
    assert_eq!(result["actions"][0]["full_gate_authoritative"], true);
    println!("fresh Cairn projection: selected policy-fresh/workspace-tests; refresh subject_changed; declared-links-only; full gates outstanding");
}
