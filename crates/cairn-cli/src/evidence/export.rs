use super::*;

struct EvidenceExportOptions {
    root: PathBuf,
    out: Option<PathBuf>,
    agent_receipt: Option<PathBuf>,
}

pub(super) fn run_evidence_export(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    // r[impl cairn.supersede.evidence_export_bundle.command]
    // r[impl cairn.supersede.evidence_export_bundle.artifacts]
    // r[impl cairn.supersede.evidence_export_bundle.failures]
    let options = evidence_export_options(&args)?;
    let receipt = evidence_export_receipt(&options);
    let valid = receipt
        .get("valid")
        .and_then(Value::as_bool)
        .unwrap_or(false);
    print_json(receipt)?;
    if !valid {
        std::process::exit(1);
    }
    Ok(())
}

fn evidence_export_options(args: &[String]) -> Result<EvidenceExportOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let out = read_string_option(args, "--out").map(PathBuf::from);
    let agent_receipt = read_string_option(args, "--agent-receipt").map(PathBuf::from);
    if out.is_none() {
        return Err("evidence export requires --out <dir>".into());
    }
    Ok(EvidenceExportOptions {
        root,
        out,
        agent_receipt,
    })
}

fn evidence_export_receipt(options: &EvidenceExportOptions) -> Value {
    let root = options.root.clone();
    let out_dir = options.out.clone().unwrap_or_else(|| PathBuf::from("."));
    let mut issues = Vec::new();
    if fs::create_dir_all(&out_dir).is_err() {
        issues.push(evidence_export_issue(
            "export.out_dir_create_failed",
            "out_dir",
            "writable output directory",
            "failed",
        ));
        return evidence_export_final_receipt(
            &root,
            &out_dir,
            issues,
            Vec::new(),
            Value::Null,
            Value::Null,
        );
    }

    let policy = match load_policy_file(default_policy_path().into()) {
        Ok(policy) => policy,
        Err(_) => {
            issues.push(evidence_export_issue(
                "export.policy_load_failed",
                "policy",
                "generated policy loads",
                "failed",
            ));
            return evidence_export_final_receipt(
                &root,
                &out_dir,
                issues,
                Vec::new(),
                Value::Null,
                Value::Null,
            );
        }
    };

    let agent_path = options
        .agent_receipt
        .clone()
        .unwrap_or_else(|| root.join("fixtures/agent-receipts/positive.json"));
    let coverage_path = out_dir.join("coverage-audit.json");
    let replay_path = out_dir.join("replay-agent.json");
    let replay_verification_path = out_dir.join("replay-verification.json");
    let release_path = out_dir.join("release-readiness.json");
    let exported_agent_path = out_dir.join("agent-receipt.json");
    let bundle_path = out_dir.join("agent-evidence-bundle.json");
    let bundle_verification_path = out_dir.join("agent-evidence-bundle-verification.json");
    let manifest_path = out_dir.join("evidence-manifest.json");
    let manifest_verification_path = out_dir.join("evidence-manifest-verification.json");

    let coverage = determinism_coverage_audit_receipt(&root, &policy);
    write_evidence_json(&coverage_path, &coverage, &mut issues, "coverage_audit");

    let replay =
        match determinism_replay_receipt(root.clone(), None, Some(REQUIRED_AGENT_REPLAY_GROUP)) {
            Ok(receipt) => receipt,
            Err(_) => {
                issues.push(evidence_export_issue(
                    "export.replay_failed",
                    "replay_receipt",
                    "valid replay receipt",
                    "failed",
                ));
                Value::Null
            }
        };
    if replay.is_object() {
        write_evidence_json(&replay_path, &replay, &mut issues, "replay_receipt");
    }

    let replay_verification = if replay.is_object() {
        write_evidence_json(&replay_path, &replay, &mut issues, "replay_receipt");
        determinism_verify_replay_receipt(&root, &replay_path)
    } else {
        Value::Null
    };
    if replay_verification.is_object() {
        write_evidence_json(
            &replay_verification_path,
            &replay_verification,
            &mut issues,
            "replay_verification",
        );
    }

    let release = release_readiness_receipt(root.clone(), default_policy_path().into());
    write_evidence_json(&release_path, &release, &mut issues, "release_readiness");

    let exported_agent_receipt = if options.agent_receipt.is_some() {
        match fs::copy(&agent_path, &exported_agent_path) {
            Ok(_) => read_json_artifact(&exported_agent_path, AGENT_RECEIPT_MAX_BYTES)
                .unwrap_or(Value::Null),
            Err(_) => {
                issues.push(evidence_export_issue(
                    "export.agent_receipt_copy_failed",
                    "agent_receipt",
                    "readable agent receipt fixture",
                    "failed",
                ));
                Value::Null
            }
        }
    } else {
        match export_agent_receipt_bound_to_replay(
            &agent_path,
            &exported_agent_path,
            &replay_verification,
            &policy,
        ) {
            Ok(receipt) => receipt,
            Err(_) => {
                issues.push(evidence_export_issue(
                    "export.agent_receipt_generate_failed",
                    "agent_receipt",
                    "agent receipt bound to exported replay evidence",
                    "failed",
                ));
                Value::Null
            }
        }
    };

    let bundle = json!({
        "agent_receipt_path": "agent-receipt.json",
        "replay_receipt_path": "replay-agent.json",
        "replay_verification_path": "replay-verification.json",
        "release_readiness_path": "release-readiness.json",
        "coverage_audit_path": "coverage-audit.json",
        "agent_receipt_hash": receipt_stated_or_computed_hash(&root, &exported_agent_receipt),
        "replay_receipt_hash": receipt_stated_or_computed_hash(&root, &replay),
        "replay_verification_receipt_hash": receipt_stated_or_computed_hash(&root, &replay_verification),
        "release_readiness_receipt_hash": receipt_stated_or_computed_hash(&root, &release),
        "coverage_audit_receipt_hash": receipt_stated_or_computed_hash(&root, &coverage),
    });
    write_evidence_json(&bundle_path, &bundle, &mut issues, "agent_evidence_bundle");

    let bundle_verification = agent_evidence_bundle_verification_receipt(&root, &bundle_path);
    write_evidence_json(
        &bundle_verification_path,
        &bundle_verification,
        &mut issues,
        "agent_evidence_bundle_verification",
    );

    let manifest_options = EvidenceManifestGenerateOptions {
        root: root.clone(),
        bundle: Some(bundle_path.clone()),
        agent_receipt: Some(exported_agent_path.clone()),
        replay: Some(replay_path.clone()),
        replay_verification: Some(replay_verification_path.clone()),
        release_readiness: Some(release_path.clone()),
        coverage_audit: Some(coverage_path.clone()),
        registry: None,
        registry_verification: None,
        registry_selection: None,
        registry_fetch_plan: None,
        signer_metadata: None,
        anchor_metadata: None,
        out: None,
    };
    let (manifest, manifest_issues) = evidence_manifest_generate_manifest(&manifest_options);
    issues.extend(
        manifest_issues
            .into_iter()
            .map(|issue| evidence_export_nested_issue("export.manifest_generate_failed", issue)),
    );
    let manifest = manifest.unwrap_or(Value::Null);
    if manifest.is_object() {
        let relative_manifest = evidence_manifest_with_relative_paths(&manifest, &out_dir);
        write_evidence_json(
            &manifest_path,
            &relative_manifest,
            &mut issues,
            "evidence_manifest",
        );
    }

    let manifest_verification = evidence_manifest_verification_receipt(&root, &manifest_path);
    write_evidence_json(
        &manifest_verification_path,
        &manifest_verification,
        &mut issues,
        "evidence_manifest_verification",
    );

    if bundle_verification.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_export_issue(
            "export.bundle_verification_failed",
            "agent_evidence_bundle_verification",
            "valid bundle verification receipt",
            "invalid",
        ));
    }
    if manifest_verification.get("valid").and_then(Value::as_bool) != Some(true) {
        issues.push(evidence_export_issue(
            "export.manifest_verification_failed",
            "evidence_manifest_verification",
            "valid manifest verification receipt",
            "invalid",
        ));
    }

    let mut artifacts = evidence_export_artifacts(&out_dir, &mut issues);
    artifacts.sort_by_key(|artifact| {
        artifact
            .get("role")
            .and_then(Value::as_str)
            .unwrap_or("")
            .to_string()
    });
    let manifest_hash = manifest_verification
        .get("manifest_hash")
        .cloned()
        .unwrap_or(Value::Null);
    let manifest_verification_hash = receipt_stated_or_computed_hash(&root, &manifest_verification);
    let receipt = evidence_export_final_receipt(
        &root,
        &out_dir,
        issues,
        artifacts,
        manifest_hash,
        manifest_verification_hash,
    );
    write_evidence_json(
        &out_dir.join("evidence-export-receipt.json"),
        &receipt,
        &mut Vec::new(),
        "evidence_export_receipt",
    );
    receipt
}

