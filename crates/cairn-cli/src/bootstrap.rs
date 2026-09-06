//! Read-only bootstrap preview composition. Apply is deliberately not admitted
//! until installation locks, publication, and recovery are implemented together.
#[cfg(test)]
#[path = "bootstrap_tests.rs"]
mod tests;
use crate::{
    bootstrap_fs::{open_root, read_optional, read_required},
    support::print_json,
};
use cairn_core::{
    validate_policy_json,
    verified::consumer_bootstrap::{
        admit_bootstrap_policy, cohort_identity, plan_bootstrap, BootstrapIdentity,
        BootstrapSourceBundle, CohortObservations, FileObservation, ManagedArtifact,
        BOOTSTRAP_COMPLETE_LOCATOR as COMPLETE, BOOTSTRAP_PREPARED_LOCATOR as PREPARED,
        BOOTSTRAP_SELECTION_LOCATOR as SELECTION, MAX_ARTIFACT_BYTES, MAX_MANAGED_ARTIFACTS,
        MAX_TOTAL_BYTES,
    },
};
use cap_std::fs::Dir;
use nickel_export_core::{
    build_manifest, build_receipt, verify_manifest_fresh, ArtifactMaterial, EvaluationObservation,
    ExportFormat, ExportManifest, ExportRequest, ImportPathPolicy,
};
use serde_json::json;
use std::{collections::BTreeMap, error::Error, io::Read, path::PathBuf};

const MAX_BUNDLE_BYTES: u64 = 65_536;
const MAX_TOOL_BYTES: u64 = 536_870_912;
const HASH_BUFFER_BYTES: usize = 65_536;

struct Options {
    root: PathBuf,
    source_root: PathBuf,
    bundle: String,
}
fn options(args: &[String]) -> Result<Options, Box<dyn Error>> {
    if args.first().map(String::as_str) != Some("plan") {
        return Err(
            "bootstrap.setup.preview_only: use setup plan; apply and recovery are not admitted"
                .into(),
        );
    }
    let mut root = None;
    let mut source_root = None;
    let mut bundle = None;
    let mut args = args[1..].iter();
    while let Some(option) = args.next() {
        let slot = match option.as_str() {
            "--root" => &mut root,
            "--source-root" => &mut source_root,
            "--bundle" => &mut bundle,
            _ => return Err("bootstrap.setup.invalid_option".into()),
        };
        let value = args
            .next()
            .filter(|value| !value.is_empty() && !value.starts_with("--"))
            .ok_or("bootstrap.setup.missing_option_value")?;
        if slot.replace(value.clone()).is_some() {
            return Err("bootstrap.setup.duplicate_option".into());
        }
    }
    Ok(Options {
        root: root.ok_or("bootstrap.setup.root_required")?.into(),
        source_root: source_root
            .ok_or("bootstrap.setup.source_root_required")?
            .into(),
        bundle: bundle.ok_or("bootstrap.setup.bundle_required")?,
    })
}

struct Sources {
    root: Dir,
    remaining: u64,
    identities: BTreeMap<String, BootstrapIdentity>,
}
impl Sources {
    fn read(&mut self, locator: &str, maximum: u64) -> Result<Vec<u8>, Box<dyn Error>> {
        let bytes = read_required(&self.root, locator, maximum.min(self.remaining))?;
        let identity = BootstrapIdentity::from_bytes(&bytes);
        if self
            .identities
            .get(locator)
            .is_some_and(|previous| previous != &identity)
        {
            return Err("bootstrap.source.changed_during_preview".into());
        }
        self.identities.insert(locator.to_owned(), identity);
        self.remaining = self
            .remaining
            .checked_sub(bytes.len() as u64)
            .ok_or("bootstrap.source.total_bound")?;
        Ok(bytes)
    }
}

