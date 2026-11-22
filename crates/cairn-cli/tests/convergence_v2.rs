//! Version 2 realization receipts: root disposition, builder fingerprint and
//! reuse link.
//!
//! The retained pilot case is materialized in a fresh directory, its version 1
//! receipt is replaced with a version 2 fixture, and each declarative record
//! names the files, case edits and JSON mutations for one check. No mutated
//! copy of the large pilot records is committed.
// r[verify cairn.convergence.realization.root_disposition]
// r[verify cairn.convergence.realization.reuse_link]
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
    process::{Command, Output},
};

use cairn_core::blake3_hex;
use serde_json::Value;

const PILOT_REQUIREMENT: &str = "octet.aeneas_consumer_contract.compatibility";

fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../fixtures/convergence")
}

fn pilot_dir() -> PathBuf {
    fixtures_dir().join("pilot")
}

fn receipts_dir() -> PathBuf {
    fixtures_dir().join("pilot-v2")
}

fn collect_files(base: &Path, directory: &Path, out: &mut Vec<(String, PathBuf)>) {
    for entry in fs::read_dir(directory).unwrap() {
        let path = entry.unwrap().path();
        let metadata = fs::symlink_metadata(&path).unwrap();
        let relative = path
            .strip_prefix(base)
            .unwrap()
            .to_string_lossy()
            .into_owned();
        if metadata.is_dir() {
            collect_files(base, &path, out);
        } else {
            out.push((relative, path));
        }
    }
}

fn hash_tree(root: &Path) -> BTreeMap<String, String> {
    let mut files = Vec::new();
    collect_files(root, root, &mut files);
    let mut entries = BTreeMap::new();
    for (relative, path) in files {
        entries.insert(relative, blake3_hex(&fs::read(&path).unwrap()));
    }
    entries
}

fn fresh_dir(label: &str) -> PathBuf {
    let directory = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("convergence-v2")
        .join(label);
    if directory.exists() {
        fs::remove_dir_all(&directory).unwrap();
    }
    fs::create_dir_all(&directory).unwrap();
    directory
}

fn link_or_copy(source: &Path, destination: &Path) {
    if fs::hard_link(source, destination).is_err() {
        fs::copy(source, destination).unwrap();
    }
}

fn materialize(destination: &Path) {
    let source = pilot_dir();
    let mut files = Vec::new();
    collect_files(&source, &source, &mut files);
    for (relative, path) in files {
        let target = destination.join(&relative);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        link_or_copy(&path, &target);
    }
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap())
        .unwrap_or_else(|error| panic!("{} is not JSON: {error}", path.display()))
}

/// Unlink a materialized link before writing, so a fixture in the worktree can
/// never be written through a hard link.
fn replace_file(path: &Path, bytes: Vec<u8>) {
    if fs::symlink_metadata(path).is_ok_and(|metadata| !metadata.is_dir()) {
        fs::remove_file(path).unwrap();
    }
    fs::write(path, bytes).unwrap();
}

fn value_at_mut<'a>(root: &'a mut Value, pointer: &str) -> &'a mut Value {
    let mut current = root;
    for token in pointer.split('/').skip(1) {
        current = match current {
            Value::Object(map) => map
                .get_mut(token)
                .unwrap_or_else(|| panic!("missing JSON pointer segment {token} in {pointer}")),
            Value::Array(items) => {
                &mut items[token
                    .parse::<usize>()
                    .unwrap_or_else(|_| panic!("array index {token} in {pointer}"))]
            }
            other => panic!("cannot walk {pointer} through {other}"),
        };
    }
    current
}

/// Assign a case field, inserting it when the case does not carry it yet.
fn case_set(directory: &Path, pointer: &str, value: Value) {
    let path = directory.join("case.json");
    let mut case = read_json(&path);
    let (parent, key) = pointer
        .rsplit_once('/')
        .unwrap_or_else(|| panic!("pointer must not be the document root: {pointer}"));
    let slot = value_at_mut(&mut case, parent);
    match slot {
        Value::Object(map) => {
            map.insert(key.to_string(), value);
        }
        Value::Array(items) => {
            let index = key.parse::<usize>().unwrap();
            items[index] = value;
        }
        other => panic!("cannot set {pointer} on {other}"),
    }
    replace_file(&path, serde_json::to_vec(&case).unwrap());
}

