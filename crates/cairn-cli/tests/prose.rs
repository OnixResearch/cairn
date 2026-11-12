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
