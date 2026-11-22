//! The retained real pilot corpus and its declared negatives.
//!
//! The positive case is the bytes copied under `fixtures/convergence/pilot/`.
//! Negatives are small declarative mutation records materialized here, at test
//! time, in a fresh directory; no mutated copy of a large record is committed.
// r[verify cairn.convergence.conformance]
// r[verify cairn.convergence.realization.root_disposition]
use std::{
    collections::{BTreeMap, BTreeSet},
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

fn negatives_dir() -> PathBuf {
    fixtures_dir().join("pilot-negatives")
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
        .join("convergence-pilot-negatives")
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

/// Hard-link every pilot file. For a `root-` mutation the root tree is copied
/// instead, so no root byte can be written through a fixture link.
fn materialize(destination: &Path, copy_root: bool) {
    let source = pilot_dir();
    let mut files = Vec::new();
    collect_files(&source, &source, &mut files);
    for (relative, path) in files {
        let target = destination.join(&relative);
        fs::create_dir_all(target.parent().unwrap()).unwrap();
        if copy_root && relative.starts_with("root/") {
            fs::copy(&path, &target).unwrap();
        } else {
            link_or_copy(&path, &target);
        }
    }
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

fn remove_at(root: &mut Value, pointer: &str) {
    let (parent, key) = pointer
        .rsplit_once('/')
        .unwrap_or_else(|| panic!("pointer must not be the document root: {pointer}"));
    let slot = value_at_mut(root, parent);
    match slot {
        Value::Object(map) => {
            assert!(map.remove(key).is_some(), "missing JSON key {pointer}");
        }
        Value::Array(items) => {
            items.remove(key.parse::<usize>().unwrap());
        }
        other => panic!("cannot remove {pointer} from {other}"),
    }
}

fn read_json(path: &Path) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap())
        .unwrap_or_else(|error| panic!("{} is not JSON: {error}", path.display()))
}

/// Unlink the materialized link before writing, so a fixture in the worktree
/// can never be written through a hard link.
fn replace_file(path: &Path, bytes: Vec<u8>) {
    if fs::symlink_metadata(path).is_ok_and(|metadata| !metadata.is_dir()) {
        fs::remove_file(path).unwrap();
    }
    fs::write(path, bytes).unwrap();
}

fn replace_bytes(path: &Path, find: &str, replace: &str) {
    assert_eq!(
        find.len(),
        replace.len(),
        "replacement must keep the file length: {find} -> {replace}"
    );
    let bytes = fs::read(path).unwrap();
    let position = bytes
        .windows(find.len())
        .position(|window| window == find.as_bytes())
        .unwrap_or_else(|| panic!("{} does not contain {find:?}", path.display()));
    let mut mutated = bytes;
    mutated[position..position + replace.len()].copy_from_slice(replace.as_bytes());
    replace_file(path, mutated);
}

fn apply_mutation(directory: &Path, mutation: &Value) {
    let op = mutation["op"].as_str().expect("mutation op");
    match op {
        "json-set" => {
            let path = directory.join(mutation["file"].as_str().unwrap());
            let mut value = read_json(&path);
            *value_at_mut(&mut value, mutation["pointer"].as_str().unwrap()) =
                mutation["value"].clone();
            replace_file(&path, serde_json::to_vec(&value).unwrap());
        }
        "replace-bytes" => replace_bytes(
            &directory.join(mutation["file"].as_str().unwrap()),
            mutation["find"].as_str().unwrap(),
            mutation["replace"].as_str().unwrap(),
        ),
        "root-replace-bytes" => replace_bytes(
            &directory
                .join("root")
                .join(mutation["file"].as_str().unwrap()),
            mutation["find"].as_str().unwrap(),
            mutation["replace"].as_str().unwrap(),
        ),
        "case-remove" => {
            let path = directory.join("case.json");
            let mut case = read_json(&path);
            for pointer in mutation["keys"].as_array().unwrap() {
                remove_at(&mut case, pointer.as_str().unwrap());
            }
            replace_file(&path, serde_json::to_vec(&case).unwrap());
        }
        "case-set" => {
            let path = directory.join("case.json");
            let mut case = read_json(&path);
            *value_at_mut(&mut case, mutation["pointer"].as_str().unwrap()) =
                mutation["value"].clone();
            replace_file(&path, serde_json::to_vec(&case).unwrap());
        }
        other => panic!("unknown mutation op {other}"),
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

#[test]
fn pilot_case_is_current_and_deterministic() {
    let pilot = pilot_dir();
    let root_before = hash_tree(&pilot.join("root"));

    let first = run_case(&pilot);
    assert!(
        first.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&first.stdout),
        String::from_utf8_lossy(&first.stderr)
    );
    let receipt = receipt_json(&first);
    assert_eq!(receipt["status"], "evaluated");

    let rows = receipt["rows"].as_array().unwrap();
    let current: Vec<&Value> = rows
        .iter()
        .filter(|row| row["state"] == "current")
        .collect();
    assert_eq!(current.len(), 1, "exactly one pilot row is current");
    assert_eq!(current[0]["requirement_id"], PILOT_REQUIREMENT);
    assert_eq!(current[0]["assurance"], "checked_proof");
    assert_eq!(
        current[0]["bound"]["realization_route"],
        "cache-only-preserve-v1"
    );
    assert_eq!(current[0]["bound"]["root_disposition"], "preserved");
    assert_eq!(current[0]["bound"]["original_receipt_blake3"], Value::Null);
    assert!(current[0]["issues"].as_array().unwrap().is_empty());
    for row in rows {
        if row["requirement_id"] != PILOT_REQUIREMENT {
            assert_eq!(
                row["state"], "untargeted",
                "row {} must stay untargeted",
                row["requirement_id"]
            );
        }
    }
    assert_eq!(receipt["targets_current"], true);
    assert_eq!(receipt["coverage_complete"], false);
    assert_eq!(receipt["complete"], false);
    assert!(receipt["unbound_target_issues"]
        .as_array()
        .unwrap()
        .is_empty());

    let second = run_case(&pilot);
    assert!(second.status.success());
    assert_eq!(
        first.stdout, second.stdout,
        "two runs must print byte-identical receipts"
    );
    let second_receipt = receipt_json(&second);
    assert_eq!(
        receipt["receipt_blake3"], second_receipt["receipt_blake3"],
        "receipt hash must be stable"
    );

    assert_eq!(
        root_before,
        hash_tree(&pilot.join("root")),
        "the pilot root must be unchanged"
    );
}

#[test]
fn pilot_negatives_report_the_designed_state() {
    let pilot = pilot_dir();
    let baseline = hash_tree(&pilot);

    let mut records: Vec<PathBuf> = fs::read_dir(negatives_dir())
        .unwrap()
        .map(|entry| entry.unwrap().path())
        .filter(|path| {
            path.extension()
                .is_some_and(|extension| extension == "json")
        })
        .collect();
    records.sort();
    assert!(
        records.len() >= 19,
        "expected one negative per binding check, found {}",
        records.len()
    );

    let mut checks = BTreeSet::new();
    for record_path in records {
        let record: Value = read_json(&record_path);
        let label = record_path
            .file_stem()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let check = record["binding_check"]
            .as_str()
            .unwrap_or_else(|| panic!("{label} has no binding_check"))
            .to_string();
        assert!(
            checks.insert(check.clone()),
            "{label} duplicates binding check {check:?}"
        );

        let mutation = &record["mutation"];
        let copy_root = mutation["op"] == "root-replace-bytes";
        let directory = fresh_dir(&label);
        materialize(&directory, copy_root);
        apply_mutation(&directory, mutation);

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
        let expected_issues: Vec<&str> = expected["issues"]
            .as_array()
            .unwrap()
            .iter()
            .map(|issue| issue.as_str().unwrap())
            .collect();

        if expected_exit == 0 {
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
            for issue in &expected_issues {
                assert!(
                    observed.contains(issue),
                    "{label}: expected {issue} in {observed:?}"
                );
            }
        } else {
            assert!(expected["row_state"].is_null());
            let observed: Vec<&str> = receipt["issues"]
                .as_array()
                .unwrap()
                .iter()
                .map(|issue| issue["code"].as_str().unwrap())
                .collect();
            for issue in &expected_issues {
                assert!(
                    observed.contains(issue),
                    "{label}: expected {issue} in {observed:?}"
                );
            }
        }
        // A failing negative keeps its directory for inspection; a passing
        // one is removed so copy fallbacks never accumulate.
        fs::remove_dir_all(&directory).unwrap();
    }

    assert_eq!(
        baseline,
        hash_tree(&pilot),
        "the committed pilot fixture must be unchanged after every negative"
    );
}
