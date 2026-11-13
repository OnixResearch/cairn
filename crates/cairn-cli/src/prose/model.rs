//! Report identities and hard resource ceilings shared by checking and rechecking.
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::error::Error;

pub(super) const MAX_DOCUMENT_BYTES: u64 = 1_048_576;
pub(super) const MAX_TOTAL_BYTES: u64 = 16 * MAX_DOCUMENT_BYTES;
pub(super) const MAX_DIAGNOSTICS: usize = 4096;
pub(super) const MAX_REPORT_BYTES: usize = 1_048_576;
pub(super) const ENGINE_REVISION: &str = "336ed696eec73e5b9858c5d7952b54c8e93b9df9";
pub(super) const NON_CLAIMS: &[&str] = &[
    "prose diagnostics are advisory and do not determine lifecycle gate outcomes",
    "grammar observations do not prove requirement validity, consistency, completeness, or satisfaction",
    "prose diagnostics are not implementation, verification, readiness, or release evidence",
    "all suggestions require author review and are never applied by this command",
];

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Bounds {
    pub(super) max_total_bytes: u64,
    pub(super) max_diagnostics: usize,
    pub(super) max_report_bytes: usize,
}

impl Default for Bounds {
    fn default() -> Self {
        Self {
            max_total_bytes: MAX_TOTAL_BYTES,
            max_diagnostics: MAX_DIAGNOSTICS,
            max_report_bytes: MAX_REPORT_BYTES,
        }
    }
}

impl Bounds {
    pub(super) fn validate(&self) -> Result<(), Box<dyn Error>> {
        if self.max_total_bytes == 0
            || self.max_total_bytes > MAX_TOTAL_BYTES
            || self.max_diagnostics == 0
            || self.max_diagnostics > MAX_DIAGNOSTICS
            || self.max_report_bytes == 0
            || self.max_report_bytes > MAX_REPORT_BYTES
        {
            return Err("prose bounds must be positive and cannot exceed the hard ceilings".into());
        }
        Ok(())
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub(super) enum Selection {
    Explicit,
    Change { change: String },
}

pub(super) fn bind_report(report: &mut Value) {
    let hash = cairn_core::canonical_json_hash(report);
    report["report_blake3"] = Value::String(hash);
}

pub(super) fn encoded(report: &Value, maximum: usize) -> Result<String, Box<dyn Error>> {
    struct Limited {
        bytes: Vec<u8>,
        maximum: usize,
    }
    impl std::io::Write for Limited {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if bytes.len() > self.maximum.saturating_sub(self.bytes.len()) {
                return Err(std::io::Error::other(
                    "prose report exceeds its output limit",
                ));
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut writer = Limited {
        bytes: Vec::new(),
        maximum: maximum.saturating_sub(1),
    };
    serde_json::to_writer_pretty(&mut writer, report)?;
    let mut result = String::from_utf8(writer.bytes)?;
    result.push('\n');
    Ok(result)
}
