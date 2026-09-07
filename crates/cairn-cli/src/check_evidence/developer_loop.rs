//! Developer-loop receipt producer for Cairn's local profile.
//!
//! The original `developer-loop verify` command remains the consumer. This
//! module only executes the already admitted developer-loop profile grammar and
//! publishes the receipt that the original verifier consumes.

use super::{named_options, publication::Publisher};
use crate::support::{load_policy_file, print_json};
use bounded_exec::{
    CommandSpec, Completion, Disposition, EnvironmentMode, ExecutionLimits, Input, OutcomePolicy,
    RunRequest, TerminationScope,
};
use cairn_core::{
    admit_developer_loop_policy, build_developer_loop_plan, canonical_json_hash,
    developer_loop_cache_key, DeveloperLoopCacheKeyInput, DeveloperLoopPlanStep,
};
use serde_json::{json, Value};
use std::{collections::BTreeSet, error::Error, path::Path, time::Instant};

const LOOP_RECEIPT_LEAF: &str = "developer-loop-receipt.json";
const PRODUCER_RECEIPT_LEAF: &str = "developer-loop-producer.json";
const MAX_LOOP_STDOUT_BYTES: usize = 1_048_576;
const MAX_LOOP_STDERR_BYTES: usize = 1_048_576;
const NULL_STDIN_LIMIT: usize = 1;
const POLL_INTERVAL_MS: u64 = 10;
const TEARDOWN_TIMEOUT_MS: u64 = 10_000;
const MILLIS_PER_SECOND: u128 = 1_000;
const DIGEST_HEX_BYTES: usize = 64;
const FMT_ARGV: &[&str] = &["cargo", "fmt", "--all", "--", "--check"];
const VALIDATE_ARGV: &[&str] = &["cairn", "validate", "--root", "."];
const TRACEABILITY_ARGV: &[&str] = &["cairn", "traceability", "coverage", "--root", "."];

