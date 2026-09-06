use super::{local, named_options, publication::Publisher, read_input};
use crate::support::{load_policy_file, print_json};
use cairn_core::verified::check_evidence::*;
use cairn_core::{canonical_json_hash, AgentVerificationContract};
use serde_json::json;
use std::{collections::BTreeMap, error::Error, path::Path};

const ATTEMPT_DIGEST_HEX_BYTES: usize = 64;

#[derive(Default)]
struct AttemptState {
    attempts: Vec<NormalizedAttempt>,
    observations: Vec<AttemptObservation>,
    published: Vec<PublishedArtifact>,
    artifacts: BTreeMap<String, String>,
    blockers: Vec<&'static str>,
}

pub(super) fn source(args: &[String]) -> Result<(), Box<dyn Error>> {
    let options = named_options(args, &["--root", "--projection"])?;
    let root = local::open_root(Path::new(options.get("--root").copied().unwrap_or(".")))?;
    let paths: Vec<String> = read_input(
        &root,
        options.get("--projection").ok_or("missing --projection")?,
    )?;
    print_json(
        json!({"schema":"cairn.check-source-receipt.v1", "source_projection_hash":canonical_json_hash(&json!(paths)),
        "subject_hash":local::source_hash(&root, &paths)?, "checks_executed":false,
        "non_claims":["the selected file set does not prove source closure completeness"]}),
    )
}

// r[impl cairn.check_evidence_producer.local_run]
pub(super) fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    // Denial precedes all reads and protected effects.
    if args.iter().filter(|a| a.as_str() == "--execute").count() != 1 {
        return Err("check-evidence run denied: exactly one --execute is required".into());
    }
    let pairs: Vec<_> = args
        .iter()
        .filter(|a| a.as_str() != "--execute")
        .cloned()
        .collect();
    let opts = named_options(
        &pairs,
        &[
            "--root",
            "--contract",
            "--profiles",
            "--applicability",
            "--policy",
            "--projection",
            "--plan-hash",
            "--attempt",
            "--out",
            "--cancel-file",
        ],
    )?;
    let required = |name| {
        opts.get(name)
            .copied()
            .ok_or("check-evidence run missing required option")
    };
    let root = local::open_root(Path::new(opts.get("--root").copied().unwrap_or(".")))?;
    let contract: AgentVerificationContract = read_input(&root, required("--contract")?)?;
    let profiles: Vec<CheckProfile> = read_input(&root, required("--profiles")?)?;
    let applicability: Applicability = read_input(&root, required("--applicability")?)?;
    let paths: Vec<String> = read_input(&root, required("--projection")?)?;
    let policy = load_policy_file(
        opts.get("--policy")
            .copied()
            .unwrap_or(crate::support::default_policy_path())
            .into(),
    )?;
    if applicability.policy_hash != canonical_json_hash(&policy.raw) {
        return Err("check-evidence policy identity is stale".into());
    }
    if !applicability.dependency_hashes.is_empty() {
        return Err("local dependency observation contract unavailable; no execution".into());
    }
    if applicability.source_projection_hash != canonical_json_hash(&json!(paths))
        || applicability.subject_hash != local::source_hash(&root, &paths)?
    {
        return Err("check-evidence source projection is stale".into());
    }
    let plan = admit_plan(&contract, &profiles, &applicability)?;
    let reviewed = required("--plan-hash")?;
    let attempt = required("--attempt")?;
    if attempt.len() != ATTEMPT_DIGEST_HEX_BYTES
        || !attempt
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
    {
        return Err("attempt requires an explicit BLAKE3 identity".into());
    }
    let cancel = required("--cancel-file")?;
    if root
        .read_bounded(Path::new(cancel), local::CANCEL_BYTES)
        .map_err(|_| "cancellation control unavailable")?
        != b"0"
    {
        return Err("check-evidence run cancelled before execution".into());
    }
    // Preflight every executable before the attempt journal or child is created.
    for profile in plan.profiles() {
        if reviewed != plan.identity()
            || local::executable_hash(&profile.executable)? != profile.executable_hash
        {
            return Err("reviewed plan or executable changed; no execution".into());
        }
    }
    let mut publisher = Publisher::open(Path::new(required("--out")?))
        .map_err(|_| "publication root unavailable")?;
    // No-replace journal stays after failure or interruption. There is no implicit retry/recovery.
    publisher.publish(
        "attempt.json",
        &serde_json::to_vec(
            &json!({"attempt_hash":attempt, "plan_hash":reviewed, "state":"started"}),
        )?,
    )?;
    let mut state = AttemptState::default();
    if let Err(blocker) = execute_attempts(
        &root,
        &paths,
        &plan,
        attempt,
        cancel,
        &mut publisher,
        &mut state,
    ) {
        state.blockers.push(blocker);
    }
    let results: Vec<_> = state
        .attempts
        .iter()
        .map(NormalizedAttempt::result)
        .collect();
    // Preserve observations even when their applicability was rejected. They are not result rows.
    for (leaf, bytes) in [
        (
            "observations.json",
            serde_json::to_vec(&state.observations)?,
        ),
        ("results.json", serde_json::to_vec(&results)?),
    ] {
        if publisher.publish(leaf, &bytes).is_ok() {
            state
                .artifacts
                .insert(leaf.into(), blake3::hash(&bytes).to_hex().to_string());
        } else {
            state.blockers.push("artifact_publication_failed");
        }
    }
    for (leaf, expected) in &state.artifacts {
        if publisher.verify(leaf, expected).is_err() {
            state.blockers.push("artifact_readback_changed");
        }
    }
    let current = local::source_hash(&root, &paths).map(|subject_hash| Applicability {
        subject_hash,
        ..applicability
    });
    if current.is_err() {
        state.blockers.push("source_unavailable");
    }
    let complete = state.blockers.is_empty()
        && current.is_ok_and(|current| {
            publication_decision(&plan, &state.attempts, &state.published, &current)
                == PublicationDecision::PublishFinalManifest
        });
    let receipt = json!({"schema":"cairn.check-evidence-run.v1", "valid":complete, "complete":complete, "attempt_hash":attempt,
        "plan_hash":plan.identity(), "results":results, "blockers":state.blockers, "artifacts":state.artifacts,
        "non_claims":NON_CLAIMS, "source_scope":"explicit_file_projection"});
    publisher.publish(
        if complete {
            "manifest.json"
        } else {
            "incomplete.json"
        },
        &serde_json::to_vec(&receipt)?,
    )?;
    print_json(receipt)
}

