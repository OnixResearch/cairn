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
    source.delta_specs.sort();
    let task_counts = source.tasks_markdown.as_deref().map(count_tasks);
    ChangeStatus {
        name: source.name,
        path: source.path,
        has_proposal: source.has_proposal,
        has_design: source.has_design,
        has_tasks: source.has_tasks,
        delta_specs: source.delta_specs,
        task_counts,
    }
}

pub fn build_change_list(mut changes: Vec<ChangeStatus>) -> ChangeList {
    // r[impl cairn.supersede.read_parity] r[impl cairn.supersede.read_parity.change_list]
    // r[impl cairn.supersede.boundary] r[impl cairn.supersede.boundary.read_models]
    changes.sort_by(|left, right| left.name.cmp(&right.name));
    ChangeList { changes }
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
            }],
        );
        assert_eq!(status.specs[0], "openspec/specs/a/spec.md");
        assert_eq!(status.changes.len(), 1);
        assert_eq!(status.changes[0].name, "demo");
        assert_eq!(status.changes[0].task_counts.unwrap().total, 2);
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
