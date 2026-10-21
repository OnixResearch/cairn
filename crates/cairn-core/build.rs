use std::{env, error::Error, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    let root = PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").ok_or("missing crate root")?);
    let source = root.join("../../fixtures/check-evidence/lattice-cohort.json");
    println!("cargo:rerun-if-changed={}", source.display());
    // This fixture is compiled only by tests, not by packaged production builds.
    if source.try_exists()? {
        let canonical = source.canonicalize()?;
        let canonical = canonical
            .to_str()
            .ok_or("non-UTF8 compile-time input path")?;
        println!("cargo:rustc-env=CAIRN_LATTICE_COHORT={canonical}");
    }
    Ok(())
}
