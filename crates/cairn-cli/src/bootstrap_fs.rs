//! Capability-relative, bounded bootstrap observations. No publication authority.
use cap_fs_ext::{DirExt, FollowSymlinks, OpenOptionsFollowExt, OpenOptionsSyncExt};
use cap_root_core::{AdmissionMode, RootRelativePath};
use cap_std::{
    ambient_authority,
    fs::{Dir, OpenOptions},
};
use std::{
    error::Error,
    io::{self, Read},
    path::{Component, Path},
};

const MAX_ROOT_COMPONENTS: usize = 256;

// The caller supplies root authority. Every component is opened without following links.
pub(crate) fn open_root(root: &Path) -> Result<Dir, Box<dyn Error>> {
    open_components(root, false)
}

/// Rebind path-based observations to the original directory capability.
pub(crate) fn recheck_root(root: &Dir, path: &Path) -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::MetadataExt;
    let current = open_root(path)?;
    let expected = root.try_clone()?.into_std_file().metadata()?;
    let observed = current.into_std_file().metadata()?;
    if (expected.dev(), expected.ino()) != (observed.dev(), observed.ino()) {
        return Err("bootstrap.setup.root_substituted".into());
    }
    Ok(())
}

fn open_components(root: &Path, explicit_parent_navigation: bool) -> Result<Dir, Box<dyn Error>> {
    let absolute = if root.is_absolute() {
        root.to_path_buf()
    } else {
        std::env::current_dir()?.join(root)
    };
    let mut directories = vec![Dir::open_ambient_dir(Path::new("/"), ambient_authority())?];
    for (index, component) in absolute.components().enumerate() {
        if index >= MAX_ROOT_COMPONENTS {
            return Err("bootstrap.path.root_bound".into());
        }
        match component {
            Component::Normal(leaf) => {
                let next = directories
                    .last()
                    .ok_or("bootstrap.path.unsafe_root")?
                    .open_dir_nofollow(leaf)?;
                directories.push(next);
            }
            // Preserve explicit --policy ../path compatibility. Open all preceding
            // components first: lexical cancellation must not conceal a symlink.
            Component::ParentDir if explicit_parent_navigation && directories.len() > 1 => {
                directories.pop();
            }
            Component::RootDir | Component::CurDir => {}
            _ => return Err("bootstrap.path.unsafe_root".into()),
        }
    }
    directories
        .pop()
        .ok_or_else(|| "bootstrap.path.unsafe_root".into())
}

/// Only a proven absent component returns None. A failed open after a positive
/// no-follow observation is an error, never permission to select a fallback.
pub(crate) fn read_optional(
    root: &Dir,
    locator: &str,
    maximum: u64,
) -> Result<Option<Vec<u8>>, Box<dyn Error>> {
    let admitted = RootRelativePath::new(locator, AdmissionMode::NonEmptyRelative)?;
    let mut directory = root.try_clone()?;
    if let Some(parent) = admitted.parent() {
        for component in parent.components() {
            let leaf = component.as_os_str();
            match directory.symlink_metadata(leaf) {
                Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
                Err(error) => return Err(error.into()),
                Ok(metadata) if !metadata.is_dir() => {
                    return Err("bootstrap.path.unsafe_parent".into())
                }
                Ok(_) => directory = directory.open_dir_nofollow(leaf)?,
            }
        }
    }
    let leaf = admitted.file_name().ok_or("bootstrap.path.invalid_leaf")?;
    match directory.symlink_metadata(leaf) {
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(None),
        Err(error) => return Err(error.into()),
        Ok(metadata) if !metadata.is_file() => return Err("bootstrap.path.invalid_file".into()),
        Ok(_) => {}
    }
    let mut options = OpenOptions::new();
    options.read(true).follow(FollowSymlinks::No).nonblock(true);
    let file = directory.open_with(leaf, &options)?;
    let metadata = file.metadata()?;
    if !metadata.is_file() || metadata.len() > maximum {
        return Err("bootstrap.path.invalid_file_or_bound".into());
    }
    let read_limit = maximum
        .checked_add(1)
        .ok_or("bootstrap.path.invalid_bound")?;
    let mut bytes = Vec::new();
    file.take(read_limit).read_to_end(&mut bytes)?;
    if bytes.len() as u64 > maximum {
        return Err("bootstrap.path.bound".into());
    }
    Ok(Some(bytes))
}

pub(crate) fn read_required(
    root: &Dir,
    locator: &str,
    maximum: u64,
) -> Result<Vec<u8>, Box<dyn Error>> {
    read_optional(root, locator, maximum)?.ok_or_else(|| "bootstrap.path.missing".into())
}

pub(crate) fn read_external(path: &Path, maximum: u64) -> Result<Vec<u8>, Box<dyn Error>> {
    let parent = path
        .parent()
        .filter(|path| !path.as_os_str().is_empty())
        .unwrap_or(Path::new("."));
    let leaf = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or("bootstrap.path.invalid_leaf")?;
    read_required(&open_components(parent, true)?, leaf, maximum)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn absence_bounds_and_unsafe_files_remain_distinct() -> Result<(), Box<dyn Error>> {
        let root = std::env::temp_dir().join(format!("cairn-bootstrap-fs-{}", std::process::id()));
        fs::create_dir(&root)?;
        let cap = open_root(&root)?;
        fs::write(root.join("file"), b"1234")?;
        assert_eq!(read_required(&cap, "file", 4)?, b"1234");
        assert!(read_required(&cap, "file", 3).is_err());
        assert_eq!(read_optional(&cap, "absent/leaf", 4)?, None);
        assert!(read_required(&cap, "file/leaf", 4).is_err());
        assert!(read_required(&cap, "../file", 4).is_err());
        fs::create_dir(root.join("directory"))?;
        assert_eq!(read_external(&root.join("directory/../file"), 4)?, b"1234");
        assert!(read_optional(&cap, "directory", 4).is_err());
        #[cfg(unix)]
        {
            for target in ["absent", "file", "directory"] {
                std::os::unix::fs::symlink(target, root.join("link"))?;
                assert!(read_optional(&cap, "link", 4).is_err());
                assert!(read_optional(&cap, "link/leaf", 4).is_err());
                assert!(read_external(&root.join("link/../file"), 4).is_err());
                fs::remove_file(root.join("link"))?;
            }
        }
        fs::remove_dir_all(root)?;
        Ok(())
    }
}