/// Binds the currently running artifact, not a caller assertion or version label.
fn tool_identity() -> Result<BootstrapIdentity, Box<dyn Error>> {
    let path = std::env::current_exe()?;
    let file = std::fs::File::open(path)?;
    if !file.metadata()?.is_file() || file.metadata()?.len() > MAX_TOOL_BYTES {
        return Err("bootstrap.source.tool_bound".into());
    }
    let mut reader = file.take(MAX_TOOL_BYTES + 1);
    let mut hasher = blake3::Hasher::new();
    let mut total = 0_u64;
    let mut buffer = [0_u8; HASH_BUFFER_BYTES];
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        total += count as u64;
        if total > MAX_TOOL_BYTES {
            return Err("bootstrap.source.tool_bound".into());
        }
        hasher.update(&buffer[..count]);
    }
    Ok(hasher.finalize().to_hex().to_string().try_into()?)
}

struct CheckedExport {
    output: Vec<u8>,
    source: BootstrapIdentity,
    evaluator: BootstrapIdentity,
    manifest: BootstrapIdentity,
    destination: String,
}
fn check_export(
    sources: &mut Sources,
    bundle: &BootstrapSourceBundle,
) -> Result<CheckedExport, Box<dyn Error>> {
    let request_bytes = sources.read(&bundle.export_request, MAX_BUNDLE_BYTES)?;
    let request: ExportRequest = serde_json::from_slice(&request_bytes)?;
    let manifest_bytes = sources.read(&bundle.export_manifest, MAX_ARTIFACT_BYTES)?;
    let manifest: ExportManifest = serde_json::from_slice(&manifest_bytes)?;
    if manifest.exports.len() != 1
        || request.dependencies.len() > MAX_MANAGED_ARTIFACTS
        || request.format != ExportFormat::Json
        || request.allow_secret_material
        || manifest.evaluator.import_path_policy != ImportPathPolicy::DeclaredOnly
        || manifest.evaluator.identity != crate::nickel_export_boundary::NICKEL_EVALUATOR_IDENTITY
        || manifest.evaluator.version != crate::nickel_export_boundary::NICKEL_EVALUATOR_VERSION
    {
        return Err("bootstrap.export.unsupported".into());
    }
    let recorded = &manifest.exports[0];
    let source = sources.read(&request.source, MAX_ARTIFACT_BYTES)?;
    let mut dependencies = Vec::new();
    for path in &request.dependencies {
        dependencies.push((path.clone(), sources.read(path, MAX_ARTIFACT_BYTES)?));
    }
    let output = sources.read(&request.destination, MAX_ARTIFACT_BYTES)?;
    let actual = build_receipt(&EvaluationObservation {
        request: &request,
        source: ArtifactMaterial {
            path: &request.source,
            bytes: &source,
        },
        dependencies: dependencies
            .iter()
            .map(|(path, bytes)| ArtifactMaterial { path, bytes })
            .collect(),
        output: ArtifactMaterial {
            path: &request.destination,
            bytes: &output,
        },
        evaluator: &manifest.evaluator,
        observed_dependencies: Vec::new(),
        diagnostics: recorded.diagnostics.clone(),
    })
    .map_err(|error| format!("bootstrap.export.rejected: {error:?}"))?;
    let actual_manifest = build_manifest(&[actual])
        .map_err(|error| format!("bootstrap.export.rejected: {error:?}"))?;
    verify_manifest_fresh(&manifest, &actual_manifest)
        .map_err(|error| format!("bootstrap.export.stale: {error:?}"))?;
    Ok(CheckedExport {
        output,
        source: BootstrapIdentity::from_bytes(&source),
        evaluator: BootstrapIdentity::from_bytes(&serde_json::to_vec(&manifest.evaluator)?),
        manifest: BootstrapIdentity::from_bytes(&manifest_bytes),
        destination: request.destination,
    })
}

