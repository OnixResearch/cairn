use cairn_core::verified::incremental_observations::{
    build_snapshot, ObservationCohort, ObservationView,
};
use serde_json::{json, Value};
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicUsize, Ordering},
};

static NEXT_DIR: AtomicUsize = AtomicUsize::new(0);

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn temp_dir(label: &str) -> Result<PathBuf, Box<dyn Error>> {
    let index = NEXT_DIR.fetch_add(1, Ordering::Relaxed);
    let path = std::env::temp_dir().join(format!(
        "cairn-incremental-{label}-{}-{index}",
        std::process::id()
    ));
    if path.exists() {
        fs::remove_dir_all(&path)?;
    }
    fs::create_dir_all(&path)?;
    Ok(path)
}

fn copy_dir(source: &Path, destination: &Path) -> Result<(), Box<dyn Error>> {
    fs::create_dir_all(destination)?;
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let target = destination.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else {
            fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}

fn basic_fixture(label: &str) -> Result<PathBuf, Box<dyn Error>> {
    let root = temp_dir(label)?;
    copy_dir(
        &repo_root().join("fixtures/basic/.cairn"),
        &root.join(".cairn"),
    )?;
    Ok(root)
}

fn run(cwd: &Path, args: &[&str]) -> Result<std::process::Output, Box<dyn Error>> {
    Ok(Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(cwd)
        .args(args)
        .output()?)
}

fn run_json(cwd: &Path, args: &[&str]) -> Result<Value, Box<dyn Error>> {
    let output = run(cwd, args)?;
    assert!(
        output.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn run_failure(cwd: &Path, args: &[&str]) -> Result<String, Box<dyn Error>> {
    let output = run(cwd, args)?;
    assert!(!output.status.success(), "unexpected success: {args:?}");
    Ok(String::from_utf8_lossy(&output.stderr).into_owned())
}

fn mcp_call(cwd: &Path, arguments: Value) -> Result<Value, Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(cwd)
        .args(["mcp", "--call", "cairn_observe", &arguments.to_string()])
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}

fn observe(root: &Path, args: &[&str]) -> Result<Value, Box<dyn Error>> {
    let mut all = vec!["observe"];
    all.extend_from_slice(args);
    all.push("--root");
    let root_text = root.to_string_lossy().into_owned();
    all.push(&root_text);
    run_json(&repo_root(), &all)
}

fn tree(root: &Path) -> Result<Vec<String>, Box<dyn Error>> {
    let mut paths = Vec::new();
    let mut pending = vec![root.to_path_buf()];
    while let Some(directory) = pending.pop() {
        for entry in fs::read_dir(&directory)? {
            let entry = entry?;
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                pending.push(path.clone());
            }
            paths.push(
                path.strip_prefix(root)?
                    .to_string_lossy()
                    .replace('\\', "/"),
            );
        }
    }
    paths.sort();
    Ok(paths)
}

#[test]
// r[verify cairn.incremental_observations.pages]
// r[verify cairn.incremental_observations.conformance]
fn active_pages_reconstruct_restart_and_replay_exactly() -> Result<(), Box<dyn Error>> {
    let root = basic_fixture("active-pages")?;
    let baseline = observe(&root, &["active", "--output", "baseline.json"])?;
    assert_eq!(baseline["schema"], "cairn.incremental-observation.v1");
    assert_eq!(baseline["complete"], true);
    assert_eq!(baseline["entries"][0]["identity"], "change:demo");

    let mut collected = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let mut args = vec!["active"];
        if cursor.is_none() {
            args.push("--page-entries");
            args.push("1");
        }
        let requested_cursor = cursor.clone();
        if let Some(cursor) = &cursor {
            args.push("--cursor");
            args.push(cursor);
        }
        let page = observe(&root, &args)?;
        assert_eq!(page["schema"], "cairn.incremental-observation-page.v1");
        if let Some(requested_cursor) = &requested_cursor {
            // A restarted process replays the same page identity.
            let replay = observe(&root, &["active", "--cursor", requested_cursor])?;
            assert_eq!(replay, page);
            assert_eq!(replay["page_hash"], page["page_hash"]);
        }
        collected.extend(
            page["entries"]
                .as_array()
                .ok_or("page entries")?
                .iter()
                .cloned(),
        );
        match page["next_cursor"].as_str() {
            None => {
                assert_eq!(page["complete"], true);
                break;
            }
            Some(next) => cursor = Some(next.to_string()),
        }
    }
    assert_eq!(Value::Array(collected), baseline["entries"]);
    Ok(())
}

