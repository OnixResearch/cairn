// r[impl test_harness.receipts_replay.receipts] r[impl test_harness.receipts_replay.receipts.schema]
// r[impl test_harness.receipts_replay.receipts.portable_hash] r[impl test_harness.receipts_replay.replay]
// r[impl test_harness.receipts_replay.replay.failure_command] r[impl test_harness.receipts_replay.replay.regression_fixture]
// r[impl test_harness.failure_taxonomy.classification] r[impl test_harness.failure_taxonomy.classification.product]
// r[impl test_harness.failure_taxonomy.classification.harness] r[impl test_harness.failure_taxonomy.classification.environment]
// r[impl test_harness.failure_taxonomy.classification.flake] r[impl test_harness.failure_taxonomy.usability]
// r[impl test_harness.failure_taxonomy.usability.summary] r[impl test_harness.fixture_artifact_integrity.artifacts]
// r[impl test_harness.fixture_artifact_integrity.artifacts.existence] r[impl test_harness.fixture_artifact_integrity.artifacts.schema_hash]
// r[impl test_harness.fixture_artifact_integrity.artifacts.corrupt] r[impl test_harness.fixture_artifact_integrity.fixtures]
// r[impl test_harness.fixture_artifact_integrity.fixtures.canonical] r[impl test_harness.fixture_artifact_integrity.fixtures.golden_update]
// r[impl test_harness.rail_split_selftest.rails] r[impl test_harness.rail_split_selftest.rails.fast]
// r[impl test_harness.rail_split_selftest.rails.full] r[impl test_harness.rail_split_selftest.self_tests]
// r[impl test_harness.rail_split_selftest.self_tests.intentional_failure] r[impl test_harness.rail_split_selftest.self_tests.intentional_success]
// r[impl test_harness.flake_triage.rerun] r[impl test_harness.flake_triage.rerun.bounded]
// r[impl test_harness.flake_triage.rerun.not_greenwash] r[impl test_harness.flake_triage.promotion]
// r[impl test_harness.flake_triage.promotion.evidence] r[impl test_harness.flake_triage.promotion.fixture]
// r[impl test_harness.real_rails.receipts] r[impl test_harness.real_rails.receipts.validate]
// r[impl test_harness.real_rails.receipts.tracey] r[impl test_harness.real_rails.receipts.release_readiness]
// r[impl test_harness.real_rails.classification] r[impl test_harness.real_rails.classification.product]
// r[impl test_harness.real_rails.classification.environment] r[impl test_harness.real_rails.selection]
// r[impl test_harness.real_rails.selection.known] r[impl test_harness.real_rails.selection.unknown]
use crate::{read_option, read_string_option, safe_relative_path, with_portable_receipt_hash};
use cairn_core::blake3_hex;
use serde_json::{json, Value};
use std::{error::Error, fs, path::Path, process::Command};
const ARTIFACT_SCHEMA: &str = "cairn.test-harness.artifact.v1";
#[derive(Debug, Clone, PartialEq, Eq)]
enum Outcome {
    Passed,
    Failed,
    Flaky,
}
impl Outcome {
    fn s(&self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Flaky => "flaky_or_timeout",
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
enum Class {
    None,
    Product,
    Harness,
    Environment,
    Flaky,
}
impl Class {
    fn s(&self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Product => "product",
            Self::Harness => "harness",
            Self::Environment => "environment",
            Self::Flaky => "flaky_or_timeout",
        }
    }
}
struct Report {
    outcome: Outcome,
    class: Class,
    code: &'static str,
    diag: &'static str,
    attempts: Vec<Value>,
}
pub(crate) fn run_test_harness(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let root = read_option(args.clone(), "--root", ".");
    if let Some(rail) = read_string_option(&args, "--rail") {
        return run_real_rail(&root, &rail, read_string_option(&args, "--out"));
    }
    let case = read_string_option(&args, "--case").unwrap_or_else(|| "pass".into());
    let seed = read_string_option(&args, "--seed").unwrap_or_else(|| "0".into());
    let max = read_string_option(&args, "--max-reruns")
        .and_then(|x| x.parse::<usize>().ok())
        .unwrap_or(2);
    let out = read_string_option(&args, "--out");
    let artifact = root
        .join("target/cairn-test-harness")
        .join(format!("{case}-{seed}.json"));
    let report = case_report(&root, &case, &seed, max, &artifact)?;
    let bytes = fs::read(&artifact).unwrap_or_default();
    let rel = safe_relative_path(&root, &artifact);
    let receipt = with_portable_receipt_hash(
        &root,
        json!({"schema":"cairn.test-harness.receipt.v1","command":"test-harness","case_id":case,"seed":seed,"root":".","outcome":report.outcome.s(),"failure_class":report.class.s(),"issue_code":report.code,"diagnostic":report.diag,"replay_command":format!("cairn test-harness --root . --case {case} --seed {seed}"),"artifact":{"path":rel,"schema":artifact_schema(&bytes),"blake3":blake3_hex(&bytes),"bytes":bytes.len()},"retry_policy":{"max_reruns":max,"same_case_identity":true},"attempts":report.attempts}),
    );
    let rendered = serde_json::to_string_pretty(&receipt)?;
    if let Some(out) = out {
        let p = root.join(out);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(p, format!("{rendered}\n"))?;
    }
    println!("{rendered}");
    if report.outcome == Outcome::Passed {
        Ok(())
    } else {
        Err(format!("test harness case failed: {}", report.code).into())
    }
}
fn run_real_rail(root: &Path, rail: &str, out: Option<String>) -> Result<(), Box<dyn Error>> {
    let report = real_rail_report(root, rail);
    let receipt = with_portable_receipt_hash(
        root,
        json!({
            "schema":"cairn.test-harness.rail-receipt.v1",
            "command":"test-harness",
            "rail_id":rail,
            "root":".",
            "outcome":report.outcome.s(),
            "failure_class":report.class.s(),
            "issue_code":report.code,
            "diagnostic":report.diag,
            "replay_command":format!("cairn test-harness --root . --rail {rail}"),
            "rail":report.attempts.first().cloned().unwrap_or_else(|| json!({"rail_id":rail})),
        }),
    );
    let rendered = serde_json::to_string_pretty(&receipt)?;
    if let Some(out) = out {
        let p = root.join(out);
        if let Some(parent) = p.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(p, format!("{rendered}\n"))?;
    }
    println!("{rendered}");
    if report.outcome == Outcome::Passed {
        Ok(())
    } else {
        Err(format!("test harness rail failed: {}", report.code).into())
    }
}

fn real_rail_report(root: &Path, rail: &str) -> Report {
    let Some(args) = rail_args(rail) else {
        return Report {
            outcome: Outcome::Failed,
            class: Class::Harness,
            code: "unknown_harness_rail",
            diag: "choose a supported real rail id",
            attempts: vec![json!({"rail_id":rail,"status":"not_launched"})],
        };
    };
    let exe = match std::env::current_exe() {
        Ok(exe) => exe,
        Err(_) => {
            return Report {
                outcome: Outcome::Failed,
                class: Class::Environment,
                code: "rail_launcher_unavailable",
                diag: "current executable path is unavailable",
                attempts: vec![json!({"rail_id":rail,"status":"not_launched"})],
            };
        }
    };
    let output = Command::new(exe).args(&args).current_dir(root).output();
    match output {
        Ok(output) => rail_output_report(
            rail,
            &args,
            output.status.code(),
            &output.stdout,
            &output.stderr,
        ),
        Err(_) => Report {
            outcome: Outcome::Failed,
            class: Class::Environment,
            code: "rail_launch_failed",
            diag: "the selected rail could not be launched in this environment",
            attempts: vec![json!({"rail_id":rail,"argv":args,"status":"launch_failed"})],
        },
    }
}

fn rail_output_report(
    rail: &str,
    args: &[String],
    exit_code: Option<i32>,
    stdout: &[u8],
    stderr: &[u8],
) -> Report {
    let passed = exit_code == Some(0);
    Report {
        outcome: if passed {
            Outcome::Passed
        } else {
            Outcome::Failed
        },
        class: if passed { Class::None } else { Class::Product },
        code: if passed { "ok" } else { "rail_failed" },
        diag: if passed {
            "real verification rail passed"
        } else {
            "debug the selected verification rail output before changing harness plumbing"
        },
        attempts: vec![json!({
            "rail_id":rail,
            "argv":args,
            "status":if passed {"passed"} else {"failed"},
            "exit_code":exit_code,
            "stdout_blake3":blake3_hex(stdout),
            "stderr_blake3":blake3_hex(stderr),
            "stdout_bytes":stdout.len(),
            "stderr_bytes":stderr.len(),
        })],
    }
}

fn rail_args(rail: &str) -> Option<Vec<String>> {
    let args = match rail {
        "validate" => vec!["validate", "--root", "."],
        "tracey-coverage" => vec!["tracey", "coverage", "--root", "."],
        "release-readiness" => vec!["release-readiness", "--root", "."],
        "policy-export-check" => vec!["policy", "export", "--root", ".", "--check"],
        _ => return None,
    };
    Some(args.into_iter().map(str::to_string).collect())
}

fn case_report(
    root: &Path,
    case: &str,
    seed: &str,
    max: usize,
    artifact: &Path,
) -> Result<Report, Box<dyn Error>> {
    if let Some(parent) = artifact.parent() {
        fs::create_dir_all(parent)?;
    }
    match case {
        "pass" => {
            write_artifact(artifact, case, seed, "ok")?;
            Ok(Report {
                outcome: Outcome::Passed,
                class: Class::None,
                code: "ok",
                diag: "harness case passed and artifact integrity verified",
                attempts: vec![json!({"attempt":1,"status":"passed"})],
            })
        }
        "product-fail" => {
            write_artifact(artifact, case, seed, "product_assertion_failed")?;
            Ok(Report {
                outcome: Outcome::Failed,
                class: Class::Product,
                code: "product_invariant_failed",
                diag: "debug the product invariant before changing harness plumbing",
                attempts: vec![json!({"attempt":1,"status":"failed"})],
            })
        }
        "harness-fail" => {
            fs::write(artifact, b"not-json\n")?;
            Ok(Report {
                outcome: Outcome::Failed,
                class: Class::Harness,
                code: "harness_artifact_invalid",
                diag: "debug the harness artifact contract",
                attempts: vec![json!({"attempt":1,"status":"failed"})],
            })
        }
        "environment-fail" => Ok(Report {
            outcome: Outcome::Failed,
            class: Class::Environment,
            code: "environment_precondition_missing",
            diag: "satisfy the local precondition before debugging product behavior",
            attempts: vec![
                json!({"attempt":1,"status":"failed","precondition":safe_relative_path(root,&root.join("target/cairn-test-harness"))}),
            ],
        }),
        "timeout-flake" => {
            write_artifact(artifact, case, seed, "timeout_then_passed")?;
            let attempts = (0..=max)
                .map(|i| json!({"attempt":i+1,"status":if i==0{"timeout"}else{"passed"}}))
                .collect();
            Ok(Report {
                outcome: Outcome::Flaky,
                class: Class::Flaky,
                code: "flaky_or_timeout_reproduced",
                diag: "promote this case identity to a regression fixture if it repeats",
                attempts,
            })
        }
        _ => Ok(Report {
            outcome: Outcome::Failed,
            class: Class::Harness,
            code: "unknown_harness_case",
            diag: "choose a supported case id",
            attempts: vec![json!({"attempt":1,"status":"failed"})],
        }),
    }
}
fn write_artifact(path: &Path, case: &str, seed: &str, status: &str) -> Result<(), Box<dyn Error>> {
    fs::write(
        path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(
                &json!({"schema":ARTIFACT_SCHEMA,"case_id":case,"seed":seed,"status":status})
            )?
        ),
    )?;
    Ok(())
}
fn artifact_schema(bytes: &[u8]) -> &'static str {
    serde_json::from_slice::<Value>(bytes)
        .ok()
        .and_then(|v| v.get("schema").and_then(Value::as_str).map(str::to_string))
        .filter(|s| s == ARTIFACT_SCHEMA)
        .map(|_| ARTIFACT_SCHEMA)
        .unwrap_or("invalid")
}
#[cfg(test)]
mod tests {
    use super::*;
    fn root(n: &str) -> std::path::PathBuf {
        std::env::temp_dir().join(format!("cairn-harness-{n}-{}", std::process::id()))
    }
    #[test]
    fn pass_case() {
        let r = root("pass");
        let a = r.join("a.json");
        let report = case_report(&r, "pass", "1", 1, &a).expect("case");
        assert_eq!(report.outcome, Outcome::Passed);
        assert_eq!(
            artifact_schema(&fs::read(&a).expect("artifact")),
            ARTIFACT_SCHEMA
        );
        fs::remove_dir_all(r).ok();
    }
    #[test]
    fn product_class() {
        let r = root("prod");
        let a = r.join("a.json");
        let report = case_report(&r, "product-fail", "1", 1, &a).expect("case");
        assert_eq!(report.class, Class::Product);
        fs::remove_dir_all(r).ok();
    }
    #[test]
    fn corrupt_artifact() {
        let r = root("harness");
        let a = r.join("a.json");
        let report = case_report(&r, "harness-fail", "1", 1, &a).expect("case");
        assert_eq!(report.class, Class::Harness);
        assert_eq!(artifact_schema(&fs::read(&a).expect("artifact")), "invalid");
        fs::remove_dir_all(r).ok();
    }
    #[test]
    fn known_rail_args_are_bounded() {
        assert_eq!(
            rail_args("validate").expect("validate"),
            vec!["validate", "--root", "."]
        );
        assert_eq!(
            rail_args("tracey-coverage").expect("tracey"),
            vec!["tracey", "coverage", "--root", "."]
        );
        assert!(rail_args("arbitrary-shell").is_none());
    }
    #[test]
    fn rail_output_classifies_product_failure() {
        let args = rail_args("validate").expect("validate");
        let report = rail_output_report("validate", &args, Some(2), b"bad", b"err");
        assert_eq!(report.outcome, Outcome::Failed);
        assert_eq!(report.class, Class::Product);
        assert_eq!(report.code, "rail_failed");
    }
    #[test]
    fn unknown_rail_is_harness_failure() {
        let r = root("unknown-rail");
        let report = real_rail_report(&r, "not-a-rail");
        assert_eq!(report.class, Class::Harness);
        assert_eq!(report.code, "unknown_harness_rail");
        fs::remove_dir_all(r).ok();
    }
    #[test]
    fn flake_not_green() {
        let r = root("flake");
        let a = r.join("a.json");
        let report = case_report(&r, "timeout-flake", "1", 2, &a).expect("case");
        assert_eq!(report.outcome, Outcome::Flaky);
        assert_eq!(report.attempts.len(), 3);
        fs::remove_dir_all(r).ok();
    }
}
