use serde_json::Value;
use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::Command,
};

fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn policy_path() -> PathBuf {
    repository().join("cairn-policy/generated/cairn-policy.json")
}

fn isolated_workdir(name: &str) -> Result<PathBuf, Box<dyn Error>> {
    let work = std::env::temp_dir().join(format!(
        "cairn-public-release-{name}-{}",
        std::process::id()
    ));
    if work.exists() {
        fs::remove_dir_all(&work)?;
    }
    fs::create_dir_all(&work)?;
    Ok(work)
}

fn run(work: &Path, args: &[&str], policy: &Path) -> Result<Value, Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(work)
        .env("HOME", work)
        .env("CAIRN_DATA_DIR", work.join("data"))
        .env("CAIRN_POLICY", policy)
        .args(args)
        .output()?;
    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    Ok(serde_json::from_slice(&output.stdout)?)
}

// r[verify cairn.supersede.packaged_default_policy.validation]
#[test]
fn version_flag_reports_the_package_version() -> Result<(), Box<dyn Error>> {
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .arg("--version")
        .output()?;
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout).trim(),
        format!("cairn {}", env!("CARGO_PKG_VERSION"))
    );
    Ok(())
}

// r[verify cairn.supersede.packaged_default_policy.foreign]
#[test]
fn packaged_policy_default_works_outside_a_source_checkout() -> Result<(), Box<dyn Error>> {
    let work = isolated_workdir("foreign")?;
    let root = work.join("repo");
    run(&work, &["init", "--root", "repo"], &policy_path())?;
    let result = run(&work, &["validate", "--root", "repo"], &policy_path())?;
    assert_eq!(result["valid"], true);
    assert_eq!(result["policy_selection"]["source"], "legacy_default");
    assert_eq!(result["layout"], "cairn");
    assert!(root.join(".cairn/changes").is_dir());
    fs::remove_dir_all(work)?;
    Ok(())
}

// r[verify cairn.supersede.packaged_default_policy.docs]
#[test]
fn install_docs_state_the_policy_override() -> Result<(), Box<dyn Error>> {
    let readme = fs::read_to_string(repository().join("README.md"))?;
    assert!(readme.contains("CAIRN_POLICY"));
    assert!(readme.contains("`--policy PATH`"));
    Ok(())
}

// r[verify cairn.supersede.packaged_default_policy.explicit]
#[test]
fn explicit_policy_overrides_an_unavailable_environment_default() -> Result<(), Box<dyn Error>> {
    let work = isolated_workdir("explicit")?;
    run(&work, &["init", "--root", "repo"], &policy_path())?;
    let explicit = policy_path().to_string_lossy().into_owned();
    let output = Command::new(env!("CARGO_BIN_EXE_cairn"))
        .current_dir(&work)
        .env("HOME", &work)
        .env("CAIRN_DATA_DIR", work.join("data"))
        .env("CAIRN_POLICY", "/nonexistent/cairn-policy.json")
        .args(["validate", "--root", "repo", "--policy", &explicit])
        .output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let result: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(result["valid"], true);
    assert_eq!(result["policy_selection"]["source"], "explicit");
    fs::remove_dir_all(work)?;
    Ok(())
}
