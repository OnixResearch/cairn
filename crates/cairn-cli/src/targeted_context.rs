//! Read-only narrow lifecycle source admission shared by CLI and MCP.
use crate::support::{print_json, SourceReader};
use cairn_core::verified::validation::{select_context, ContextReadBounds, ContextSelector};
use cairn_core::{
    parse_policy_json, select_workflow_profile, workflow_change_profile_metadata_from_value,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    error::Error,
    path::{Path, PathBuf},
};

const TARGETED_POLICY: &str = include_str!("../../../cairn-policy/generated/targeted-context.json");
const GLOBAL_POLICY: &str = include_str!("../../../cairn-policy/generated/cairn-policy.json");

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct TargetedPolicy {
    schema: String,
    bounds: ContextReadBounds,
    required_non_claims: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct ReadRequest {
    pub root: String,
    pub scope: String,
    pub artifact: String,
    #[serde(default)]
    pub change: Option<String>,
    #[serde(default)]
    pub spec: Option<String>,
    #[serde(default)]
    pub requirement_id: Option<String>,
    #[serde(default)]
    pub task_id: Option<String>,
    #[serde(default)]
    pub heading_path: Option<Vec<String>>,
    #[serde(default)]
    pub expected_source_blake3: Option<String>,
}

fn safe_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 512
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
        && id != "."
        && id != ".."
        && !id.starts_with('.')
}

fn selector(request: &ReadRequest) -> Result<ContextSelector, Box<dyn Error>> {
    match (
        &request.requirement_id,
        &request.task_id,
        &request.heading_path,
    ) {
        (Some(id), None, None) => Ok(ContextSelector::Requirement { id: id.clone() }),
        (None, Some(id), None) => Ok(ContextSelector::Task { id: id.clone() }),
        (None, None, Some(path)) => Ok(ContextSelector::Section { path: path.clone() }),
        _ => {
            Err("context read requires exactly one requirement_id, task_id, or heading_path".into())
        }
    }
}

fn relative_path(
    request: &ReadRequest,
    bounds: &ContextReadBounds,
) -> Result<PathBuf, Box<dyn Error>> {
    if request.artifact.len() > bounds.max_selector_bytes || !safe_id(&request.artifact) {
        return Err("invalid context artifact".into());
    }
    let base = match request.scope.as_str() {
        "accepted" if request.change.is_none() => PathBuf::from(".cairn/specs"),
        "active" | "archived" => {
            let change = request
                .change
                .as_deref()
                .ok_or("change scope requires change")?;
            if !safe_id(change) {
                return Err("unsafe change identity".into());
            }
            PathBuf::from(if request.scope == "active" {
                ".cairn/changes"
            } else {
                ".cairn/archive"
            })
            .join(change)
        }
        _ => return Err("unsupported context lifecycle scope or unexpected change".into()),
    };
    if request.scope == "accepted" || request.artifact == "specs" {
        if request.artifact != "specs" {
            return Err("accepted scope requires specs artifact".into());
        }
        let spec = request
            .spec
            .as_deref()
            .ok_or("specs artifact requires spec ID")?;
        if !safe_id(spec) {
            return Err("unsafe specification identity".into());
        }
        Ok(base.join(if request.scope == "accepted" {
            Path::new(spec).join("spec.md")
        } else {
            Path::new("specs").join(spec).join("spec.md")
        }))
    } else {
        if request.spec.is_some() {
            return Err("spec ID requires specs artifact".into());
        }
        // The selected workflow schema resolves other artifact IDs below, not user paths.
        Ok(base)
    }
}