// r[impl cairn.check_evidence_producer.conformance]
// r[impl cairn.check_evidence_producer.local_run]
pub(super) fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    if args
        .iter()
        .filter(|arg| arg.as_str() == "--execute")
        .count()
        != 1
    {
        return Err(
            "check-evidence developer-loop denied: exactly one --execute is required".into(),
        );
    }
    let pairs = args
        .iter()
        .filter(|arg| arg.as_str() != "--execute")
        .cloned()
        .collect::<Vec<_>>();
    let options = named_options(
        &pairs,
        &["--root", "--profile", "--policy", "--subject-hash", "--out"],
    )?;
    let root = std::fs::canonicalize(Path::new(options.get("--root").copied().unwrap_or(".")))
        .map_err(|_| "developer-loop root unavailable")?;
    let profile_id = options
        .get("--profile")
        .copied()
        .ok_or("check-evidence developer-loop requires --profile")?;
    let subject_hash = options
        .get("--subject-hash")
        .copied()
        .ok_or("check-evidence developer-loop requires --subject-hash")?;
    if !is_blake3_hex(subject_hash) {
        return Err("check-evidence developer-loop requires a BLAKE3 --subject-hash".into());
    }
    let policy_path = Path::new(
        options
            .get("--policy")
            .copied()
            .unwrap_or(crate::support::default_policy_path()),
    );
    let policy = load_policy_file(policy_path.to_path_buf())?;
    let policy_hash = canonical_json_hash(&policy.raw);
    let admitted = admit_developer_loop_policy(policy.developer_loop_policy.clone())
        .map_err(|issues| format!("developer-loop policy is invalid: {issues:?}"))?;
    let plan = build_developer_loop_plan(&admitted, profile_id)
        .map_err(|issues| format!("developer-loop profile is not executable: {issues:?}"))?;
    if profile_id != plan.profile {
        return Err("developer-loop profile identity changed during planning".into());
    }
    let mut publisher = Publisher::open(Path::new(
        options
            .get("--out")
            .copied()
            .ok_or("check-evidence developer-loop requires --out")?,
    ))
    .map_err(|_| "developer-loop publication root unavailable")?;

    let started = Instant::now();
    let mut statuses = Vec::new();
    let mut facts = Vec::new();
    for step in &plan.steps {
        let output = bounded_exec::run(request_for_step(step, &root, plan.budget.max_seconds)?);
        let status = status_for_output(step, &output)?;
        facts.push(facts_for_output(step, &output));
        statuses.push(json!({
            "check_id": step.check_id,
            "status": status,
            "command_identity": step.command_identity,
        }));
    }
    let elapsed_ms = started.elapsed().as_millis();
    let observed_duration_seconds = u64::try_from(elapsed_ms.div_ceil(MILLIS_PER_SECOND))?;
    let all_passed = statuses.iter().all(|status| status["status"] == "passed");
    let dependencies = BTreeSet::new();
    let cache_key_identity = developer_loop_cache_key(&DeveloperLoopCacheKeyInput {
        profile_id: &plan.profile,
        subject_hash,
        dependencies: &dependencies,
        target: &plan.target,
        toolchain: &plan.toolchain,
        policy_hash: &policy_hash,
        checks: &plan
            .steps
            .iter()
            .map(|step| step.check_id.clone())
            .collect::<Vec<_>>(),
        key_inputs: &plan.cache_policy.key_inputs,
    });
    let loop_receipt = crate::support::with_receipt_hash(json!({
        "schema": "cairn.developer-loop-receipt.v1",
        "valid": all_passed,
        "profile": plan.profile,
        "policy_hash": policy_hash,
        "subject_hash": subject_hash,
        "target": plan.target,
        "toolchain": plan.toolchain,
        "observed_duration_seconds": observed_duration_seconds,
        "measurement_class": "wall",
        "result_statuses": statuses,
        "cache": "miss",
        "cache_key_identity": cache_key_identity,
        "posture_states": ["declared"],
        "full_gate_status": "outstanding",
        "issues": if all_passed { Vec::<String>::new() } else { vec!["developer_loop.check.not_passing".to_string()] },
        "non_claims": plan.non_claims,
        "producer": {
            "schema": "cairn.check-evidence-developer-loop-producer.v1",
            "step_facts": facts,
            "plan_hash": plan.plan_hash,
            "publication": "durable-file-publication.no-replace"
        }
    }));
    let loop_bytes = serde_json::to_vec(&loop_receipt)?;
    publisher.publish(LOOP_RECEIPT_LEAF, &loop_bytes)?;
    let producer_receipt = crate::support::with_receipt_hash(json!({
        "schema": "cairn.check-evidence-developer-loop-run.v1",
        "valid": all_passed,
        "profile": profile_id,
        "policy_hash": policy_hash,
        "subject_hash": subject_hash,
        "receipt": LOOP_RECEIPT_LEAF,
        "loop_receipt_hash": blake3::hash(&loop_bytes).to_hex().to_string(),
        "checks_executed": true,
        "observed_duration_seconds": observed_duration_seconds,
        "publication_performed": true,
        "non_claims": [
            "developer-loop production runs only admitted profile argv and does not prove source correctness",
            "declared read-only posture does not prove command hermeticity, network denial, mutation absence, or clock accuracy",
            "the original developer-loop verifier remains the consumer authority"
        ]
    }));
    let producer_bytes = serde_json::to_vec(&producer_receipt)?;
    publisher.publish(PRODUCER_RECEIPT_LEAF, &producer_bytes)?;
    print_json(producer_receipt)
}

fn request_for_step(
    step: &DeveloperLoopPlanStep,
    root: &Path,
    budget_seconds: u64,
) -> Result<RunRequest, Box<dyn Error>> {
    if step.working_root != "." {
        return Err("developer-loop producer only admits repository-root steps".into());
    }
    let (program, args) = resolved_argv(step)?;
    Ok(RunRequest {
        command: CommandSpec {
            program: program.into(),
            args: args.into_iter().map(Into::into).collect(),
            current_dir: root.into(),
            environment_mode: EnvironmentMode::Inherit,
            environment: Vec::new(),
            input: Input::Null,
        },
        limits: ExecutionLimits {
            timeout_ms: budget_seconds.saturating_mul(u64::try_from(MILLIS_PER_SECOND)?),
            stdin_max_bytes: NULL_STDIN_LIMIT,
            stdout_max_bytes: MAX_LOOP_STDOUT_BYTES,
            stderr_max_bytes: MAX_LOOP_STDERR_BYTES,
            poll_interval_ms: POLL_INTERVAL_MS,
            teardown_timeout_ms: TEARDOWN_TIMEOUT_MS,
        },
        termination_scope: TerminationScope::ProcessGroup,
        outcome_policy: OutcomePolicy::new(vec![0], true, true)
            .map_err(|_| "developer-loop outcome policy rejected")?,
    })
}

