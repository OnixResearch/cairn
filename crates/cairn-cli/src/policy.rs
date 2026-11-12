use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use cairn_core::{validate_policy_json, Policy};
use serde_json::Value;

use crate::determinism::{
    determinism_surfaces_from_policy, replay_cases_from_policy, replay_groups_from_policy,
};
use crate::nickel_export_boundary::export_json_dual_run;
use crate::support::{
    default_policy_path, has_flag, load_policy_file, print_json, read_option,
    reject_positional_arguments, reject_unknown_options, reject_unknown_options_with_flags,
};

const MAX_POLICY_OUTPUT_BYTES: u64 = 16 * 1024 * 1024;
const GENERATED_POLICY_RELATIVE_PATH: &str = "cairn-policy/generated/cairn-policy.json";

pub(crate) fn run_policy(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.first().map(String::as_str) == Some("export") {
        let rest = args.into_iter().skip(1).collect::<Vec<_>>();
        reject_unknown_options_with_flags(&rest, &["--source", "--output"], &["--check"])?;
        return run_policy_export(rest);
    }
    reject_unknown_options(&args, &["--policy"])?;
    reject_positional_arguments(&args)?;
    let policy_path = read_option(args, "--policy", default_policy_path());
    let policy = load_policy_file(policy_path)?;
    print_json(policy.raw)
}

fn read_existing_policy_output(path: &Path) -> Result<String, Box<dyn Error>> {
    crate::support::read_existing_bounded_text_no_symlink(
        path,
        MAX_POLICY_OUTPUT_BYTES,
        "generated policy",
    )
}

fn run_policy_export(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.rust_nickel_only_gates.policy_export]
    // r[impl cairn.determinism.preserves_content_hash_binding_readiness.policy]
    let is_check = has_flag(&args, "--check");
    let default_source = default_policy_source_path();
    let source = read_option(
        args.clone(),
        "--source",
        default_source
            .to_str()
            .unwrap_or("cairn-policy/default.ncl"),
    );
    // The packaged read default (`CAIRN_POLICY`) must not redirect generated
    // policy output outside the repository.
    let output = read_option(args, "--output", GENERATED_POLICY_RELATIVE_PATH);
    let rendered = export_policy_from_nickel(&source, &output)?;
    check_invalid_policy_fixtures(source.parent().unwrap_or(Path::new(".")).join("fixtures"))?;
    if is_check {
        let existing = read_existing_policy_output(&output)?;
        if existing != rendered {
            return Err(format!("generated policy is stale: {}", output.display()).into());
        }
        println!("policy fresh: {}", output.display());
        return Ok(());
    }
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(&output, rendered)?;
    println!("wrote {}", output.display());
    Ok(())
}

fn default_policy_source_path() -> PathBuf {
    PathBuf::from("cairn-policy/default.ncl")
}

