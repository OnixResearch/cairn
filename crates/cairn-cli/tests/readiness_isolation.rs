use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

#[path = "readiness_isolation/profile_admission.rs"]
mod profile_admission;

type TestResult<T = ()> = Result<T, Box<dyn Error>>;
const GATES: [(usize, &str); 3] = [(1, "proposal"), (2, "design"), (3, "tasks")];

fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn fixture(name: &str, profile: &str) -> TestResult<PathBuf> {
    let root = std::env::temp_dir().join(format!(
        "cairn-readiness-isolation-{name}-{}",
        std::process::id()
    ));
    fs::create_dir_all(root.join(".cairn/archive"))?;
    for path in [
        ".cairn/specs/demo/spec.md",
        ".cairn/changes/demo/proposal.md",
        ".cairn/changes/demo/design.md",
        ".cairn/changes/demo/tasks.md",
        ".cairn/changes/demo/specs/demo/spec.md",
    ] {
        let destination = root.join(path);
        fs::create_dir_all(destination.parent().ok_or("parent")?)?;
        fs::copy(repository().join("fixtures/basic").join(path), destination)?;
    }
    for file in ["contract.json", "results.json"] {
        fs::copy(
            repository().join("fixtures/agent-completion").join(file),
            root.join(file),
        )?;
    }
    fs::copy(
        repository().join("cairn-policy/generated/cairn-policy.json"),
        root.join("policy.json"),
    )?;
    let matrix: Value = serde_json::from_slice(&fs::read(
        repository().join("fixtures/cross-repo-dependencies/matrix.json"),
    )?)?;
    fs::write(
        root.join("upstream.json"),
        serde_json::to_vec(&matrix["positive"][0]["evidence"])?,
    )?;
    let mut metadata = json!({"cross_repo_dependencies": [matrix["positive"][0]["dependency"]]});
    if profile != "compatibility" {
        metadata["workflow_profile"] =
            json!({"id": profile, "version": 1, "source": "change_create"});
    }
    fs::write(
        root.join(".cairn/changes/demo/metadata.json"),
        serde_json::to_vec(&metadata)?,
    )?;
    Ok(root)
}

fn run(args: &[&str]) -> TestResult<Value> {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(repository())
        .args(args)
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}

// Includes directories and link destinations; inspection cannot silently create
// evidence, rewrite a task, replace a link, or leave a mutation staging tree.
fn tree(root: &Path) -> TestResult<BTreeMap<PathBuf, Vec<u8>>> {
    let mut found = BTreeMap::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(directory)? {
            let entry = entry?;
            let path = entry.path();
            let relative = path.strip_prefix(root)?.to_path_buf();
            if entry.file_type()?.is_symlink() {
                found.insert(
                    relative,
                    format!("link:{}", fs::read_link(path)?.display()).into_bytes(),
                );
            } else if entry.file_type()?.is_dir() {
                found.insert(relative, b"directory".to_vec());
                pending.push(path);
            } else {
                found.insert(relative, fs::read(path)?);
            }
        }
    }
    Ok(found)
}

fn observe(root: &Path) -> TestResult<Value> {
    let before = tree(root)?;
    let policy = root.join("policy.json");
    let subject = "c".repeat(64);
    let input = json!({"root": root, "change": "demo", "policy": policy,
        "contract": "contract.json", "results": "results.json", "subject_hash": subject,
        "cross_repo_evidence": ["upstream.json"]});
    let report = run(&[
        "readiness",
        "demo",
        "--root",
        root.to_str().ok_or("root")?,
        "--policy",
        policy.to_str().ok_or("policy")?,
        "--contract",
        "contract.json",
        "--results",
        "results.json",
        "--subject-hash",
        &subject,
        "--cross-repo-evidence",
        "upstream.json",
    ])?;
    assert_eq!(tree(root)?, before, "CLI effects");
    let mcp = run(&["mcp", "--call", "cairn_readiness", &input.to_string()])?;
    assert_eq!(mcp["structuredContent"], report, "transport parity");
    assert_eq!(tree(root)?, before, "MCP effects");
    assert_eq!(report["mutation_authorized"], false);
    for action in report["actions"].as_array().ok_or("actions")? {
        assert_eq!(action["authority"], "advisory");
        assert!(!action["argv"]
            .as_array()
            .ok_or("argv")?
            .iter()
            .any(|arg| arg == "--execute"));
    }
    Ok(report)
}

fn gate_issues(value: &Value) -> Vec<(String, String)> {
    value
        .as_array()
        .unwrap()
        .iter()
        .map(|issue| {
            (
                issue["code"].as_str().unwrap().to_string(),
                issue["message"].as_str().unwrap().to_string(),
            )
        })
        .collect()
}

