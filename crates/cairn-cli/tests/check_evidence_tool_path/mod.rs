//! Bounded `PATH` for check-evidence fixture profiles.
//!
//! Admission caps every profile environment value at `MAX_TEXT_BYTES`. The
//! harness therefore passes only the directories that hold the tools a fixture
//! runs, resolved from the current `PATH`, deduplicated in the order the tools
//! are named. A result above the bound is a harness error, never a truncation.
use std::{error::Error, path::PathBuf};

use cairn_core::verified::check_evidence::MAX_TEXT_BYTES;

pub fn tool_path(tools: &[&str]) -> Result<String, Box<dyn Error>> {
    let ambient = std::env::var_os("PATH").ok_or("missing PATH")?;
    let mut directories: Vec<PathBuf> = Vec::new();
    for tool in tools {
        let directory = std::env::split_paths(&ambient)
            .find(|directory| directory.join(tool).is_file())
            .ok_or_else(|| format!("required fixture tool {tool} is not on PATH"))?;
        if !directories.contains(&directory) {
            directories.push(directory);
        }
    }
    let joined = std::env::join_paths(&directories)?
        .into_string()
        .map_err(|_| "fixture tool directory is not UTF-8")?;
    if joined.len() > MAX_TEXT_BYTES {
        return Err(format!(
            "fixture tool PATH is {} bytes, above the {MAX_TEXT_BYTES}-byte profile bound",
            joined.len()
        )
        .into());
    }
    Ok(joined)
}
