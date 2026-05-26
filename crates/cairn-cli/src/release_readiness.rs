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

use cairn_core::canonical_json_hash;
use serde_json::{json, Value};

use crate::determinism::determinism_coverage_audit_receipt;
use crate::mcp::mcp_agent_smoke_receipt;
use crate::native::cairn_validation_receipt;
use crate::preserves_corpus::preserves_corpus_freshness_receipt;
use crate::release_evidence::release_evidence_check_index_receipt_for_path;
use crate::support::{
    default_policy_path, load_policy_file, path_string, print_json, read_option,
    read_string_option, safe_relative_path, with_portable_receipt_hash,
};
use crate::tracey::{repository_gate_summary, tracey_coverage_receipt};

pub(crate) fn run_release_readiness(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    let policy_path = read_option(args.clone(), "--policy", default_policy_path());
    let review_ledger = read_string_option(&args, "--review-ledger").map(PathBuf::from);
    print_json(release_readiness_receipt_with_review_ledger(
        root,
        policy_path,
        review_ledger,
    ))
}

pub(crate) fn release_readiness_receipt(root: PathBuf, policy_path: PathBuf) -> Value {
    release_readiness_receipt_with_review_ledger(root, policy_path, None)
}

pub(crate) fn release_readiness_receipt_with_review_ledger(
    root: PathBuf,
    policy_path: PathBuf,
    review_ledger: Option<PathBuf>,
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
    let tracey = readiness_component("tracey_coverage", || tracey_coverage_receipt(root.clone()));
    let mcp_smoke =
        readiness_component("mcp_agent_smoke", || mcp_agent_smoke_receipt(root.clone()));
    let coverage_audit = readiness_component("determinism_coverage_audit", || {
        let policy = load_policy_file(policy_path.clone())?;
        Ok(determinism_coverage_audit_receipt(&root, &policy))
    });
    let release_evidence_index = conventional_release_evidence_index_component(root.clone());
    let preserves_corpus = preserves_corpus_component(root.clone());
    let semantic_review_ledger = semantic_review_ledger_component(root.clone(), review_ledger);
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
        tracey.check,
        mcp_smoke.check,
        coverage_audit.check,
        release_evidence_index.check,
        preserves_corpus.check,
        semantic_review_ledger.check,
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
    with_portable_receipt_hash(
        &root,
        json!({
            "command": "release-readiness",
            "root": root_text,
            "valid": is_valid,
            "verdict": if is_valid { "pass" } else { "fail" },
            "checks": checks,
            "cairn_validation_hash": validation.receipt_hash,
            "tracey_coverage_hash": tracey.receipt_hash,
            "mcp_smoke_receipt_hash": mcp_smoke.receipt_hash,
            "coverage_audit_valid": coverage_audit_valid,
            "coverage_audit_receipt_hash": coverage_audit.receipt_hash,
            "release_evidence_index_check_hash": release_evidence_index.receipt_hash,
            "preserves_corpus_freshness_hash": preserves_corpus.receipt_hash,
            "semantic_review_ledger_hash": semantic_review_ledger.receipt_hash,
            "repository_gate_summary": repository_gate_summary,
            "determinism_replay": replay_preflight,
            "mcp": {"read_only": true, "destructive": false}
        }),
    )
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
        "tracey_coverage" => "cairn tracey coverage --root <root>",
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
        _ => "unknown",
    }
}
