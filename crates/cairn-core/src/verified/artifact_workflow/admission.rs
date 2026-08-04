use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Component, Path},
};

use serde_json::Value;

use super::model::*;

const TEMPLATE_OPEN: &str = "{{";
const TEMPLATE_CLOSE: &str = "}}";
const TEMPLATE_TOKEN_CHANGE: &str = "change";
const TEMPLATE_TOKEN_ARTIFACT: &str = "artifact";
const PREREQUISITE_GATE_PREFIX: &str = "gate:";
const PREREQUISITE_ARTIFACT_PREFIX: &str = "artifact:";

// r[impl cairn.artifact_workflow_contract.policy]
// r[impl cairn.artifact_workflow_contract.policy.valid]
// r[impl cairn.artifact_workflow_contract.functional_boundary]
// r[impl cairn.artifact_workflow_contract.functional_boundary.pure]
pub fn workflow_schemas_from_value(
    value: &Value,
) -> Result<Vec<WorkflowSchemaPolicy>, WorkflowAdmissionError> {
    let schemas_value = value.get("schemas").cloned().ok_or_else(|| {
        admission_error(
            WORKFLOW_ISSUE_POLICY_SHAPE,
            "schemas",
            "workflow schemas are missing",
        )
    })?;
    let schemas =
        serde_json::from_value::<Vec<WorkflowSchemaPolicy>>(schemas_value).map_err(|error| {
            admission_error(
                WORKFLOW_ISSUE_POLICY_SHAPE,
                "schemas",
                format!("workflow schema shape is invalid: {error}"),
            )
        })?;
    let allowed_stages = value
        .get("lifecycle_stages")
        .and_then(Value::as_array)
        .map(|stages| {
            stages
                .iter()
                .filter_map(Value::as_str)
                .map(ToOwned::to_owned)
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();
    admit_workflow_schemas(&schemas, &allowed_stages)?;
    Ok(schemas)
}

pub fn admit_workflow_schemas(
    schemas: &[WorkflowSchemaPolicy],
    allowed_stages: &[String],
) -> Result<(), WorkflowAdmissionError> {
    if schemas.is_empty() || schemas.len() > MAX_WORKFLOW_SCHEMAS_LIMIT {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_BOUNDS,
            "schemas",
            "workflow schema count is outside the supported bounds",
        ));
    }
    let mut ids = BTreeSet::new();
    for (schema_index, schema) in schemas.iter().enumerate() {
        let schema_path = format!("schemas[{schema_index}]");
        if !ids.insert(schema.id.clone()) {
            return Err(admission_error(
                WORKFLOW_ISSUE_POLICY_DUPLICATE_SCHEMA,
                format!("{schema_path}.id"),
                format!("duplicate workflow schema id: {}", schema.id),
            ));
        }
        admit_schema(schema, allowed_stages, &schema_path)?;
    }
    Ok(())
}

pub fn workflow_topological_order(
    schema: &WorkflowSchemaPolicy,
) -> Result<Vec<String>, WorkflowAdmissionError> {
    let mut indegree = schema
        .artifacts
        .iter()
        .map(|artifact| (artifact.id.clone(), artifact.requires.len()))
        .collect::<BTreeMap<_, _>>();
    let mut emitted = BTreeSet::new();
    let mut order = Vec::with_capacity(schema.artifacts.len());

    loop {
        let next = schema
            .artifacts
            .iter()
            .find(|artifact| {
                !emitted.contains(&artifact.id) && indegree.get(&artifact.id).copied() == Some(0)
            })
            .map(|artifact| artifact.id.clone());
        let Some(next) = next else {
            break;
        };
        assert!(emitted.insert(next.clone()));
        order.push(next.clone());
        for artifact in &schema.artifacts {
            if artifact.requires.iter().any(|required| required == &next) {
                let degree = indegree.get_mut(&artifact.id).ok_or_else(|| {
                    admission_error(
                        WORKFLOW_ISSUE_POLICY_UNKNOWN_DEPENDENCY,
                        format!("schemas.{}.artifacts.{}.requires", schema.id, artifact.id),
                        "topological admission lost an artifact",
                    )
                })?;
                *degree = degree.checked_sub(1).ok_or_else(|| {
                    admission_error(
                        WORKFLOW_ISSUE_POLICY_CYCLE,
                        format!("schemas.{}.artifacts", schema.id),
                        "artifact dependency indegree underflow",
                    )
                })?;
            }
        }
    }

    if order.len() != schema.artifacts.len() {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_CYCLE,
            format!("schemas.{}.artifacts", schema.id),
            "workflow artifact dependencies contain a cycle",
        ));
    }
    Ok(order)
}

