//! Read-only source identity and byte-freshness check for imported producer fixtures.
use std::{error::Error, fs, path::Path, process::Command};
const PRODUCERS: [(&str, &str, &str); 2] = [
    (
        "70496e67c7fd4a8b05914161a8e09de2759bebc8",
        "crates/lattice-workflow-contract/fixtures/producer-fixtures-v1.json",
        "fixtures/check-evidence/lattice-producer-fixtures.json",
    ),
    (
        "0bbab77e23ab95caa4e4184b9945cfaa5c8040e2",
        "fixtures/mantle-build-contract/producer/fixtures-v1.json",
        "fixtures/check-evidence/mantle-producer-fixtures.json",
    ),
];
fn main() -> Result<(), Box<dyn Error>> {
    let roots: Vec<_> = std::env::args_os().skip(1).collect();
    if roots.len() != PRODUCERS.len() {
        return Err("expected exact Lattice and Mantle source roots".into());
    }
    for (root, (revision, source, local)) in roots.iter().zip(PRODUCERS) {
        let head = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["rev-parse", "HEAD"])
            .output()?;
        if !head.status.success() || String::from_utf8(head.stdout)?.trim() != revision {
            return Err("producer revision mismatch".into());
        }
        let bytes = Command::new("git")
            .arg("-C")
            .arg(root)
            .args(["show", &format!("{revision}:{source}")])
            .output()?;
        if !bytes.status.success() || bytes.stdout != fs::read(Path::new(local))? {
            return Err("producer fixture bytes differ".into());
        }
    }
    println!("PASS: two original producer fixture identities and exact bytes");
    Ok(())
}
