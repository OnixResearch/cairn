use serde_json::{Map, Value};

use crate::verified::evidence::canonical_json_hash;

use super::common::{entropy_hash_material_messages, is_blake3_hex, strip_non_hash_fields};

// r[impl electrical_circuit_primitives.topology]
// r[impl electrical_circuit_primitives.topology.reference_node]
// r[impl electrical_circuit_primitives.topology.connection_arity]
// r[impl electrical_circuit_primitives.components]
// r[impl electrical_circuit_primitives.components.passive_primitives]
// r[impl electrical_circuit_primitives.components.model_caveats]
// r[impl electrical_circuit_primitives.units]
// r[impl electrical_circuit_primitives.units.dimensioned_values]
// r[impl electrical_circuit_primitives.receipts]
// r[impl electrical_circuit_primitives.receipts.portable_hashes]
// r[impl electrical_circuit_primitive_receipts.validator]
// r[impl electrical_circuit_primitive_receipts.validator.core]
// r[impl electrical_circuit_primitive_receipts.fixtures]
// r[impl electrical_circuit_primitive_receipts.fixtures.positive]
// r[impl electrical_circuit_primitive_receipts.fixtures.negative]
pub const CIRCUIT_PRIMITIVE_SCHEMA_V1: &str = "electrical.circuit-primitive-graph.v1";