fn admit_schema(
    schema: &WorkflowSchemaPolicy,
    allowed_stages: &[String],
    path: &str,
) -> Result<(), WorkflowAdmissionError> {
    validate_schema_header(schema, path)?;
    validate_bounds(&schema.bounds, path)?;
    validate_artifacts(schema, allowed_stages, path)?;
    workflow_topological_order(schema)?;
    Ok(())
}

fn validate_schema_header(
    schema: &WorkflowSchemaPolicy,
    path: &str,
) -> Result<(), WorkflowAdmissionError> {
    if schema.schema_version != WORKFLOW_SCHEMA_VERSION_V1 {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_SCHEMA_VERSION,
            format!("{path}.schema_version"),
            format!(
                "unsupported workflow schema version: {}",
                schema.schema_version
            ),
        ));
    }
    if schema.id.is_empty() || schema.description.is_empty() {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_SHAPE,
            path,
            "workflow schema id and description must be non-empty",
        ));
    }
    let expected_states = WORKFLOW_SUPPORTED_STATES.map(ToOwned::to_owned).to_vec();
    if schema.states != expected_states {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_STATES,
            format!("{path}.states"),
            "workflow states must use the supported deterministic order",
        ));
    }
    if schema.required_non_claims.is_empty()
        || schema.required_non_claims.len() > schema.bounds.max_non_claims
    {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_BOUNDS,
            format!("{path}.required_non_claims"),
            "workflow non-claim count is outside the schema bounds",
        ));
    }
    Ok(())
}

fn validate_bounds(bounds: &WorkflowBounds, path: &str) -> Result<(), WorkflowAdmissionError> {
    let checks = [
        (bounds.max_artifacts, MAX_WORKFLOW_ARTIFACTS_LIMIT),
        (bounds.max_dependencies, MAX_WORKFLOW_DEPENDENCIES_LIMIT),
        (bounds.max_files_per_artifact, MAX_WORKFLOW_FILES_LIMIT),
        (bounds.max_path_bytes, MAX_WORKFLOW_PATH_BYTES_LIMIT),
        (bounds.max_artifact_bytes, MAX_WORKFLOW_ARTIFACT_BYTES_LIMIT),
        (bounds.max_template_bytes, MAX_WORKFLOW_TEMPLATE_BYTES_LIMIT),
        (bounds.max_constraints, MAX_WORKFLOW_CONSTRAINTS_LIMIT),
        (bounds.max_non_claims, MAX_WORKFLOW_NON_CLAIMS_LIMIT),
        (bounds.max_issues, MAX_WORKFLOW_ISSUES_LIMIT),
        (
            bounds.max_directory_depth,
            MAX_WORKFLOW_DIRECTORY_DEPTH_LIMIT,
        ),
        (
            bounds.max_directory_entries,
            MAX_WORKFLOW_DIRECTORY_ENTRIES_LIMIT,
        ),
    ];
    if checks
        .into_iter()
        .any(|(selected, limit)| selected == 0 || selected > limit)
    {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_BOUNDS,
            format!("{path}.bounds"),
            "workflow bounds must be positive and within implementation limits",
        ));
    }
    Ok(())
}