// r[impl cairn.consumer_bootstrap.boundary]
// r[impl cairn.consumer_bootstrap.compatibility]
pub(crate) fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let options = options(&args)?;
    let root = open_root(&options.root)?;
    crate::support::reject_legacy_only_lifecycle_root(&options.root)?;
    // This increment admits fresh-root previews only. Existing selections must
    // not be mistaken for unmanaged files or overwritten by an implicit upgrade.
    for locator in [PREPARED, COMPLETE, SELECTION] {
        if read_optional(&root, locator, MAX_BUNDLE_BYTES)?.is_some() {
            return Err("bootstrap.setup.existing_state_requires_upgrade_or_recovery".into());
        }
    }
    let mut sources = Sources {
        root: open_root(&options.source_root)?,
        remaining: MAX_TOTAL_BYTES,
        identities: BTreeMap::new(),
    };
    let bundle_bytes = sources.read(&options.bundle, MAX_BUNDLE_BYTES)?;
    let bundle: BootstrapSourceBundle = serde_json::from_slice(&bundle_bytes)?;
    bundle.validate()?;
    let export = check_export(&mut sources, &bundle)?;
    let policy = validate_policy_json(serde_json::from_slice(&export.output)?)?;
    admit_bootstrap_policy(&bundle.cohort, &policy)?;
    let workflow = sources.read(&bundle.workflow_source, MAX_ARTIFACT_BYTES)?;
    let observed = CohortObservations {
        tool: tool_identity()?,
        policy: BootstrapIdentity::from_bytes(&export.output),
        export: export.manifest,
        evaluator: export.evaluator,
        source: export.source,
        workflow: BootstrapIdentity::from_bytes(&workflow),
        export_current: true,
    };
    let mut artifacts = Vec::new();
    for artifact in &bundle.artifacts {
        let bytes = sources.read(&artifact.source, bundle.cohort.bounds.max_artifact_bytes)?;
        if artifact.destination == bundle.policy_locator
            && (artifact.source != export.destination || bytes != export.output)
        {
            return Err("bootstrap.setup.policy_mapping_mismatch".into());
        }
        let observed = match read_optional(
            &root,
            &artifact.destination,
            bundle.cohort.bounds.max_artifact_bytes,
        )? {
            None => FileObservation::Missing,
            Some(bytes) => FileObservation::Regular {
                identity: BootstrapIdentity::from_bytes(&bytes),
                bytes: bytes.len() as u64,
            },
        };
        artifacts.push(ManagedArtifact {
            locator: BootstrapIdentity::from_bytes(artifact.destination.as_bytes()),
            desired: BootstrapIdentity::from_bytes(&bytes),
            desired_bytes: bytes.len() as u64,
            previously_owned: None,
            observed,
        });
    }
    let plan = plan_bootstrap(&bundle.cohort, &observed, None, None, false, &artifacts)?;
    print_json(crate::support::with_receipt_hash(json!({
        "schema": "cairn.bootstrap-preview.v1", "valid": true,
        "artifact_plan_hash": plan.identity(), "cohort": cohort_identity(&bundle.cohort),
        "bundle_blake3": BootstrapIdentity::from_bytes(&bundle_bytes),
        "source_files": sources.identities,
        "policy": observed.policy, "policy_name": policy.name,
        "artifacts": bundle.artifacts.iter().zip(artifacts.iter()).zip(plan.actions()).map(|((source, observed), action)| json!({
            "destination": source.destination, "desired": observed.desired,
            "desired_bytes": observed.desired_bytes, "action": action,
        })).collect::<Vec<_>>(),
        "dry_run": true, "mutated": false, "installation_receipt": false,
        "execution_supported": false,
        "execution_blockers": ["bootstrap.setup.installation_shell_incomplete"],
        "producer_non_claim": nickel_export_core::NON_CLAIM,
        "non_claims": ["Saved export consistency does not prove evaluator execution or policy correctness.",
            "Workflow bytes are identity-bound only; generated-guidance compatibility is not checked.",
            "This artifact preview does not authorize installation, upgrade, recovery, or release."]
    })))
}
