use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").ok_or("missing crate root")?);
    for (name, relative) in [
        ("CAIRN_WORKSPACE_LOCK", "../../Cargo.lock"),
        (
            "CAIRN_INTEROP_PRODUCER_LOCK",
            "../../fixtures/radicle-cob-interoperability/producer/Cargo.lock",
        ),
    ] {
        let source = root.join(relative);
        println!("cargo:rerun-if-changed={}", source.display());
        // Both lockfiles are test-only inputs absent from production crate packages.
        if !source.try_exists()? {
            continue;
        }
        let canonical = source.canonicalize()?;
        let canonical = canonical.to_str().ok_or("non-UTF8 compile-time input path")?;
        println!("cargo:rustc-env={name}={canonical}");
    }
    Ok(())
}
