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
const VALENCE_REVISION: &str = "e40c76b4d2070a29636e00c85c0dff93f03dba2f";
const OCTET_REVISION: &str = "890f424cd7b1b2015f39c0eb5747f1bd1e576028";
const SOURCE_ROOT_COUNT: usize = 4;
const VALENCE_ROOT_INDEX: usize = 2;
const OCTET_ROOT_INDEX: usize = 3;
fn octet(root: &std::ffi::OsStr) -> Result<(), Box<dyn Error>> {
    let git = |args: &[&str]| Command::new("git").arg("-C").arg(root).args(args).output();
    let head = git(&["rev-parse", "HEAD"])?;
    if !head.status.success() || String::from_utf8(head.stdout)?.trim() != OCTET_REVISION {
        return Err("Octet producer revision mismatch".into());
    }
    if !git(&["diff", "--exit-code", "HEAD", "--", "standards", "LICENSE"])?
        .status
        .success()
    {
        return Err("Octet export source differs from the pinned revision".into());
    }
    let license = git(&["show", &format!("{OCTET_REVISION}:LICENSE")])?;
    if !license.status.success()
        || license.stdout != fs::read("LICENSES/octet-producer-fixtures-LICENSE")?
    {
        return Err("Octet notice differs from the pinned source".into());
    }
    let export = |fixture: &str| {
        Command::new("nickel")
            .current_dir(root)
            .args(["export", "--format", "json", fixture])
            .output()
    };
    let positive = export("standards/receipts/fixtures/valid/acceptance_receipt.ncl")?;
    if !positive.status.success()
        || positive.stdout != fs::read("fixtures/check-evidence/octet-producer-fixture.json")?
    {
        return Err("Octet exported fixture differs".into());
    }
    if export("standards/receipts/fixtures/invalid/missing_config_digest.ncl")?
        .status
        .success()
    {
        return Err("Octet negative fixture unexpectedly exported".into());
    }
    Ok(())
}
fn valence(root: &std::ffi::OsStr) -> Result<(), Box<dyn Error>> {
    let git = |args: &[&str]| Command::new("git").arg("-C").arg(root).args(args).output();
    let head = git(&["rev-parse", "HEAD"])?;
    if !head.status.success() || String::from_utf8(head.stdout)?.trim() != VALENCE_REVISION {
        return Err("Valence producer revision mismatch".into());
    }
    // Reject modified tracked inputs before asking Nickel to resolve imports.
    if !git(&["diff", "--exit-code", "HEAD", "--", "contracts", "LICENSES"])?
        .status
        .success()
    {
        return Err("Valence export source differs from the pinned revision".into());
    }
    let license = git(&[
        "show",
        &format!("{VALENCE_REVISION}:LICENSES/AGPL-3.0-or-later.txt"),
    ])?;
    if !license.status.success()
        || license.stdout != fs::read("LICENSES/valence-producer-fixtures-LICENSE")?
    {
        return Err("Valence notice differs from the pinned source".into());
    }
    let export = |fixture: &str| {
        Command::new("nickel")
            .current_dir(root)
            .args(["export", "--format", "json", fixture])
            .output()
    };
    let positive = export("contracts/stack-provenance/fixtures/pass/sidecar.ncl")?;
    if !positive.status.success()
        || positive.stdout != fs::read("fixtures/check-evidence/valence-sidecar.json")?
    {
        return Err("Valence exported fixture differs".into());
    }
    for fixture in [
        "bad-schema",
        "malformed-blake3",
        "missing-non-claim",
        "missing-sidecar-role",
    ] {
        if export(&format!(
            "contracts/stack-provenance/fixtures/fail/{fixture}.ncl"
        ))?
        .status
        .success()
        {
            return Err("Valence negative fixture unexpectedly exported".into());
        }
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let roots: Vec<_> = std::env::args_os().skip(1).collect();
    if roots.len() != SOURCE_ROOT_COUNT {
        return Err("expected exact Lattice, Mantle, Valence, and Octet source roots".into());
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
    valence(&roots[VALENCE_ROOT_INDEX])?;
    octet(&roots[OCTET_ROOT_INDEX])?;
    println!("PASS: four exact producer revisions, original fixture bytes/exports, notices, and five negative exports");
    Ok(())
}