fn execute_attempts(
    root: &crate::support::CapabilityRoot,
    paths: &[String],
    plan: &CheckEvidencePlan,
    attempt: &str,
    cancel: &str,
    publisher: &mut Publisher,
    state: &mut AttemptState,
) -> Result<(), &'static str> {
    use std::os::fd::AsRawFd;
    let directory = root
        .clone_directory_file()
        .map_err(|_| "working_root_unavailable")?;
    let cwd = std::path::PathBuf::from(format!("/proc/self/fd/{}", directory.as_raw_fd()));
    for profile in plan.profiles() {
        if local::source_hash(root, paths).map_err(|_| "source_unavailable")?
            != plan.applicability().subject_hash
        {
            return Err("source_changed");
        }
        let executable =
            local::executable_hash(&profile.executable).map_err(|_| "executable_unavailable")?;
        authorize_attempt(
            plan,
            &profile.check_id,
            true,
            plan.identity(),
            &executable,
            &state.attempts,
        )
        .map_err(|_| "attempt_not_authorized")?;
        if root
            .read_bounded(Path::new(cancel), local::CANCEL_BYTES)
            .map_err(|_| "cancellation_control_unavailable")?
            != b"0"
        {
            return Err("cancelled_before_spawn");
        }
        let request = local::request(profile, &cwd).map_err(|_| "execution_request_rejected")?;
        let output = local::execute(request, root, cancel);
        let after = local::source_hash(root, paths);
        // Empty identity explicitly records an unavailable observation. Core admission rejects it.
        let (observation, evidence) = local::observation(
            plan,
            profile,
            attempt,
            after.as_ref().cloned().unwrap_or_default(),
            output,
        );
        state.observations.push(observation.clone());
        let bytes = serde_json::to_vec(&evidence).map_err(|_| "facts_encoding_failed")?;
        let leaf = format!("{}.facts.json", profile.check_id);
        publisher
            .publish(&leaf, &bytes)
            .map_err(|_| "facts_publication_failed")?;
        let hash = blake3::hash(&bytes).to_hex().to_string();
        state.artifacts.insert(leaf, hash.clone());
        if local::executable_hash(&profile.executable).map_err(|_| "executable_unavailable")?
            != profile.executable_hash
        {
            return Err("executable_changed");
        }
        if after.map_err(|_| "source_unavailable")? != plan.applicability().subject_hash {
            return Err("source_changed");
        }
        let normalized =
            normalize_attempt(plan, &observation).map_err(|_| "observation_not_applicable")?;
        state.published.push(PublishedArtifact {
            expected_hash: observation.evidence_hash,
            observed_hash: hash,
            durable: true,
        });
        state.attempts.push(normalized);
    }
    Ok(())
}