const REQUIRED_TOP_LEVEL_FIELDS: &[&str] = &[
    "schema",
    "graph_hash",
    "normalization",
    "source_artifact_hash",
    "tool",
    "reference_node",
    "nodes",
    "nets",
    "components",
    "non_claims",
];
const REQUIRED_NORMALIZATION_FIELDS: &[&str] = &["version"];
const REQUIRED_TOOL_FIELDS: &[&str] = &["name", "version"];
const REQUIRED_VALUE_FIELDS: &[&str] = &[
    "normalized_value",
    "si_dimension",
    "display_unit",
    "normalization_version",
];
// r[impl electrical_circuit_primitives.non_claims]
// r[impl electrical_circuit_primitives.non_claims.no_safety_claim]
const REQUIRED_NON_CLAIMS: &[&str] = &[
    "no_simulation_correctness",
    "no_analog_equivalence",
    "no_electrical_safety",
    "no_regulatory_compliance",
    "no_layout_correctness",
    "no_manufacturability",
];
const TWO_TERMINAL_KINDS: &[&str] = &[
    "resistor",
    "capacitor",
    "inductor",
    "voltage-source",
    "current-source",
    "switch",
];
const MODEL_CAVEATED_KINDS: &[&str] = &["behavioral-source", "vendor-model", "semiconductor-model"];
const PASSIVE_KINDS: &[&str] = &["resistor", "capacitor", "inductor"];
const REQUIRED_TERMINALS: &[&str] = &["p", "n"];
const CIRCUIT_ISSUE_CAPACITY: usize = 24;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitPrimitiveIssue {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CircuitPrimitiveReport {
    pub valid: bool,
    pub verdict: String,
    pub issues: Vec<CircuitPrimitiveIssue>,
    pub computed_graph_hash: String,
}

pub fn validate_circuit_primitive_receipt(receipt: &Value) -> CircuitPrimitiveReport {
    let computed_graph_hash = circuit_primitive_graph_hash(receipt);
    let issues = circuit_primitive_issues(receipt, &computed_graph_hash);
    let is_valid = issues.is_empty();
    CircuitPrimitiveReport {
        valid: is_valid,
        verdict: if is_valid { "PASS" } else { "FAIL" }.to_string(),
        issues,
        computed_graph_hash,
    }
}

pub fn circuit_primitive_graph_hash(receipt: &Value) -> String {
    // r[impl electrical_circuit_primitives.receipts]
    // r[impl electrical_circuit_primitives.receipts.portable_hashes]
    canonical_json_hash(&circuit_primitive_hash_material(receipt))
}

pub fn circuit_primitive_hash_material(receipt: &Value) -> Value {
    let mut material = receipt.clone();
    strip_non_hash_fields(&mut material);
    material
}

fn circuit_primitive_issues(
    receipt: &Value,
    computed_graph_hash: &str,
) -> Vec<CircuitPrimitiveIssue> {
    let Some(object) = receipt.as_object() else {
        return vec![issue(
            "circuit_primitive.not_object",
            "circuit primitive receipt must be a JSON object",
        )];
    };

    let mut issues = Vec::with_capacity(CIRCUIT_ISSUE_CAPACITY);
    issues.extend(required_top_level_issues(object));
    issues.extend(hash_issues(object, computed_graph_hash));
    issues.extend(required_nested_object_issues(object));
    issues.extend(topology_issues(object));
    issues.extend(component_issues(object));
    issues.extend(non_claim_issues(object));
    issues.extend(entropy_issues(
        "$",
        &circuit_primitive_hash_material(receipt),
    ));
    issues
}

fn required_top_level_issues(object: &Map<String, Value>) -> Vec<CircuitPrimitiveIssue> {
    let mut issues = Vec::with_capacity(REQUIRED_TOP_LEVEL_FIELDS.len() + 1);
    for field in REQUIRED_TOP_LEVEL_FIELDS {
        if !object.contains_key(*field) {
            issues.push(issue(
                "circuit_primitive.missing_field",
                format!("missing required field: {field}"),
            ));
        }
    }
    if object.get("schema").and_then(Value::as_str) != Some(CIRCUIT_PRIMITIVE_SCHEMA_V1) {
        issues.push(issue(
            "circuit_primitive.unsupported_schema",
            "schema must be electrical.circuit-primitive-graph.v1",
        ));
    }
    issues
}

fn hash_issues(
    object: &Map<String, Value>,
    computed_graph_hash: &str,
) -> Vec<CircuitPrimitiveIssue> {
    let mut issues = Vec::with_capacity(2);
    let stated_hash = object
        .get("graph_hash")
        .and_then(Value::as_str)
        .unwrap_or_default();
    if stated_hash != computed_graph_hash || !is_blake3_hex(stated_hash) {
        issues.push(issue(
            "circuit_primitive.hash_mismatch",
            "graph_hash must equal the BLAKE3 hash of canonical circuit primitive content with non-hash fields excluded",
        ));
    }
    if !object
        .get("source_artifact_hash")
        .and_then(Value::as_str)
        .is_some_and(is_blake3_hex)
    {
        issues.push(issue(
            "circuit_primitive.invalid_source_artifact_hash",
            "source_artifact_hash must be a BLAKE3 hex string",
        ));
    }
    issues
}

fn required_nested_object_issues(object: &Map<String, Value>) -> Vec<CircuitPrimitiveIssue> {
    let mut issues =
        Vec::with_capacity(REQUIRED_NORMALIZATION_FIELDS.len() + REQUIRED_TOOL_FIELDS.len());
    issues.extend(required_object_issues(
        object.get("normalization"),
        "normalization",
        REQUIRED_NORMALIZATION_FIELDS,
    ));
    issues.extend(required_object_issues(
        object.get("tool"),
        "tool",
        REQUIRED_TOOL_FIELDS,
    ));
    issues
}

fn topology_issues(object: &Map<String, Value>) -> Vec<CircuitPrimitiveIssue> {
    let mut issues = Vec::with_capacity(8);
    let node_ids = collect_ids(object.get("nodes"));
    let net_ids = collect_ids(object.get("nets"));

    let Some(reference_node) = object.get("reference_node").and_then(Value::as_str) else {
        issues.push(issue(
            "circuit_primitive.reference_node_missing",
            "reference_node must identify a node id or an explicit none value in future schemas",
        ));
        return issues;
    };
    if !node_ids.iter().any(|node| node == reference_node) {
        issues.push(issue(
            "circuit_primitive.reference_node_unknown",
            "reference_node must match a declared node id",
        ));
    }

    issues.extend(duplicate_id_issues("node", &node_ids));
    issues.extend(duplicate_id_issues("net", &net_ids));
    issues.extend(component_binding_issues(object.get("components"), &net_ids));
    issues
}

fn component_issues(object: &Map<String, Value>) -> Vec<CircuitPrimitiveIssue> {
    let mut issues = Vec::with_capacity(8);
    let Some(components) = object.get("components").and_then(Value::as_array) else {
        issues.push(issue(
            "circuit_primitive.components_not_array",
            "components must be an array",
        ));
        return issues;
    };
    for component in components {
        let Some(component_object) = component.as_object() else {
            issues.push(issue(
                "circuit_primitive.component_not_object",
                "each component must be a JSON object",
            ));
            continue;
        };
        let kind = component_object
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if PASSIVE_KINDS.contains(&kind) {
            issues.extend(passive_value_issues(component_object));
        }
        if MODEL_CAVEATED_KINDS.contains(&kind) {
            issues.extend(model_caveat_issues(component_object));
        }
    }
    issues
}

fn component_binding_issues(
    components: Option<&Value>,
    net_ids: &[String],
) -> Vec<CircuitPrimitiveIssue> {
    let mut issues = Vec::with_capacity(8);
    let Some(component_entries) = components.and_then(Value::as_array) else {
        return issues;
    };
    for component in component_entries {
        let Some(component_object) = component.as_object() else {
            continue;
        };
        let kind = component_object
            .get("kind")
            .and_then(Value::as_str)
            .unwrap_or_default();
        if !TWO_TERMINAL_KINDS.contains(&kind) && !MODEL_CAVEATED_KINDS.contains(&kind) {
            issues.push(issue(
                "circuit_primitive.unknown_component_kind",
                format!("component kind is outside the closed first-pass vocabulary: {kind}"),
            ));
            continue;
        }
        let Some(bindings) = component_object
            .get("terminal_bindings")
            .and_then(Value::as_object)
        else {
            issues.push(issue(
                "circuit_primitive.terminal_bindings_missing",
                "component must declare terminal_bindings",
            ));
            continue;
        };
        for terminal in REQUIRED_TERMINALS {
            match bindings.get(*terminal).and_then(Value::as_str) {
                Some(net_id) if net_ids.iter().any(|declared| declared == net_id) => {}
                Some(_) => issues.push(issue(
                    "circuit_primitive.terminal_net_unknown",
                    format!("terminal {terminal} must bind to a declared net id"),
                )),
                None => issues.push(issue(
                    "circuit_primitive.terminal_binding_missing",
                    format!("missing required terminal binding: {terminal}"),
                )),
            }
        }
    }
    issues
}

fn passive_value_issues(component: &Map<String, Value>) -> Vec<CircuitPrimitiveIssue> {
    let mut issues = Vec::with_capacity(REQUIRED_VALUE_FIELDS.len());
    let Some(value) = component.get("value").and_then(Value::as_object) else {
        return vec![issue(
            "circuit_primitive.value_missing",
            "passive primitives must carry a dimensioned value object",
        )];
    };
    for field in REQUIRED_VALUE_FIELDS {
        if !value.contains_key(*field) {
            issues.push(issue(
                "circuit_primitive.dimensioned_value_missing_field",
                format!("dimensioned value missing required field: {field}"),
            ));
        }
    }
    if !value
        .get("normalized_value")
        .is_some_and(|entry| entry.is_number())
    {
        issues.push(issue(
            "circuit_primitive.normalized_value_not_number",
            "normalized_value must be numeric",
        ));
    }
    issues
}

fn model_caveat_issues(component: &Map<String, Value>) -> Vec<CircuitPrimitiveIssue> {
    let has_provenance = component
        .get("model_provenance")
        .and_then(Value::as_str)
        .is_some_and(|value| !value.is_empty());
    let has_caveats = component
        .get("model_caveats")
        .and_then(Value::as_array)
        .is_some_and(|entries| !entries.is_empty());
    let mut issues = Vec::with_capacity(2);
    if !has_provenance {
        issues.push(issue(
            "circuit_primitive.model_provenance_missing",
            "behavioral and vendor-model components must record model provenance",
        ));
    }
    if !has_caveats {
        issues.push(issue(
            "circuit_primitive.model_caveats_missing",
            "behavioral and vendor-model components must record explicit caveats",
        ));
    }
    issues
}

fn non_claim_issues(object: &Map<String, Value>) -> Vec<CircuitPrimitiveIssue> {
    let Some(non_claims) = object.get("non_claims").and_then(Value::as_array) else {
        return vec![issue(
            "circuit_primitive.non_claims_missing",
            "non_claims must list circuit evidence boundaries",
        )];
    };
    REQUIRED_NON_CLAIMS
        .iter()
        .filter(|claim| {
            !non_claims
                .iter()
                .any(|entry| entry.as_str() == Some(**claim))
        })
        .map(|claim| {
            issue(
                "circuit_primitive.non_claim_missing",
                format!("missing required non-claim: {claim}"),
            )
        })
        .collect()
}

fn collect_ids(value: Option<&Value>) -> Vec<String> {
    value
        .and_then(Value::as_array)
        .map(|entries| {
            entries
                .iter()
                .filter_map(|entry| entry.get("id").and_then(Value::as_str))
                .map(ToString::to_string)
                .collect()
        })
        .unwrap_or_default()
}

fn duplicate_id_issues(kind: &str, ids: &[String]) -> Vec<CircuitPrimitiveIssue> {
    let mut issues = Vec::with_capacity(ids.len());
    for (index, id) in ids.iter().enumerate() {
        if ids.iter().skip(index + 1).any(|other| other == id) {
            issues.push(issue(
                "circuit_primitive.duplicate_id",
                format!("duplicate {kind} id: {id}"),
            ));
        }
    }
    issues
}

fn required_object_issues(
    value: Option<&Value>,
    name: &str,
    required_fields: &[&str],
) -> Vec<CircuitPrimitiveIssue> {
    let Some(object) = value.and_then(Value::as_object) else {
        return vec![issue(
            "circuit_primitive.required_object_missing",
            format!("{name} must be an object"),
        )];
    };
    required_fields
        .iter()
        .filter(|field| !object.contains_key(**field))
        .map(|field| {
            issue(
                "circuit_primitive.required_object_field_missing",
                format!("{name}.{field} is required"),
            )
        })
        .collect()
}

fn entropy_issues(root_path: &str, root: &Value) -> Vec<CircuitPrimitiveIssue> {
    entropy_hash_material_messages(root_path, root)
        .into_iter()
        .map(|message| {
            issue(
                "circuit_primitive.ambient_entropy_in_hash_material",
                message,
            )
        })
        .collect()
}

fn issue(code: impl Into<String>, message: impl Into<String>) -> CircuitPrimitiveIssue {
    CircuitPrimitiveIssue {
        code: code.into(),
        message: message.into(),
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, Value};

    use super::*;

    fn hash_placeholder() -> String {
        "0".repeat(64)
    }

    fn base_receipt(display_root: &str) -> Value {
        json!({
            "schema": CIRCUIT_PRIMITIVE_SCHEMA_V1,
            "graph_hash": hash_placeholder(),
            "normalization": {
                "version": "electrical-circuit-primitive-v1",
                "display_root": display_root
            },
            "source_artifact_hash": "1111111111111111111111111111111111111111111111111111111111111111",
            "tool": {
                "name": "cairn-fixture",
                "version": "0.1.0"
            },
            "reference_node": "gnd",
            "nodes": [
                {"id": "gnd"},
                {"id": "vin"},
                {"id": "vout"}
            ],
            "nets": [
                {"id": "net_gnd", "nodes": ["gnd"]},
                {"id": "net_vin", "nodes": ["vin"]},
                {"id": "net_vout", "nodes": ["vout"]}
            ],
            "components": [
                {
                    "id": "r1",
                    "kind": "resistor",
                    "terminal_bindings": {"p": "net_vin", "n": "net_vout"},
                    "value": {
                        "normalized_value": 1000.0,
                        "si_dimension": "ohm",
                        "display_unit": "kOhm",
                        "normalization_version": "si-v1"
                    },
                    "tolerance": "1%"
                },
                {
                    "id": "c1",
                    "kind": "capacitor",
                    "terminal_bindings": {"p": "net_vout", "n": "net_gnd"},
                    "value": {
                        "normalized_value": 0.000001,
                        "si_dimension": "farad",
                        "display_unit": "uF",
                        "normalization_version": "si-v1"
                    }
                }
            ],
            "non_claims": [
                "no_simulation_correctness",
                "no_analog_equivalence",
                "no_electrical_safety",
                "no_regulatory_compliance",
                "no_layout_correctness",
                "no_manufacturability"
            ]
        })
    }

    fn with_current_hash(mut receipt: Value) -> Value {
        let hash = circuit_primitive_graph_hash(&receipt);
        receipt["graph_hash"] = Value::String(hash);
        receipt
    }

    #[test]
    fn accepts_dimensioned_two_terminal_graph_with_non_claims() {
        let receipt = with_current_hash(base_receipt("/workspace/one"));
        let report = validate_circuit_primitive_receipt(&receipt);
        assert_eq!(report.verdict, "PASS");
        assert!(report.valid, "{:?}", report.issues);
        assert_eq!(receipt["graph_hash"], report.computed_graph_hash);
    }

    #[test]
    fn graph_hash_excludes_display_root_for_portability() {
        let left = base_receipt("/workspace/one");
        let right = base_receipt("/workspace/two");
        assert_eq!(
            circuit_primitive_graph_hash(&left),
            circuit_primitive_graph_hash(&right)
        );
    }

    #[test]
    fn rejects_missing_terminal_binding() {
        let mut receipt = base_receipt("/workspace/one");
        receipt["components"][0]["terminal_bindings"]
            .as_object_mut()
            .unwrap()
            .remove("n");
        let receipt = with_current_hash(receipt);
        let report = validate_circuit_primitive_receipt(&receipt);
        assert!(!report.valid);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "circuit_primitive.terminal_binding_missing"));
    }

    #[test]
    fn rejects_safety_non_claim_overclaim() {
        let mut receipt = base_receipt("/workspace/one");
        receipt["non_claims"] = json!(["no_simulation_correctness"]);
        let receipt = with_current_hash(receipt);
        let report = validate_circuit_primitive_receipt(&receipt);
        assert!(!report.valid);
        assert!(report
            .issues
            .iter()
            .any(|issue| issue.code == "circuit_primitive.non_claim_missing"));
    }
}