pub(crate) fn read_value(request: ReadRequest) -> Result<Value, Box<dyn Error>> {
    let policy: TargetedPolicy = serde_json::from_str(TARGETED_POLICY)?;
    if policy.schema != "cairn.targeted-context-policy.v1" {
        return Err("unsupported targeted context policy".into());
    }
    let selected = selector(&request)?;
    if matches!(selected, ContextSelector::Requirement { .. }) && request.artifact != "specs" {
        return Err("requirement selector requires specs artifact".into());
    }
    if matches!(selected, ContextSelector::Task { .. }) && request.artifact != "tasks" {
        return Err("task selector requires tasks artifact".into());
    }
    if let Some(expected) = &request.expected_source_blake3 {
        if expected.len() != 64
            || !expected
                .bytes()
                .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
        {
            return Err("expected source BLAKE3 must be 64 lowercase hex characters".into());
        }
    }
    let mut path = relative_path(&request, &policy.bounds)?;
    let root = Path::new(&request.root);
    let mut reader = SourceReader::open(root)?;
    let global = parse_policy_json(GLOBAL_POLICY)?;
    let mut constraints = Vec::new();
    let mut schema_id = None;
    if request.artifact == "specs" {
        constraints.push(format!("specification requires at least {} substantive requirement block(s) with normative strength from [{}]", global.artifact_substance_policy.minimum_requirement_blocks, global.artifact_substance_policy.normative_tokens.join(", ")));
    } else if request.artifact == "tasks" {
        constraints.push(format!("tasks require at least {} substantive task(s); dependency marker {} remains policy-owned", global.artifact_substance_policy.minimum_substantive_tasks, global.artifact_substance_policy.dependency_marker_id));
    }
    if request.scope != "accepted" {
        let base = PathBuf::from(if request.scope == "active" {
            ".cairn/changes"
        } else {
            ".cairn/archive"
        })
        .join(
            request
                .change
                .as_deref()
                .ok_or("change scope requires change")?,
        );
        let metadata = match reader.read_to_string(&base.join("metadata.json")) {
            Ok(text) => serde_json::from_str::<Value>(&text)?,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => json!({}),
            Err(error) => return Err(error.into()),
        };
        let metadata = workflow_change_profile_metadata_from_value(&metadata)?;
        let (_profile, schema) = select_workflow_profile(
            &global.workflow_profile_policy,
            &global.workflow_schemas,
            &metadata,
        )?;
        schema_id = Some(schema.id.clone());
        let artifact = schema
            .artifacts
            .iter()
            .find(|entry| entry.id == request.artifact)
            .ok_or("artifact is not declared in selected workflow profile")?;
        constraints.extend(artifact.constraints.iter().cloned());
        if request.artifact != "specs" {
            if artifact.path_kind != "file" {
                return Err("section requires one declared file artifact".into());
            }
            let relative = Path::new(&artifact.output_path);
            if relative
                .components()
                .any(|component| !matches!(component, std::path::Component::Normal(_)))
            {
                return Err("unsafe policy artifact path".into());
            }
            path = base.join(relative);
        }
    }
    if request.scope == "accepted" && request.artifact != "specs" {
        return Err("accepted scope requires specs".into());
    }
    let markdown = reader.read_to_string(&path)?;
    if markdown.len() > policy.bounds.max_artifact_bytes {
        return Err("context artifact byte bound exceeded".into());
    }
    let source_blake3 = blake3::hash(markdown.as_bytes()).to_hex().to_string();
    if request
        .expected_source_blake3
        .as_deref()
        .is_some_and(|expected| expected != source_blake3)
    {
        return Err("stale expected source BLAKE3; no content disclosed".into());
    }
    let artifact_path = path.to_string_lossy().to_string();
    let mut result = select_context(
        &markdown,
        &selected,
        &artifact_path,
        &global,
        &policy.bounds,
    )?;
    for reference in &mut result.references {
        reference.route = Some(if reference.kind == "task" {
            json!({"tool": "cairn_context_read", "arguments": {
                "root": request.root, "scope": request.scope, "change": request.change,
                "artifact": "tasks", "task_id": reference.id
            }})
            .to_string()
        } else if request.scope == "accepted" {
            json!({"tool": "cairn_list_content", "arguments": {"root": request.root}}).to_string()
        } else {
            json!({"tool": "cairn_change_show", "arguments": {"root": request.root, "change": request.change}}).to_string()
        });
    }
    result.diagnostics_route =
        Some(json!({"tool": "cairn_validate", "arguments": {"root": request.root}}).to_string());
    result.references_route = Some(if request.scope == "accepted" {
        json!({"tool": "cairn_read_spec", "arguments": {"root": request.root, "spec": request.spec}}).to_string()
    } else {
        json!({"read_source": {"root": request.root, "path": artifact_path, "source_blake3": source_blake3}}).to_string()
    });
    // Required constraints are never silently dropped from narrow context: they
    // are policy text, distinct from the selected source and advisory guidance.
    constraints.sort();
    constraints.dedup();
    let selection_blake3 = cairn_core::canonical_json_hash(&json!({
        "schema": "cairn.targeted-context-selection.v1",
        "scope": request.scope, "change": request.change, "spec": request.spec,
        "artifact": request.artifact, "path": artifact_path, "selector": selected,
        "source_blake3": source_blake3, "span": result.span,
        "status": result.status, "completeness": result.completeness,
    }));
    let mut guidance_args = json!({"root": request.root, "artifact": request.artifact});
    if let Some(profile) = &schema_id {
        guidance_args["profile"] = json!(profile);
    }
    let output = json!({
        "schema": "cairn.targeted-context-read.v1", "command": "context read",
        "root": request.root, "scope": request.scope, "change": request.change,
        "artifact": request.artifact, "spec": request.spec, "path": artifact_path,
        "selector": selected, "source_blake3": source_blake3, "selection_blake3": selection_blake3,
        "selection": result, "policy_constraints": constraints,
        "authority": {
            "selected_text": "lifecycle_source", "policy_constraints": "generated_policy",
            "references": "declared_links", "project_fact": "not_included",
            "advisory_guidance": "not_included"
        },
        "guidance_route": json!({"tool": "cairn_context", "arguments": guidance_args}),
        "workflow_schema": schema_id,
        "global_policy_blake3": cairn_core::canonical_json_hash(&global.raw),
        "targeted_policy_blake3": blake3::hash(TARGETED_POLICY.as_bytes()).to_hex().to_string(),
        "non_claims": policy.required_non_claims,
        "read_only": true, "mutated": false,
    });
    Ok(output)
}

