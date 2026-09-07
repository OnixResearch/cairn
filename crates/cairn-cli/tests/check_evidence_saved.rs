use mantle_build_contract::{
    observation_identity, request_identity, BuildObservation, BuildRequest,
};
use serde_json::{json, Value};
use std::{error::Error, fs, path::PathBuf, process::Command};

#[test]
fn native_mantle_admission_preserves_status_and_rejects_substitution() -> Result<(), Box<dyn Error>>
{
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixture = root
        .join("target")
        .join(format!("saved-mantle-{}", std::process::id()));
    fs::create_dir_all(&fixture)?;
    let id = format!("b3:{}", "a".repeat(64));
    let mut request: BuildRequest = serde_json::from_value(json!({
        "schema":"mantle.build-request.v1", "identity":id, "idempotency_key":id, "effect_identity":id,
        "attempt_identity":id, "candidate_identity":id, "pipeline_revision_identity":id,
        "plan_identity":id, "policy_identity":id, "platform":"x86_64-linux", "required_products":["out"]
    }))?;
    request.identity = request_identity(&request);
    fs::write(fixture.join("request.json"), serde_json::to_vec(&request)?)?;
    fs::write(fixture.join("builder.json"), serde_json::to_vec(&id)?)?;
    for status in ["success", "failed", "cancelled", "timed_out", "unknown"] {
        let mut observation: BuildObservation = serde_json::from_value(json!({
            "schema":"mantle.build-observation.v1", "identity":id, "request_identity":request.identity,
            "outcome":status, "products":[{"name":"out", "artifact_identity":id, "store_identity":id}],
            "builder_identity":id, "worker_identity":id, "store_identity":id,
            "cache":{"kind":"none", "source_identity":null}, "logs":[], "metrics":[],
            "receipt_identity":if status == "unknown" {None} else {Some(&id)},
            "non_claims":mantle_build_contract::REQUIRED_NON_CLAIMS
        }))?;
        observation.identity = observation_identity(&observation);
        fs::write(
            fixture.join("observation.json"),
            serde_json::to_vec(&observation)?,
        )?;
        let invoke = || {
            Command::new(env!("CARGO_BIN_EXE_cairn"))
                .current_dir(&root)
                .args(["check-evidence", "import", "mantle", "--root"])
                .arg(&fixture)
                .args([
                    "--request",
                    "request.json",
                    "--builder",
                    "builder.json",
                    "--observation",
                    "observation.json",
                ])
                .output()
        };
        let output = invoke()?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stderr)
        );
        let receipt: Value = serde_json::from_slice(&output.stdout)?;
        assert_eq!(receipt["native_contract_valid"], true);
        assert_eq!(receipt["native_status"], status);
        assert_eq!(receipt["completion_eligible"], false);
        assert_eq!(receipt["completion_results"], json!([]));
        assert_eq!(
            receipt["native_cache"],
            json!({"kind":"none", "source_identity":null})
        );
        assert_eq!(
            receipt["native_non_claims"],
            json!(mantle_build_contract::REQUIRED_NON_CLAIMS)
        );
        // Recompute producer identities so binding failures are not merely stale-hash tests.
        for (field, value) in [
            ("schema", json!("mantle.build-observation.v999")),
            ("request_identity", json!(format!("b3:{}", "b".repeat(64)))),
            ("builder_identity", json!(format!("b3:{}", "b".repeat(64)))),
            ("cache", json!({"kind":"local", "source_identity":null})),
        ] {
            let mut substituted = serde_json::to_value(&observation)?;
            substituted[field] = value;
            let mut substituted: BuildObservation = serde_json::from_value(substituted)?;
            substituted.identity = observation_identity(&substituted);
            fs::write(
                fixture.join("observation.json"),
                serde_json::to_vec(&substituted)?,
            )?;
            assert!(!invoke()?.status.success(), "accepted substituted {field}");
        }
        observation.non_claims.clear();
        observation.identity = observation_identity(&observation);
        fs::write(
            fixture.join("observation.json"),
            serde_json::to_vec(&observation)?,
        )?;
        assert!(!invoke()?.status.success());
    }
    fs::write(fixture.join("observation.json"), b"secret-do-not-echo")?;
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .args(["check-evidence", "import", "mantle", "--root"])
        .arg(fixture)
        .args([
            "--request",
            "request.json",
            "--builder",
            "builder.json",
            "--observation",
            "observation.json",
        ])
        .output()?;
    assert!(!output.status.success());
    assert!(!String::from_utf8_lossy(&output.stderr).contains("secret-do-not-echo"));
    Ok(())
}

#[test]
fn native_octet_json_translation_is_bounded_redacted_and_never_strict_acceptance(
) -> Result<(), Box<dyn Error>> {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let fixture = root
        .join("target")
        .join(format!("saved-octet-{}", std::process::id()));
    fs::create_dir_all(&fixture)?;
    let mut value = json!({"schema_version":"octet-acceptance-receipt/v1","run_id":"run","started_at":"start","finished_at":"finish",
        "tool":"octet","tool_version":"reviewed","git_revision":"revision","config_digest":"digest",
        "config_digest_detail":{"algorithm":"blake3","value":"digest"},"inputs":[],"stages":[],
        "artifacts":[],"status":"passed","warnings":["secret-do-not-echo"],"errors":[],
        "verification_commands":[],"verification_evidence":[],"raw_log_path":"private-log"});
    let invoke = || {
        Command::new(env!("CARGO_BIN_EXE_cairn"))
            .current_dir(&root)
            .args(["check-evidence", "import", "octet", "--root"])
            .arg(&fixture)
            .args(["--observation", "observation.json"])
            .output()
    };
    for status in ["passed", "failed", "blocked"] {
        value["status"] = json!(status);
        fs::write(
            fixture.join("observation.json"),
            serde_json::to_vec(&value)?,
        )?;
        let output = invoke()?;
        assert!(output.status.success());
        let receipt: Value = serde_json::from_slice(&output.stdout)?;
        assert_eq!(receipt["admission"]["native_status"], status);
        assert_eq!(receipt["completion_eligible"], false);
        assert_eq!(receipt["completion_results"], json!([]));
        assert!(!String::from_utf8_lossy(&output.stdout).contains("secret-do-not-echo"));
        assert!(!String::from_utf8_lossy(&output.stdout).contains("private-log"));
    }
    value["schema_version"] = json!("unknown");
    fs::write(
        fixture.join("observation.json"),
        serde_json::to_vec(&value)?,
    )?;
    assert!(!invoke()?.status.success());
    fs::write(fixture.join("observation.json"), b"secret-do-not-echo")?;
    let output = invoke()?;
    assert!(!output.status.success());
    assert!(!String::from_utf8_lossy(&output.stderr).contains("secret-do-not-echo"));
    Ok(())
}
