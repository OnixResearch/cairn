//! Linux local execution adapter. Bounded Exec owns child mechanics.
use crate::support::{CapabilityRoot, CapabilityRootKind};
use bounded_exec::{
    CommandSpec, Completion, Disposition, EnvironmentMode, ExecutionLimits, Input, OutcomePolicy,
    RunRequest, TerminationScope,
};
use cairn_core::verified::check_evidence::*;
use cairn_core::{authoritative_command_hash, canonical_json_hash};
use serde_json::{json, Value};
use std::{
    error::Error,
    fs::File,
    io::Read,
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
    time::Duration,
};

pub(super) const PRODUCER: &str = LOCAL_EXECUTOR_REVISION;
pub(super) const CANCEL_BYTES: u64 = 1;
const MAX_SOURCE_FILES: usize = 4096;
const MAX_SOURCE_FILE_BYTES: u64 = 16 * 1024 * 1024;
const MAX_SOURCE_TOTAL_BYTES: usize = 64 * 1024 * 1024;
const MAX_EXECUTABLE_BYTES: usize = 256 * 1024 * 1024;
const HASH_BUFFER_BYTES: usize = 8192;
const CONTROL_POLL_MS: u64 = 10;
const NULL_STDIN_LIMIT: usize = 1;

pub(super) fn source_hash(
    root: &CapabilityRoot,
    paths: &[String],
) -> Result<String, Box<dyn Error>> {
    if paths.is_empty() || paths.len() > MAX_SOURCE_FILES || !paths.windows(2).all(|p| p[0] < p[1])
    {
        return Err("source projection must contain a bounded sorted unique file set".into());
    }
    let mut rows = Vec::new();
    let mut total = 0_usize;
    for path in paths {
        let relative = root
            .relative(Path::new(path))
            .map_err(|_| "unsafe source projection")?;
        let mut prefix = std::path::PathBuf::new();
        for component in relative.components() {
            prefix.push(component);
            if root
                .symlink_metadata(&prefix)
                .map_err(|_| "source unavailable")?
                .is_symlink()
            {
                return Err("source projection rejects symbolic links".into());
            }
        }
        if !root
            .symlink_metadata(&relative)
            .map_err(|_| "source unavailable")?
            .is_file()
        {
            return Err("source projection requires regular files".into());
        }
        let bytes = super::reads::input(root, &relative, MAX_SOURCE_FILE_BYTES)
            .map_err(|_| "source unavailable or oversized")?;
        total = total
            .checked_add(bytes.len())
            .ok_or("source size overflow")?;
        if total > MAX_SOURCE_TOTAL_BYTES {
            return Err("source projection exceeds aggregate bound".into());
        }
        rows.push(json!([path, blake3::hash(&bytes).to_hex().to_string()]));
    }
    Ok(canonical_json_hash(
        &json!({"schema": "cairn.check-source.v1", "files": rows}),
    ))
}

pub(super) fn executable_hash(path: &str) -> Result<String, Box<dyn Error>> {
    // Only immutable Nix-store executables pass core profile admission.
    let mut file = File::open(path).map_err(|_| "executable unavailable")?;
    if !file.metadata()?.is_file() {
        return Err("executable is not a regular file".into());
    }
    let mut hash = blake3::Hasher::new();
    let mut buffer = [0_u8; HASH_BUFFER_BYTES];
    let mut total = 0_usize;
    loop {
        let count = file
            .read(&mut buffer)
            .map_err(|_| "executable read failed")?;
        if count == 0 {
            break;
        }
        total = total.checked_add(count).ok_or("executable size overflow")?;
        if total > MAX_EXECUTABLE_BYTES {
            return Err("executable exceeds byte bound".into());
        }
        hash.update(&buffer[..count]);
    }
    Ok(hash.finalize().to_hex().to_string())
}

