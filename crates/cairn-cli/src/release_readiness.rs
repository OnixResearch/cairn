// r[impl cli_maintainability.main_compat_release_cli.focused_modules]
// r[impl cli_maintainability.main_compat_release_cli.focused_modules.visible_boundaries]
// r[impl cli_maintainability.main_compat_release_cli.focused_modules.narrow_bridges]
// r[impl cli_maintainability.main_compat_release_cli.behavior_preserved]
// r[verify cli_maintainability.main_compat_release_cli.behavior_preserved.focused_checks]
// r[verify cli_maintainability.main_compat_release_cli.behavior_preserved.repository_checks]
// r[impl release_evidence.current_head_evidence_index.focused_modules]
// r[impl release_evidence.current_head_evidence_index.focused_modules.visible_boundaries]
// r[impl release_evidence.current_head_evidence_index.focused_modules.narrow_bridges]
// r[verify release_evidence.current_head_evidence_index.behavior_preserved]
// r[verify release_evidence.current_head_evidence_index.behavior_preserved.focused_checks]
// r[verify release_evidence.current_head_evidence_index.behavior_preserved.repository_checks]
use std::{error::Error, fs, path::PathBuf};

use cairn_core::verified::verification_role::VERIFICATION_ROLE_BOUNDARY;
use cairn_core::{
    canonical_json_hash, evaluate_workspace_contract_matrix_readiness,
    workspace_contract_readiness_issues_json, workspace_contract_readiness_policy_from_value,
};
use serde_json::{json, Value};

use crate::determinism::determinism_coverage_audit_receipt;
use crate::mcp::mcp_agent_smoke_receipt;
use crate::native::cairn_validation_receipt;
use crate::preserves_corpus::preserves_corpus_freshness_receipt;
use crate::release_evidence::release_evidence_check_index_receipt_for_path;
use crate::stack_provenance::{stack_provenance_hash, stack_provenance_receipt_value};
use crate::support::{
    default_policy_path, load_policy_file, path_string, print_json, read_option,
    read_string_option, safe_relative_path, with_portable_content_hash_receipt,
};
use crate::tracey::{repository_gate_summary, traceability_coverage_receipt};

pub(crate) fn run_release_readiness(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let review_ledger = read_string_option(&args, "--review-ledger").map(PathBuf::from);
    let stack_provenance = read_string_option(&args, "--stack-provenance").map(PathBuf::from);
    let stack_provenance_graph =
        read_string_option(&args, "--stack-provenance-graph").map(PathBuf::from);
    let workspace_contract_matrix_receipt =
        read_string_option(&args, "--workspace-contract-matrix-receipt").map(PathBuf::from);
    print_json(release_readiness_receipt_with_review_ledger_and_stack(
        root,
        policy_path,
        review_ledger,
        stack_provenance,
        stack_provenance_graph,
        workspace_contract_matrix_receipt,
    ))
}

pub(crate) fn release_readiness_receipt(root: PathBuf, policy_path: PathBuf) -> Value {
    release_readiness_receipt_with_review_ledger(root, policy_path, None)
}

// r[verify cairn.determinism.preserves_content_hash_binding_readiness.validation]
pub(crate) fn release_readiness_receipt_with_review_ledger(
    root: PathBuf,
    policy_path: PathBuf,
    review_ledger: Option<PathBuf>,
) -> Value {
    release_readiness_receipt_with_review_ledger_and_stack(
        root,
        policy_path,
        review_ledger,
        None,
        None,
        None,
    )
}

