use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use super::evidence::canonical_json_hash;

pub const ADOPTION_SNAPSHOT_SCHEMA_V1: &str = "cairn.adoption.snapshot.v1";
pub const ADOPTION_REPORT_SCHEMA_V1: &str = "cairn.adoption.report.v1";
pub const ADOPTION_STAGE_UPSTREAM_LIFECYCLE: &str = "upstream_lifecycle";
pub const ADOPTION_STAGE_CONSUMER_PIN: &str = "consumer_pin";
pub const ADOPTION_STAGE_CONSUMER_COMPATIBILITY: &str = "consumer_compatibility";
pub const ADOPTION_STAGE_INTEGRATION_EVIDENCE: &str = "integration_evidence";
pub const ADOPTION_ACTION_AUTHORITY: &str = "advisory";
pub const ADOPTION_ACTION_KIND_EVIDENCE_REFRESH: &str = "evidence_refresh";
pub const ADOPTION_ACTION_KIND_DIAGNOSTIC_INSPECTION: &str = "diagnostic_inspection";
pub const ADOPTION_ACTION_KIND_UNRESOLVED: &str = "unresolved";
pub const ADOPTION_BOUND_MAX_RELATIONSHIPS: usize = 512;
pub const ADOPTION_BOUND_MAX_OBSERVATIONS: usize = 4_096;
pub const ADOPTION_BOUND_MAX_PROVENANCE: usize = 128;
const RELATIONSHIP_BOUND_EXHAUSTED: &str = "relationships";
const OBSERVATION_BOUND_EXHAUSTED: &str = "observations";
const PROVENANCE_BOUND_EXHAUSTED: &str = "provenance";

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdoptionStageKind {
    UpstreamLifecycle,
    ConsumerPin,
    ConsumerCompatibility,
    IntegrationEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AdoptionStageState {
    Supported,
    Gap,
    Missing,
    Stale,
    Unavailable,
    Blocked,
    Diverged,
    Conflict,
    Partial,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptionRelationshipKey {
    pub relationship_id: String,
    pub upstream_repository: String,
    pub upstream_change: String,
    pub consumer_repository: String,
    pub required_revision: String,
    pub subject_id: String,
    pub policy_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptionQueryBounds {
    pub max_relationships: usize,
    pub max_observations: usize,
    pub max_provenance: usize,
}

impl Default for AdoptionQueryBounds {
    fn default() -> Self {
        Self {
            max_relationships: ADOPTION_BOUND_MAX_RELATIONSHIPS,
            max_observations: ADOPTION_BOUND_MAX_OBSERVATIONS,
            max_provenance: ADOPTION_BOUND_MAX_PROVENANCE,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptionQuery {
    pub scope_id: String,
    pub relationships: Vec<AdoptionRelationshipKey>,
    pub bounds: AdoptionQueryBounds,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptionProducerProvenance {
    pub producer_id: String,
    pub schema: String,
    pub source_revision: String,
    pub tool_identity: String,
    pub policy_identity: String,
    pub observation_scope: String,
    pub receipt_hash: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptionStageObservation {
    pub relationship: AdoptionRelationshipKey,
    pub stage: AdoptionStageKind,
    pub state: AdoptionStageState,
    pub producer_id: String,
    pub producer_status: String,
    pub evidence_hashes: Vec<String>,
    pub observed_revision: Option<String>,
    pub message: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptionCoverage {
    pub partial: bool,
    pub selected_relationships: usize,
    pub observed_relationships: usize,
    pub omitted_relationships: Vec<String>,
    pub bound_exhausted: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptionSnapshot {
    pub schema: String,
    pub query: AdoptionQuery,
    pub provenance: Vec<AdoptionProducerProvenance>,
    pub observations: Vec<AdoptionStageObservation>,
    pub coverage: AdoptionCoverage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptionStageReport {
    pub stage: AdoptionStageKind,
    pub state: AdoptionStageState,
    pub producer_statuses: Vec<String>,
    pub evidence_hashes: Vec<String>,
    pub observed_revisions: Vec<String>,
    pub messages: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptionRelationshipReport {
    pub relationship: AdoptionRelationshipKey,
    pub complete: bool,
    pub stages: Vec<AdoptionStageReport>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptionAction {
    pub id: String,
    pub kind: String,
    pub authority: String,
    pub owner_repository: String,
    pub relationship_id: String,
    pub stage: AdoptionStageKind,
    pub prerequisites: Vec<String>,
    pub expected_observation: String,
    pub effect_authorized: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AdoptionReport {
    pub schema: String,
    pub valid: bool,
    pub complete: bool,
    pub scope_id: String,
    pub input_hash: String,
    pub report_hash: String,
    pub coverage: AdoptionCoverage,
    pub provenance: Vec<AdoptionProducerProvenance>,
    pub relationships: Vec<AdoptionRelationshipReport>,
    pub actions: Vec<AdoptionAction>,
    pub diagnostics: Vec<String>,
}

#[must_use]
pub fn adoption_stage_kinds() -> [AdoptionStageKind; 4] {
    [
        AdoptionStageKind::UpstreamLifecycle,
        AdoptionStageKind::ConsumerPin,
        AdoptionStageKind::ConsumerCompatibility,
        AdoptionStageKind::IntegrationEvidence,
    ]
}

#[must_use]
pub fn derive_adoption_report(snapshot: &AdoptionSnapshot) -> AdoptionReport {
    let mut diagnostics = validate_snapshot(snapshot);
    let mut relationships = snapshot.query.relationships.clone();
    relationships.sort();
    relationships.dedup();
    let original_relationship_count = relationships.len();
    let max_relationships = nonzero_bound(snapshot.query.bounds.max_relationships);
    if relationships.len() > max_relationships {
        relationships.truncate(max_relationships);
        diagnostics.push("adoption.bound.relationships_exhausted".to_string());
    }

    let relationship_set = relationships.iter().cloned().collect::<BTreeSet<_>>();
    let mut observations = snapshot
        .observations
        .iter()
        .filter(|observation| relationship_set.contains(&observation.relationship))
        .cloned()
        .collect::<Vec<_>>();
    observations.sort_by_key(observation_sort_key);
    observations.dedup();
    let max_observations = nonzero_bound(snapshot.query.bounds.max_observations);
    if observations.len() > max_observations {
        observations.truncate(max_observations);
        diagnostics.push("adoption.bound.observations_exhausted".to_string());
    }

    let mut grouped: BTreeMap<AdoptionRelationshipKey, Vec<AdoptionStageObservation>> =
        BTreeMap::new();
    for observation in observations {
        grouped
            .entry(observation.relationship.clone())
            .or_default()
            .push(observation);
    }

    let reports = relationships
        .into_iter()
        .map(|relationship| {
            let group = grouped.get(&relationship).cloned().unwrap_or_default();
            relationship_report(relationship, &group)
        })
        .collect::<Vec<_>>();

    let mut coverage = snapshot.coverage.clone();
    coverage.selected_relationships = reports.len();
    coverage.observed_relationships = grouped.len();
    if original_relationship_count > reports.len() {
        coverage.partial = true;
        coverage
            .bound_exhausted
            .push(RELATIONSHIP_BOUND_EXHAUSTED.to_string());
    }
    if snapshot.observations.len() > max_observations {
        coverage.partial = true;
        coverage
            .bound_exhausted
            .push(OBSERVATION_BOUND_EXHAUSTED.to_string());
    }
    if snapshot.provenance.len() > snapshot.query.bounds.max_provenance {
        coverage.partial = true;
        coverage
            .bound_exhausted
            .push(PROVENANCE_BOUND_EXHAUSTED.to_string());
    }
    coverage.bound_exhausted.sort();
    coverage.bound_exhausted.dedup();
    coverage.omitted_relationships.sort();
    coverage.omitted_relationships.dedup();

    let mut provenance = snapshot.provenance.clone();
    provenance.sort_by_key(provenance_sort_key);
    provenance.dedup();
    let max_provenance = nonzero_bound(snapshot.query.bounds.max_provenance);
    if provenance.len() > max_provenance {
        provenance.truncate(max_provenance);
    }

    let mut actions = reports
        .iter()
        .flat_map(actions_for_relationship)
        .collect::<Vec<_>>();
    actions.sort_by(|left, right| left.id.cmp(&right.id));
    actions.dedup_by(|left, right| left.id == right.id);

    let valid = diagnostics.is_empty();
    let complete = valid && reports.iter().all(|row| row.complete) && !coverage.partial;
    let input_hash = canonical_json_hash(&snapshot_fingerprint_value(snapshot));
    let mut report = AdoptionReport {
        schema: ADOPTION_REPORT_SCHEMA_V1.to_string(),
        valid,
        complete,
        scope_id: snapshot.query.scope_id.clone(),
        input_hash,
        report_hash: String::new(),
        coverage,
        provenance,
        relationships: reports,
        actions,
        diagnostics,
    };
    report.report_hash = report_fingerprint(&report);
    report
}

fn validate_snapshot(snapshot: &AdoptionSnapshot) -> Vec<String> {
    let mut diagnostics = Vec::new();
    if snapshot.schema != ADOPTION_SNAPSHOT_SCHEMA_V1 {
        diagnostics.push("adoption.schema.unsupported".to_string());
    }
    if snapshot.query.scope_id.is_empty() {
        diagnostics.push("adoption.query.scope_missing".to_string());
    }
    if snapshot.query.relationships.is_empty() {
        diagnostics.push("adoption.query.relationships_missing".to_string());
    }
    if snapshot.query.bounds.max_relationships == 0
        || snapshot.query.bounds.max_observations == 0
        || snapshot.query.bounds.max_provenance == 0
    {
        diagnostics.push("adoption.bounds.zero".to_string());
    }
    diagnostics.sort();
    diagnostics.dedup();
    diagnostics
}

fn relationship_report(
    relationship: AdoptionRelationshipKey,
    observations: &[AdoptionStageObservation],
) -> AdoptionRelationshipReport {
    let stages = adoption_stage_kinds()
        .into_iter()
        .map(|stage| stage_report(stage, observations))
        .collect::<Vec<_>>();
    let complete = stages
        .iter()
        .all(|stage| stage.state == AdoptionStageState::Supported);
    AdoptionRelationshipReport {
        relationship,
        complete,
        stages,
    }
}

fn stage_report(
    stage: AdoptionStageKind,
    observations: &[AdoptionStageObservation],
) -> AdoptionStageReport {
    let mut matches = observations
        .iter()
        .filter(|observation| observation.stage == stage)
        .cloned()
        .collect::<Vec<_>>();
    matches.sort_by_key(observation_sort_key);
    matches.dedup();
    if matches.is_empty() {
        return AdoptionStageReport {
            stage,
            state: AdoptionStageState::Missing,
            producer_statuses: Vec::new(),
            evidence_hashes: Vec::new(),
            observed_revisions: Vec::new(),
            messages: vec!["no admitted observation for this stage".to_string()],
        };
    }

    let states = matches
        .iter()
        .map(|observation| observation.state.clone())
        .collect::<BTreeSet<_>>();
    let state = if states.len() == 1 {
        states.iter().next().expect("state present").clone()
    } else {
        AdoptionStageState::Conflict
    };
    let mut producer_statuses = matches
        .iter()
        .map(|observation| {
            format!(
                "{}:{}",
                observation.producer_id, observation.producer_status
            )
        })
        .collect::<Vec<_>>();
    producer_statuses.sort();
    producer_statuses.dedup();
    let mut evidence_hashes = matches
        .iter()
        .flat_map(|observation| observation.evidence_hashes.clone())
        .collect::<Vec<_>>();
    evidence_hashes.sort();
    evidence_hashes.dedup();
    let mut observed_revisions = matches
        .iter()
        .filter_map(|observation| observation.observed_revision.clone())
        .collect::<Vec<_>>();
    observed_revisions.sort();
    observed_revisions.dedup();
    let mut messages = matches
        .iter()
        .map(|observation| observation.message.clone())
        .collect::<Vec<_>>();
    if state == AdoptionStageState::Conflict {
        messages.push("conflicting producer observations for this stage".to_string());
    }
    messages.sort();
    messages.dedup();
    AdoptionStageReport {
        stage,
        state,
        producer_statuses,
        evidence_hashes,
        observed_revisions,
        messages,
    }
}

fn actions_for_relationship(row: &AdoptionRelationshipReport) -> Vec<AdoptionAction> {
    row.stages
        .iter()
        .filter(|stage| stage.state != AdoptionStageState::Supported)
        .map(|stage| action_for_stage(row, stage))
        .collect()
}

fn action_for_stage(
    row: &AdoptionRelationshipReport,
    stage: &AdoptionStageReport,
) -> AdoptionAction {
    let (kind, owner, expected_observation) = match stage.stage {
        AdoptionStageKind::UpstreamLifecycle => (
            ADOPTION_ACTION_KIND_EVIDENCE_REFRESH,
            row.relationship.upstream_repository.as_str(),
            "fresh lifecycle completion evidence",
        ),
        AdoptionStageKind::ConsumerPin => (
            ADOPTION_ACTION_KIND_DIAGNOSTIC_INSPECTION,
            row.relationship.consumer_repository.as_str(),
            "consumer pin observation for the required upstream revision",
        ),
        AdoptionStageKind::ConsumerCompatibility => (
            ADOPTION_ACTION_KIND_EVIDENCE_REFRESH,
            row.relationship.consumer_repository.as_str(),
            "consumer compatibility evidence for the selected subject",
        ),
        AdoptionStageKind::IntegrationEvidence => (
            ADOPTION_ACTION_KIND_EVIDENCE_REFRESH,
            row.relationship.consumer_repository.as_str(),
            "current consumer integration check evidence",
        ),
    };
    let kind = if stage.state == AdoptionStageState::Conflict {
        ADOPTION_ACTION_KIND_UNRESOLVED
    } else {
        kind
    };
    let prerequisites = match stage.state {
        AdoptionStageState::Missing => vec!["supply saved producer receipt".to_string()],
        AdoptionStageState::Stale => vec!["refresh saved producer receipt".to_string()],
        AdoptionStageState::Unavailable => vec!["inspect producer availability".to_string()],
        AdoptionStageState::Blocked => vec!["resolve producer-owned blocker".to_string()],
        AdoptionStageState::Diverged => vec!["review exact revision divergence".to_string()],
        AdoptionStageState::Conflict => {
            vec!["resolve contradictory saved observations".to_string()]
        }
        AdoptionStageState::Partial => vec!["raise or narrow the explicit query bound".to_string()],
        AdoptionStageState::Gap => vec!["perform consumer-owned adoption step".to_string()],
        AdoptionStageState::Unknown => vec!["classify producer-owned unknown state".to_string()],
        AdoptionStageState::Supported => Vec::new(),
    };
    let id = canonical_json_hash(&json!({
        "relationship_id": row.relationship.relationship_id,
        "stage": stage.stage,
        "state": stage.state,
        "kind": kind,
        "owner": owner,
        "expected_observation": expected_observation,
    }));
    AdoptionAction {
        id,
        kind: kind.to_string(),
        authority: ADOPTION_ACTION_AUTHORITY.to_string(),
        owner_repository: owner.to_string(),
        relationship_id: row.relationship.relationship_id.clone(),
        stage: stage.stage.clone(),
        prerequisites,
        expected_observation: expected_observation.to_string(),
        effect_authorized: false,
    }
}

fn nonzero_bound(value: usize) -> usize {
    if value == 0 {
        1
    } else {
        value
    }
}

fn observation_sort_key(observation: &AdoptionStageObservation) -> String {
    canonical_json_hash(&serde_json::to_value(observation).expect("observation serializes"))
}

fn provenance_sort_key(provenance: &AdoptionProducerProvenance) -> String {
    canonical_json_hash(&serde_json::to_value(provenance).expect("provenance serializes"))
}

fn snapshot_fingerprint_value(snapshot: &AdoptionSnapshot) -> Value {
    let mut relationships = snapshot.query.relationships.clone();
    relationships.sort();
    relationships.dedup();
    let mut provenance = snapshot.provenance.clone();
    provenance.sort_by_key(provenance_sort_key);
    provenance.dedup();
    let mut observations = snapshot.observations.clone();
    observations.sort_by_key(observation_sort_key);
    observations.dedup();
    json!({
        "schema": snapshot.schema,
        "query": {
            "scope_id": snapshot.query.scope_id,
            "relationships": relationships,
            "bounds": snapshot.query.bounds,
        },
        "provenance": provenance,
        "observations": observations,
        "coverage": snapshot.coverage,
    })
}

fn report_fingerprint(report: &AdoptionReport) -> String {
    let mut value = serde_json::to_value(report).expect("report serializes");
    if let Some(object) = value.as_object_mut() {
        object.remove("report_hash");
    }
    canonical_json_hash(&value)
}

#[cfg(test)]
mod tests {
    use super::*;

    const HASH_A: &str = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
    const HASH_B: &str = "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb";
    const HASH_C: &str = "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc";

    fn key(id: &str, consumer: &str) -> AdoptionRelationshipKey {
        AdoptionRelationshipKey {
            relationship_id: id.to_string(),
            upstream_repository: "onixresearch/cairn".to_string(),
            upstream_change: "change-1".to_string(),
            consumer_repository: consumer.to_string(),
            required_revision: "rev-new".to_string(),
            subject_id: "adoption-subject".to_string(),
            policy_id: "policy-a".to_string(),
        }
    }

    fn provenance(id: &str) -> AdoptionProducerProvenance {
        AdoptionProducerProvenance {
            producer_id: id.to_string(),
            schema: "producer.schema.v1".to_string(),
            source_revision: "source-rev".to_string(),
            tool_identity: "tool@source-rev".to_string(),
            policy_identity: "policy@source-rev".to_string(),
            observation_scope: "fixture-scope".to_string(),
            receipt_hash: HASH_A.to_string(),
        }
    }

    fn observation(
        relationship: &AdoptionRelationshipKey,
        stage: AdoptionStageKind,
        state: AdoptionStageState,
        producer: &str,
        message: &str,
    ) -> AdoptionStageObservation {
        AdoptionStageObservation {
            relationship: relationship.clone(),
            stage,
            state,
            producer_id: producer.to_string(),
            producer_status: "recorded".to_string(),
            evidence_hashes: vec![HASH_B.to_string()],
            observed_revision: Some("rev-new".to_string()),
            message: message.to_string(),
        }
    }

    fn complete_snapshot() -> AdoptionSnapshot {
        let relationship = key("cairn-to-octet", "onixresearch/octet");
        AdoptionSnapshot {
            schema: ADOPTION_SNAPSHOT_SCHEMA_V1.to_string(),
            query: AdoptionQuery {
                scope_id: "fixture".to_string(),
                relationships: vec![relationship.clone()],
                bounds: AdoptionQueryBounds::default(),
            },
            provenance: vec![provenance("stack-deps")],
            observations: adoption_stage_kinds()
                .into_iter()
                .map(|stage| {
                    observation(
                        &relationship,
                        stage,
                        AdoptionStageState::Supported,
                        "stack-deps",
                        "supported",
                    )
                })
                .collect(),
            coverage: AdoptionCoverage::default(),
        }
    }

    #[test]
    fn complete_saved_chain_reports_supported_stages() {
        let report = derive_adoption_report(&complete_snapshot());
        assert!(report.valid, "{:?}", report.diagnostics);
        assert!(report.complete);
        assert!(report.actions.is_empty());
        assert_eq!(report.relationships.len(), 1);
        assert!(report.relationships[0].complete);
        assert!(report.relationships[0]
            .stages
            .iter()
            .all(|stage| stage.state == AdoptionStageState::Supported));
    }

    #[test]
    fn equivalent_receipts_in_different_orders_keep_semantic_identity() {
        let mut first = complete_snapshot();
        let mut second = first.clone();
        second.query.relationships.reverse();
        second.provenance.reverse();
        second.observations.reverse();
        first.observations.push(first.observations[0].clone());
        let first_report = derive_adoption_report(&first);
        let second_report = derive_adoption_report(&second);
        assert_eq!(first_report.relationships, second_report.relationships);
        assert_eq!(first_report.actions, second_report.actions);
        assert_eq!(first_report.report_hash, second_report.report_hash);
    }

    #[test]
    fn upstream_archive_with_old_consumer_pin_remains_a_gap() {
        let relationship = key("cairn-to-octet", "onixresearch/octet");
        let snapshot = AdoptionSnapshot {
            schema: ADOPTION_SNAPSHOT_SCHEMA_V1.to_string(),
            query: AdoptionQuery {
                scope_id: "old-pin".to_string(),
                relationships: vec![relationship.clone()],
                bounds: AdoptionQueryBounds::default(),
            },
            provenance: vec![provenance("stack-deps")],
            observations: vec![
                observation(
                    &relationship,
                    AdoptionStageKind::UpstreamLifecycle,
                    AdoptionStageState::Supported,
                    "cairn",
                    "archive present",
                ),
                observation(
                    &relationship,
                    AdoptionStageKind::ConsumerPin,
                    AdoptionStageState::Gap,
                    "stack-deps",
                    "consumer pins old revision",
                ),
            ],
            coverage: AdoptionCoverage::default(),
        };
        let report = derive_adoption_report(&snapshot);
        assert!(report.valid);
        assert!(!report.complete);
        let pin = report.relationships[0]
            .stages
            .iter()
            .find(|stage| stage.stage == AdoptionStageKind::ConsumerPin)
            .expect("pin stage");
        assert_eq!(pin.state, AdoptionStageState::Gap);
        assert!(report
            .actions
            .iter()
            .any(|action| action.owner_repository == "onixresearch/octet"
                && action.kind == ADOPTION_ACTION_KIND_DIAGNOSTIC_INSPECTION
                && !action.effect_authorized));
    }

    #[test]
    fn matching_pin_without_current_checks_remains_incomplete() {
        let mut snapshot = complete_snapshot();
        snapshot
            .observations
            .retain(|observation| observation.stage != AdoptionStageKind::IntegrationEvidence);
        let report = derive_adoption_report(&snapshot);
        assert!(report.valid);
        assert!(!report.complete);
        let integration = report.relationships[0]
            .stages
            .iter()
            .find(|stage| stage.stage == AdoptionStageKind::IntegrationEvidence)
            .expect("integration stage");
        assert_eq!(integration.state, AdoptionStageState::Missing);
    }

    #[test]
    fn unknown_conflict_and_partial_states_are_visible_not_successful() {
        let relationship = key("octet-to-lattice", "onixresearch/lattice");
        let coverage = AdoptionCoverage {
            partial: true,
            omitted_relationships: vec!["onixresearch/extra".to_string()],
            ..AdoptionCoverage::default()
        };
        let snapshot = AdoptionSnapshot {
            schema: ADOPTION_SNAPSHOT_SCHEMA_V1.to_string(),
            query: AdoptionQuery {
                scope_id: "unknowns".to_string(),
                relationships: vec![relationship.clone()],
                bounds: AdoptionQueryBounds::default(),
            },
            provenance: vec![provenance("stack-deps")],
            observations: vec![
                observation(
                    &relationship,
                    AdoptionStageKind::ConsumerPin,
                    AdoptionStageState::Unknown,
                    "stack-deps",
                    "producer reported unknown",
                ),
                observation(
                    &relationship,
                    AdoptionStageKind::ConsumerPin,
                    AdoptionStageState::Diverged,
                    "stack-deps-reconcile",
                    "producer reported diverged",
                ),
                observation(
                    &relationship,
                    AdoptionStageKind::IntegrationEvidence,
                    AdoptionStageState::Partial,
                    "stack-doctor",
                    "bounded query omitted checks",
                ),
            ],
            coverage,
        };
        let report = derive_adoption_report(&snapshot);
        assert!(report.valid);
        assert!(report.coverage.partial);
        assert!(!report.complete);
        let pin = report.relationships[0]
            .stages
            .iter()
            .find(|stage| stage.stage == AdoptionStageKind::ConsumerPin)
            .expect("pin stage");
        assert_eq!(pin.state, AdoptionStageState::Conflict);
        assert!(report
            .actions
            .iter()
            .any(|action| action.kind == ADOPTION_ACTION_KIND_UNRESOLVED));
    }

    #[test]
    fn unsupported_snapshot_schema_is_rejected_without_hidden_repair() {
        let mut snapshot = complete_snapshot();
        snapshot.schema = "cairn.adoption.snapshot.v0".to_string();
        snapshot.provenance[0].receipt_hash = HASH_C.to_string();
        let report = derive_adoption_report(&snapshot);
        assert!(!report.valid);
        assert!(report
            .diagnostics
            .contains(&"adoption.schema.unsupported".to_string()));
        assert!(!report.complete);
    }
}
