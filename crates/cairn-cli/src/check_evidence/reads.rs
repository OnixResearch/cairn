//! Linux capability-relative reads without following any path component.
//! O_PATH opens an inode without device/FIFO read effects. O_NOFOLLOW makes a
//! final symlink an inspectable symlink descriptor, which we reject. Each parent
//! is opened separately and held. Only a verified regular descriptor is reopened
//! through procfs; that reopen names the held inode, not the mutable input path.
use cap_std::fs::{Dir, OpenOptions, OpenOptionsExt};
use std::{
    io::{self, Read},
    os::fd::AsRawFd,
    path::{Component, Path},
};

// Linux asm-generic flags shared by x86_64 and aarch64. Other ABIs fail closed.
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
const LINUX_O_PATH: i32 = 0o10000000;
#[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
const LINUX_O_NOFOLLOW: i32 = 0o400000;

pub(super) fn read(root: &Dir, path: &Path, maximum: u64) -> io::Result<Vec<u8>> {
    #[cfg(any(target_arch = "x86_64", target_arch = "aarch64"))]
    {
        let mut directory = root.try_clone()?;
        let mut components = path.components().peekable();
        while let Some(component) = components.next() {
            let Component::Normal(name) = component else {
                return Err(rejected());
            };
            let mut options = OpenOptions::new();
            options
                .read(true)
                .custom_flags(LINUX_O_PATH | LINUX_O_NOFOLLOW);
            let held = directory.open_with(name, &options)?;
            let metadata = held.metadata()?;
            if components.peek().is_some() {
                if !metadata.is_dir() {
                    return Err(rejected());
                }
                directory = Dir::from_std_file(held.into_std());
                continue;
            }
            if !metadata.is_file() || metadata.len() > maximum {
                return Err(rejected());
            }
            let descriptor = format!("/proc/self/fd/{}", held.as_raw_fd());
            let file = std::fs::File::open(descriptor)?;
            let mut bytes = Vec::new();
            file.take(maximum.saturating_add(1))
                .read_to_end(&mut bytes)?;
            if u64::try_from(bytes.len()).map_err(|_| rejected())? > maximum {
                return Err(rejected());
            }
            return Ok(bytes);
        }
        Err(rejected())
    }
    #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
    {
        let _ = (root, path, maximum);
        Err(rejected())
    }
}

fn rejected() -> io::Error {
    io::Error::new(
        io::ErrorKind::InvalidInput,
        "producer input is not a bounded no-follow regular file",
    )
}

pub(super) fn input(
    root: &crate::support::CapabilityRoot,
    path: &Path,
    maximum: u64,
) -> io::Result<Vec<u8>> {
    let relative = root.relative(path)?;
    read(
        &Dir::from_std_file(root.clone_directory_file()?),
        &relative,
        maximum,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use cap_std::ambient_authority;
    use std::{
        fs,
        os::unix::{fs::symlink, net::UnixListener},
    };

    #[test]
    fn held_inode_reads_reject_links_special_files_and_bounds() -> io::Result<()> {
        let path =
            std::env::temp_dir().join(format!("cairn-producer-nofollow-{}", std::process::id()));
        fs::create_dir_all(path.join("nested"))?;
        fs::write(path.join("nested/source"), b"source")?;
        symlink("nested", path.join("parent-link"))?;
        symlink("nested/source", path.join("leaf-link"))?;
        let _socket = UnixListener::bind(path.join("socket"))?;
        let root = Dir::open_ambient_dir(&path, ambient_authority())?;
        const LIMIT: u64 = 6;
        assert_eq!(read(&root, Path::new("nested/source"), LIMIT)?, b"source");
        for name in [
            "parent-link/source",
            "leaf-link",
            "socket",
            "nested",
            "../outside",
            "/etc/passwd",
            "absent",
        ] {
            assert!(
                read(&root, Path::new(name), LIMIT).is_err(),
                "admitted {name}"
            );
        }
        assert!(read(&root, Path::new("nested/source"), LIMIT - 1).is_err());
        fs::write(path.join("secret"), b"secret")?;
        const RACE_ROUNDS: usize = 128;
        std::thread::scope(|scope| -> io::Result<()> {
            let writer = scope.spawn(|| -> io::Result<()> {
                for _ in 0..RACE_ROUNDS {
                    symlink("../secret", path.join("nested/replacement"))?;
                    fs::rename(path.join("nested/replacement"), path.join("nested/source"))?;
                    fs::write(path.join("nested/replacement"), b"source")?;
                    fs::rename(path.join("nested/replacement"), path.join("nested/source"))?;
                }
                Ok(())
            });
            for _ in 0..RACE_ROUNDS {
                if let Ok(bytes) = read(&root, Path::new("nested/source"), LIMIT) {
                    assert_eq!(bytes, b"source", "followed a substituted symlink");
                }
            }
            writer.join().expect("substitution worker panicked")?;
            Ok(())
        })?;
        fs::remove_dir_all(path)?;
        Ok(())
    }
}
