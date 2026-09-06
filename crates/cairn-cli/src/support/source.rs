//! Bounded, no-follow admission for lifecycle source observations.
use super::capability::{CapabilityDirEntry, CapabilityRoot, CapabilityRootKind};
use std::{io, path::Path};

pub(crate) const MAX_SOURCE_FILE_BYTES: usize = 1024 * 1024;
pub(crate) const MAX_SOURCE_TOTAL_BYTES: usize = 16 * MAX_SOURCE_FILE_BYTES;
pub(crate) const MAX_SOURCE_ENTRIES: usize = 16 * 1024;
pub(crate) const MAX_SOURCE_DEPTH: usize = 64;

pub(crate) struct SourceReader {
    capability: CapabilityRoot,
    remaining_bytes: usize,
    remaining_entries: usize,
}

impl SourceReader {
    pub(crate) fn open(root: &Path) -> io::Result<Self> {
        Ok(Self {
            capability: CapabilityRoot::open_nofollow(CapabilityRootKind::Scan, root)?,
            remaining_bytes: MAX_SOURCE_TOTAL_BYTES,
            remaining_entries: MAX_SOURCE_ENTRIES,
        })
    }

    pub(crate) fn display_path(&self, path: &Path) -> io::Result<std::path::PathBuf> {
        self.capability.display_path(path)
    }

    pub(crate) fn read_to_string(&mut self, path: &Path) -> io::Result<String> {
        let limit = self.remaining_bytes.min(MAX_SOURCE_FILE_BYTES);
        let bytes = self.capability.read_bounded(path, limit as u64)?;
        self.remaining_bytes -= bytes.len();
        String::from_utf8(bytes).map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
    }

    pub(crate) fn read_to_string_path(&mut self, path: &Path) -> io::Result<String> {
        let relative = self.capability.relative_from_display_path(path)?;
        self.read_to_string(&relative)
    }

    pub(crate) fn metadata(&self, path: &Path) -> io::Result<cap_std::fs::Metadata> {
        let metadata = self.capability.metadata_nofollow(path)?;
        if !metadata.is_file() {
            return Err(source_limit_error(
                "lifecycle artifact is not a regular file",
            ));
        }
        Ok(metadata)
    }

    pub(crate) fn metadata_path(&self, path: &Path) -> io::Result<cap_std::fs::Metadata> {
        let relative = self.capability.relative_from_display_path(path)?;
        self.metadata(&relative)
    }

    pub(crate) fn entries(&mut self, path: &Path) -> io::Result<Vec<CapabilityDirEntry>> {
        if path.components().count() > MAX_SOURCE_DEPTH {
            return Err(source_limit_error(
                "lifecycle traversal exceeds its depth bound",
            ));
        }
        let mut entries = self
            .capability
            .entries_bounded(path, self.remaining_entries)?;
        self.remaining_entries -= entries.len();
        if entries.iter().any(|entry| entry.file_type().is_symlink()) {
            return Err(source_limit_error(
                "lifecycle traversal rejects symbolic links",
            ));
        }
        entries.sort_by(|left, right| left.file_name().cmp(right.file_name()));
        Ok(entries)
    }
}

fn source_limit_error(message: &'static str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message)
}

#[cfg(test)]
#[path = "source_tests.rs"]
mod tests;