fn release_readiness_receipt_with_review_ledger_and_stack(
    root: PathBuf,
    policy_path: PathBuf,
    review_ledger: Option<PathBuf>,
    stack_provenance: Option<PathBuf>,
    stack_provenance_graph: Option<PathBuf>,
    workspace_contract_matrix_receipt: Option<PathBuf>,
) -> Value {
    // r[impl cairn.supersede.release_readiness_bundle]
    // r[impl cairn.supersede.release_readiness_bundle.schema]
    // r[impl cairn.supersede.release_readiness_bundle.local]
    // r[impl cairn.supersede.release_readiness_bundle.evidence]
    // r[impl cairn.supersede.release_readiness_bundle.failure]
    let root_text = path_string(root.clone());
    let validation = readiness_component("cairn_validate", || {
        cairn_validation_receipt(root.clone(), policy_path.clone())
    });
    let traceability = readiness_component("traceability_coverage", || {
        traceability_coverage_receipt(root.clone(), policy_path.clone())
    });
    let mcp_smoke =
        readiness_component("mcp_agent_smoke", || mcp_agent_smoke_receipt(root.clone()));
    let coverage_audit = readiness_component("determinism_coverage_audit", || {
        let policy = load_policy_file(policy_path.clone())?;
        Ok(determinism_coverage_audit_receipt(&root, &policy))
    });
    let release_evidence_index = conventional_release_evidence_index_component(root.clone());
    let preserves_corpus = preserves_corpus_component(root.clone());
    let semantic_review_ledger = semantic_review_ledger_component(root.clone(), review_ledger);
    let policy = load_policy_file(policy_path.clone()).ok();
    let stack_provenance = stack_provenance_component(
        policy_path.clone(),
        stack_provenance,
        stack_provenance_graph,
    );
    let workspace_contract_matrix = workspace_contract_matrix_component(
        root.clone(),
        policy.as_ref().map(|value| &value.raw),
        workspace_contract_matrix_receipt,
    );
    let repository_gate_summary = repository_gate_summary();
    let replay_preflight = json!({
        "command": "determinism replay --root <root>",
        "included": false,
        "required": true,
        "reason": "workspace replay may copy fixtures and is an explicit operator/CI preflight",
    });
    let coverage_audit_valid = coverage_audit.check["passed"].clone();
    let mut checks = vec![
        validation.check,
        traceability.check,
        mcp_smoke.check,
        coverage_audit.check,
        release_evidence_index.check,
        preserves_corpus.check,
        semantic_review_ledger.check,
        stack_provenance.check,
        workspace_contract_matrix.check,
    ];
    checks.push(json!({
        "name": "repository_gate_policy_listed",
        "passed": true,
        "source": "maintained local policy",
        "required_external_commands": repository_gate_summary["required_commands"].clone(),
        "evidence_hash": canonical_json_hash(&repository_gate_summary)
    }));
    let is_valid = checks
        .iter()
        .all(|check| check["passed"].as_bool().unwrap_or(false));
    with_portable_content_hash_receipt(
        &root,
        json!({
            "command": "release-readiness",
            "root": root_text,
            "valid": is_valid,
            "verdict": if is_valid { "pass" } else { "fail" },
            "checks": checks,
            "cairn_validation_hash": validation.receipt_hash,
            "traceability_coverage_hash": traceability.receipt_hash,
            "mcp_smoke_receipt_hash": mcp_smoke.receipt_hash,
            "coverage_audit_valid": coverage_audit_valid,
            "coverage_audit_receipt_hash": coverage_audit.receipt_hash,
            "release_evidence_index_check_hash": release_evidence_index.receipt_hash,
            "preserves_corpus_freshness_hash": preserves_corpus.receipt_hash,
            "semantic_review_ledger_hash": semantic_review_ledger.receipt_hash,
            "stack_provenance_hash": stack_provenance.receipt_hash,
            "stack_provenance": stack_provenance.value,
            "workspace_contract_matrix_hash": workspace_contract_matrix.receipt_hash,
            "workspace_contract_matrix": workspace_contract_matrix.value,
            "repository_gate_summary": repository_gate_summary,
            "determinism_replay": replay_preflight,
            "mcp": {"read_only": true, "destructive": false},
            "verification_role_boundary": VERIFICATION_ROLE_BOUNDARY,
        }),
    )
}