#[test]
// r[verify cairn.unified_readiness.decisions]
// r[verify cairn.unified_readiness.transport]
// r[verify cairn.unified_readiness.boundary]
fn explicit_profiles_keep_earlier_gates_and_metadata_when_later_text_is_unreadable() -> TestResult {
    for profile in [
        "compatibility",
        "spec-driven",
        "assurance",
        "no-spec-delta",
        "outcome-machine",
    ] {
        for (artifact, first_unavailable) in [("design.md", 2), ("tasks.md", 3)] {
            let root = fixture(&format!("{profile}-{artifact}"), profile)?;
            let before = observe(&root)?;
            for (index, stage) in GATES {
                let original = run(&[
                    "gate",
                    stage,
                    "demo",
                    "--root",
                    root.to_str().ok_or("root")?,
                    "--policy",
                    root.join("policy.json").to_str().ok_or("policy")?,
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
            let cross = before["stages"][5]["decision"]["cross_repo_dependencies"].clone();
            assert_eq!(cross["valid"], true);
            fs::write(root.join(".cairn/changes/demo").join(artifact), [0xff])?;
            let after = observe(&root)?;
            for index in 1..first_unavailable {
                assert_eq!(
                    after["stages"][index]["state"], before["stages"][index]["state"],
                    "{profile}/{artifact}: {after}"
                );
                for field in ["verdict", "issues"] {
                    assert_eq!(
                        after["stages"][index]["decision"][field],
                        before["stages"][index]["decision"][field],
                        "{profile}/{artifact}/{field}"
                    );
                }
                assert!(after["stages"][index]["decision"]["profile_input_hash"].is_string());
            }
            for index in first_unavailable..4 {
                assert_eq!(after["stages"][index]["state"], "unavailable");
            }
            assert_eq!(
                after["stages"][0]["decision"]["cross_repo_dependencies"],
                cross
            );
            assert_eq!(after["stages"][4], before["stages"][4]);
            for index in [5, 6] {
                assert_eq!(after["stages"][index]["state"], "unavailable");
            }
            fs::remove_dir_all(root)?;
        }
    }
    Ok(())
}

#[test]
// r[verify cairn.unified_readiness.conformance]
fn original_mutation_previews_and_independent_gate_admission_agree() -> TestResult {
    for (sync_gate, archive_gate) in [(true, true), (true, false), (false, true), (false, false)] {
        for checked in [false, true] {
            let root = fixture(
                &format!("mutation-{sync_gate}-{archive_gate}-{checked}"),
                "compatibility",
            )?;
            let tasks = root.join(".cairn/changes/demo/tasks.md");
            if checked {
                fs::write(
                    &tasks,
                    fs::read_to_string(&tasks)?.replace("- [ ]", "- [x]"),
                )?;
            }
            let policy_path = root.join("policy.json");
            let mut policy: Value = serde_json::from_slice(&fs::read(&policy_path)?)?;
            policy["mutation_policy"]["sync"]["require_passing_gate"] = json!(sync_gate);
            policy["mutation_policy"]["archive"]["require_passing_gate"] = json!(archive_gate);
            fs::write(&policy_path, serde_json::to_vec(&policy)?)?;
            let policy_hash = cairn_core::canonical_json_hash(&policy);
            let contract_path = root.join("contract.json");
            let mut contract: Value = serde_json::from_slice(&fs::read(&contract_path)?)?;
            contract["policy_hash"] = json!(policy_hash.clone());
            fs::write(&contract_path, serde_json::to_vec(&contract)?)?;
            let results_path = root.join("results.json");
            let mut results: Value = serde_json::from_slice(&fs::read(&results_path)?)?;
            for result in results.as_array_mut().ok_or("results")? {
                result["policy_hash"] = json!(policy_hash);
            }
            fs::write(&results_path, serde_json::to_vec(&results)?)?;
            let before = observe(&root)?;
            if checked {
                let mut preview_count = 0;
                for stage in ["sync", "archive"] {
                    if let Some(action) = before["actions"]
                        .as_array()
                        .ok_or("actions")?
                        .iter()
                        .find(|action| action["kind"] == format!("{stage}_preview"))
                    {
                        preview_count += 1;
                        let argv = action["argv"].as_array().ok_or("preview argv")?;
                        assert_eq!(argv[0], "cairn");
                        assert_eq!(argv[1], stage);
                        assert_eq!(argv[2], "demo");
                        assert!(argv.iter().any(|arg| arg == "--root"));
                        assert!(argv.iter().any(|arg| arg == "--policy"));
                        assert!(argv.iter().any(|arg| arg == "--cross-repo-evidence"));
                        assert!(!argv.iter().any(|arg| arg == "--execute"));
                    }
                }
                assert!(
                    preview_count > 0,
                    "checked fixture should offer a preview action: {before}"
                );
            }
            let unchanged = tree(&root)?;
            for (index, stage) in [(5, "sync"), (6, "archive")] {
                let original = run(&[
                    stage,
                    "demo",
                    "--root",
                    root.to_str().ok_or("root")?,
                    "--policy",
                    policy_path.to_str().ok_or("policy")?,
                    "--cross-repo-evidence",
                    "upstream.json",
                ])?;
                assert_eq!(
                    before["stages"][index]["decision"]["reasons"], original["reasons"],
                    "{stage}/{checked}: {original}"
                );
                assert_eq!(
                    before["stages"][index]["decision"]["cross_repo_dependencies"],
                    original["cross_repo_dependencies"]
                );
            }
            assert_eq!(tree(&root)?, unchanged, "legacy previews must not execute");
            let unrelated = root.join(".cairn/specs/unrelated/spec.md");
            fs::create_dir_all(unrelated.parent().ok_or("parent")?)?;
            fs::write(unrelated, [0xff])?;
            let after = observe(&root)?;
            for index in 0..5 {
                assert_eq!(after["stages"][index], before["stages"][index]);
            }
            for index in [5, 6] {
                let required = before["stages"][index]["decision"]["gate_required"] == true;
                assert_eq!(after["stages"][index]["state"] == "unavailable", required);
                if !required {
                    assert_eq!(after["stages"][index], before["stages"][index]);
                }
            }
            fs::remove_dir_all(root)?;
        }
    }
    Ok(())
}

#[test]
// r[verify cairn.unified_readiness.conformance]
fn unadmitted_policy_or_profile_never_substitutes_defaults_for_missing_inputs() -> TestResult {
    for failure in [
        "missing-policy",
        "malformed-policy",
        "invalid-policy",
        "unknown-profile",
        "malformed-metadata",
    ] {
        let root = fixture(failure, "spec-driven")?;
        let before = observe(&root)?;
        match failure {
            "missing-policy" => fs::remove_file(root.join("policy.json"))?,
            "malformed-policy" => fs::write(root.join("policy.json"), b"{")?,
            "invalid-policy" => fs::write(root.join("policy.json"), b"{}")?,
            "unknown-profile" => {
                let path = root.join(".cairn/changes/demo/metadata.json");
                let mut metadata: Value = serde_json::from_slice(&fs::read(&path)?)?;
                metadata["workflow_profile"]["id"] = json!("not-admitted");
                fs::write(path, serde_json::to_vec(&metadata)?)?;
            }
            _ => fs::write(root.join(".cairn/changes/demo/metadata.json"), b"{")?,
        }
        let after = observe(&root)?;
        if failure.ends_with("policy") {
            assert_eq!(after["stages"][4]["state"], "unavailable");
            assert!(after["stages"][4]["decision"]["admitted_inputs"]["contract_hash"].is_string());
            assert!(after["stages"][4]["decision"].get("verdict").is_none());
        } else {
            assert_eq!(after["stages"][4], before["stages"][4]);
        }
        for index in [0, 1, 2, 3, 5, 6] {
            assert_eq!(after["stages"][index]["state"], "unavailable", "{failure}");
        }
        if failure == "unknown-profile" {
            assert_eq!(
                after["stages"][0]["decision"]["cross_repo_dependencies"],
                before["stages"][5]["decision"]["cross_repo_dependencies"]
            );
        } else if failure.ends_with("policy") {
            assert!(after["stages"][0]["decision"]["metadata"]["input_hash"].is_string());
        } else {
            assert!(after["stages"][0]["decision"]["cross_repo_dependencies"].is_null());
        }
        // Changed evidence binds the report even when no policy permits its evaluation.
        fs::write(root.join("results.json"), b"[]")?;
        let missing = observe(&root)?;
        assert_eq!(
            missing["stages"][4]["state"],
            if failure.ends_with("policy") {
                "unavailable"
            } else {
                "blocked"
            }
        );
        assert_ne!(missing["receipt_hash"], after["receipt_hash"]);
        assert!(!missing["actions"]
            .as_array()
            .unwrap()
            .iter()
            .any(|a| a["kind"].as_str().unwrap().ends_with("_preview")));
        fs::remove_dir_all(root)?;
    }
    Ok(())
}