#[test]
// r[verify cairn.incremental_observations.delta]
fn active_difference_reports_edited_added_removed_and_renamed_entries() -> Result<(), Box<dyn Error>>
{
    let root = basic_fixture("active-diff")?;
    let baseline = observe(&root, &["active", "--output", "baseline.json"])?;

    let unchanged = observe(
        &root,
        &["active", "--prior", "baseline.json", "--mode", "unchanged"],
    )?;
    assert_eq!(unchanged["unchanged"], true);
    assert_eq!(
        unchanged["schema"],
        "cairn.incremental-observation-unchanged.v1"
    );
    assert_eq!(unchanged["current_hash"], unchanged["prior_hash"]);

    // A source edit that does not change the listing entry still changes the
    // observation identity; an unchanged claim must not hide a dirty source.
    let proposal = root.join(".cairn/changes/demo/proposal.md");
    let proposal_text = fs::read(&proposal)?;
    fs::write(&proposal, b"## Why\n\nEdited source only.\n")?;
    let dirty = observe(
        &root,
        &["active", "--prior", "baseline.json", "--mode", "unchanged"],
    )?;
    assert_eq!(dirty["unchanged"], false);
    fs::write(&proposal, proposal_text)?;

    let tasks = root.join(".cairn/changes/demo/tasks.md");
    let text = fs::read_to_string(&tasks)?;
    fs::write(&tasks, text.replace("- [ ]", "- [x]"))?;
    let edited = observe(
        &root,
        &["active", "--prior", "baseline.json", "--mode", "difference"],
    )?;
    assert_eq!(
        edited["schema"],
        "cairn.incremental-observation-difference.v1"
    );
    assert_eq!(edited["unchanged"], false);
    assert!(edited["changed"]
        .as_array()
        .ok_or("changed")?
        .iter()
        .any(|entry| entry["identity"] == "change:demo"));

    let duplicate = root.join(".cairn/changes/demo-two");
    fs::create_dir_all(duplicate.join("specs"))?;
    for file in ["proposal.md", "design.md", "tasks.md"] {
        fs::copy(
            root.join(".cairn/changes/demo").join(file),
            duplicate.join(file),
        )?;
    }
    fs::create_dir_all(duplicate.join("specs/demo"))?;
    fs::copy(
        root.join(".cairn/changes/demo/specs/demo/spec.md"),
        duplicate.join("specs/demo/spec.md"),
    )?;
    let added = observe(
        &root,
        &["active", "--prior", "baseline.json", "--mode", "difference"],
    )?;
    assert!(added["added"]
        .as_array()
        .ok_or("added")?
        .iter()
        .any(|entry| entry["identity"] == "change:demo-two"));

    let _ = observe(&root, &["active", "--output", "two.json"])?;
    fs::rename(duplicate, root.join(".cairn/changes/demo-renamed"))?;
    let renamed = observe(
        &root,
        &["active", "--prior", "two.json", "--mode", "difference"],
    )?;
    assert!(renamed["added"]
        .as_array()
        .ok_or("added")?
        .iter()
        .any(|entry| entry["identity"] == "change:demo-renamed"));
    assert!(renamed["removed"]
        .as_array()
        .ok_or("removed")?
        .iter()
        .any(|entry| entry["identity"] == "change:demo-two"));
    Ok(())
}

