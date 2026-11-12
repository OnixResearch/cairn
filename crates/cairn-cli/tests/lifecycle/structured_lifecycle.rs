use std::{error::Error, fs};

use super::support::{assert_hash_field, cairn, run_json, run_json_strings, temp_project};

#[test]
fn lifecycle_check_accepts_nickel_source_and_rejects_invalid_json() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.structured_lifecycle.typed_authoring]
    // r[verify cairn.structured_lifecycle.typed_authoring.nickel_contracts]
    // r[verify cairn.structured_lifecycle.typed_authoring.rust_revalidation]
    // r[verify cairn.structured_lifecycle.format_strategy.nickel_authoring]
    let valid = run_json(&[
        "lifecycle",
        "check",
        "--source",
        "fixtures/structured-lifecycle/positive-bundle.ncl",
    ])?;
    assert_eq!(valid["valid"], true);
    assert_eq!(valid["source_format"], "nickel");
    assert_hash_field(&valid, "bundle_hash")?;
    assert_hash_field(&valid, "receipt_hash")?;

    let invalid = run_json(&[
        "lifecycle",
        "check",
        "--source",
        "fixtures/structured-lifecycle/invalid-missing-then.json",
    ])?;
    assert_eq!(invalid["valid"], false);
    let issues = invalid["issues"].as_array().ok_or("issues")?;
    assert!(issues
        .iter()
        .any(|issue| issue["code"] == "structured_lifecycle.then.missing"));

    let malformed_id = run_json(&[
        "lifecycle",
        "check",
        "--source",
        "fixtures/structured-lifecycle/invalid-malformed-id.json",
    ])?;
    assert_eq!(malformed_id["valid"], false);
    assert!(malformed_id["issues"]
        .as_array()
        .ok_or("malformed id issues")?
        .iter()
        .any(|issue| issue["code"] == "structured_lifecycle.id.malformed"));

    let invalid_task_link = run_json(&[
        "lifecycle",
        "check",
        "--source",
        "fixtures/structured-lifecycle/invalid-task-link.json",
    ])?;
    assert_eq!(invalid_task_link["valid"], false);
    assert!(invalid_task_link["issues"]
        .as_array()
        .ok_or("invalid task link issues")?
        .iter()
        .any(|issue| issue["code"] == "structured_lifecycle.requirement_ref.unknown"));
    Ok(())
}

#[test]
fn lifecycle_export_check_detects_stale_bundle() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.structured_lifecycle.runtime_bundle]
    // r[verify cairn.structured_lifecycle.runtime_bundle.stale_generated_bundle]
    // r[verify cairn.structured_lifecycle.runtime_bundle.receipt_hash_binding]
    // r[verify cairn.structured_lifecycle.format_strategy.generated_json]
    let dir = temp_project("structured-lifecycle-export")?;
    let output = dir.join("bundle.json");
    let output_text = output.to_str().ok_or("utf8 output")?.to_string();
    let export = run_json_strings(&[
        "lifecycle".into(),
        "export".into(),
        "--source".into(),
        "fixtures/structured-lifecycle/positive-bundle.ncl".into(),
        "--output".into(),
        output_text.clone(),
    ])?;
    assert_eq!(export["valid"], true);
    assert!(output.exists());

    let fresh = run_json_strings(&[
        "lifecycle".into(),
        "export".into(),
        "--source".into(),
        "fixtures/structured-lifecycle/positive-bundle.ncl".into(),
        "--output".into(),
        output_text.clone(),
        "--check".into(),
    ])?;
    assert_eq!(fresh["valid"], true);

    fs::write(&output, "{}\n")?;
    let stale = run_json_strings(&[
        "lifecycle".into(),
        "export".into(),
        "--source".into(),
        "fixtures/structured-lifecycle/positive-bundle.ncl".into(),
        "--output".into(),
        output_text,
        "--check".into(),
    ])?;
    assert_eq!(stale["valid"], false);
    assert!(stale["issues"]
        .as_array()
        .ok_or("issues")?
        .iter()
        .any(|issue| issue["code"] == "structured_lifecycle.generated.stale"));
    Ok(())
}

#[test]
fn lifecycle_markdown_render_and_import_are_receipted() -> Result<(), Box<dyn Error>> {
    // r[verify cairn.structured_lifecycle.markdown_projection]
    // r[verify cairn.structured_lifecycle.markdown_projection.render_view]
    // r[verify cairn.structured_lifecycle.markdown_projection.lossy_import]
    // r[verify cairn.structured_lifecycle.migration.preview_first]
    // r[verify cairn.structured_lifecycle.migration.no_silent_delete]
    let dir = temp_project("structured-lifecycle-render")?;
    let out_dir = dir.join("rendered");
    let render = run_json_strings(&[
        "lifecycle".into(),
        "render-markdown".into(),
        "--source".into(),
        "fixtures/structured-lifecycle/positive-bundle.ncl".into(),
        "--out-dir".into(),
        out_dir.display().to_string(),
    ])?;
    assert_eq!(render["valid"], true);
    assert!(out_dir.join("specs/structured-lifecycle/spec.md").exists());

    let import = run_json_strings(&[
        "lifecycle".into(),
        "import-markdown".into(),
        "--root".into(),
        "fixtures/basic".into(),
        "--out".into(),
        "target/imported-lifecycle.json".into(),
    ])?;
    assert_eq!(import["dry_run"], true);
    assert_eq!(import["mutated"], false);
    assert_hash_field(&import, "bundle_hash")
}

#[cfg(unix)]
#[test]
fn lifecycle_export_check_rejects_symlinked_output() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;

    let dir = temp_project("structured-lifecycle-symlink-output")?;
    let outside = dir.join("outside.json");
    fs::write(&outside, b"{}\n")?;
    let link = dir.join("bundle.json");
    symlink(&outside, &link)?;
    let mut command = cairn();
    command.args([
        "lifecycle",
        "export",
        "--source",
        "fixtures/structured-lifecycle/positive-bundle.ncl",
        "--output",
        &link.to_string_lossy(),
        "--check",
    ]);
    let output = command.output()?;
    assert!(!output.status.success());
    assert!(
        String::from_utf8_lossy(&output.stderr).contains("must not be a symlink"),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    fs::remove_dir_all(dir)?;
    Ok(())
}