pub(super) fn request(profile: &CheckProfile, cwd: &Path) -> Result<RunRequest, Box<dyn Error>> {
    Ok(RunRequest {
        command: CommandSpec {
            program: profile.executable.clone().into(),
            args: profile_command(profile)
                .into_iter()
                .skip(1)
                .map(Into::into)
                .collect(),
            current_dir: cwd.into(),
            environment_mode: EnvironmentMode::Clear,
            environment: profile
                .environment
                .iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            input: Input::Null,
        },
        limits: ExecutionLimits {
            timeout_ms: profile.bounds.deadline_ms,
            stdin_max_bytes: NULL_STDIN_LIMIT,
            stdout_max_bytes: usize::try_from(profile.bounds.stdout_bytes)?,
            stderr_max_bytes: usize::try_from(profile.bounds.stderr_bytes)?,
            poll_interval_ms: profile.bounds.deadline_ms.min(CONTROL_POLL_MS),
            teardown_timeout_ms: profile.bounds.teardown_ms,
        },
        termination_scope: TerminationScope::ProcessGroup,
        outcome_policy: OutcomePolicy::new(vec![0], true, true)
            .map_err(|_| "invalid outcome policy")?,
    })
}

pub(super) fn execute(
    request: RunRequest,
    cancel_root: &CapabilityRoot,
    cancel: &str,
) -> Result<bounded_exec::ExecutionOutput, bounded_exec::RunError> {
    let cancel_relative = cancel_root.relative(Path::new(cancel)).ok();
    let cancel_directory = cancel_root.clone_directory_file().ok();
    let stopped = AtomicBool::new(false);
    let cancelled = AtomicBool::new(false);
    std::thread::scope(|scope| {
        let stopped_ref = &stopped;
        let cancelled_ref = &cancelled;
        scope.spawn(move || {
            let (Some(cancel_relative), Some(cancel_directory)) =
                (cancel_relative, cancel_directory)
            else {
                cancelled_ref.store(true, Ordering::Release);
                return;
            };
            let cancel_root = cap_std::fs::Dir::from_std_file(cancel_directory);
            while !stopped_ref.load(Ordering::Acquire) {
                // The control is explicit and capability-relative. Any nonzero byte cancels.
                match super::reads::read(&cancel_root, &cancel_relative, CANCEL_BYTES) {
                    Ok(bytes) if bytes == b"0" => {}
                    _ => {
                        cancelled_ref.store(true, Ordering::Release);
                        break;
                    }
                }
                std::thread::sleep(Duration::from_millis(CONTROL_POLL_MS));
            }
        });
        let result = bounded_exec::run_with_cancellation(request, &cancelled);
        stopped.store(true, Ordering::Release);
        result
    })
}