#[test]
// r[verify cairn.incremental_observations.pages]
// r[verify cairn.incremental_observations.delta]
fn readiness_pages_and_differences_reuse_selected_entries() -> Result<(), Box<dyn Error>> {
    let root = basic_fixture("readiness")?;
    let baseline = observe(&root, &["readiness", "demo", "--output", "readiness.json"])?;
    assert_eq!(baseline["schema"], "cairn.incremental-observation.v1");
    assert_eq!(baseline["cohort"]["view"]["view"], "readiness");
    assert!(baseline["entries"]
        .as_array()
        .ok_or("entries")?
        .iter()
        .any(|entry| entry["identity"] == "stage:tasks"));

    let mut collected = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let mut args = vec!["readiness", "demo"];
        if cursor.is_none() {
            args.push("--page-entries");
            args.push("2");
        }
        if let Some(cursor) = &cursor {
            args.push("--cursor");
            args.push(cursor);
        }
        let page = observe(&root, &args)?;
        collected.extend(
            page["entries"]
                .as_array()
                .ok_or("page entries")?
                .iter()
                .cloned(),
        );
        match page["next_cursor"].as_str() {
            None => break,
            Some(next) => cursor = Some(next.to_string()),
        }
    }
    assert_eq!(Value::Array(collected), baseline["entries"]);

    let tasks = root.join(".cairn/changes/demo/tasks.md");
    let text = fs::read_to_string(&tasks)?;
    fs::write(&tasks, text.replace("- [ ]", "- [x]"))?;
    let diff = observe(
        &root,
        &[
            "readiness",
            "demo",
            "--prior",
            "readiness.json",
            "--mode",
            "difference",
        ],
    )?;
    assert_eq!(diff["unchanged"], false);
    assert!(!diff["changed"].as_array().ok_or("changed")?.is_empty());
    Ok(())
}

#[test]
// r[verify cairn.incremental_observations.identity]
// r[verify cairn.incremental_observations.delta]
fn context_entries_are_exact_and_stable_across_content_edits() -> Result<(), Box<dyn Error>> {
    let root = temp_dir("context")?;
    fs::create_dir_all(root.join(".cairn/specs/example"))?;
    fs::create_dir_all(root.join(".cairn/changes"))?;
    fs::create_dir_all(root.join(".cairn/archive"))?;
    let source = "# Example\n## Requirements\n### Requirement: A\nr[demo.a] The system MUST preserve cafe.\n#### Scenario: One\n- GIVEN input\n- WHEN read\n- THEN text\n### Requirement: B\nr[demo.b] The system MUST work.\n#### Scenario: Two\n- GIVEN input\n- WHEN read\n- THEN output\n";
    fs::write(root.join(".cairn/specs/example/spec.md"), source)?;
    let baseline = observe(
        &root,
        &[
            "context",
            "--scope",
            "accepted",
            "--artifact",
            "specs",
            "--spec",
            "example",
            "--requirement-id",
            "demo.a",
            "--requirement-id",
            "demo.b",
            "--output",
            "context.json",
        ],
    )?;
    assert_eq!(baseline["entries"].as_array().ok_or("entries")?.len(), 2);
    assert_eq!(
        baseline["entries"][0]["identity"],
        "context:requirement:demo.a"
    );
    assert_eq!(
        baseline["entries"][1]["identity"],
        "context:requirement:demo.b"
    );

    let mut collected = Vec::new();
    let mut cursor: Option<String> = None;
    loop {
        let mut args = vec![
            "context",
            "--scope",
            "accepted",
            "--artifact",
            "specs",
            "--spec",
            "example",
            "--requirement-id",
            "demo.a",
            "--requirement-id",
            "demo.b",
        ];
        if cursor.is_none() {
            args.push("--page-entries");
            args.push("1");
        }
        if let Some(cursor) = &cursor {
            args.push("--cursor");
            args.push(cursor);
        }
        let page = observe(&root, &args)?;
        collected.extend(
            page["entries"]
                .as_array()
                .ok_or("page entries")?
                .iter()
                .cloned(),
        );
        match page["next_cursor"].as_str() {
            None => break,
            Some(next) => cursor = Some(next.to_string()),
        }
    }
    assert_eq!(Value::Array(collected), baseline["entries"]);

    fs::write(
        root.join(".cairn/specs/example/spec.md"),
        source.replace("preserve cafe", "preserve cafe au lait"),
    )?;
    let diff = observe(
        &root,
        &[
            "context",
            "--scope",
            "accepted",
            "--artifact",
            "specs",
            "--spec",
            "example",
            "--requirement-id",
            "demo.a",
            "--requirement-id",
            "demo.b",
            "--prior",
            "context.json",
            "--mode",
            "difference",
        ],
    )?;
    assert_eq!(diff["changed"][0]["identity"], "context:requirement:demo.a");
    assert_eq!(diff["changed_count"], 1);
    Ok(())
}