fn validate_artifacts(
    schema: &WorkflowSchemaPolicy,
    allowed_stages: &[String],
    path: &str,
) -> Result<(), WorkflowAdmissionError> {
    if schema.artifacts.is_empty() || schema.artifacts.len() > schema.bounds.max_artifacts {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_BOUNDS,
            format!("{path}.artifacts"),
            "workflow artifact count is outside the schema bounds",
        ));
    }
    let known = collect_artifact_ids(schema, path)?;
    for (artifact_index, artifact) in schema.artifacts.iter().enumerate() {
        let artifact_path = format!("{path}.artifacts[{artifact_index}]");
        validate_artifact_shape(schema, artifact, allowed_stages, &artifact_path)?;
        validate_artifact_dependencies(schema, artifact, &known, &artifact_path)?;
        validate_prerequisites(artifact, &known, allowed_stages, &artifact_path)?;
    }
    Ok(())
}

fn collect_artifact_ids(
    schema: &WorkflowSchemaPolicy,
    path: &str,
) -> Result<BTreeSet<String>, WorkflowAdmissionError> {
    let mut ids = BTreeSet::new();
    for artifact in &schema.artifacts {
        if artifact.id.is_empty() || !ids.insert(artifact.id.clone()) {
            return Err(admission_error(
                WORKFLOW_ISSUE_POLICY_DUPLICATE_ARTIFACT,
                format!("{path}.artifacts"),
                format!("duplicate or empty artifact id: {}", artifact.id),
            ));
        }
    }
    Ok(ids)
}

fn validate_artifact_shape(
    schema: &WorkflowSchemaPolicy,
    artifact: &WorkflowArtifactPolicy,
    allowed_stages: &[String],
    path: &str,
) -> Result<(), WorkflowAdmissionError> {
    validate_safe_path(
        &artifact.output_path,
        schema.bounds.max_path_bytes,
        path,
        false,
    )?;
    validate_safe_path(
        &artifact.generates,
        schema.bounds.max_path_bytes,
        path,
        true,
    )?;
    if !matches!(
        artifact.path_kind.as_str(),
        WORKFLOW_PATH_KIND_FILE | WORKFLOW_PATH_KIND_SPEC_DIRECTORY
    ) {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_PATH_KIND,
            format!("{path}.path_kind"),
            format!("unsupported path kind: {}", artifact.path_kind),
        ));
    }
    if !matches!(
        artifact.structural_admission.as_str(),
        WORKFLOW_ADMISSION_PROPOSAL
            | WORKFLOW_ADMISSION_DELTA_SPECS
            | WORKFLOW_ADMISSION_DESIGN
            | WORKFLOW_ADMISSION_TASKS
            | WORKFLOW_ADMISSION_MARKDOWN
    ) {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_ADMISSION,
            format!("{path}.structural_admission"),
            format!(
                "unsupported structural admission: {}",
                artifact.structural_admission
            ),
        ));
    }
    if !allowed_stages.iter().any(|stage| stage == &artifact.stage) {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_STAGE,
            format!("{path}.stage"),
            format!("unknown lifecycle stage: {}", artifact.stage),
        ));
    }
    if artifact.constraints.len() > schema.bounds.max_constraints {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_BOUNDS,
            format!("{path}.constraints"),
            "artifact constraint count exceeds the schema bound",
        ));
    }
    validate_template(
        &artifact.instruction_template,
        schema.bounds.max_template_bytes,
        path,
    )
}

fn validate_artifact_dependencies(
    schema: &WorkflowSchemaPolicy,
    artifact: &WorkflowArtifactPolicy,
    known: &BTreeSet<String>,
    path: &str,
) -> Result<(), WorkflowAdmissionError> {
    if artifact.requires.len() > schema.bounds.max_dependencies {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_BOUNDS,
            format!("{path}.requires"),
            "artifact dependency count exceeds the schema bound",
        ));
    }
    let mut unique = BTreeSet::new();
    for required in &artifact.requires {
        if required == &artifact.id {
            return Err(admission_error(
                WORKFLOW_ISSUE_POLICY_SELF_DEPENDENCY,
                format!("{path}.requires"),
                format!("artifact {} depends on itself", artifact.id),
            ));
        }
        if !unique.insert(required.clone()) {
            return Err(admission_error(
                WORKFLOW_ISSUE_POLICY_DUPLICATE_DEPENDENCY,
                format!("{path}.requires"),
                format!("duplicate artifact dependency: {required}"),
            ));
        }
        if !known.contains(required) {
            return Err(admission_error(
                WORKFLOW_ISSUE_POLICY_UNKNOWN_DEPENDENCY,
                format!("{path}.requires"),
                format!("unknown artifact dependency: {required}"),
            ));
        }
    }
    Ok(())
}

