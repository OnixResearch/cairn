//! Admission of the supported native Nix 2.31 derivation-build JSON surface.
use serde_json::Value;

use super::MAX_CHECKS;

const STORE_HASH_BYTES: usize = 32;
const MAX_STORE_NAME_BYTES: usize = 211;
const NIX32: &[u8] = b"0123456789abcdfghijklmnpqrsvwxyz";

/// Admits identities, not fresh builder execution, test coverage, or oracle truth.
///
/// Nix 2.31 `src/nix/build.cc::builtPathsWithResultToJSON` adds optional
/// timing fields to `src/libcmd/built-path.cc::BuiltPath::Built::toJSON`.
/// Opaque paths and dynamic derivation identities are deliberately unsupported.
/// Rows and each output map are bounded by the producer's 64-member check bound.
// r[impl cairn.check_evidence_producer.native_nix_build]
pub fn nix_build_payload_passed(payload: &Value) -> bool {
    let Some(rows) = payload.as_array() else {
        return false;
    };
    if rows.is_empty() || rows.len() > MAX_CHECKS {
        return false;
    }
    // Borrow identities into fixed storage: no copies or heap allocation.
    let mut derivations = [""; MAX_CHECKS];
    for (index, row) in rows.iter().enumerate() {
        let Some(fields) = row.as_object() else {
            return false;
        };
        if fields.keys().any(|key| {
            !matches!(
                key.as_str(),
                "drvPath" | "outputs" | "startTime" | "stopTime" | "cpuUser" | "cpuSystem"
            )
        }) {
            return false;
        }
        let Some(drv) = row["drvPath"].as_str() else {
            return false;
        };
        if !store_path(drv) || !drv.ends_with(".drv") || derivations[..index].contains(&drv) {
            return false;
        }
        derivations[index] = drv;
        let Some(outputs) = row["outputs"].as_object() else {
            return false;
        };
        if outputs.is_empty() || outputs.len() > MAX_CHECKS {
            return false;
        }
        let mut paths = [""; MAX_CHECKS];
        for (index, (name, value)) in outputs.iter().enumerate() {
            let Some(path) = value.as_str() else {
                return false;
            };
            if !store_name(name) || !store_path(path) || paths[..index].contains(&path) {
                return false;
            }
            paths[index] = path;
        }
        for key in ["startTime", "stopTime"] {
            if fields
                .get(key)
                .is_some_and(|value| value.as_u64().is_none_or(|time| time == 0))
            {
                return false;
            }
        }
        if let (Some(start), Some(stop)) = (row["startTime"].as_u64(), row["stopTime"].as_u64()) {
            if stop < start {
                return false;
            }
        }
        for key in ["cpuUser", "cpuSystem"] {
            if fields.get(key).is_some_and(|value| {
                value
                    .as_f64()
                    .is_none_or(|cpu| !cpu.is_finite() || cpu < 0.0)
            }) {
                return false;
            }
        }
    }
    true
}

// Nix 2.31 src/libstore/path.cc and include/nix/store/path.hh.
fn store_path(path: &str) -> bool {
    let Some(leaf) = path.strip_prefix("/nix/store/") else {
        return false;
    };
    let bytes = leaf.as_bytes();
    bytes.len() > STORE_HASH_BYTES + 1
        && bytes[STORE_HASH_BYTES] == b'-'
        && bytes[..STORE_HASH_BYTES]
            .iter()
            .all(|byte| NIX32.contains(byte))
        && store_name(&leaf[STORE_HASH_BYTES + 1..])
}

fn store_name(name: &str) -> bool {
    !name.is_empty()
        && name.len() <= MAX_STORE_NAME_BYTES
        && !matches!(name, "." | "..")
        && !name.starts_with(".-")
        && !name.starts_with("..-")
        && name
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"+-._?=".contains(&byte))
}