#[test]
// r[verify cairn.incremental_observations.pages]
// r[verify cairn.incremental_observations.delta]
fn empty_observations_are_complete_and_unchanged() -> Result<(), Box<dyn Error>> {
    let root = temp_dir("empty")?;
    let root_text = root.to_string_lossy().into_owned();
    let initialized = run(&repo_root(), &["init", "--root", &root_text])?;
    assert!(
        initialized.status.success(),
        "{}",
        String::from_utf8_lossy(&initialized.stderr)
    );
    let empty = observe(&root, &["active", "--output", "empty.json"])?;
    assert_eq!(empty["total"], 0);
    assert_eq!(empty["included"], 0);
    assert_eq!(empty["omitted"], 0);
    assert_eq!(empty["complete"], true);
    let page = observe(&root, &["active", "--page-entries", "1"])?;
    assert_eq!(page["schema"], "cairn.incremental-observation-page.v1");
    assert_eq!(page["next_cursor"], Value::Null);
    assert_eq!(page["complete"], true);
    let diff = observe(
        &root,
        &["active", "--prior", "empty.json", "--mode", "difference"],
    )?;
    assert_eq!(diff["unchanged"], true);
    assert_eq!(diff["added_count"], 0);
    assert_eq!(diff["changed_count"], 0);
    assert_eq!(diff["removed_count"], 0);
    Ok(())
}

