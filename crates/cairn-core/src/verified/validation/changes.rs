use crate::verified::{policy::Policy, repo::ChangeStatus};

pub fn validate_changes(changes: &[ChangeStatus], policy: &Policy) -> Vec<String> {
    let mut issues = Vec::with_capacity(changes.len().saturating_mul(3));
    for change in changes {
        issues.extend(change_issues(change, policy));
    }
    issues
}

pub fn change_issues(change: &ChangeStatus, policy: &Policy) -> Vec<String> {
    let mut issues = Vec::with_capacity(3);
    if !change.has_proposal {
        issues.push(format!("{}: missing proposal.md", change.name));
    }
    if !change.has_tasks {
        issues.push(format!("{}: missing tasks.md", change.name));
    }
    if let Some(message) = unmarked_tasks_issue(change, policy) {
        issues.push(message);
    }
    issues
}

fn unmarked_tasks_issue(change: &ChangeStatus, policy: &Policy) -> Option<String> {
    let counts = change.task_counts?;
    if !policy.task_markers_enabled {
        return None;
    }
    if policy.task_marker_strictness != "strict" {
        return None;
    }
    if counts.unmarked == 0 {
        return None;
    }
    Some(format!(
        "{}: {} task(s) missing concurrency markers",
        change.name, counts.unmarked
    ))
}
