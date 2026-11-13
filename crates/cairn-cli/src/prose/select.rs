//! Deterministic, confined selection of one active change's authoring files.
use std::{
    error::Error,
    io,
    path::{Path, PathBuf},
};

use crate::support::CapabilityRoot;
use cairn_core::validate_change_name;

const MAX_ENTRIES: usize = 16_384;
const MAX_DEPTH: usize = 64;
pub(super) const MAX_FILES: usize = 64;

// r[impl cairn.prose_workflow.selection]
pub(super) fn change_files(
    root: &CapabilityRoot,
    change: &str,
) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    validate_change_name(change).map_err(|error| format!("invalid prose change name: {error}"))?;
    let directory = PathBuf::from(".cairn/changes").join(change);
    if !root.metadata_nofollow(&directory)?.is_dir() {
        return Err("prose --change requires an active change directory".into());
    }
    let mut paths = Vec::new();
    for name in ["proposal.md", "design.md", "tasks.md"] {
        let path = directory.join(name);
        match root.metadata_nofollow(&path) {
            Ok(metadata) if metadata.is_file() => paths.push(path),
            Ok(_) => return Err("prose change artifact is not a regular file".into()),
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
    }
    let specs = directory.join("specs");
    match root.metadata_nofollow(&specs) {
        Ok(metadata) if metadata.is_dir() => {}
        Ok(_) => return Err("prose change specs is not a directory".into()),
        Err(error) if error.kind() == io::ErrorKind::NotFound => return finish(paths),
        Err(error) => return Err(error.into()),
    }
    let mut directories = vec![specs];
    let mut remaining = MAX_ENTRIES;
    while let Some(directory) = directories.pop() {
        if directory.components().count() > MAX_DEPTH {
            return Err("prose change traversal exceeds its depth limit".into());
        }
        let entries = root.entries_bounded(&directory, remaining)?;
        remaining -= entries.len();
        for entry in entries {
            let path = directory.join(entry.file_name());
            if entry.file_type().is_symlink() {
                return Err("prose change traversal rejects symbolic links".into());
            }
            if entry.file_type().is_dir() {
                directories.push(path);
            } else if path.file_name().is_some_and(|name| name == "spec.md") {
                if !entry.file_type().is_file() {
                    return Err("prose spec is not a regular file".into());
                }
                paths.push(path);
                if paths.len() > MAX_FILES {
                    return Err("prose check accepts at most 64 files".into());
                }
            }
        }
    }
    finish(paths)
}

fn finish(mut paths: Vec<PathBuf>) -> Result<Vec<PathBuf>, Box<dyn Error>> {
    paths.sort();
    if paths.is_empty() || paths.len() > MAX_FILES {
        return Err("prose selection requires 1 through 64 files".into());
    }
    Ok(paths)
}

pub(super) fn path_text(path: &Path) -> Result<&str, Box<dyn Error>> {
    path.to_str()
        .ok_or_else(|| "prose paths must be UTF-8".into())
}
