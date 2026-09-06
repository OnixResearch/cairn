//! Saved producer observations, shared by CLI and controlled shell replays.
use cairn_core::verified::consumer_bootstrap::{
    BootstrapBounds, BootstrapCohort, BootstrapIdentity, BootstrapSourceArtifact,
    BootstrapSourceBundle, BOOTSTRAP_SCHEMA, MAX_ARTIFACT_BYTES, MAX_TOTAL_BYTES,
};
use nickel_export_core::{
    build_manifest, build_receipt, ArtifactMaterial, EvaluationObservation, EvaluatorDescriptor,
    ExportFormat, ExportRequest, ImportPathPolicy, REQUEST_SCHEMA,
};
use std::{error::Error, fs, path::Path};

pub(super) const POLICY_DESTINATION: &str = ".cairn/bootstrap/policy.json";
const EVALUATOR_IDENTITY: &str =
    "github:nickel-lang/nickel@1320a983e6c3d1e2fb53dd2464b084b4903b1426";
pub(super) fn id(bytes: &[u8]) -> BootstrapIdentity {
    BootstrapIdentity::from_bytes(bytes)
}
pub(super) fn bundle_material(
    root: &Path,
    policy: Vec<u8>,
    tool: &[u8],
) -> Result<BootstrapSourceBundle, Box<dyn Error>> {
    let source = b"# fixture observation only\n{}";
    let workflow = br#"{"schema":"cairn.bootstrap-guidance.v1","adapter":"agent-skills","destination":".agents/skills"}"#;
    let request = ExportRequest {
        schema: REQUEST_SCHEMA.into(),
        family_id: "cairn-bootstrap".into(),
        source: "policy.ncl".into(),
        dependencies: vec![],
        import_paths: vec![],
        selector: String::new(),
        contract: String::new(),
        format: ExportFormat::Json,
        destination: "policy.json".into(),
        allow_secret_material: false,
    };
    let evaluator = EvaluatorDescriptor {
        identity: EVALUATOR_IDENTITY.into(),
        version: "nickel-lang-cli-1.17.0".into(),
        options: vec!["format=json".into()],
        import_path_policy: ImportPathPolicy::DeclaredOnly,
    };
    let receipt = build_receipt(&EvaluationObservation {
        request: &request,
        source: ArtifactMaterial {
            path: "policy.ncl",
            bytes: source,
        },
        dependencies: vec![],
        output: ArtifactMaterial {
            path: "policy.json",
            bytes: &policy,
        },
        evaluator: &evaluator,
        observed_dependencies: vec![],
        diagnostics: vec![],
    })
    .map_err(|error| format!("fixture receipt: {error:?}"))?;
    let manifest =
        build_manifest(&[receipt]).map_err(|error| format!("fixture manifest: {error:?}"))?;
    let manifest = serde_json::to_vec(&manifest)?;
    fs::write(root.join("policy.ncl"), source)?;
    fs::write(root.join("policy.json"), &policy)?;
    fs::write(root.join("request.json"), serde_json::to_vec(&request)?)?;
    fs::write(root.join("manifest.json"), &manifest)?;
    fs::write(root.join("workflow.json"), workflow)?;
    Ok(BootstrapSourceBundle {
        schema: "cairn.bootstrap-source-bundle.v1".into(),
        cohort: BootstrapCohort {
            schema: BOOTSTRAP_SCHEMA.into(),
            tool: id(tool),
            policy: id(&policy),
            export: id(&manifest),
            evaluator: id(&serde_json::to_vec(&evaluator)?),
            source: id(source),
            workflow: id(workflow),
            profile: "spec-driven".into(),
            managed_locators: vec![id(POLICY_DESTINATION.as_bytes())],
            upgrade_from: vec![],
            bounds: BootstrapBounds {
                max_artifacts: cairn_core::verified::consumer_bootstrap::MAX_MANAGED_ARTIFACTS,
                max_artifact_bytes: MAX_ARTIFACT_BYTES,
                max_total_bytes: MAX_TOTAL_BYTES,
            },
        },
        export_request: "request.json".into(),
        export_manifest: "manifest.json".into(),
        workflow_source: "workflow.json".into(),
        policy_locator: POLICY_DESTINATION.into(),
        artifacts: vec![BootstrapSourceArtifact {
            source: "policy.json".into(),
            destination: POLICY_DESTINATION.into(),
        }],
    })
}