fn export_agent_receipt_bound_to_replay(
    template_path: &Path,
    output_path: &Path,
    replay_verification: &Value,
    policy: &Policy,
) -> Result<Value, Box<dyn Error>> {
    let mut receipt = read_json_artifact(template_path, AGENT_RECEIPT_MAX_BYTES)?;
    let evidence = json!({
        "replay_artifact_path": "replay-agent.json",
        "replay_receipt_hash": replay_verification
            .get("replay_receipt_hash")
            .cloned()
            .unwrap_or(Value::Null),
        "replay_policy_hash": replay_verification
            .get("replay_policy_hash")
            .cloned()
            .unwrap_or(Value::Null),
        "replay_environment_attestation_hash": replay_verification
            .get("environment_attestation_hash")
            .cloned()
            .unwrap_or(Value::Null),
        "replay_selected_cases": replay_verification
            .get("selected_cases")
            .cloned()
            .unwrap_or_else(|| json!([])),
        "replay_selected_groups": replay_verification
            .get("selected_groups")
            .cloned()
            .unwrap_or_else(|| json!([])),
        "replay_verdict": if replay_verification.get("valid").and_then(Value::as_bool) == Some(true) { "pass" } else { "fail" },
    });
    receipt["replay_evidence"] = evidence;
    if let Some(object) = receipt.as_object_mut() {
        object.remove("receipt_hash");
    }
    let validation = agent_receipt_validation_receipt(
        "agent-receipt validate",
        Some(output_path.to_path_buf()),
        &receipt,
        false,
        policy,
    );
    receipt["receipt_hash"] = validation
        .get("agent_receipt_hash")
        .cloned()
        .unwrap_or(Value::Null);
    let final_validation = agent_receipt_validation_receipt(
        "agent-receipt validate",
        Some(output_path.to_path_buf()),
        &receipt,
        false,
        policy,
    );
    if final_validation.get("valid").and_then(Value::as_bool) != Some(true) {
        return Err("generated agent receipt failed validation".into());
    }
    write_evidence_json(output_path, &receipt, &mut Vec::new(), "agent_receipt");
    Ok(receipt)
}

