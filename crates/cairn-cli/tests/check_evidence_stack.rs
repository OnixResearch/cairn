//! Native fixtures cross the real no-follow shell and pure translation boundary.
use serde_json::{json, Value};
use std::{error::Error, fs, path::PathBuf, process::Command};
fn repo() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}
fn write(root: &std::path::Path, name: &str, value: &Value) -> Result<(), Box<dyn Error>> {
    fs::write(root.join(name), serde_json::to_vec(value)?)?;
    Ok(())
}
fn run(
    root: &std::path::Path,
    producer: &str,
    args: &[&str],
) -> Result<std::process::Output, Box<dyn Error>> {
    Ok(Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(repo())
        .args(["check-evidence", "import", producer, "--root"])
        .arg(root)
        .args(args)
        .output()?)
}
fn ineligible(output: std::process::Output) -> Result<Value, Box<dyn Error>> {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let value: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(value["completion_eligible"], false);
    assert_eq!(value["checks_executed"], false);
    assert_eq!(value["completion_results"], json!([]));
    Ok(value)
}
#[test]
fn original_mantle_fixture_intake_keeps_cache_and_failure_meaning() -> Result<(), Box<dyn Error>> {
    let root = repo().join(format!("target/stack-mantle-{}", std::process::id()));
    fs::create_dir_all(&root)?;
    let fixture: Value = serde_json::from_slice(&fs::read(
        repo().join("fixtures/check-evidence/mantle-producer-fixtures.json"),
    )?)?;
    write(&root, "request.json", &fixture["request"])?;
    write(
        &root,
        "builder.json",
        &fixture["success"]["builder_identity"],
    )?;
    for name in ["success", "cache_success", "failure"] {
        write(&root, "observation.json", &fixture[name])?;
        let receipt = ineligible(run(
            &root,
            "mantle",
            &[
                "--request",
                "request.json",
                "--builder",
                "builder.json",
                "--observation",
                "observation.json",
            ],
        )?)?;
        assert_eq!(receipt["native_status"], fixture[name]["outcome"]);
        assert_eq!(receipt["native_cache"], fixture[name]["cache"]);
    }
    fs::remove_dir_all(root)?;
    Ok(())
}
// r[verify cairn.check_evidence_producer.adapters]
#[test]
fn published_lattice_inputs_reach_the_shell_without_test_promotion() -> Result<(), Box<dyn Error>> {
    let root = repo().join(format!("target/stack-lattice-{}", std::process::id()));
    fs::create_dir_all(&root)?;
    let fixture: Value = serde_json::from_slice(&fs::read(
        repo().join("fixtures/check-evidence/lattice-producer-fixtures.json"),
    )?)?;
    for name in ["profile", "request", "accepted", "observation"] {
        write(&root, &format!("{name}.json"), &fixture[name])?;
    }
    let args = [
        "--profile",
        "profile.json",
        "--request",
        "request.json",
        "--accepted",
        "accepted.json",
        "--observation",
        "observation.json",
    ];
    let result = ineligible(run(&root, "lattice", &args)?)?;
    assert_eq!(result["admission"]["native_status"], "completed");
    for name in ["false_paused_success", "stale_execution"] {
        write(&root, "observation.json", &fixture["rejected"][name])?;
        assert!(!run(&root, "lattice", &args)?.status.success());
    }
    fs::remove_dir_all(root)?;
    Ok(())
}
#[test]
fn valence_and_kiln_json_have_native_meaning_but_no_execution_coverage(
) -> Result<(), Box<dyn Error>> {
    let root = repo().join(format!("target/stack-sidecars-{}", std::process::id()));
    fs::create_dir_all(&root)?;
    let mut sidecar: Value = serde_json::from_slice(&fs::read(
        repo().join("fixtures/check-evidence/valence-sidecar.json"),
    )?)?;
    write(&root, "observation.json", &sidecar)?;
    let result = ineligible(run(
        &root,
        "valence",
        &["--observation", "observation.json"],
    )?)?;
    assert_eq!(
        result["admission"]["native_claim_scope"],
        "identity-linkage-sidecar"
    );
    sidecar["diagnostics"] = json!(["credential-secret"]);
    write(&root, "observation.json", &sidecar)?;
    let rejected = run(&root, "valence", &["--observation", "observation.json"])?;
    assert!(!rejected.status.success());
    assert!(!String::from_utf8_lossy(&rejected.stderr).contains("credential-secret"));
    let fixture: Value = serde_json::from_slice(&fs::read(
        repo().join("fixtures/check-evidence/kiln-workflow.json"),
    )?)?;
    write(&root, "plan.json", &fixture["plan"])?;
    write(&root, "observation.json", &fixture["observation"])?;
    let args = ["--plan", "plan.json", "--observation", "observation.json"];
    let result = ineligible(run(&root, "kiln", &args)?)?;
    assert_eq!(result["admission"]["native_outcome"], "Success");
    let mut observation = fixture["observation"].clone();
    observation["candidate_id"] = json!("foreign");
    write(&root, "observation.json", &observation)?;
    assert!(!run(&root, "kiln", &args)?.status.success());
    fs::remove_dir_all(root)?;
    Ok(())
}
