use std::{
    env,
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use serde_json::{json, Value};

const REPORT_SCHEMA: &str = "cairn.prose-check.v1";
const BAD_PROSE: &str = "He are late.\n";

type TestResult<T = ()> = Result<T, Box<dyn Error>>;

struct TestRoot(PathBuf);

impl TestRoot {
    fn new(name: &str) -> TestResult<Self> {
        let path = env::temp_dir().join(format!("cairn-prose-{name}-{}", std::process::id()));
        match fs::remove_dir_all(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        fs::create_dir_all(&path)?;
        Ok(Self(path))
    }

    fn write(&self, relative: &str, bytes: impl AsRef<[u8]>) -> TestResult {
        let path = self.0.join(relative);
        fs::create_dir_all(path.parent().ok_or("file needs a parent")?)?;
        fs::write(path, bytes)?;
        Ok(())
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn invoke(root: &Path, args: &[&str], machine: bool) -> TestResult<Output> {
    let mut command = Command::new(env!("CARGO_BIN_EXE_cairn"));
    command
        .current_dir(root)
        .env_remove("CAIRN_GLOBAL_REGISTRY")
        .env(
            "CAIRN_POLICY",
            concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/../../cairn-policy/generated/cairn-policy.json"
            ),
        );
    if machine {
        command.arg("--machine");
    }
    Ok(command
        .args(["prose", "check", "--root"])
        .arg(root)
        .args(args)
        .output()?)
}

fn successful_report(output: &Output) -> TestResult<Value> {
    assert!(
        output.status.success(),
        "status: {:?}\nstdout:\n{}\nstderr:\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(report["schema_version"], REPORT_SCHEMA);
    assert_eq!(report["advisory"], true);
    assert_eq!(report["mutated"], false);
    assert!(report.get("valid").is_none());
    Ok(report)
}

fn rejected_without_report(output: &Output) {
    assert!(
        !output.status.success(),
        "unexpected success: {}",
        String::from_utf8_lossy(&output.stdout)
    );
    for value in serde_json::Deserializer::from_slice(&output.stdout).into_iter::<Value>() {
        let Ok(value) = value else { continue };
        assert_ne!(value["schema_version"], REPORT_SCHEMA);
        assert_ne!(value["result"]["schema_version"], REPORT_SCHEMA);
        if value["schema_version"] == "cairn.cli-machine.v1" {
            assert_eq!(value["ok"], false);
        }
    }
}

fn agreement_diagnostic(report: &Value) -> &Value {
    report["files"][0]["diagnostics"]
        .as_array()
        .expect("diagnostics array")
        .iter()
        .find(|diagnostic| diagnostic["code"] == "E0001")
        .expect("actual engine agreement diagnostic")
}

fn assert_agreement_span(diagnostic: &Value, start: usize, end: usize, line: usize, column: usize) {
    assert_eq!(diagnostic["code"], "E0001");
    assert_eq!(diagnostic["lint"], "subject_verb_agreement");
    assert_eq!(diagnostic["level"], "error");
    let labels = diagnostic["labels"].as_array().expect("labels array");
    let primary: Vec<_> = labels
        .iter()
        .filter(|label| label["primary"] == true)
        .collect();
    assert_eq!(primary.len(), 1);
    assert_eq!(primary[0]["start"], start);
    assert_eq!(primary[0]["end"], end);
    assert_eq!(primary[0]["line"], line);
    assert_eq!(primary[0]["column"], column);
    let suggestions: Vec<_> = diagnostic["helps"]
        .as_array()
        .expect("helps array")
        .iter()
        .filter_map(|help| help.get("suggestion").filter(|value| !value.is_null()))
        .collect();
    assert_eq!(
        suggestions,
        vec![&json!({
            "start": start,
            "end": end,
            "replacement": "is",
            "review_required": true
        })]
    );
}

// r[verify cairn.prose_check.advisory] r[verify cairn.prose_check.read_only]
#[test]
fn upstream_error_is_advisory_and_never_applies_its_suggestion() -> TestResult {
    let root = TestRoot::new("advisory")?;
    root.write("bad.md", BAD_PROSE)?;
    let output = invoke(&root.0, &["bad.md"], false)?;
    let report = successful_report(&output)?;
    assert_eq!(output.status.code(), Some(0));
    assert_eq!(report["files"].as_array().expect("files array").len(), 1);
    assert_eq!(report["files"][0]["path"], "bad.md");
    assert_eq!(
        report["files"][0]["diagnostics"]
            .as_array()
            .expect("diagnostics array")
            .len(),
        1
    );
    let diagnostic = agreement_diagnostic(&report);
    assert_agreement_span(diagnostic, 3, 6, 1, 4);
    assert!(diagnostic["labels"]
        .as_array()
        .expect("labels array")
        .iter()
        .any(|label| { label["primary"] == false && label["start"] == 0 && label["end"] == 2 }));
    assert_eq!(fs::read(root.0.join("bad.md"))?, BAD_PROSE.as_bytes());
    Ok(())
}

#[test]
fn clean_prose_and_opaque_markdown_regions_have_no_findings() -> TestResult {
    let root = TestRoot::new("suppression")?;
    let sources = [
        ("notes/clean.md", "He is late.\n"),
        ("fence.md", "```text\nHe are late. A apple.\n```\n"),
        ("inline.md", "`He are late. A apple.`\n"),
        ("url.md", "https://example.invalid/teh/alot\n"),
    ];
    for (path, source) in sources {
        root.write(path, source)?;
    }
    let output = invoke(
        &root.0,
        &["./notes/clean.md", "fence.md", "inline.md", "url.md"],
        false,
    )?;
    let report = successful_report(&output)?;
    let files = report["files"].as_array().expect("files array");
    assert_eq!(files.len(), sources.len());
    for (path, source) in sources {
        let file = files
            .iter()
            .find(|file| file["path"] == path)
            .expect("explicit input file");
        assert_eq!(
            file["diagnostics"],
            json!([]),
            "unexpected findings for {path}"
        );
        assert_eq!(fs::read(root.0.join(path))?, source.as_bytes());
    }
    Ok(())
}

// r[verify cairn.prose_check.diagnostics]
#[test]
fn unicode_prefix_uses_byte_spans_but_scalar_columns() -> TestResult {
    let root = TestRoot::new("unicode")?;
    let source = "He is here.\n🙂 He are late.\n";
    root.write("unicode.md", source)?;
    let report = successful_report(&invoke(&root.0, &["unicode.md"], false)?)?;
    let diagnostic = agreement_diagnostic(&report);
    assert_agreement_span(diagnostic, 20, 23, 2, 6);
    assert_eq!(&source[20..23], "are");
    assert_eq!(fs::read(root.0.join("unicode.md"))?, source.as_bytes());
    Ok(())
}

#[test]
fn machine_mode_nests_the_same_successful_advisory_report() -> TestResult {
    let root = TestRoot::new("machine")?;
    root.write("bad.md", BAD_PROSE)?;
    let output = invoke(&root.0, &["bad.md"], true)?;
    assert_eq!(output.status.code(), Some(0));
    assert!(output.stderr.is_empty());
    let envelope: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(envelope["schema_version"], "cairn.cli-machine.v1");
    assert_eq!(envelope["ok"], true);
    assert_eq!(envelope["exit_class"], "success");
    assert_eq!(envelope["result"]["advisory"], true);
    assert_eq!(agreement_diagnostic(&envelope["result"])["level"], "error");
    assert_eq!(fs::read(root.0.join("bad.md"))?, BAD_PROSE.as_bytes());
    Ok(())
}

// r[verify cairn.prose_check.inputs]
#[test]
fn unsafe_or_unreadable_inputs_fail_without_a_partial_success_report() -> TestResult {
    let root = TestRoot::new("admission")?;
    root.write("bad.md", BAD_PROSE)?;
    let invalid_utf8 = [0xff, 0xfe];
    let oversized = vec![b'a'; 1024 * 1024 + 1];
    root.write("invalid.md", invalid_utf8)?;
    root.write("large.md", &oversized)?;
    fs::create_dir(root.0.join("directory"))?;
    let absolute = root.0.join("bad.md");
    let absolute = absolute.to_str().ok_or("UTF-8 temporary path")?;
    for path in [
        absolute,
        "../bad.md",
        "notes/../bad.md",
        "missing.md",
        "invalid.md",
        "large.md",
        "directory",
    ] {
        for machine in [false, true] {
            rejected_without_report(&invoke(&root.0, &["bad.md", path], machine)?);
        }
    }
    assert_eq!(fs::read(root.0.join("bad.md"))?, BAD_PROSE.as_bytes());
    assert_eq!(fs::read(root.0.join("invalid.md"))?, invalid_utf8);
    assert_eq!(fs::read(root.0.join("large.md"))?, oversized);
    Ok(())
}

#[cfg(unix)]
#[test]
fn symlink_escape_fails_without_touching_the_external_source() -> TestResult {
    use std::os::unix::fs::symlink;

    let sandbox = TestRoot::new("symlink")?;
    let root = sandbox.0.join("root");
    fs::create_dir(&root)?;
    sandbox.write("outside.md", BAD_PROSE)?;
    symlink(sandbox.0.join("outside.md"), root.join("escape.md"))?;
    for machine in [false, true] {
        rejected_without_report(&invoke(&root, &["escape.md"], machine)?);
    }
    assert_eq!(
        fs::read(sandbox.0.join("outside.md"))?,
        BAD_PROSE.as_bytes()
    );
    assert!(fs::symlink_metadata(root.join("escape.md"))?
        .file_type()
        .is_symlink());
    Ok(())
}

#[test]
fn unsupported_options_and_missing_paths_cannot_trigger_a_fix() -> TestResult {
    let root = TestRoot::new("arguments")?;
    root.write("bad.md", BAD_PROSE)?;
    let cases: &[&[&str]] = &[
        &["--fix", "bad.md"],
        &["bad.md", "--fix"],
        &["--unknown-option", "bad.md"],
        &[],
    ];
    for args in cases {
        for machine in [false, true] {
            rejected_without_report(&invoke(&root.0, args, machine)?);
            assert_eq!(fs::read(root.0.join("bad.md"))?, BAD_PROSE.as_bytes());
        }
    }
    Ok(())
}

fn enriched_report(output: &Output) -> TestResult<Value> {
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let report: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(report["schema_version"], "cairn.prose-check.v2");
    Ok(report)
}

fn verify_saved(root: &Path) -> TestResult<Output> {
    Ok(Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(root)
        .args(["prose", "verify", "--root"])
        .arg(root)
        .args(["--report", "report.json"])
        .output()?)
}

// r[verify cairn.prose_workflow.binding] r[verify cairn.prose_workflow.verification]
#[test]
fn saved_reports_reject_changed_bytes_and_rehashed_altered_suggestions() -> TestResult {
    let root = TestRoot::new("freshness")?;
    root.write("bad.md", BAD_PROSE)?;
    let report = enriched_report(&invoke(
        &root.0,
        &["--report-version", "2", "bad.md"],
        false,
    )?)?;
    assert_eq!(
        report["files"][0]["content_blake3"],
        blake3::hash(BAD_PROSE.as_bytes()).to_hex().to_string()
    );
    root.write("report.json", serde_json::to_vec(&report)?)?;
    let fresh = verify_saved(&root.0)?;
    assert!(
        fresh.status.success(),
        "{}",
        String::from_utf8_lossy(&fresh.stderr)
    );
    assert_eq!(
        serde_json::from_slice::<Value>(&fresh.stdout)?["current"],
        true
    );
    root.write("bad.md", format!("{BAD_PROSE}\n"))?;
    rejected_without_report(&verify_saved(&root.0)?);
    root.write("bad.md", BAD_PROSE)?;
    let mut altered = report;
    altered["files"][0]["diagnostics"][0]["helps"][0]["suggestion"]["replacement"] =
        json!("must not exist");
    altered
        .as_object_mut()
        .ok_or("report object")?
        .remove("report_blake3");
    altered["report_blake3"] = json!(cairn_core::canonical_json_hash(&altered));
    root.write("report.json", serde_json::to_vec(&altered)?)?;
    rejected_without_report(&verify_saved(&root.0)?);
    Ok(())
}

// r[verify cairn.prose_workflow.selection]
#[test]
fn change_selection_is_sorted_confined_and_bound_to_the_current_file_set() -> TestResult {
    let root = TestRoot::new("change-selection")?;
    let base = ".cairn/changes/example";
    let selected = [
        format!("{base}/design.md"),
        format!("{base}/proposal.md"),
        format!("{base}/specs/a/spec.md"),
        format!("{base}/specs/z/spec.md"),
        format!("{base}/tasks.md"),
    ];
    for path in &selected {
        root.write(path, BAD_PROSE)?;
    }
    for path in [
        ".cairn/archive/old/proposal.md",
        "vendor/bad.md",
        ".cairn/changes/example/evidence/bad.md",
    ] {
        root.write(path, [0xff])?;
    }
    let report = enriched_report(&invoke(
        &root.0,
        &["--change", "example", "--report-version", "2"],
        false,
    )?)?;
    let paths: Vec<_> = report["files"]
        .as_array()
        .ok_or("files")?
        .iter()
        .map(|file| file["path"].as_str().unwrap_or_default())
        .collect();
    assert_eq!(
        paths,
        selected.iter().map(String::as_str).collect::<Vec<_>>()
    );
    root.write("report.json", serde_json::to_vec(&report)?)?;
    assert!(verify_saved(&root.0)?.status.success());
    root.write(&format!("{base}/specs/new/spec.md"), "He is late.\n")?;
    rejected_without_report(&verify_saved(&root.0)?);
    rejected_without_report(&invoke(
        &root.0,
        &["--change", "example", &selected[0]],
        false,
    )?);
    rejected_without_report(&invoke(&root.0, &["--change", "../example"], false)?);
    Ok(())
}

// r[verify cairn.prose_workflow.configuration]
#[test]
fn selected_profiles_filter_rules_reject_unknown_configuration_and_detect_drift() -> TestResult {
    let root = TestRoot::new("profile")?;
    root.write("bad.md", "He are late. A apple.\n")?;
    let mut profiles = json!({"schema_version":"cairn.prose-profiles.v1", "profiles":[{"name":"focused", "include_rules":["E0001"], "exclude_rules":[]}]});
    root.write("profiles.json", serde_json::to_vec(&profiles)?)?;
    let report = enriched_report(&invoke(
        &root.0,
        &[
            "--profiles",
            "profiles.json",
            "--profile",
            "focused",
            "--report-version",
            "2",
            "bad.md",
        ],
        false,
    )?)?;
    let diagnostics = report["files"][0]["diagnostics"]
        .as_array()
        .ok_or("diagnostics")?;
    assert_eq!(
        diagnostics
            .iter()
            .map(|finding| &finding["code"])
            .collect::<Vec<_>>(),
        vec![&json!("E0001")]
    );
    root.write("report.json", serde_json::to_vec(&report)?)?;
    assert!(verify_saved(&root.0)?.status.success());
    profiles["profiles"][0]["exclude_rules"] = json!(["subject_verb_agreement"]);
    root.write("profiles.json", serde_json::to_vec(&profiles)?)?;
    rejected_without_report(&verify_saved(&root.0)?);
    let excluded = successful_report(&invoke(
        &root.0,
        &[
            "--profiles",
            "profiles.json",
            "--profile",
            "focused",
            "bad.md",
        ],
        false,
    )?)?;
    assert_eq!(excluded["files"][0]["diagnostics"], json!([]));
    profiles["profiles"][0]["include_rules"] = json!(["invented_rule"]);
    root.write("profiles.json", serde_json::to_vec(&profiles)?)?;
    rejected_without_report(&invoke(
        &root.0,
        &[
            "--profiles",
            "profiles.json",
            "--profile",
            "focused",
            "bad.md",
        ],
        false,
    )?);
    profiles["profiles"][0]["include_rules"] = json!(["E0001"]);
    profiles["profiles"][0]["unexpected"] = json!(true);
    root.write("profiles.json", serde_json::to_vec(&profiles)?)?;
    rejected_without_report(&invoke(
        &root.0,
        &[
            "--profiles",
            "profiles.json",
            "--profile",
            "focused",
            "bad.md",
        ],
        false,
    )?);
    Ok(())
}

// r[verify cairn.prose_workflow.presentation]
#[test]
fn actual_engine_findings_render_as_text_warning_annotations_and_sarif() -> TestResult {
    let root = TestRoot::new("formats")?;
    let path = "notes %,:🙂.md";
    root.write(path, BAD_PROSE)?;
    let text = invoke(&root.0, &["--format", "text", path], false)?;
    assert!(
        text.status.success(),
        "{}",
        String::from_utf8_lossy(&text.stderr)
    );
    let text = String::from_utf8(text.stdout)?;
    assert!(text.contains("notes %,:🙂.md:1:4"));
    assert!(text.contains("E0001") && text.contains("author review"));
    let github = invoke(&root.0, &["--format", "github", path], false)?;
    assert!(github.status.success());
    let github = String::from_utf8(github.stdout)?;
    assert!(github.contains("::warning file=notes %25%2C%3A🙂.md"));
    assert!(!github.contains("::error"));
    let sarif = invoke(&root.0, &["--format", "sarif", path], false)?;
    assert!(sarif.status.success());
    let sarif: Value = serde_json::from_slice(&sarif.stdout)?;
    assert_eq!(sarif["version"], "2.1.0");
    let result = &sarif["runs"][0]["results"][0];
    assert_eq!(result["level"], "warning");
    assert_eq!(result["properties"]["upstreamSeverity"], "error");
    assert_eq!(
        result["locations"][0]["physicalLocation"]["artifactLocation"]["uri"],
        "notes%20%25%2C%3A%F0%9F%99%82.md"
    );
    assert!(result.get("fixes").is_none());
    rejected_without_report(&invoke(&root.0, &["--format", "text", path], true)?);
    let machine = invoke(&root.0, &["--report-version", "2", path], true)?;
    assert!(machine.status.success());
    let machine: Value = serde_json::from_slice(&machine.stdout)?;
    assert_eq!(machine["ok"], true);
    assert_eq!(machine["result"]["schema_version"], "cairn.prose-check.v2");
    assert_eq!(fs::read(root.0.join(path))?, BAD_PROSE.as_bytes());
    Ok(())
}

// r[verify cairn.prose_workflow.bounds]
#[test]
fn byte_diagnostic_and_output_limits_accept_the_boundary_and_fail_atomically() -> TestResult {
    let root = TestRoot::new("budgets")?;
    root.write("bad.md", BAD_PROSE)?;
    let exact_bytes = BAD_PROSE.len().to_string();
    successful_report(&invoke(
        &root.0,
        &[
            "--max-total-bytes",
            &exact_bytes,
            "--max-diagnostics",
            "1",
            "bad.md",
        ],
        false,
    )?)?;
    let short = (BAD_PROSE.len() - 1).to_string();
    rejected_without_report(&invoke(
        &root.0,
        &["--max-total-bytes", &short, "bad.md"],
        false,
    )?);
    let default = invoke(&root.0, &["bad.md"], false)?;
    successful_report(&default)?;
    let exact_output = default.stdout.len().to_string();
    let limited = invoke(
        &root.0,
        &["--max-report-bytes", &exact_output, "bad.md"],
        false,
    )?;
    assert_eq!(limited.stdout, default.stdout);
    assert!(limited.status.success());
    let short_output = (default.stdout.len() - 1).to_string();
    let exhausted = invoke(
        &root.0,
        &["--max-report-bytes", &short_output, "bad.md"],
        false,
    )?;
    rejected_without_report(&exhausted);
    assert!(exhausted.stdout.is_empty());
    root.write("two.md", format!("{BAD_PROSE}{BAD_PROSE}"))?;
    let exhausted = invoke(&root.0, &["--max-diagnostics", "1", "two.md"], false)?;
    rejected_without_report(&exhausted);
    assert!(exhausted.stdout.is_empty());
    successful_report(&invoke(
        &root.0,
        &["--max-diagnostics", "2", "two.md"],
        false,
    )?)?;
    rejected_without_report(&invoke(
        &root.0,
        &["--max-diagnostics", "4097", "bad.md"],
        false,
    )?);
    Ok(())
}

// r[verify cairn.prose_workflow.documents] r[verify cairn.prose_workflow.requirement_context]
#[test]
fn projected_technical_examples_preserve_real_requirement_locations() -> TestResult {
    let root = TestRoot::new("projection")?;
    let source = "---\r\ntitle: He are late.\r\n---\r\n~~~text\r\nHe are late.\r\n~~~\r\n[source,rust]\r\n----\r\nHe are late.\r\n----\r\n### Requirement: Timer\r\nr[fixture.timer] The timer SHALL wait.\r\n\r\n🙂 He are late.\r\n";
    root.write("spec.md", source)?;
    let report = enriched_report(&invoke(
        &root.0,
        &["--report-version", "2", "spec.md"],
        false,
    )?)?;
    let finding = agreement_diagnostic(&report);
    assert_eq!(finding["requirement_id"], "fixture.timer");
    let label = finding["labels"]
        .as_array()
        .ok_or("labels")?
        .iter()
        .find(|label| label["primary"] == true)
        .ok_or("primary")?;
    let start = source.rfind("are").ok_or("expected word")?;
    assert_eq!(label["start"], start);
    assert_eq!(label["line"], 14);
    assert_eq!(label["column"], 6);
    assert_eq!(
        report["files"][0]["diagnostics"]
            .as_array()
            .ok_or("diagnostics")?
            .iter()
            .filter(|finding| finding["code"] == "E0001")
            .count(),
        1
    );
    assert_eq!(fs::read(root.0.join("spec.md"))?, source.as_bytes());
    Ok(())
}

#[test]
fn markdown_dividers_do_not_hide_prose_but_bare_asciidoc_listings_do() -> TestResult {
    let root = TestRoot::new("document-kind")?;
    let markdown = "---\n## Heading\n\n----\n\nHe are late.\n";
    root.write("notes.md", markdown)?;
    let report = successful_report(&invoke(&root.0, &["notes.md"], false)?)?;
    let finding = agreement_diagnostic(&report);
    assert_eq!(
        finding["labels"][0]["start"],
        markdown.find("are").ok_or("word")?
    );
    let asciidoc = "= Heading\n\n----\nHe are late.\n----\n\nHe are here.\n";
    root.write("notes.adoc", asciidoc)?;
    let report = successful_report(&invoke(&root.0, &["notes.adoc"], false)?)?;
    let findings = report["files"][0]["diagnostics"]
        .as_array()
        .ok_or("diagnostics")?;
    assert_eq!(
        findings
            .iter()
            .filter(|finding| finding["code"] == "E0001")
            .count(),
        1
    );
    assert_eq!(
        agreement_diagnostic(&report)["labels"][0]["start"],
        asciidoc.rfind("are").ok_or("word")?
    );
    Ok(())
}
