use super::*;

pub(crate) fn print_json(value: Value) -> Result<(), Box<dyn Error>> {
    println!("{}", serde_json::to_string_pretty(&value)?);
    Ok(())
}

pub(crate) fn with_receipt_hash(mut value: Value) -> Value {
    let receipt_hash = canonical_json_hash(&value);
    value["receipt_hash"] = json!(receipt_hash);
    value
}

pub(crate) fn with_portable_receipt_hash(root: &Path, mut value: Value) -> Value {
    let receipt_hash = portable_json_hash(root, &value);
    value["receipt_hash"] = json!(receipt_hash);
    value
}