fn apply_ops(directory: &Path, ops: &[Value]) {
    for op in ops {
        match op["op"].as_str().expect("op") {
            "case-set" => case_set(
                directory,
                op["pointer"].as_str().unwrap(),
                op["value"].clone(),
            ),
            "json-set" => {
                let path = directory.join(op["file"].as_str().unwrap());
                let mut value = read_json(&path);
                *value_at_mut(&mut value, op["pointer"].as_str().unwrap()) = op["value"].clone();
                replace_file(&path, serde_json::to_vec(&value).unwrap());
            }
            other => panic!("unknown mutation op {other}"),
        }
    }
}

fn apply_files(directory: &Path, files: &Value) {
    for (destination, source) in files.as_object().expect("files") {
        let target = directory.join(destination);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        // The materialized pilot file is a hard link; unlink it so the copy
        // cannot write through it into the committed fixture.
        if fs::symlink_metadata(&target).is_ok() {
            fs::remove_file(&target).unwrap();
        }
        fs::copy(receipts_dir().join(source.as_str().unwrap()), &target).unwrap();
    }
}

fn run_case(directory: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_cairn"))
        .arg("convergence")
        .arg("inspect")
        .arg("--case")
        .arg(directory.join("case.json"))
        .arg("--root")
        .arg(directory.join("root"))
        .current_dir(directory)
        .output()
        .expect("run cairn convergence inspect")
}

fn receipt_json(output: &Output) -> Value {
    serde_json::from_slice(&output.stdout).unwrap_or_else(|error| {
        panic!(
            "stdout is not JSON: {error}\nstdout: {}\nstderr: {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        )
    })
}

fn pilot_row(receipt: &Value) -> &Value {
    receipt["rows"]
        .as_array()
        .expect("receipt rows")
        .iter()
        .find(|row| row["requirement_id"] == PILOT_REQUIREMENT)
        .expect("the pilot requirement row")
}

// r[verify cairn.convergence.realization.root_disposition]
// r[verify cairn.convergence.realization.reuse_link]
#[test]
fn receipt_v2_cases_report_the_designed_disposition_and_link() {
    let pilot = pilot_dir();
    let receipts = receipts_dir();
    let pilot_before = hash_tree(&pilot);
    let receipts_before = hash_tree(&receipts);

    let mut records: Vec<PathBuf> = fs::read_dir(&receipts)
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
                && path.file_name().is_some_and(|name| {
                    name != "built-receipt.json" && name != "reused-receipt.json"
                })
        })
        .collect();
    records.sort();
    assert!(
        records.len() >= 8,
        "expected positive and negative version 2 records, found {}",
        records.len()
    );

    let mut checks = std::collections::BTreeSet::new();
    for record_path in records {
        let record = read_json(&record_path);
        let label = record_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let claim = record["claim"]
            .as_str()
            .unwrap_or_else(|| panic!("{label} has no claim"))
            .to_string();
        assert!(checks.insert(claim.clone()), "{label} duplicates {claim:?}");

        let directory = fresh_dir(&label);
        materialize(&directory);
        apply_files(&directory, &record["files"]);
        apply_ops(&directory, record["ops"].as_array().expect("ops"));

        let output = run_case(&directory);
        let receipt = receipt_json(&output);
        let expected = &record["expected"];
        let expected_exit = expected["exit"].as_i64().unwrap();
        assert_eq!(
            output.status.code(),
            Some(expected_exit as i32),
            "{label}: stdout {}\nstderr {}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );

        let row = pilot_row(&receipt);
        assert_eq!(
            row["state"], expected["row_state"],
            "{label}: row issues {:?}",
            row["issues"]
        );
        let observed: Vec<&str> = row["issues"]
            .as_array()
            .unwrap()
            .iter()
            .map(|issue| issue.as_str().unwrap())
            .collect();
        for issue in expected["issues"].as_array().unwrap() {
            let issue = issue.as_str().unwrap();
            assert!(
                observed.contains(&issue),
                "{label}: expected {issue} in {observed:?}"
            );
        }
        assert_eq!(
            row["bound"]["root_disposition"], expected["root_disposition"],
            "{label}: root disposition"
        );
        assert_eq!(
            row["bound"]["original_receipt_blake3"], expected["original_receipt_blake3"],
            "{label}: original receipt link"
        );

        fs::remove_dir_all(&directory).unwrap();
    }

    assert_eq!(
        pilot_before,
        hash_tree(&pilot),
        "the committed pilot fixture must be unchanged"
    );
    assert_eq!(
        receipts_before,
        hash_tree(&receipts),
        "the committed version 2 fixtures must be unchanged"
    );
}
