use preserves::value::{self, packed::PackedWriter};
use serde_json::Value;

// r[impl cairn.supersede.blake3_receipts] r[impl cairn.supersede.blake3_receipts.canonical]
// r[impl cairn.supersede.blake3_receipts.gate] r[impl cairn.supersede.blake3_receipts.lifecycle_plans]
// r[impl cairn.supersede.blake3_receipts.migration] r[impl cairn.supersede.deterministic_gates.policy_freshness]
pub fn blake3_hex(bytes: &[u8]) -> String {
    blake3::hash(bytes).to_hex().to_string()
}

pub fn canonical_json(value: &Value) -> String {
    match serde_json::to_string(value) {
        Ok(text) => text,
        Err(error) => format!("serde_json_value_serialization_error:{error}"),
    }
}

pub fn canonical_json_hash(value: &Value) -> String {
    blake3_hex(canonical_json(value).as_bytes())
}

// r[impl cairn.determinism.preserves_receipt_canonicalization.additive_internal_seam]
pub fn canonical_preserves_bytes(value: &Value) -> Result<Vec<u8>, String> {
    let preserves_value = value::to_value(value);
    PackedWriter::encode_iovalue(&preserves_value).map_err(|error| error.to_string())
}

// r[impl cairn.determinism.preserves_receipt_canonicalization.additive_internal_seam]
// r[impl cairn.determinism.preserves_receipt_canonicalization.stable_hashing]
pub fn canonical_preserves_hash(value: &Value) -> Result<String, String> {
    canonical_preserves_bytes(value).map(|bytes| blake3_hex(&bytes))
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn hashes_canonical_json_deterministically() {
        let left = json!({"b": 2, "a": 1});
        let right = json!({"a": 1, "b": 2});
        assert_eq!(canonical_json(&left), canonical_json(&right));
        assert_eq!(canonical_json_hash(&left), canonical_json_hash(&right));
    }

    #[test]
    fn hash_changes_when_content_changes() {
        assert_ne!(
            canonical_json_hash(&json!({"actions": ["a"]})),
            canonical_json_hash(&json!({"actions": ["b"]}))
        );
    }

    #[test]
    // r[impl cairn.determinism.preserves_receipt_canonicalization.additive_internal_seam.encode]
    // r[verify cairn.determinism.preserves_receipt_canonicalization.stable_hashing.object_order]
    fn preserves_hashes_receipt_objects_deterministically() {
        let left = json!({"schema": "cairn.receipt.v1", "result": {"ok": true, "count": 2}});
        let right = json!({"result": {"count": 2, "ok": true}, "schema": "cairn.receipt.v1"});

        assert_eq!(
            canonical_preserves_bytes(&left).expect("left Preserves bytes"),
            canonical_preserves_bytes(&right).expect("right Preserves bytes")
        );
        assert_eq!(
            canonical_preserves_hash(&left).expect("left Preserves hash"),
            canonical_preserves_hash(&right).expect("right Preserves hash")
        );
    }

    #[test]
    // r[verify cairn.determinism.preserves_receipt_canonicalization.stable_hashing.content_change]
    // r[verify cairn.determinism.preserves_receipt_canonicalization.validation]
    // r[verify cairn.determinism.preserves_receipt_canonicalization.validation.focused]
    fn preserves_hash_changes_when_receipt_content_changes() {
        assert_ne!(
            canonical_preserves_hash(&json!({"actions": ["a"]})).expect("first hash"),
            canonical_preserves_hash(&json!({"actions": ["b"]})).expect("second hash")
        );
    }
}
