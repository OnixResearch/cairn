use std::collections::{BTreeMap, BTreeSet};

use crate::verified::repo::ChangeStatus;

pub const DEPENDENCY_ISSUE_DUPLICATE: &str = "change_dependency.duplicate_dependency";
pub const DEPENDENCY_ISSUE_UNKNOWN: &str = "change_dependency.unknown_dependency";
pub const DEPENDENCY_ISSUE_SELF: &str = "change_dependency.self_dependency";
pub const DEPENDENCY_ISSUE_CYCLE: &str = "change_dependency.cycle";
pub const DEPENDENCY_ISSUE_INVALID_TYPE: &str =
    "change_dependency.metadata.depends_on.invalid_type";
pub const DEPENDENCY_ISSUE_INVALID_ITEM: &str =
    "change_dependency.metadata.depends_on.invalid_item";
pub const DEPENDENCY_ISSUE_EMPTY_ID: &str = "change_dependency.metadata.depends_on.empty_id";
pub const DEPENDENCY_ISSUE_INVALID_ID: &str = "change_dependency.metadata.depends_on.invalid_id";
pub const DEPENDENCY_BLOCKER_UNSATISFIED: &str = "change_dependency.unsatisfied_dependency";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeDependencyGraph {
    pub valid: bool,
    pub nodes: Vec<ChangeDependencyNode>,
    pub edges: Vec<ChangeDependencyEdge>,
    pub issues: Vec<ChangeDependencyIssue>,
    pub drain_order: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeDependencyNode {
    pub name: String,
    pub dependencies: Vec<String>,
    pub satisfied_dependencies: Vec<String>,
    pub blocked_by: Vec<String>,
    pub blocks: Vec<String>,
    pub dependency_ready: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeDependencyEdge {
    pub dependent: String,
    pub dependency: String,
    pub satisfied: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeDependencyIssue {
    pub change: String,
    pub code: String,
    pub dependency: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct TopologicalOrder {
    order: Vec<String>,
    cyclic_nodes: Vec<String>,
}

struct GraphBuildContext {
    known_set: BTreeSet<String>,
    active_set: BTreeSet<String>,
    archived_set: BTreeSet<String>,
    nodes: BTreeMap<String, ChangeDependencyNode>,
    edges: Vec<ChangeDependencyEdge>,
    topological_edges: Vec<(String, String)>,
    issues: Vec<ChangeDependencyIssue>,
}

pub fn build_change_dependency_graph(
    changes: &[ChangeStatus],
    archived_changes: &[String],
) -> ChangeDependencyGraph {
    // r[impl cairn.change_dependency_graph.graph_validation]
    // r[impl cairn.change_dependency_graph.trellis_primitives]
    // r[impl cairn.change_dependency_graph.trellis_primitives.portable]
    let active_names = sorted_active_names(changes);
    let active_set = active_names.iter().cloned().collect::<BTreeSet<_>>();
    let archived_set = sorted_archived_names(archived_changes)
        .into_iter()
        .collect::<BTreeSet<_>>();
    let known_set = known_change_names(&active_set, &archived_set);
    let mut context = GraphBuildContext {
        known_set,
        active_set,
        archived_set,
        nodes: initial_nodes(&active_names),
        edges: Vec::new(),
        topological_edges: Vec::new(),
        issues: Vec::new(),
    };

    for change in sorted_changes(changes) {
        let declared_dependencies = sorted_dependencies(&change.metadata.depends_on);
        let unique_dependencies =
            unique_dependencies(&declared_dependencies, &change.name, &mut context.issues);
        for dependency in unique_dependencies {
            record_dependency_edge(&change.name, &dependency, &mut context);
        }
    }

    sort_node_fields(&mut context.nodes);
    context.edges.sort_by(|left, right| {
        left.dependent
            .cmp(&right.dependent)
            .then(left.dependency.cmp(&right.dependency))
    });

    let topological = topological_drain_order(&active_names, &context.topological_edges);
    for cyclic_node in &topological.cyclic_nodes {
        context.issues.push(ChangeDependencyIssue {
            change: cyclic_node.clone(),
            code: DEPENDENCY_ISSUE_CYCLE.to_string(),
            dependency: None,
            message: format!("change {cyclic_node} is in a dependency cycle"),
        });
    }
    context.issues.sort_by(|left, right| {
        left.change
            .cmp(&right.change)
            .then(left.code.cmp(&right.code))
            .then(left.dependency.cmp(&right.dependency))
    });

    mark_dependency_readiness(&mut context.nodes, &context.issues);
    let nodes = context.nodes.into_values().collect::<Vec<_>>();

    ChangeDependencyGraph {
        valid: context.issues.is_empty(),
        nodes,
        edges: context.edges,
        issues: context.issues,
        drain_order: topological.order,
    }
}

pub fn change_dependency_issue_messages(graph: &ChangeDependencyGraph) -> Vec<String> {
    graph
        .issues
        .iter()
        .map(change_dependency_issue_message)
        .collect()
}

pub fn dependency_blockers_for_change(
    graph: &ChangeDependencyGraph,
    change_name: &str,
) -> Vec<String> {
    let mut blockers = Vec::new();
    if let Some(node) = graph.nodes.iter().find(|node| node.name == change_name) {
        for dependency in &node.blocked_by {
            blockers.push(format!(
                "{change_name}: {DEPENDENCY_BLOCKER_UNSATISFIED}: dependency {dependency} is not archived"
            ));
        }
    }
    blockers.extend(
        graph
            .issues
            .iter()
            .filter(|issue| issue.change == change_name)
            .map(change_dependency_issue_message),
    );
    blockers.sort();
    blockers.dedup();
    blockers
}

fn change_dependency_issue_message(issue: &ChangeDependencyIssue) -> String {
    format!("{}: {}: {}", issue.change, issue.code, issue.message)
}

fn sorted_active_names(changes: &[ChangeStatus]) -> Vec<String> {
    let mut names = changes
        .iter()
        .map(|change| change.name.clone())
        .collect::<Vec<_>>();
    names.sort();
    names.dedup();
    names
}

fn sorted_archived_names(archived_changes: &[String]) -> Vec<String> {
    let mut names = archived_changes.to_vec();
    names.sort();
    names.dedup();
    names
}

fn known_change_names(
    active_set: &BTreeSet<String>,
    archived_set: &BTreeSet<String>,
) -> BTreeSet<String> {
    let mut known = active_set.clone();
    known.extend(archived_set.iter().cloned());
    known
}

fn initial_nodes(active_names: &[String]) -> BTreeMap<String, ChangeDependencyNode> {
    active_names
        .iter()
        .map(|name| {
            (
                name.clone(),
                ChangeDependencyNode {
                    name: name.clone(),
                    dependencies: Vec::new(),
                    satisfied_dependencies: Vec::new(),
                    blocked_by: Vec::new(),
                    blocks: Vec::new(),
                    dependency_ready: true,
                },
            )
        })
        .collect()
}

fn sorted_changes(changes: &[ChangeStatus]) -> Vec<&ChangeStatus> {
    let mut sorted = changes.iter().collect::<Vec<_>>();
    sorted.sort_by(|left, right| left.name.cmp(&right.name));
    sorted
}

fn sorted_dependencies(dependencies: &[String]) -> Vec<String> {
    let mut sorted = dependencies.to_vec();
    sorted.sort();
    sorted
}

fn unique_dependencies(
    sorted_dependencies: &[String],
    change_name: &str,
    issues: &mut Vec<ChangeDependencyIssue>,
) -> Vec<String> {
    let mut unique = Vec::new();
    let mut seen = BTreeSet::new();
    let mut duplicates = BTreeSet::new();
    for dependency in sorted_dependencies {
        if !seen.insert(dependency.clone()) {
            duplicates.insert(dependency.clone());
            continue;
        }
        unique.push(dependency.clone());
    }
    for dependency in duplicates {
        issues.push(ChangeDependencyIssue {
            change: change_name.to_string(),
            code: DEPENDENCY_ISSUE_DUPLICATE.to_string(),
            dependency: Some(dependency.clone()),
            message: format!("dependency {dependency} is declared more than once"),
        });
    }
    unique
}

fn record_dependency_edge(change_name: &str, dependency: &str, context: &mut GraphBuildContext) {
    if dependency == change_name {
        context.issues.push(ChangeDependencyIssue {
            change: change_name.to_string(),
            code: DEPENDENCY_ISSUE_SELF.to_string(),
            dependency: Some(dependency.to_string()),
            message: format!("change {change_name} depends on itself"),
        });
        return;
    }
    if !context.known_set.contains(dependency) {
        context.issues.push(ChangeDependencyIssue {
            change: change_name.to_string(),
            code: DEPENDENCY_ISSUE_UNKNOWN.to_string(),
            dependency: Some(dependency.to_string()),
            message: format!("dependency {dependency} is not active or archived"),
        });
        return;
    }

    let is_archived = context.archived_set.contains(dependency);
    context.edges.push(ChangeDependencyEdge {
        dependent: change_name.to_string(),
        dependency: dependency.to_string(),
        satisfied: is_archived,
    });
    if let Some(node) = context.nodes.get_mut(change_name) {
        node.dependencies.push(dependency.to_string());
        if is_archived {
            node.satisfied_dependencies.push(dependency.to_string());
        }
    }
    if context.active_set.contains(dependency) {
        if let Some(node) = context.nodes.get_mut(change_name) {
            node.blocked_by.push(dependency.to_string());
        }
        if let Some(dependency_node) = context.nodes.get_mut(dependency) {
            dependency_node.blocks.push(change_name.to_string());
        }
        context
            .topological_edges
            .push((dependency.to_string(), change_name.to_string()));
    }
}

fn sort_node_fields(nodes: &mut BTreeMap<String, ChangeDependencyNode>) {
    for node in nodes.values_mut() {
        node.dependencies.sort();
        node.dependencies.dedup();
        node.satisfied_dependencies.sort();
        node.satisfied_dependencies.dedup();
        node.blocked_by.sort();
        node.blocked_by.dedup();
        node.blocks.sort();
        node.blocks.dedup();
    }
}

fn mark_dependency_readiness(
    nodes: &mut BTreeMap<String, ChangeDependencyNode>,
    issues: &[ChangeDependencyIssue],
) {
    let issue_changes = issues
        .iter()
        .map(|issue| issue.change.clone())
        .collect::<BTreeSet<_>>();
    for node in nodes.values_mut() {
        node.dependency_ready = node.blocked_by.is_empty() && !issue_changes.contains(&node.name);
    }
}

fn topological_drain_order(
    active_names: &[String],
    dependency_edges: &[(String, String)],
) -> TopologicalOrder {
    // r[impl cairn.change_dependency_graph.trellis_primitives]
    let mut indegrees = active_names
        .iter()
        .map(|name| (name.clone(), 0usize))
        .collect::<BTreeMap<_, _>>();
    let mut dependents = active_names
        .iter()
        .map(|name| (name.clone(), Vec::new()))
        .collect::<BTreeMap<_, _>>();

    for (dependency, dependent) in dependency_edges {
        if let Some(indegree) = indegrees.get_mut(dependent) {
            *indegree += 1;
        }
        if let Some(dependent_list) = dependents.get_mut(dependency) {
            dependent_list.push(dependent.clone());
        }
    }
    for dependent_list in dependents.values_mut() {
        dependent_list.sort();
        dependent_list.dedup();
    }

    let mut ready = indegrees
        .iter()
        .filter(|(_name, indegree)| **indegree == 0)
        .map(|(name, _indegree)| name.clone())
        .collect::<BTreeSet<_>>();
    let mut order = Vec::with_capacity(active_names.len());

    while let Some(name) = ready.iter().next().cloned() {
        ready.remove(&name);
        order.push(name.clone());
        if let Some(dependent_list) = dependents.get(&name) {
            for dependent in dependent_list {
                if let Some(indegree) = indegrees.get_mut(dependent) {
                    if *indegree > 0 {
                        *indegree -= 1;
                    }
                    if *indegree == 0 {
                        ready.insert(dependent.clone());
                    }
                }
            }
        }
    }

    let ordered = order.iter().cloned().collect::<BTreeSet<_>>();
    let cyclic_nodes = active_names
        .iter()
        .filter(|name| !ordered.contains(*name))
        .cloned()
        .collect::<Vec<_>>();

    TopologicalOrder {
        order,
        cyclic_nodes,
    }
}

impl ChangeDependencyGraph {
    pub fn node(&self, name: &str) -> Option<&ChangeDependencyNode> {
        self.nodes.iter().find(|node| node.name == name)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::verified::repo::{ChangeMetadata, TaskCounts};

    fn change(name: &str, depends_on: &[&str]) -> ChangeStatus {
        ChangeStatus {
            name: name.to_string(),
            path: format!("cairn/changes/{name}"),
            has_proposal: true,
            has_design: true,
            has_tasks: true,
            delta_specs: vec![format!("cairn/changes/{name}/specs/demo/spec.md")],
            task_counts: Some(TaskCounts {
                total: TASK_TOTAL,
                done: TASK_TOTAL,
                in_progress: 0,
                todo: 0,
                unmarked: 0,
            }),
            metadata: ChangeMetadata {
                groups: Vec::new(),
                target_date: None,
                status: None,
                depends_on: depends_on
                    .iter()
                    .map(|dependency| (*dependency).to_string())
                    .collect(),
            },
            metadata_issues: Vec::new(),
        }
    }

    const TASK_TOTAL: usize = 2;

    #[test]
    fn empty_graph_is_ready() {
        // r[verify cairn.change_dependency_graph.positive_negative_tests]
        // r[verify cairn.change_dependency_graph.positive_negative_tests.positive]
        let graph = build_change_dependency_graph(&[change("alpha", &[])], &[]);
        assert!(graph.valid);
        assert_eq!(graph.drain_order, vec!["alpha".to_string()]);
        assert!(graph
            .node("alpha")
            .is_some_and(|node| node.dependency_ready));
        assert!(graph.issues.is_empty());
    }

    #[test]
    fn archived_dependencies_are_satisfied() {
        // r[verify cairn.change_dependency_graph.positive_negative_tests]
        // r[verify cairn.change_dependency_graph.positive_negative_tests.positive]
        // r[verify cairn.change_dependency_graph.drain_order.archived]
        let graph =
            build_change_dependency_graph(&[change("beta", &["alpha"])], &["alpha".to_string()]);
        assert!(graph.valid);
        let node = graph.node("beta");
        assert!(node.is_some_and(|node| node.blocked_by.is_empty()));
        assert!(graph.edges[0].satisfied);
        assert_eq!(graph.drain_order, vec!["beta".to_string()]);
    }

    #[test]
    fn active_dependencies_order_before_dependents() {
        // r[verify cairn.change_dependency_graph.positive_negative_tests]
        // r[verify cairn.change_dependency_graph.positive_negative_tests.positive]
        // r[verify cairn.change_dependency_graph.graph_validation.acyclic]
        let graph =
            build_change_dependency_graph(&[change("beta", &["alpha"]), change("alpha", &[])], &[]);
        assert!(graph.valid);
        assert_eq!(
            graph.drain_order,
            vec!["alpha".to_string(), "beta".to_string()]
        );
        let beta = graph.node("beta");
        assert!(beta.is_some_and(|node| node.blocked_by == vec!["alpha".to_string()]));
        let alpha = graph.node("alpha");
        assert!(alpha.is_some_and(|node| node.blocks == vec!["beta".to_string()]));
    }

    #[test]
    fn invalid_edges_report_issue_codes() {
        // r[verify cairn.change_dependency_graph.positive_negative_tests]
        // r[verify cairn.change_dependency_graph.positive_negative_tests.negative]
        let graph = build_change_dependency_graph(
            &[
                change("alpha", &["missing", "missing"]),
                change("beta", &["beta"]),
            ],
            &[],
        );
        assert!(!graph.valid);
        let codes = graph
            .issues
            .iter()
            .map(|issue| issue.code.as_str())
            .collect::<Vec<_>>();
        assert!(codes.contains(&DEPENDENCY_ISSUE_DUPLICATE));
        assert!(codes.contains(&DEPENDENCY_ISSUE_UNKNOWN));
        assert!(codes.contains(&DEPENDENCY_ISSUE_SELF));
    }

    #[test]
    fn cycles_are_rejected_without_ready_nodes() {
        // r[verify cairn.change_dependency_graph.positive_negative_tests]
        // r[verify cairn.change_dependency_graph.positive_negative_tests.negative]
        // r[verify cairn.change_dependency_graph.graph_validation.cycle]
        let graph = build_change_dependency_graph(
            &[change("alpha", &["beta"]), change("beta", &["alpha"])],
            &[],
        );
        assert!(!graph.valid);
        assert!(graph.drain_order.is_empty());
        let codes = graph
            .issues
            .iter()
            .map(|issue| issue.code.as_str())
            .collect::<Vec<_>>();
        assert!(codes.iter().all(|code| *code == DEPENDENCY_ISSUE_CYCLE));
        assert!(graph
            .node("alpha")
            .is_some_and(|node| !node.dependency_ready));
    }
}
