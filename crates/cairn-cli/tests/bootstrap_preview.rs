//! Saved producer observations are fixtures, not executed Nickel evidence.
use cairn_core::verified::consumer_bootstrap::{
    BootstrapBounds, BootstrapCohort, BootstrapIdentity, BootstrapSourceArtifact,
    BootstrapSourceBundle, BOOTSTRAP_SCHEMA, MAX_ARTIFACT_BYTES, MAX_TOTAL_BYTES,
};
use nickel_export_core::{
    build_manifest, build_receipt, ArtifactMaterial, EvaluationObservation, EvaluatorDescriptor,
    ExportFormat, ExportRequest, ImportPathPolicy, REQUEST_SCHEMA,
};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

const POLICY_DESTINATION: &str = ".cairn/bootstrap/policy.json";
const EVALUATOR_IDENTITY: &str =
    "github:nickel-lang/nickel@1320a983e6c3d1e2fb53dd2464b084b4903b1426";
fn id(bytes: &[u8]) -> BootstrapIdentity {
    BootstrapIdentity::from_bytes(bytes)
}
fn bundle(root: &Path) -> Result<BootstrapSourceBundle, Box<dyn Error>> {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let policy = fs::read(repo.join("cairn-policy/generated/cairn-policy.json"))?;
    let source = b"# fixture observation only\n{}";
    let workflow = b"fixture workflow identity only";
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
            tool: id(&fs::read(env!("CARGO_BIN_EXE_cairn"))?),
            policy: id(&policy),
            export: id(&manifest),
            evaluator: id(&serde_json::to_vec(&evaluator)?),
            source: id(source),
            workflow: id(workflow),
            profile: "spec-driven".into(),
            managed_locators: vec![id(POLICY_DESTINATION.as_bytes())],
            upgrade_from: vec![],
            bounds: BootstrapBounds {
                max_artifacts: 1,
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

fn invoke(root: &Path, sources: &Path, extra: &[&str]) -> Result<Output, Box<dyn Error>> {
    Ok(Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .env("HOME", root)
        .env("CAIRN_DATA_DIR", root.join("data"))
        .args(["setup", "plan", "--root"])
        .arg(root)
        .arg("--source-root")
        .arg(sources)
        .args(["--bundle", "bundle.json"])
        .args(extra)
        .output()?)
}

// r[verify cairn.consumer_bootstrap.boundary]
// r[verify cairn.consumer_bootstrap.compatibility]
#[test]
fn preview_binds_observations_without_authorizing_installation() -> Result<(), Box<dyn Error>> {
    let temp = std::env::temp_dir().join(format!("cairn-bootstrap-preview-{}", std::process::id()));
    fs::create_dir(&temp)?;
    let root = temp.join("consumer");
    let sources = temp.join("sources");
    fs::create_dir(&root)?;
    fs::create_dir(&sources)?;
    let bundle = bundle(&sources)?;
    let bytes = serde_json::to_vec(&bundle)?;
    fs::write(sources.join("bundle.json"), &bytes)?;
    let good = invoke(&root, &sources, &[])?;
    assert!(
        good.status.success(),
        "{}",
        String::from_utf8_lossy(&good.stderr)
    );
    let result: serde_json::Value = serde_json::from_slice(&good.stdout)?;
    assert_eq!(result["valid"], true);
    assert_eq!(result["mutated"], false);
    assert_eq!(result["installation_receipt"], false);
    assert_eq!(result["execution_supported"], false);
    assert_eq!(result["artifacts"][0]["action"], "create");
    assert_eq!(result["policy"], bundle.cohort.policy.as_str());
    assert_eq!(result["producer_non_claim"], nickel_export_core::NON_CLAIM);
    assert_eq!(invoke(&root, &sources, &[])?.stdout, good.stdout);
    assert_eq!(fs::read_dir(&root)?.count(), 0);
    for args in [
        vec!["--execute"],
        vec!["--bundle", "bundle.json"],
        vec!["--unknown"],
    ] {
        assert!(!invoke(&root, &sources, &args)?.status.success());
        assert_eq!(fs::read_dir(&root)?.count(), 0);
    }
    let mut bad = bundle.clone();
    bad.cohort.tool = id(b"other tool");
    fs::write(sources.join("bundle.json"), serde_json::to_vec(&bad)?)?;
    assert!(!invoke(&root, &sources, &[])?.status.success());
    bad = bundle.clone();
    bad.cohort.profile = "unknown".into();
    fs::write(sources.join("bundle.json"), serde_json::to_vec(&bad)?)?;
    assert!(!invoke(&root, &sources, &[])?.status.success());
    fs::write(sources.join("bundle.json"), &bytes)?;
    let original = fs::read(sources.join("policy.ncl"))?;
    fs::write(sources.join("policy.ncl"), b"changed")?;
    let stale = invoke(&root, &sources, &[])?;
    assert!(!stale.status.success());
    assert!(String::from_utf8_lossy(&stale.stderr).contains("bootstrap.export.stale"));
    fs::write(sources.join("policy.ncl"), original)?;
    fs::create_dir_all(root.join(".cairn/bootstrap"))?;
    fs::write(root.join(POLICY_DESTINATION), b"unowned")?;
    assert!(!invoke(&root, &sources, &[])?.status.success());
    assert_eq!(fs::read(root.join(POLICY_DESTINATION))?, b"unowned");
    fs::remove_file(root.join(POLICY_DESTINATION))?;
    fs::write(root.join(".cairn/bootstrap/prepared.json"), b"{}")?;
    assert!(!invoke(&root, &sources, &[])?.status.success());
    fs::remove_file(root.join(".cairn/bootstrap/prepared.json"))?;
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(sources.join("policy.json"), root.join(POLICY_DESTINATION))?;
        assert!(!invoke(&root, &sources, &[])?.status.success());
        fs::remove_file(root.join(POLICY_DESTINATION))?;
        fs::rename(
            sources.join("policy.json"),
            sources.join("real-policy.json"),
        )?;
        std::os::unix::fs::symlink("real-policy.json", sources.join("policy.json"))?;
        assert!(!invoke(&root, &sources, &[])?.status.success());
    }
    fs::remove_dir_all(temp)?;
    Ok(())
}
