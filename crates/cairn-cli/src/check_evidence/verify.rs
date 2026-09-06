//! Read-only bundle consumer. Interrupted output never authorizes recovery mutation.
use super::{local, named_options, read_input, reads};
use crate::support::{load_policy_file, print_json, CapabilityRoot, CapabilityRootKind};
use cairn_core::verified::check_evidence::*;
use cairn_core::{canonical_json_hash, AgentVerificationContract};
use serde_json::{json, Value};
use std::{collections::BTreeMap, error::Error, path::Path};
const MAX_ARTIFACT_BYTES: u64 = 4 * 1024 * 1024;

pub(super) fn verify(args: &[String]) -> Result<(), Box<dyn Error>> {
    let options = named_options(
        args,
        &[
            "--root",
            "--out",
            "--contract",
            "--profiles",
            "--applicability",
            "--projection",
            "--policy",
            "--plan-hash",
            "--attempt",
        ],
    )?;
    let required = |name| {
        options
            .get(name)
            .copied()
            .ok_or("missing verification option")
    };
    let root = local::open_root(Path::new(options.get("--root").copied().unwrap_or(".")))?;
    let contract: AgentVerificationContract = read_input(&root, required("--contract")?)?;
    let profiles: Vec<CheckProfile> = read_input(&root, required("--profiles")?)?;
    let applicability: Applicability = read_input(&root, required("--applicability")?)?;
    let projection: Vec<String> = read_input(&root, required("--projection")?)?;
    let policy = load_policy_file(
        options
            .get("--policy")
            .copied()
            .unwrap_or(crate::support::default_policy_path())
            .into(),
    )?;
    if applicability.policy_hash != canonical_json_hash(&policy.raw)
        || applicability.source_projection_hash != canonical_json_hash(&json!(projection))
        || applicability.subject_hash != local::source_hash(&root, &projection)?
        || !applicability.dependency_hashes.is_empty()
    {
        return Err("manifest applicability is stale or unsupported".into());
    }
    let plan = admit_plan(&contract, &profiles, &applicability)?;
    if plan.identity() != required("--plan-hash")? {
        return Err("reviewed manifest plan changed".into());
    }
    let bundle = CapabilityRoot::open(
        CapabilityRootKind::EvidenceBundle,
        Path::new(required("--out")?),
    )?;
    let admitted = (|| -> Result<(), Box<dyn Error>> {
        let manifest: Value = serde_json::from_slice(&reads::input(
            &bundle,
            Path::new("manifest.json"),
            MAX_ARTIFACT_BYTES,
        )?)?;
        let mut leaves = vec![
            "attempt.json".into(),
            "results.json".into(),
            "observations.json".into(),
        ];
        leaves.extend(
            plan.profiles()
                .iter()
                .map(|p| format!("{}.facts.json", p.check_id)),
        );
        let mut artifacts = BTreeMap::new();
        let mut total_bytes = 0_usize;
        for leaf in leaves {
            let bytes = reads::input(&bundle, Path::new(&leaf), MAX_ARTIFACT_BYTES)?;
            total_bytes = total_bytes
                .checked_add(bytes.len())
                .ok_or("bundle size overflow")?;
            if total_bytes > MAX_BUNDLE_BYTES {
                return Err("bundle exceeds aggregate bound".into());
            }
            artifacts.insert(leaf, bytes);
        }
        admit_manifest(&plan, required("--attempt")?, &manifest, &artifacts)?;
        Ok(())
    })()
    .is_ok();
    print_json(
        json!({"schema":"cairn.check-evidence-verification.v1", "valid":admitted,
        "recovery":recovery_decision(admitted), "mutation_performed":false,
        "non_claims":["manifest admission proves presented artifact linkage, not execution truth or release eligibility", "interrupted output requires inspection and a separately authorized new output root"]}),
    )
}