pub(super) fn observation(
    plan: &CheckEvidencePlan,
    profile: &CheckProfile,
    attempt: &str,
    source_after: String,
    output: Result<bounded_exec::ExecutionOutput, bounded_exec::RunError>,
) -> (AttemptObservation, Value) {
    // A runner error can precede spawn; it cannot establish execution coverage.
    let execution_observed = output.is_ok();
    let (outcome, payload, tests, teardown, facts) = match output {
        Ok(output) => {
            let outcome = match output.disposition {
                Disposition::Succeeded => ProcessOutcome::ExitedZero,
                Disposition::ExitFailed => ProcessOutcome::ExitedNonzero,
                Disposition::TimedOut => ProcessOutcome::TimedOut,
                Disposition::Cancelled => ProcessOutcome::Cancelled,
                Disposition::OutputLimitExceeded(_) => ProcessOutcome::OutputOverflow,
            };
            let coverage = cargo_coverage(&output.stdout.bytes);
            let payload = match profile.kind {
                CheckKind::CargoTest => coverage.passed,
                // Nix exit zero is not a separately admitted build payload.
                // Until a native payload contract is available, retain the process fact only.
                CheckKind::NixCheck => false,
            };
            let facts = json!({"outcome": outcome, "exit_code": output.exit_code, "signal": output.signal,
                "stdout_hash": blake3::hash(&output.stdout.bytes).to_hex().to_string(),
                "stderr_hash": blake3::hash(&output.stderr.bytes).to_hex().to_string(),
                "stdout_observed_bytes": output.stdout.observed_bytes, "stderr_observed_bytes": output.stderr.observed_bytes,
                "stdout_truncated": output.stdout.truncated, "stderr_truncated": output.stderr.truncated,
                "natural_exit": output.completion == Completion::Exited});
            (outcome, payload, coverage.executed, true, facts)
        }
        Err(_) => (
            ProcessOutcome::TeardownFailed,
            false,
            0,
            false,
            json!({"outcome":"infrastructure_or_teardown_unknown"}),
        ),
    };
    let evidence = json!({"schema":"cairn.local-check-facts.v1", "producer_revision": PRODUCER,
        "attempt_hash": attempt, "plan_hash": plan.identity(), "check_id": profile.check_id, "facts": facts,
        "payload_passed":payload, "executed_tests":tests});
    let observation = AttemptObservation {
        check_id: profile.check_id.clone(),
        attempt_hash: attempt.into(),
        plan_hash: plan.identity().into(),
        applicability: plan.applicability().clone(),
        source_after_hash: source_after,
        executable_hash: profile.executable_hash.clone(),
        tool_cohort_hash: profile.tool_cohort_hash.clone(),
        command_hash: authoritative_command_hash(&profile_command(profile)),
        producer_identity: blake3::hash(PRODUCER.as_bytes()).to_hex().to_string(),
        native_status: if outcome == ProcessOutcome::ExitedZero && payload {
            "passed"
        } else {
            "non_passing"
        }
        .into(),
        native_non_claims: vec![
            "local summaries do not prove test-oracle truth or sandboxing".into(),
        ],
        role: match profile.kind {
            CheckKind::CargoTest => EvidenceRole::Test,
            CheckKind::NixCheck => EvidenceRole::Build,
        },
        outcome,
        payload_passed: payload,
        executed_tests: tests,
        execution_observed,
        cache_claimed: false,
        teardown_complete: teardown,
        evidence_hash: canonical_json_hash(&evidence),
    };
    (observation, evidence)
}

pub(super) fn open_root(path: &Path) -> Result<CapabilityRoot, Box<dyn Error>> {
    CapabilityRoot::open(CapabilityRootKind::Repository, path)
        .map_err(|_| "check source root unavailable".into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        os::unix::{fs::symlink, net::UnixListener},
    };

    #[test]
    fn source_projection_accepts_regular_files_and_rejects_unsafe_members(
    ) -> Result<(), Box<dyn Error>> {
        let path =
            std::env::temp_dir().join(format!("cairn-producer-source-{}", std::process::id()));
        if path.exists() {
            fs::remove_dir_all(&path)?;
        }
        fs::create_dir_all(path.join("nested"))?;
        fs::write(path.join("nested/source"), b"source")?;
        symlink("nested", path.join("linked"))?;
        symlink("nested/source", path.join("leaf"))?;
        let _socket = UnixListener::bind(path.join("socket"))?;
        let root = open_root(&path)?;
        let paths = vec!["nested/source".into()];
        let before = source_hash(&root, &paths)?;
        assert_eq!(before, source_hash(&root, &paths)?);
        fs::write(path.join("nested/source"), b"changed")?;
        assert_ne!(before, source_hash(&root, &paths)?);
        for member in [
            "linked/source",
            "leaf",
            "nested",
            "socket",
            "../source",
            "absent",
        ] {
            assert!(
                source_hash(&root, &[member.into()]).is_err(),
                "admitted {member}"
            );
        }
        assert!(source_hash(&root, &[]).is_err());
        assert!(source_hash(&root, &[paths[0].clone(), paths[0].clone()]).is_err());
        let oversized = fs::File::create(path.join("oversized"))?;
        oversized.set_len(MAX_SOURCE_FILE_BYTES + 1)?;
        assert!(source_hash(&root, &["oversized".into()]).is_err());
        fs::remove_dir_all(path)?;
        Ok(())
    }
}
