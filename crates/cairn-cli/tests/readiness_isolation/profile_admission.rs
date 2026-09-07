use super::*;

const ARTIFACT_BOUND: usize = 4096;

#[test]
// r[verify cairn.unified_readiness.decisions]
// r[verify cairn.unified_readiness.boundary]
// r[verify cairn.unified_readiness.transport]
// r[verify cairn.unified_readiness.conformance]
fn admission_failures_preserve_only_independent_profile_gates() -> TestResult {
    for (profile, admission, first_stage) in [
        (
            "no-spec-delta",
            cairn_core::WORKFLOW_ADMISSION_ACCEPTANCE,
            1,
        ),
        (
            "assurance",
            cairn_core::WORKFLOW_ADMISSION_SEMANTIC_REVIEW,
            3,
        ),
        (
            "outcome-machine",
            cairn_core::WORKFLOW_ADMISSION_OUTCOME_MACHINE,
            2,
        ),
    ] {
        for early in [false, true] {
            for failure in ["non-utf8", "read-bound", "leaf-link", "parent-link"] {
                let root = fixture(&format!("{profile}-{early}-{failure}"), profile)?;
                let change = root.join(".cairn/changes/demo");
                let policy_path = root.join("policy.json");
                let mut policy: Value = serde_json::from_slice(&fs::read(&policy_path)?)?;
                let schema = policy["schemas"]
                    .as_array_mut()
                    .ok_or("schemas")?
                    .iter_mut()
                    .find(|schema| schema["id"] == profile)
                    .ok_or("schema")?;
                schema["bounds"]["max_artifact_bytes"] = json!(ARTIFACT_BOUND);
                if profile == "assurance" {
                    schema["review_obligations"][0]["required_artifacts"] = json!(["proposal"]);
                }
                let artifact = schema["artifacts"]
                    .as_array_mut()
                    .ok_or("artifacts")?
                    .iter_mut()
                    .find(|artifact| artifact["structural_admission"] == admission)
                    .ok_or("artifact")?;
                // The outcome-machine schema pins its admission artifact to
                // `outcome-machine.json` at the design stage, so only its two
                // sibling profiles accept the relocated-path fixture.
                let relocate = profile != "outcome-machine";
                if relocate {
                    artifact["output_path"] = json!("admission/input");
                }
                // The outcome-machine schema requires its admission artifact to
                // stay at the design stage, so the early-stage rearrangement does
                // not apply to that profile.
                let early_applies = early && relocate;
                if early_applies {
                    artifact["stage"] = json!("proposal");
                    artifact["requires"] = json!([]);
                }
                // Assert actual policy admission, not only JSON shape.
                cairn_core::validate_policy_json(policy.clone())?;
                fs::write(&policy_path, serde_json::to_vec(&policy)?)?;
                if relocate {
                    fs::create_dir_all(change.join("admission"))?;
                }
                let path = change.join(if relocate {
                    "admission/input"
                } else {
                    "outcome-machine.json"
                });
                fs::write(
                    &path,
                    if profile == "no-spec-delta" {
                        "# Acceptance\n\na[demo] A bounded acceptance criterion.\n"
                    } else {
                        "{}"
                    },
                )?;
                let before = observe(&root)?;
                let baseline_tree = tree(&root)?;
                for (index, stage) in GATES {
                    let original = run(&[
                        "gate",
                        stage,
                        "demo",
                        "--root",
                        root.to_str().ok_or("root")?,
                        "--policy",
                        policy_path.to_str().ok_or("policy")?,
                    ])?;
                    assert_eq!(
                        before["stages"][index]["decision"]["verdict"],
                        original["verdict"]
                    );
                    assert_eq!(
                        gate_issues(&before["stages"][index]["decision"]["issues"]),
                        gate_issues(&original["issues"])
                    );
                }
                assert_eq!(tree(&root)?, baseline_tree, "original validator effects");
                match failure {
                    "non-utf8" => fs::write(&path, [0xff])?,
                    "read-bound" => fs::write(&path, vec![b'x'; ARTIFACT_BOUND + 1])?,
                    #[cfg(unix)]
                    "leaf-link" => {
                        let real = if relocate {
                            change.join("admission/real")
                        } else {
                            change.join("outcome-machine-real.json")
                        };
                        fs::rename(&path, &real)?;
                        std::os::unix::fs::symlink(real.file_name().ok_or("name")?, &path)?;
                    }
                    #[cfg(unix)]
                    "parent-link" if relocate => {
                        fs::rename(change.join("admission"), change.join("real-admission"))?;
                        std::os::unix::fs::symlink("real-admission", change.join("admission"))?;
                    }
                    _ => {
                        fs::remove_dir_all(root)?;
                        continue;
                    }
                }
                let after = observe(&root)?;
                let first_unavailable = if early_applies { 1 } else { first_stage };
                for (index, _) in GATES {
                    if index >= first_unavailable {
                        assert_eq!(
                            after["stages"][index]["state"], "unavailable",
                            "{profile}/{early}/{failure}: {after}"
                        );
                    } else {
                        for field in ["verdict", "issues"] {
                            assert_eq!(
                                after["stages"][index]["decision"][field],
                                before["stages"][index]["decision"][field],
                                "{profile}/{early}/{failure}/{field}"
                            );
                        }
                        assert!(
                            after["stages"][index]["decision"]["profile_input_hash"].is_string()
                        );
                    }
                }
                assert_eq!(
                    after["stages"][4], before["stages"][4],
                    "completion isolation"
                );
                assert_eq!(
                    after["stages"][0]["decision"]["cross_repo_dependencies"],
                    before["stages"][5]["decision"]["cross_repo_dependencies"],
                    "metadata isolation {profile}/{early}/{failure}: {after}"
                );
                fs::remove_dir_all(root)?;
            }
        }
    }
    Ok(())
}
