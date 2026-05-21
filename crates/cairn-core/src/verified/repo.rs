#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepoStatus {
    pub root: String,
    pub specs: Vec<String>,
    pub changes: Vec<ChangeStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecSource {
    pub id: String,
    pub path: String,
    pub markdown: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecSummary {
    pub id: String,
    pub path: String,
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecDetail {
    pub id: String,
    pub path: String,
    pub title: Option<String>,
    pub markdown: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeList {
    pub changes: Vec<ChangeStatus>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeDetail {
    pub change: ChangeStatus,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecList {
    pub specs: Vec<SpecSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSource {
    pub name: String,
    pub path: String,
    pub has_proposal: bool,
    pub has_design: bool,
    pub has_tasks: bool,
    pub delta_specs: Vec<String>,
    pub tasks_markdown: Option<String>,
    pub metadata_json: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ChangeMetadata {
    pub groups: Vec<String>,
    pub target_date: Option<String>,
    pub status: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeStatus {
    pub name: String,
    pub path: String,
    pub has_proposal: bool,
    pub has_design: bool,
    pub has_tasks: bool,
    pub delta_specs: Vec<String>,
    pub task_counts: Option<TaskCounts>,
    pub metadata: ChangeMetadata,
    pub metadata_issues: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaskCounts {
    pub total: usize,
    pub done: usize,
    pub in_progress: usize,
    pub todo: usize,
    pub unmarked: usize,
}

pub fn build_repo_status(
    root: String,
    mut specs: Vec<String>,
    changes: Vec<ChangeSource>,
) -> RepoStatus {
    specs.sort();
    let mut changes = changes
        .into_iter()
        .map(build_change_status)
        .collect::<Vec<_>>();
    changes.sort_by(|left, right| left.name.cmp(&right.name));
    RepoStatus {
        root,
        specs,
        changes,
    }
}

pub fn build_change_status(mut source: ChangeSource) -> ChangeStatus {
    // r[impl cairn.supersede.change_metadata.schema]
    // r[impl cairn.supersede.change_metadata.validation]
    source.delta_specs.sort();
    let task_counts = source.tasks_markdown.as_deref().map(count_tasks);
    let (metadata, metadata_issues) =
        parse_change_metadata(&source.name, source.metadata_json.as_deref());
    ChangeStatus {
        name: source.name,
        path: source.path,
        has_proposal: source.has_proposal,
        has_design: source.has_design,
        has_tasks: source.has_tasks,
        delta_specs: source.delta_specs,
        task_counts,
        metadata,
        metadata_issues,
    }
}

pub fn build_change_list(mut changes: Vec<ChangeStatus>) -> ChangeList {
    // r[impl cairn.supersede.read_parity] r[impl cairn.supersede.read_parity.change_list]
    // r[impl cairn.supersede.change_metadata.read_receipts] r[impl cairn.supersede.change_metadata.filters]
    // r[impl cairn.supersede.boundary] r[impl cairn.supersede.boundary.read_models]
    changes.sort_by(|left, right| left.name.cmp(&right.name));
    ChangeList { changes }
}

pub fn filter_changes_by_metadata(
    changes: Vec<ChangeStatus>,
    group: Option<&str>,
    target_after: Option<&str>,
    target_before: Option<&str>,
) -> Vec<ChangeStatus> {
    // r[impl cairn.supersede.change_metadata.filters]
    changes
        .into_iter()
        .filter(|change| matches_metadata_filter(change, group, target_after, target_before))
        .collect()
}

fn matches_metadata_filter(
    change: &ChangeStatus,
    group: Option<&str>,
    target_after: Option<&str>,
    target_before: Option<&str>,
) -> bool {
    if let Some(group) = group {
        if !change
            .metadata
            .groups
            .iter()
            .any(|candidate| candidate == group)
        {
            return false;
        }
    }
    if target_after.is_some() || target_before.is_some() {
        let Some(target_date) = change.metadata.target_date.as_deref() else {
            return false;
        };
        if target_after.is_some_and(|bound| target_date <= bound) {
            return false;
        }
        if target_before.is_some_and(|bound| target_date >= bound) {
            return false;
        }
    }
    true
}

fn parse_change_metadata(
    change_name: &str,
    metadata_json: Option<&str>,
) -> (ChangeMetadata, Vec<String>) {
    let Some(metadata_json) = metadata_json else {
        return (ChangeMetadata::default(), Vec::new());
    };
    let Ok(value) = serde_json::from_str::<serde_json::Value>(metadata_json) else {
        return (
            ChangeMetadata::default(),
            vec![format!("{change_name}: metadata.json is not valid JSON")],
        );
    };
    let Some(object) = value.as_object() else {
        return (
            ChangeMetadata::default(),
            vec![format!(
                "{change_name}: metadata.json must contain an object"
            )],
        );
    };

    let mut issues = Vec::new();
    let groups = match object.get("groups") {
        None => Vec::new(),
        Some(value) => string_array(value).unwrap_or_else(|| {
            issues.push(format!(
                "{change_name}: metadata groups must be an array of strings"
            ));
            Vec::new()
        }),
    };
    let target_date = optional_string(object.get("target_date"), change_name, "target_date");
    issues.extend(target_date.1);
    let target_date = target_date.0;
    let status = optional_string(object.get("status"), change_name, "status");
    issues.extend(status.1);
    let status = status.0;
    let mut sorted_groups = groups;
    sorted_groups.sort();
    let has_duplicates = sorted_groups
        .windows(2)
        .any(|window| window[0] == window[1]);
    if has_duplicates {
        issues.push(format!("{change_name}: metadata groups must be unique"));
        sorted_groups.dedup();
    }
    (
        ChangeMetadata {
            groups: sorted_groups,
            target_date,
            status,
        },
        issues,
    )
}

fn string_array(value: &serde_json::Value) -> Option<Vec<String>> {
    value
        .as_array()?
        .iter()
        .map(|item| item.as_str().map(str::to_string))
        .collect()
}

fn optional_string(
    value: Option<&serde_json::Value>,
    change_name: &str,
    field: &str,
) -> (Option<String>, Option<String>) {
    match value {
        None => (None, None),
        Some(value) => match value.as_str() {
            Some(text) => (Some(text.to_string()), None),
            None => {
                let issue = format!("{change_name}: metadata {field} must be a string");
                (None, Some(issue))
            }
        },
    }
}

fn is_yyyy_mm_dd(value: &str) -> bool {
    let bytes = value.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| index == 4 || index == 7 || byte.is_ascii_digit())
}

pub fn validate_change_metadata_policy(
    change: &ChangeStatus,
    allowed_groups: &[String],
    group_prefixes: &[String],
    allowed_statuses: &[String],
) -> Vec<String> {
    // r[impl cairn.supersede.change_metadata.validation]
    let mut issues = change.metadata_issues.clone();
    for group in &change.metadata.groups {
        let is_allowed_group = allowed_groups.iter().any(|allowed| allowed == group)
            || group_prefixes
                .iter()
                .any(|prefix| group.starts_with(prefix));
        if !is_allowed_group {
            issues.push(format!(
                "{}: metadata group {group} is not allowed by policy",
                change.name
            ));
        }
    }
    if let Some(target_date) = &change.metadata.target_date {
        if !is_yyyy_mm_dd(target_date) {
            issues.push(format!(
                "{}: metadata target_date must be YYYY-MM-DD",
                change.name
            ));
        }
    }
    if let Some(status) = &change.metadata.status {
        if !allowed_statuses.iter().any(|allowed| allowed == status) {
            issues.push(format!(
                "{}: metadata status {status} is not allowed by policy",
                change.name
            ));
        }
    }
    issues
}

pub fn build_change_detail(change: ChangeStatus) -> ChangeDetail {
    // r[impl cairn.supersede.read_parity.change_show]
    ChangeDetail { change }
}

pub fn build_spec_list(sources: Vec<SpecSource>) -> SpecList {
    // r[impl cairn.supersede.read_parity.spec_list]
    let mut specs = sources
        .into_iter()
        .map(|source| SpecSummary {
            id: source.id,
            path: source.path,
            title: spec_title(&source.markdown),
        })
        .collect::<Vec<_>>();
    specs.sort_by(|left, right| left.id.cmp(&right.id));
    SpecList { specs }
}

pub fn build_spec_detail(source: SpecSource) -> SpecDetail {
    // r[impl cairn.supersede.read_parity.spec_show]
    SpecDetail {
        id: source.id,
        path: source.path,
        title: spec_title(&source.markdown),
        markdown: source.markdown,
    }
}

pub fn spec_title(markdown: &str) -> Option<String> {
    markdown.lines().find_map(markdown_heading)
}

fn markdown_heading(line: &str) -> Option<String> {
    let trimmed = line.trim();
    let title = trimmed.strip_prefix("# ")?.trim();
    (!title.is_empty()).then(|| title.to_string())
}

pub fn count_tasks(markdown: &str) -> TaskCounts {
    let mut counts = TaskCounts {
        total: 0,
        done: 0,
        in_progress: 0,
        todo: 0,
        unmarked: 0,
    };
    for line in markdown.lines() {
        let trimmed = line.trim_start();
        let Some(state) = task_state(trimmed) else {
            continue;
        };
        counts.total += 1;
        increment_state(&mut counts, state);
        if !has_concurrency_marker(trimmed) {
            counts.unmarked += 1;
        }
    }
    counts
}

fn task_state(trimmed: &str) -> Option<&'static str> {
    if trimmed.starts_with("- [x]") || trimmed.starts_with("- [X]") {
        return Some("done");
    }
    if trimmed.starts_with("- [~]") {
        return Some("in_progress");
    }
    if trimmed.starts_with("- [ ]") {
        return Some("todo");
    }
    None
}

fn increment_state(counts: &mut TaskCounts, state: &str) {
    match state {
        "done" => counts.done += 1,
        "in_progress" => counts.in_progress += 1,
        "todo" => counts.todo += 1,
        _ => debug_assert!(false, "unknown task state"),
    }
}

fn has_concurrency_marker(trimmed: &str) -> bool {
    trimmed.contains("[parallel]") || trimmed.contains("[serial]") || trimmed.contains("[depends:")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_task_states_and_markers() {
        let counts = count_tasks("- [ ] [serial] todo\n- [x] done\n- [~] [depends:a] doing\n");
        assert_eq!(counts.total, 3);
        assert_eq!(counts.todo, 1);
        assert_eq!(counts.done, 1);
        assert_eq!(counts.in_progress, 1);
        assert_eq!(counts.unmarked, 1);
    }

    #[test]
    fn builds_sorted_repo_status_from_shell_snapshot() {
        let status = build_repo_status(
            "fixture".to_string(),
            vec![
                "openspec/specs/z/spec.md".to_string(),
                "openspec/specs/a/spec.md".to_string(),
            ],
            vec![ChangeSource {
                name: "demo".to_string(),
                path: "openspec/changes/demo".to_string(),
                has_proposal: true,
                has_design: false,
                has_tasks: true,
                delta_specs: vec!["openspec/changes/demo/specs/a/spec.md".to_string()],
                tasks_markdown: Some("- [ ] [serial] todo\n- [x] done\n".to_string()),
                metadata_json: Some(r#"{"groups":["future","current"],"target_date":"2026-05-21","status":"planned"}"#.to_string()),
            }],
        );
        assert_eq!(status.specs[0], "openspec/specs/a/spec.md");
        assert_eq!(status.changes.len(), 1);
        assert_eq!(status.changes[0].name, "demo");
        assert_eq!(status.changes[0].task_counts.unwrap().total, 2);
        assert_eq!(status.changes[0].metadata.groups, vec!["current", "future"]);
    }

    #[test]
    fn filters_changes_by_metadata() {
        let matching = ChangeStatus {
            name: "matching".to_string(),
            path: "cairn/changes/matching".to_string(),
            has_proposal: true,
            has_design: true,
            has_tasks: true,
            delta_specs: Vec::new(),
            task_counts: None,
            metadata: ChangeMetadata {
                groups: vec!["current".to_string(), "feature:registry".to_string()],
                target_date: Some("2026-06-01".to_string()),
                status: Some("planned".to_string()),
            },
            metadata_issues: Vec::new(),
        };
        let without_date = ChangeStatus {
            name: "without-date".to_string(),
            metadata: ChangeMetadata {
                groups: vec!["current".to_string()],
                target_date: None,
                status: None,
            },
            ..matching.clone()
        };
        let filtered = filter_changes_by_metadata(
            vec![matching, without_date],
            Some("current"),
            Some("2026-05-01"),
            Some("2026-07-01"),
        );
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].name, "matching");
    }

    #[test]
    fn builds_sorted_spec_list_with_titles() {
        let specs = build_spec_list(vec![
            SpecSource {
                id: "zeta".to_string(),
                path: "openspec/specs/zeta/spec.md".to_string(),
                markdown: "# Zeta Spec\n\nBody".to_string(),
            },
            SpecSource {
                id: "alpha".to_string(),
                path: "openspec/specs/alpha/spec.md".to_string(),
                markdown: "\n# Alpha Spec\n".to_string(),
            },
        ]);
        assert_eq!(specs.specs[0].id, "alpha");
        assert_eq!(specs.specs[0].title.as_deref(), Some("Alpha Spec"));
        assert_eq!(specs.specs[1].id, "zeta");
    }

    #[test]
    fn builds_spec_detail_with_content() {
        let detail = build_spec_detail(SpecSource {
            id: "demo".to_string(),
            path: "openspec/specs/demo/spec.md".to_string(),
            markdown: "# Demo\n\nRequirement".to_string(),
        });
        assert_eq!(detail.id, "demo");
        assert_eq!(detail.title.as_deref(), Some("Demo"));
        assert!(detail.markdown.contains("Requirement"));
    }
}
