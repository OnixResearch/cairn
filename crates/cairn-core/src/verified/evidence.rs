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
}
