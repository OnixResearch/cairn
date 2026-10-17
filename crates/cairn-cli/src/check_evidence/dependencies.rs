//! Bounded dependency-file observation; supplied digest assertions are never evidence.
use super::{local, named_options, read_input, reads};
use crate::support::{print_json, CapabilityRoot};
use cairn_core::canonical_json_hash;
use serde_json::json;
use std::{error::Error, path::Path};

const MAX_DEPENDENCY_FILES: usize = 64;

pub(super) fn observe(
    root: &CapabilityRoot,
    paths: &[String],
) -> Result<Vec<String>, &'static str> {
    if paths.len() > MAX_DEPENDENCY_FILES || !paths.windows(2).all(|p| p[0] < p[1]) {
        return Err("dependency projection requires a bounded sorted unique file set");
    }
    let mut hashes = Vec::with_capacity(paths.len());
    let mut total = 0_usize;
    for path in paths {
        // Reject aliases as well as escapes: the identity binds exactly one relative spelling.
        if path.contains('\0')
            || path
                .split('/')
                .any(|part| part.is_empty() || part == "." || part == "..")
        {
            return Err("unsafe dependency projection");
        }
        let relative = root
            .relative(Path::new(path))
            .map_err(|_| "unsafe dependency projection")?;
        let bytes = reads::input(root, &relative, local::MAX_SOURCE_FILE_BYTES)
            .map_err(|_| "dependency unavailable or oversized")?;
        total = total
            .checked_add(bytes.len())
            .ok_or("dependency size overflow")?;
        if total > local::MAX_SOURCE_TOTAL_BYTES {
            return Err("dependency projection exceeds aggregate bound");
        }
        hashes.push(canonical_json_hash(&json!({
            "schema":"cairn.check-dependency.v1", "path":path,
            "content_hash":blake3::hash(&bytes).to_hex().to_string()
        })));
    }
    hashes.sort_unstable();
    hashes.dedup();
    Ok(hashes)
}

pub(super) fn load(
    root: &CapabilityRoot,
    projection: Option<&str>,
    declared: &[String],
) -> Result<Vec<String>, Box<dyn Error>> {
    let paths = match projection {
        Some(name) => read_input(root, name)?,
        None if declared.is_empty() => Vec::new(),
        None => return Err("nonempty dependencies require --dependencies projection".into()),
    };
    recheck(root, &paths, declared)?;
    Ok(paths)
}

pub(super) fn recheck(
    root: &CapabilityRoot,
    paths: &[String],
    declared: &[String],
) -> Result<(), &'static str> {
    if observe(root, paths)? != declared {
        return Err("dependency identities changed");
    }
    Ok(())
}

// r[impl cairn.check_evidence_producer.dependency_observation]
pub(super) fn run(args: &[String]) -> Result<(), Box<dyn Error>> {
    let options = named_options(args, &["--root", "--projection"])?;
    let root = local::open_root(Path::new(options.get("--root").copied().unwrap_or(".")))?;
    let paths: Vec<String> = read_input(
        &root,
        options.get("--projection").ok_or("missing --projection")?,
    )?;
    let hashes = observe(&root, &paths)?;
    print_json(json!({
        "schema":"cairn.check-dependency-receipt.v1",
        "dependency_hashes":hashes,
        "dependency_projection_hash":canonical_json_hash(&json!(paths)),
        "checks_executed":false,
        "non_claims":["the selected dependency files do not prove dependency closure completeness, package resolution, or remote-source attestation"]
    }))
}