fn export_policy_from_nickel(source: &Path, destination: &Path) -> Result<String, Box<dyn Error>> {
    // r[verify cairn.supersede.rust_nickel_only_gates.no_python_scripts]
    // r[impl cairn.nickel_export_cutover.boundary]
    // r[impl cairn.nickel_export_cutover.dual_run]
    let export = export_json_dual_run("cairn.policy", source, destination, true)?;
    let value: Value = serde_json::from_slice(&export.bytes)?;
    validate_exported_policy(&value)?;
    Ok(serde_json::to_string_pretty(&value)?
        + "
")
}

fn validate_exported_policy(value: &Value) -> Result<(), Box<dyn Error>> {
    let policy = validate_policy_json(value.clone())?;
    require_policy_condition(
        value["receipt_hash_policy"]["algorithm"].as_str() == Some("BLAKE3"),
        "receipt hash algorithm must be BLAKE3",
    )?;
    require_policy_condition(
        value["receipt_hash_policy"]["canonicalization"].as_str()
            == Some("serde_json_compact_sorted_keys"),
        "receipt hash canonicalization must be serde_json_compact_sorted_keys",
    )?;
    for field in ["schemas", "lifecycle_stages", "receipt_schemas"] {
        require_policy_condition(
            value[field]
                .as_array()
                .is_some_and(|items| !items.is_empty()),
            &format!("{field} must be non-empty"),
        )?;
    }
    require_unique_policy_strings(
        json_string_array(&value["lifecycle_stages"]),
        "lifecycle stages",
    )?;
    let marker_policy = &value["task_marker_policy"];
    require_unique_policy_strings(
        json_string_array(&marker_policy["stages"]),
        "task marker stages",
    )?;
    let markers = marker_policy["markers"]
        .as_array()
        .cloned()
        .unwrap_or_default();
    require_unique_policy_strings(
        markers
            .iter()
            .filter_map(|v| v["id"].as_str().map(ToOwned::to_owned))
            .collect(),
        "marker ids",
    )?;
    require_unique_policy_strings(
        markers
            .iter()
            .filter_map(|v| v["token"].as_str().map(ToOwned::to_owned))
            .collect(),
        "marker tokens",
    )?;
    for schema in &policy.receipt_schemas {
        require_unique_policy_strings(
            schema.top_level_keys.clone(),
            &format!("{} top-level receipt keys", schema.command),
        )?;
        require_unique_policy_strings(
            schema.action_keys.clone(),
            &format!("{} action keys", schema.command),
        )?;
        require_unique_policy_strings(
            schema.issue_keys.clone(),
            &format!("{} issue keys", schema.command),
        )?;
        for hash_field in &schema.hash_fields {
            require_policy_condition(
                schema.top_level_keys.iter().any(|key| key == hash_field),
                &format!(
                    "{} hash field {hash_field} is not a top-level key",
                    schema.command
                ),
            )?;
        }
    }
    validate_artifact_dependencies(value)?;
    validate_determinism_surface_inventory(&policy)?;
    Ok(())
}

fn check_invalid_policy_fixtures(fixtures: PathBuf) -> Result<(), Box<dyn Error>> {
    if !fixtures.exists() {
        return Ok(());
    }
    for entry in fs::read_dir(fixtures)?.collect::<Result<Vec<_>, _>>()? {
        let path = entry.path();
        let Some(name) = path.file_name().and_then(|value| value.to_str()) else {
            continue;
        };
        if !name.starts_with("invalid-")
            || path.extension().and_then(|value| value.to_str()) != Some("ncl")
        {
            continue;
        }
        let output = Command::new("nickel")
            .arg("export")
            .arg("--format")
            .arg("json")
            .arg(&path)
            .output()?;
        if !output.status.success() {
            continue;
        }
        let Ok(value) = serde_json::from_slice::<Value>(&output.stdout) else {
            continue;
        };
        if validate_exported_policy(&value).is_ok() {
            return Err(format!(
                "expected invalid policy fixture to fail: {}",
                path.display()
            )
            .into());
        }
    }
    Ok(())
}

fn require_policy_condition(condition: bool, message: &str) -> Result<(), Box<dyn Error>> {
    if condition {
        Ok(())
    } else {
        Err(format!("policy semantic check failed: {message}").into())
    }
}

fn require_unique_policy_strings(values: Vec<String>, label: &str) -> Result<(), Box<dyn Error>> {
    let mut seen = std::collections::BTreeSet::new();
    let mut duplicates = std::collections::BTreeSet::new();
    for value in values {
        if !seen.insert(value.clone()) {
            duplicates.insert(value);
        }
    }
    require_policy_condition(
        duplicates.is_empty(),
        &format!(
            "duplicate {label}: {}",
            duplicates.into_iter().collect::<Vec<_>>().join(", ")
        ),
    )
}

fn json_string_array(value: &Value) -> Vec<String> {
    value
        .as_array()
        .map(|items| {
            items
                .iter()
                .filter_map(|v| v.as_str().map(ToOwned::to_owned))
                .collect()
        })
        .unwrap_or_default()
}

fn validate_artifact_dependencies(value: &Value) -> Result<(), Box<dyn Error>> {
    for schema in value["schemas"].as_array().cloned().unwrap_or_default() {
        let artifacts = schema["artifacts"].as_array().cloned().unwrap_or_default();
        let artifact_ids = artifacts
            .iter()
            .filter_map(|artifact| artifact["id"].as_str().map(ToOwned::to_owned))
            .collect::<Vec<_>>();
        require_unique_policy_strings(
            artifact_ids.clone(),
            &format!(
                "artifact ids in schema {}",
                schema["id"].as_str().unwrap_or("<unknown>")
            ),
        )?;
        let known = artifact_ids
            .into_iter()
            .collect::<std::collections::BTreeSet<_>>();
        for artifact in artifacts {
            for required in json_string_array(&artifact["requires"]) {
                require_policy_condition(
                    known.contains(&required),
                    &format!(
                        "artifact {} requires unknown artifact {required}",
                        artifact["id"].as_str().unwrap_or("<unknown>")
                    ),
                )?;
            }
        }
    }
    Ok(())
}

fn validate_determinism_surface_inventory(policy: &Policy) -> Result<(), Box<dyn Error>> {
    let cases = replay_cases_from_policy(policy)?
        .into_iter()
        .map(|case| case.id)
        .collect::<std::collections::BTreeSet<_>>();
    let groups = replay_groups_from_policy(policy)?
        .into_iter()
        .map(|group| {
            (
                group.id,
                group
                    .cases
                    .into_iter()
                    .collect::<std::collections::BTreeSet<_>>(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    let surfaces = determinism_surfaces_from_policy(policy)?;
    require_policy_condition(
        !surfaces.is_empty(),
        "determinism_surface_inventory must be non-empty",
    )?;
    require_unique_policy_strings(
        surfaces.iter().map(|surface| surface.id.clone()).collect(),
        "determinism surface ids",
    )?;
    for surface in surfaces {
        require_policy_condition(
            !surface.required_replay_cases.is_empty(),
            &format!(
                "determinism surface {} requires at least one replay case",
                surface.id
            ),
        )?;
        require_policy_condition(
            !surface.required_replay_groups.is_empty(),
            &format!(
                "determinism surface {} requires at least one replay group",
                surface.id
            ),
        )?;
        for case_id in &surface.required_replay_cases {
            require_policy_condition(
                cases.contains(case_id),
                &format!(
                    "determinism surface {} references unknown replay case {case_id}",
                    surface.id
                ),
            )?;
        }
        for group_id in &surface.required_replay_groups {
            let Some(group_cases) = groups.get(group_id) else {
                return Err(format!("policy semantic check failed: determinism surface {} references unknown replay group {group_id}", surface.id).into());
            };
            let missing = surface
                .required_replay_cases
                .iter()
                .filter(|case_id| !group_cases.contains(*case_id))
                .cloned()
                .collect::<Vec<_>>();
            require_policy_condition(
                missing.is_empty(),
                &format!(
                    "determinism surface {} group {group_id} missing cases: {}",
                    surface.id,
                    missing.join(", ")
                ),
            )?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::read_existing_policy_output;
    use std::{error::Error, fs, path::PathBuf};

    fn test_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "cairn-policy-output-{}-{name}.json",
            std::process::id()
        ))
    }

    #[test]
    fn missing_output_is_empty() -> Result<(), Box<dyn Error>> {
        let path = test_path("missing");
        let _ = fs::remove_file(&path);
        assert_eq!(read_existing_policy_output(&path)?, "");
        Ok(())
    }

    #[test]
    fn oversized_output_is_rejected() -> Result<(), Box<dyn Error>> {
        let path = test_path("oversized");
        let file = fs::File::create(&path)?;
        file.set_len(16 * 1024 * 1024 + 1)?;
        let error = read_existing_policy_output(&path).expect_err("oversized output must fail");
        assert!(error
            .to_string()
            .contains("exceeds the reviewed byte bound"));
        fs::remove_file(path)?;
        Ok(())
    }

    #[cfg(unix)]
    #[test]
    fn symlinked_output_is_rejected() -> Result<(), Box<dyn Error>> {
        use std::os::unix::fs::symlink;

        let outside = test_path("outside");
        let link = test_path("symlink");
        fs::write(&outside, b"{}")?;
        symlink(&outside, &link)?;
        let error = read_existing_policy_output(&link).expect_err("symlinked output must fail");
        assert!(error.to_string().contains("must not be a symlink"));
        fs::remove_file(link)?;
        fs::remove_file(outside)?;
        Ok(())
    }
}