fn workspace_contract_matrix_component(
    root: PathBuf,
    policy: Option<&Value>,
    receipt_path: Option<PathBuf>,
) -> WorkspaceContractMatrixComponent {
    // r[impl cairn.workspace_contract_matrix.release_readiness]
    // r[impl cairn.workspace_contract_matrix.release_readiness.optional]
    // r[impl cairn.workspace_contract_matrix.release_readiness.required]
    let policy_value = policy.unwrap_or(&Value::Null);
    let readiness_policy = workspace_contract_readiness_policy_from_value(policy_value);
    let receipt_value = receipt_path
        .as_ref()
        .and_then(|path| fs::read_to_string(path).ok())
        .and_then(|text| serde_json::from_str::<Value>(&text).ok());
    let readiness =
        evaluate_workspace_contract_matrix_readiness(&readiness_policy, receipt_value.as_ref());
    let value = json!({
        "mode": readiness.mode,
        "required": readiness.required,
        "passed": readiness.passed,
        "skipped": readiness.skipped,
        "disposition": readiness.disposition,
        "receipt_hash": readiness.receipt_hash,
        "boundary": readiness.boundary,
        "issues": workspace_contract_readiness_issues_json(&readiness),
    });
    let receipt_hash = canonical_json_hash(&value);
    let mut check = json!({
        "name": "workspace_contract_matrix",
        "passed": readiness.passed,
        "source": component_source("workspace_contract_matrix"),
        "receipt_hash": receipt_hash,
        "disposition": value["disposition"].clone(),
    });
    if readiness.skipped {
        check["skipped"] = Value::Bool(true);
    }
    if let Some(path) = receipt_path.as_ref() {
        check["path"] = json!(safe_relative_path(&root, path));
    }
    WorkspaceContractMatrixComponent {
        check,
        receipt_hash,
        value,
    }
}

fn stack_provenance_component(
    policy_path: PathBuf,
    receipt_path: Option<PathBuf>,
    graph_path: Option<PathBuf>,
) -> StackProvenanceComponent {
    let value = stack_provenance_receipt_value(policy_path, receipt_path, graph_path);
    let receipt_hash = stack_provenance_hash(&value);
    let mut check = json!({
        "name": "stack_provenance",
        "passed": value["passed"].as_bool().unwrap_or(false),
        "source": component_source("stack_provenance"),
        "receipt_hash": receipt_hash,
        "disposition": value["disposition"].clone(),
    });
    if value["skipped"].as_bool() == Some(true) {
        check["skipped"] = Value::Bool(true);
    }
    StackProvenanceComponent {
        check,
        receipt_hash,
        value,
    }
}

fn preserves_corpus_component(root: PathBuf) -> ReadinessComponent {
    // r[impl release_evidence.preserves_readiness_summary.output]
    // r[impl release_evidence.preserves_readiness_summary.fail_closed]
    let path = root.join("fixtures/preserves-canonicalization/corpus.json");
    readiness_component("preserves_corpus_freshness", || {
        Ok(preserves_corpus_freshness_receipt(path))
    })
}

fn semantic_review_ledger_component(
    root: PathBuf,
    explicit_path: Option<PathBuf>,
) -> ReadinessComponent {
    // r[impl cairn.semantic_review.readiness_gate.release_readiness]
    // r[impl cairn.semantic_review.readiness_gate.release_readiness.supplied_valid]
    // r[impl cairn.semantic_review.readiness_gate.release_readiness.supplied_invalid]
    // r[impl cairn.semantic_review.readiness_gate.release_readiness.skipped_absent]
    // r[impl cairn.semantic_review.readiness_gate.boundary]
    // r[impl cairn.semantic_review.readiness_gate.boundary.no_reviewer_execution]
    let path = explicit_path.or_else(|| {
        let conventional = root.join("docs/semantic-review-ledger.json");
        conventional.exists().then_some(conventional)
    });
    let Some(path) = path else {
        let receipt = json!({
            "command": "review ledger",
            "valid": true,
            "verdict": "PASS",
            "skipped": true,
            "reason": "semantic review ledger not supplied and conventional docs/semantic-review-ledger.json is absent; Cairn did not run reviewers or infer semantic correctness",
        });
        let receipt_hash = canonical_json_hash(&receipt);
        return ReadinessComponent {
            check: json!({
                "name": "semantic_review_ledger",
                "passed": true,
                "source": component_source("semantic_review_ledger"),
                "skipped": true,
                "receipt_hash": receipt_hash,
            }),
            receipt_hash,
        };
    };

    match fs::read_to_string(&path)
        .map_err(|error| error.to_string())
        .and_then(|text| serde_json::from_str::<Value>(&text).map_err(|error| error.to_string()))
    {
        Ok(receipt) => {
            let is_passed = receipt["valid"].as_bool().unwrap_or(false)
                && matches!(
                    receipt["verdict"].as_str(),
                    Some("pass" | "PASS" | "approved" | "APPROVED")
                );
            let receipt_hash = receipt["receipt_hash"]
                .as_str()
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| canonical_json_hash(&receipt));
            ReadinessComponent {
                check: json!({
                    "name": "semantic_review_ledger",
                    "passed": is_passed,
                    "source": component_source("semantic_review_ledger"),
                    "path": safe_relative_path(&root, &path),
                    "receipt_hash": receipt_hash,
                    "verdict": receipt["verdict"].clone(),
                }),
                receipt_hash,
            }
        }
        Err(issue) => ReadinessComponent {
            check: json!({
                "name": "semantic_review_ledger",
                "passed": false,
                "source": component_source("semantic_review_ledger"),
                "path": safe_relative_path(&root, &path),
                "issue": issue,
            }),
            receipt_hash: String::new(),
        },
    }
}