fn resolved_argv(step: &DeveloperLoopPlanStep) -> Result<(String, Vec<String>), Box<dyn Error>> {
    let expected = match step.check_id.as_str() {
        "fmt" => FMT_ARGV,
        "validate" => VALIDATE_ARGV,
        "traceability" => TRACEABILITY_ARGV,
        _ => return Err("developer-loop producer rejected an unsupported check".into()),
    };
    let actual = step.argv.iter().map(String::as_str).collect::<Vec<_>>();
    if actual.as_slice() != expected {
        return Err("developer-loop producer rejected a non-profile command grammar".into());
    }
    let program = match step.argv[0].as_str() {
        "cairn" => std::env::current_exe()?.to_string_lossy().to_string(),
        "cargo" => resolve_cargo()?,
        _ => return Err("developer-loop producer rejected an unsupported program".into()),
    };
    Ok((program, step.argv.iter().skip(1).cloned().collect()))
}

fn resolve_cargo() -> Result<String, Box<dyn Error>> {
    if let Ok(cargo) = std::env::var("CARGO") {
        if !cargo.is_empty() {
            return Ok(cargo);
        }
    }
    let path = std::env::var_os("PATH").ok_or("developer-loop producer cannot find cargo")?;
    for directory in std::env::split_paths(&path) {
        let candidate = directory.join("cargo");
        if candidate.is_file() {
            return Ok(candidate.to_string_lossy().to_string());
        }
    }
    Err("developer-loop producer cannot find cargo".into())
}

fn status_for_output(
    step: &DeveloperLoopPlanStep,
    output: &Result<bounded_exec::ExecutionOutput, bounded_exec::RunError>,
) -> Result<&'static str, Box<dyn Error>> {
    let expected_success = match step.expected_status_class.as_str() {
        "success" => true,
        "failure" => false,
        _ => return Err("developer-loop producer rejected an unsupported status class".into()),
    };
    let matched = match output {
        Ok(output) => match output.disposition {
            Disposition::Succeeded => expected_success,
            Disposition::ExitFailed => !expected_success,
            Disposition::TimedOut
            | Disposition::Cancelled
            | Disposition::OutputLimitExceeded(_) => false,
        },
        Err(_) => false,
    };
    Ok(if matched { "passed" } else { "failed" })
}

fn facts_for_output(
    step: &DeveloperLoopPlanStep,
    output: &Result<bounded_exec::ExecutionOutput, bounded_exec::RunError>,
) -> Value {
    match output {
        Ok(output) => json!({
            "check_id": step.check_id,
            "command_identity": step.command_identity,
            "disposition": format!("{:?}", output.disposition),
            "completion": output.completion == Completion::Exited,
            "exit_code": output.exit_code,
            "signal": output.signal,
            "stdout_hash": blake3::hash(&output.stdout.bytes).to_hex().to_string(),
            "stderr_hash": blake3::hash(&output.stderr.bytes).to_hex().to_string(),
            "stdout_observed_bytes": output.stdout.observed_bytes,
            "stderr_observed_bytes": output.stderr.observed_bytes,
            "stdout_truncated": output.stdout.truncated,
            "stderr_truncated": output.stderr.truncated,
        }),
        Err(_) => json!({
            "check_id": step.check_id,
            "command_identity": step.command_identity,
            "disposition": "runner_error",
        }),
    }
}

fn is_blake3_hex(value: &str) -> bool {
    value.len() == DIGEST_HEX_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}