#[test]
// r[verify cairn.incremental_observations.identity]
// r[verify cairn.incremental_observations.storage]
fn negative_prior_paths_cursors_counts_and_cohorts_are_explicit() -> Result<(), Box<dyn Error>> {
    let root = basic_fixture("negative")?;
    let baseline = observe(&root, &["active", "--output", "baseline.json"])?;

    let outside = temp_dir("outside")?;
    let outside_prior = outside.join("prior.json");
    fs::write(&outside_prior, serde_json::to_vec(&baseline)?)?;
    let stderr = run_failure(
        &repo_root(),
        &[
            "observe",
            "active",
            "--root",
            &root.to_string_lossy(),
            "--prior",
            "../prior.json",
            "--mode",
            "difference",
        ],
    )?;
    assert!(stderr.contains("normal path"), "{stderr}");
    let absolute_output = outside.join("observed.json");
    let stderr = run_failure(
        &repo_root(),
        &[
            "observe",
            "active",
            "--root",
            &root.to_string_lossy(),
            "--output",
            &absolute_output.to_string_lossy(),
        ],
    )?;
    assert!(stderr.contains("normal path"), "{stderr}");

    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(&outside_prior, root.join("link.json"))?;
        let stderr = run_failure(
            &repo_root(),
            &[
                "observe",
                "active",
                "--root",
                &root.to_string_lossy(),
                "--prior",
                "link.json",
                "--mode",
                "difference",
            ],
        )?;
        assert!(
            stderr.contains("capability root")
                || stderr.contains("outside")
                || stderr.contains("symlink"),
            "{stderr}"
        );
        std::os::unix::fs::symlink(outside_prior.join("missing.json"), root.join("out.json"))?;
        let stderr = run_failure(
            &repo_root(),
            &[
                "observe",
                "active",
                "--root",
                &root.to_string_lossy(),
                "--output",
                "out.json",
            ],
        )?;
        assert!(
            stderr.contains("symlink") || stderr.contains("Permission"),
            "{stderr}"
        );
    }

    let second = root.join(".cairn/changes/demo-two");
    fs::create_dir_all(second.join("specs/demo"))?;
    for file in ["proposal.md", "design.md", "tasks.md"] {
        fs::copy(
            root.join(".cairn/changes/demo").join(file),
            second.join(file),
        )?;
    }
    fs::copy(
        root.join(".cairn/changes/demo/specs/demo/spec.md"),
        second.join("specs/demo/spec.md"),
    )?;
    let first = observe(&root, &["active", "--page-entries", "1"])?;
    let cursor = first["next_cursor"]
        .as_str()
        .ok_or("next cursor")?
        .to_string();
    let tasks = root.join(".cairn/changes/demo/tasks.md");
    let text = fs::read_to_string(&tasks)?;
    fs::write(&tasks, text.replace("- [ ]", "- [x]"))?;
    let stderr = run_failure(
        &repo_root(),
        &[
            "observe",
            "active",
            "--root",
            &root.to_string_lossy(),
            "--cursor",
            &cursor,
        ],
    )?;
    assert!(stderr.contains("stale_cursor"), "{stderr}");

    let stderr = run_failure(
        &repo_root(),
        &[
            "observe",
            "active",
            "--root",
            &root.to_string_lossy(),
            "--cursor",
            "not-hex",
        ],
    )?;
    assert!(stderr.contains("invalid_cursor"), "{stderr}");

    fs::write(root.join("corrupt.json"), b"{not-json")?;
    let stderr = run_failure(
        &repo_root(),
        &[
            "observe",
            "active",
            "--root",
            &root.to_string_lossy(),
            "--prior",
            "corrupt.json",
            "--mode",
            "difference",
        ],
    )?;
    assert!(stderr.contains("malformed or truncated"), "{stderr}");

    let mut forged = baseline.clone();
    forged["total"] = json!(forged["total"].as_u64().unwrap_or(0) + 1);
    fs::write(root.join("forged.json"), serde_json::to_vec(&forged)?)?;
    let stderr = run_failure(
        &repo_root(),
        &[
            "observe",
            "active",
            "--root",
            &root.to_string_lossy(),
            "--prior",
            "forged.json",
            "--mode",
            "difference",
        ],
    )?;
    assert!(stderr.contains("forged_counts"), "{stderr}");

    let mut oversized = baseline.clone();
    oversized["entries"][0]["content"] = json!({"payload": "x".repeat(600_000)});
    fs::write(root.join("oversized.json"), serde_json::to_vec(&oversized)?)?;
    let stderr = run_failure(
        &repo_root(),
        &[
            "observe",
            "active",
            "--root",
            &root.to_string_lossy(),
            "--prior",
            "oversized.json",
            "--mode",
            "difference",
        ],
    )?;
    assert!(stderr.contains("entry_bounds"), "{stderr}");

    let readiness = observe(&root, &["readiness", "demo", "--output", "readiness.json"])?;
    let stderr = run_failure(
        &repo_root(),
        &[
            "observe",
            "active",
            "--root",
            &root.to_string_lossy(),
            "--prior",
            "readiness.json",
            "--mode",
            "difference",
        ],
    )?;
    assert!(stderr.contains("mixed_cohort"), "{stderr}");
    assert_eq!(readiness["cohort"]["view"]["view"], "readiness");
    Ok(())
}

#[test]
// r[verify cairn.incremental_observations.delta]
fn partial_baseline_is_rejected_instead_of_becoming_empty() -> Result<(), Box<dyn Error>> {
    let root = basic_fixture("partial")?;
    let cohort = ObservationCohort::new(
        root.to_string_lossy().to_string(),
        None,
        "a".repeat(64),
        "b".repeat(64),
        "c".repeat(64),
        "d".repeat(64),
        ObservationView::ActiveChanges,
    )?;
    let partial = build_snapshot(cohort, Vec::new(), 1, true)?;
    assert_eq!(partial.complete, false);
    fs::write(root.join("partial.json"), serde_json::to_vec(&partial)?)?;
    let stderr = run_failure(
        &repo_root(),
        &[
            "observe",
            "active",
            "--root",
            &root.to_string_lossy(),
            "--prior",
            "partial.json",
            "--mode",
            "difference",
        ],
    )?;
    assert!(stderr.contains("partial_baseline"), "{stderr}");
    Ok(())
}