fn conventional_release_evidence_index_component(root: PathBuf) -> ReadinessComponent {
    // r[impl release_evidence.current_head_staleness_guard.release_readiness]
    let index = root.join("docs/release-evidence-current-head.md");
    if index.exists() {
        readiness_component("release_evidence_index_freshness", || {
            Ok(release_evidence_check_index_receipt_for_path(root, index))
        })
    } else {
        let receipt = json!({
            "command": "release-evidence check-index",
            "valid": true,
            "verdict": "pass",
            "skipped": true,
            "reason": "conventional checked-in current-head evidence page is absent; checked-in evidence is treated as immutable audited payload snapshots",
        });
        ReadinessComponent {
            check: json!({
                "name": "release_evidence_index_freshness",
                "passed": true,
                "source": component_source("release_evidence_index_freshness"),
                "skipped": true,
                "receipt_hash": canonical_json_hash(&receipt),
            }),
            receipt_hash: canonical_json_hash(&receipt),
        }
    }
}

struct ReadinessComponent {
    check: Value,
    receipt_hash: String,
}

struct StackProvenanceComponent {
    check: Value,
    receipt_hash: String,
    value: Value,
}

struct WorkspaceContractMatrixComponent {
    check: Value,
    receipt_hash: String,
    value: Value,
}

fn readiness_component<F>(name: &str, operation: F) -> ReadinessComponent
where
    F: FnOnce() -> Result<Value, Box<dyn Error>>,
{
    match operation() {
        Ok(receipt) => {
            let is_passed = receipt["valid"].as_bool().unwrap_or(false)
                || receipt["verdict"].as_str() == Some("pass")
                || receipt["verdict"].as_str() == Some("PASS");
            let receipt_hash = receipt["receipt_hash"]
                .as_str()
                .map(ToOwned::to_owned)
                .unwrap_or_else(|| canonical_json_hash(&receipt));
            ReadinessComponent {
                check: json!({
                    "name": name,
                    "passed": is_passed,
                    "source": component_source(name),
                    "receipt_hash": receipt_hash,
                }),
                receipt_hash,
            }
        }
        Err(error) => ReadinessComponent {
            check: json!({
                "name": name,
                "passed": false,
                "source": component_source(name),
                "issue": error.to_string(),
            }),
            receipt_hash: String::new(),
        },
    }
}

fn component_source(name: &str) -> &'static str {
    match name {
        "cairn_validate" => "cairn validate --root <root>",
        "traceability_coverage" => "cairn traceability coverage --root <root>",
        "mcp_agent_smoke" => "cairn agent-receipt smoke-mcp --root <root>",
        "determinism_coverage_audit" => "cairn determinism coverage-audit --root <root>",
        "release_evidence_index_freshness" => {
            "cairn release-evidence check-index --root <root> --index docs/release-evidence-current-head.md"
        }
        "preserves_corpus_freshness" => {
            "cairn preserves-corpus update --path fixtures/preserves-canonicalization/corpus.json"
        }
        "semantic_review_ledger" => {
            "cairn release-readiness --root <root> --review-ledger <review-ledger.json>"
        }
        "stack_provenance" => {
            "cairn release-readiness --root <root> --stack-provenance <valence-receipt.json> --stack-provenance-graph <graph-summary.json>"
        }
        "workspace_contract_matrix" => {
            "cairn release-readiness --root <root> --workspace-contract-matrix-receipt <receipt.json>"
        }
        _ => "unknown",
    }
}