pub(crate) fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.first().map(String::as_str) != Some("read") {
        return Err("usage: context read --root ROOT --scope accepted|active|archived --artifact ID --requirement-id ID|--task-id ID|--heading HEADING...".into());
    }
    let mut value = serde_json::Map::new();
    let mut headings = Vec::new();
    let mut iter = args.into_iter().skip(1);
    while let Some(flag) = iter.next() {
        let key = match flag.as_str() {
            "--root" => "root",
            "--scope" => "scope",
            "--change" => "change",
            "--artifact" => "artifact",
            "--spec" => "spec",
            "--requirement-id" => "requirement_id",
            "--task-id" => "task_id",
            "--heading" => "heading_path",
            "--expected-source-blake3" => "expected_source_blake3",
            _ => return Err(format!("unknown context read argument: {flag}").into()),
        };
        let text = iter
            .next()
            .ok_or_else(|| format!("{flag} requires a value"))?;
        if key == "heading_path" {
            headings.push(text);
        } else if value.insert(key.into(), Value::String(text)).is_some() {
            return Err(format!("duplicate context read argument {flag}").into());
        }
    }
    if !headings.is_empty() {
        value.insert("heading_path".into(), json!(headings));
    }
    if !value.contains_key("root")
        || !value.contains_key("scope")
        || !value.contains_key("artifact")
    {
        return Err("context read requires explicit root, scope, and artifact".into());
    }
    print_json(read_value(serde_json::from_value(Value::Object(value))?)?)?;
    Ok(())
}