#[test]
// r[verify cairn.incremental_observations.storage]
fn ordinary_reads_create_no_hidden_state_and_do_not_skip_preflight() -> Result<(), Box<dyn Error>> {
    let root = basic_fixture("state")?;
    let before = tree(&root)?;
    let sync_before = run_json(
        &repo_root(),
        &["sync", "demo", "--root", &root.to_string_lossy()],
    )?;
    let _ = observe(&root, &["active"])?;
    let _ = observe(&root, &["readiness", "demo"])?;
    let after = tree(&root)?;
    assert_eq!(before, after, "ordinary observation wrote hidden state");
    let sync_after = run_json(
        &repo_root(),
        &["sync", "demo", "--root", &root.to_string_lossy()],
    )?;
    assert_eq!(sync_before["blocked"], sync_after["blocked"]);
    assert_eq!(sync_before["operations"], sync_after["operations"]);
    Ok(())
}

#[test]
// r[verify cairn.incremental_observations.storage]
// r[verify cairn.incremental_observations.conformance]
fn mcp_inline_and_declared_prior_paths_match_cli_semantics() -> Result<(), Box<dyn Error>> {
    let root = basic_fixture("mcp")?;
    let baseline = observe(&root, &["active", "--output", "baseline.json"])?;
    let root_text = root.to_string_lossy().into_owned();
    let mcp = mcp_call(
        &repo_root(),
        json!({"root": root_text, "view": "active_changes"}),
    )?;
    assert_eq!(mcp["structuredContent"], baseline);

    let inline = mcp_call(
        &repo_root(),
        json!({
            "root": root.to_string_lossy(),
            "view": "active_changes",
            "mode": "unchanged",
            "prior": baseline
        }),
    )?;
    assert_eq!(inline["structuredContent"]["unchanged"], true);
    let declared = mcp_call(
        &repo_root(),
        json!({
            "root": root.to_string_lossy(),
            "view": "active_changes",
            "mode": "difference",
            "prior_path": "baseline.json"
        }),
    )?;
    assert_eq!(declared["structuredContent"]["unchanged"], true);

    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(repo_root())
        .args([
            "mcp",
            "--call",
            "cairn_observe",
            &json!({
                "root": root.to_string_lossy(),
                "view": "active_changes",
                "output": "nope.json"
            })
            .to_string(),
        ])
        .output()?;
    assert!(!output.status.success());
    assert!(String::from_utf8_lossy(&output.stderr).contains("unknown argument"));
    Ok(())
}

#[test]
// r[verify cairn.incremental_observations.identity]
fn context_selector_identity_and_bounds_are_enforced() -> Result<(), Box<dyn Error>> {
    let root = temp_dir("context-negative")?;
    fs::create_dir_all(root.join(".cairn/specs/example"))?;
    fs::create_dir_all(root.join(".cairn/changes"))?;
    fs::write(
        root.join(".cairn/specs/example/spec.md"),
        "# Example\n## Requirements\n### Requirement: A\nr[demo.a] The system MUST work.\n#### Scenario: One\n- GIVEN input\n- WHEN read\n- THEN output\n",
    )?;
    let stderr = run_failure(
        &repo_root(),
        &[
            "observe",
            "context",
            "--root",
            &root.to_string_lossy(),
            "--scope",
            "accepted",
            "--artifact",
            "specs",
            "--spec",
            "example",
            "--requirement-id",
            "../escape",
        ],
    )?;
    assert!(
        stderr.contains("unsafe") || stderr.contains("invalid"),
        "{stderr}"
    );
    Ok(())
}