fn evidence_manifest_with_relative_paths(manifest: &Value, out_dir: &Path) -> Value {
    let mut value = manifest.clone();
    if let Some(entries) = value.get_mut("artifacts").and_then(Value::as_array_mut) {
        for entry in entries {
            if let Some(path_text) = entry.get("path").and_then(Value::as_str) {
                let path = PathBuf::from(path_text);
                if let Ok(relative) = path.strip_prefix(out_dir) {
                    entry["path"] = json!(path_string(relative));
                }
            }
        }
    }
    value
}

fn evidence_export_artifacts(out_dir: &Path, issues: &mut Vec<Value>) -> Vec<Value> {
    let roles = [
        ("agent_evidence_bundle", "agent-evidence-bundle.json", true),
        (
            "agent_evidence_bundle_verification",
            "agent-evidence-bundle-verification.json",
            true,
        ),
        ("agent_receipt", "agent-receipt.json", true),
        ("coverage_audit", "coverage-audit.json", true),
        ("evidence_manifest", "evidence-manifest.json", true),
        (
            "evidence_manifest_verification",
            "evidence-manifest-verification.json",
            true,
        ),
        ("release_readiness", "release-readiness.json", true),
        ("replay_receipt", "replay-agent.json", true),
        ("replay_verification", "replay-verification.json", true),
    ];
    roles
        .iter()
        .map(|(role, relative, required)| {
            let path = out_dir.join(relative);
            let hash = match fs::read(&path) {
                Ok(bytes) => Value::String(blake3_hex(&bytes)),
                Err(_) => {
                    issues.push(evidence_export_issue(
                        "export.artifact_missing",
                        role,
                        "generated artifact file",
                        "missing",
                    ));
                    Value::Null
                }
            };
            json!({
                "role": role,
                "path": relative,
                "hash": hash,
                "required": required,
            })
        })
        .collect()
}

fn write_evidence_json(path: &Path, value: &Value, issues: &mut Vec<Value>, role: &str) {
    if let Some(parent) = path.parent() {
        if fs::create_dir_all(parent).is_err() {
            issues.push(evidence_export_issue(
                "export.artifact_parent_create_failed",
                role,
                "writable parent directory",
                "failed",
            ));
            return;
        }
    }
    if fs::write(
        path,
        format!(
            "{}\n",
            serde_json::to_string_pretty(value).unwrap_or_default()
        ),
    )
    .is_err()
    {
        issues.push(evidence_export_issue(
            "export.artifact_write_failed",
            role,
            "writable JSON artifact",
            "failed",
        ));
    }
}

fn evidence_export_nested_issue(code: &str, issue: Value) -> Value {
    json!({
        "code": code,
        "field_path": issue.get("field_path").and_then(Value::as_str).unwrap_or("nested"),
        "expected": issue.get("expected").and_then(Value::as_str).unwrap_or("valid nested receipt"),
        "actual": issue.get("actual").and_then(Value::as_str).unwrap_or("invalid"),
    })
}

fn evidence_export_issue(code: &str, field_path: &str, expected: &str, actual: &str) -> Value {
    json!({
        "code": code,
        "field_path": field_path,
        "expected": expected,
        "actual": actual,
    })
}

fn evidence_export_final_receipt(
    root: &Path,
    _out_dir: &Path,
    mut issues: Vec<Value>,
    artifacts: Vec<Value>,
    manifest_hash: Value,
    manifest_verification_receipt_hash: Value,
) -> Value {
    issues.sort_by_key(canonical_json_hash);
    let valid = issues.is_empty();
    with_portable_receipt_hash(
        root,
        json!({
            "command": "evidence export",
            "root": path_string(root),
            "output": "<out>",
            "valid": valid,
            "verdict": if valid { "pass" } else { "fail" },
            "issues": issues,
            "artifacts": artifacts,
            "manifest_hash": manifest_hash,
            "manifest_verification_receipt_hash": manifest_verification_receipt_hash,
        }),
    )
}
