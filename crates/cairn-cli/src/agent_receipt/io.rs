use std::{error::Error, path::Path};

use serde_json::Value;

use super::AGENT_RECEIPT_MAX_BYTES;

pub(crate) fn read_json_artifact(path: &Path, max_bytes: u64) -> Result<Value, Box<dyn Error>> {
    crate::support::read_bounded_json_no_symlink(path, max_bytes, "artifact")
}

pub(crate) fn read_agent_receipt_file(path: &Path) -> Result<Value, Box<dyn Error>> {
    // r[impl cairn.supersede.agent_receipt_validation_surface.cli]
    crate::support::read_bounded_json_no_symlink(path, AGENT_RECEIPT_MAX_BYTES, "agent receipt")
}

pub(crate) fn ensure_inline_receipt_is_bounded(receipt: &Value) -> Result<(), Box<dyn Error>> {
    let bytes = serde_json::to_vec(receipt)?;
    if bytes.len() as u64 > AGENT_RECEIPT_MAX_BYTES {
        return Err(format!(
            "inline agent receipt exceeds bounded read limit: {} bytes > {}",
            bytes.len(),
            AGENT_RECEIPT_MAX_BYTES
        )
        .into());
    }
    Ok(())
}
