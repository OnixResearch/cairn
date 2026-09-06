//! Fixture-only source intake, not a product dependency on a working tree.
use std::{env, error::Error, fs, path::PathBuf, process::Command};
const REVISION: &str = "5d99f4e9bb954743443e340baece31256b4c53f1";
const SOURCE: &str = "crates/kiln-core/src/workflow/tests.rs";
fn main() -> Result<(), Box<dyn Error>> {
    println!("cargo:rerun-if-env-changed=KILN_FIXTURE_SOURCE");
    let root = PathBuf::from(
        env::var_os("KILN_FIXTURE_SOURCE").ok_or("explicit pinned Kiln source required")?,
    );
    let head = Command::new("git")
        .arg("-C")
        .arg(&root)
        .args(["rev-parse", "HEAD"])
        .output()?;
    if !head.status.success() || String::from_utf8(head.stdout)?.trim() != REVISION {
        return Err("Kiln fixture source revision mismatch".into());
    }
    // Read the immutable object, never a possibly substituted working file.
    let source = Command::new("git")
        .arg("-C")
        .arg(&root)
        .args(["show", &format!("{REVISION}:{SOURCE}")])
        .output()?;
    if !source.status.success() {
        return Err("pinned Kiln fixture unavailable".into());
    }
    fs::write(
        PathBuf::from(env::var_os("OUT_DIR").ok_or("output directory missing")?)
            .join("producer-tests.rs"),
        source.stdout,
    )?;
    println!("cargo:rerun-if-changed={}", root.join(".git").display());
    Ok(())
}