fn validate_prerequisites(
    artifact: &WorkflowArtifactPolicy,
    known: &BTreeSet<String>,
    allowed_stages: &[String],
    path: &str,
) -> Result<(), WorkflowAdmissionError> {
    for prerequisite in &artifact.apply_prerequisites {
        let is_known_gate = prerequisite
            .strip_prefix(PREREQUISITE_GATE_PREFIX)
            .is_some_and(|stage| allowed_stages.iter().any(|known| known == stage));
        let is_known_artifact = prerequisite
            .strip_prefix(PREREQUISITE_ARTIFACT_PREFIX)
            .is_some_and(|id| known.contains(id));
        if !is_known_gate && !is_known_artifact {
            return Err(admission_error(
                WORKFLOW_ISSUE_POLICY_PREREQUISITE,
                format!("{path}.apply_prerequisites"),
                format!("unsupported apply prerequisite: {prerequisite}"),
            ));
        }
    }
    Ok(())
}

fn validate_safe_path(
    value: &str,
    max_bytes: usize,
    path: &str,
    allow_glob: bool,
) -> Result<(), WorkflowAdmissionError> {
    let has_forbidden_component = Path::new(value).components().any(|component| {
        matches!(
            component,
            Component::ParentDir | Component::RootDir | Component::Prefix(_)
        )
    });
    let has_forbidden_text = value.is_empty()
        || value.len() > max_bytes
        || value.contains('\\')
        || value.contains('\0')
        || (!allow_glob && (value.contains('*') || value.contains('?')));
    if has_forbidden_component || has_forbidden_text {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_UNSAFE_PATH,
            format!("{path}.output_path"),
            format!("unsafe workflow artifact path: {value}"),
        ));
    }
    Ok(())
}

fn validate_template(
    template: &str,
    max_bytes: usize,
    path: &str,
) -> Result<(), WorkflowAdmissionError> {
    if template.is_empty() || template.len() > max_bytes || template.contains('\0') {
        return Err(admission_error(
            WORKFLOW_ISSUE_POLICY_TEMPLATE,
            format!("{path}.instruction_template"),
            "instruction template is empty or exceeds its bound",
        ));
    }
    let mut remaining = template;
    loop {
        let Some(open_index) = remaining.find(TEMPLATE_OPEN) else {
            if remaining.contains(TEMPLATE_CLOSE) {
                return Err(template_error(
                    path,
                    "instruction template has an unmatched close",
                ));
            }
            return Ok(());
        };
        if remaining[..open_index].contains(TEMPLATE_CLOSE) {
            return Err(template_error(
                path,
                "instruction template has an unmatched close",
            ));
        }
        let token_start = open_index + TEMPLATE_OPEN.len();
        let Some(close_offset) = remaining[token_start..].find(TEMPLATE_CLOSE) else {
            return Err(template_error(
                path,
                "instruction template has an unmatched open",
            ));
        };
        let token_end = token_start + close_offset;
        let token = &remaining[token_start..token_end];
        if !matches!(token, TEMPLATE_TOKEN_CHANGE | TEMPLATE_TOKEN_ARTIFACT) {
            return Err(template_error(
                path,
                format!("instruction template uses an unknown token: {token}"),
            ));
        }
        remaining = &remaining[token_end + TEMPLATE_CLOSE.len()..];
    }
}

fn template_error(path: &str, message: impl Into<String>) -> WorkflowAdmissionError {
    admission_error(
        WORKFLOW_ISSUE_POLICY_TEMPLATE,
        format!("{path}.instruction_template"),
        message,
    )
}

fn admission_error(
    code: &'static str,
    path: impl Into<String>,
    message: impl Into<String>,
) -> WorkflowAdmissionError {
    WorkflowAdmissionError::new(code, path, message)
}
