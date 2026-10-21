use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").ok_or("missing crate root")?);
    for (name, relative, mandatory) in [
        (
            "CAIRN_LIFECYCLE_STORE_CONTRACTS",
            "../../cairn-policy/lifecycle-store-contracts.ncl",
            true,
        ),
        (
            "CAIRN_BOOTSTRAP_TEST_MATERIAL",
            "tests/bootstrap_preview/material.rs",
            false,
        ),
        ("CAIRN_FLAKE_LOCK", "../../flake.lock", false),
    ] {
        let source = root.join(relative);
        println!("cargo:rerun-if-changed={}", source.display());
        // Test-only inputs need not be shipped in a production crate package.
        if !mandatory && !source.try_exists()? {
            continue;
        }
        let canonical = source.canonicalize()?;
        let canonical = canonical
            .to_str()
            .ok_or("non-UTF8 compile-time input path")?;
        println!("cargo:rustc-env={name}={canonical}");
    }
    Ok(())
}
