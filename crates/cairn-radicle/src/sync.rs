use std::{path::Path, str::FromStr, time::Duration};

use cairn_core::{
    change_cob_content_root, change_cob_state_hash, inspect_change_cob_conflicts,
    validate_change_name, ChangeArtifactConflict, ChangeCobError, ChangeLifecycleConflictHead,
};
use radicle::{
    crypto::PublicKey,
    node::{Address, ConnectOptions, ConnectResult, FetchResult, Handle as _, Node},
    storage::{git::Repository, ReadStorage, RefUpdate, RepositoryError},
};
use serde::Serialize;
use thiserror::Error;

use crate::{
    checkout::{acquire_checkout_lock, CheckoutLock},
    import::{acquire_import_lock, load_profile_at, parse_repository_id, ImportLock},
    inspect_change_import, inspect_change_status,
    update::{acquire_update_lock, UpdateLock},
    ChangeImportError, TYPENAME,
};

pub const SYNC_CONNECT_TIMEOUT_SECONDS: u64 = 3;
pub const SYNC_FETCH_TIMEOUT_SECONDS: u64 = 5;

const BLOCKER_LOCAL_STATE_DRIFT: &str = "local Radicle state changed after preview";
const BLOCKER_LOCAL_STATE_UNAVAILABLE: &str = "guarded local Radicle state is unavailable";
const BLOCKER_LOCAL_NODE_UNAVAILABLE: &str = "local Radicle node identity is unavailable";
const BLOCKER_CONNECT_NOT_ATTEMPTED: &str = "peer connection was not attempted";
const BLOCKER_FETCH_NOT_ATTEMPTED: &str = "peer fetch was not attempted";
const BLOCKER_POST_STATE_UNAVAILABLE: &str = "post-fetch local Radicle state is unavailable";
const BLOCKER_POST_CHANGE_MISSING: &str = "requested typed Cairn change is missing after fetch";
const BLOCKER_PEER_REF_UNAVAILABLE: &str = "post-fetch peer change-ref evidence is unavailable";
const BLOCKER_PEER_NAMESPACE_MISSING: &str =
    "fetch result does not identify the requested signed peer namespace";
const BLOCKER_PEER_REF_MISSING: &str =
    "requested peer namespace lacks the exact Cairn change object ref";

// r[impl cairn.collaboration_sync.planning]

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
pub enum ChangeSyncLocalState {
    RepositoryMissing,
    ChangeMissing {
        repository_head_oid: String,
        current_identity_oid: String,
        local_is_delegate: bool,
    },
    Present {
        repository_head_oid: String,
        current_identity_oid: String,
        local_is_delegate: bool,
        object_id: String,
        state_hash: String,
        content_root: Option<String>,
        artifact_conflict_count: usize,
        conflict_head_count: usize,
        lifecycle_conflict_head_count: usize,
        artifact_conflicts: Vec<ChangeArtifactConflict>,
        lifecycle_conflicts: Vec<ChangeLifecycleConflictHead>,
    },
}

impl ChangeSyncLocalState {
    pub fn has_change(&self) -> bool {
        matches!(self, Self::Present { .. })
    }

    pub fn has_conflicts(&self) -> bool {
        matches!(
            self,
            Self::Present {
                artifact_conflicts,
                lifecycle_conflicts,
                ..
            } if !artifact_conflicts.is_empty() || !lifecycle_conflicts.is_empty()
        )
    }

    fn object_id(&self) -> Option<&str> {
        match self {
            Self::Present { object_id, .. } => Some(object_id),
            Self::RepositoryMissing | Self::ChangeMissing { .. } => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSyncInspection {
    pub local_actor: String,
    pub repository_id: String,
    pub state: ChangeSyncLocalState,
}

#[derive(Debug)]
struct ChangeSyncLocks {
    import: ImportLock,
    update: UpdateLock,
    checkout: CheckoutLock,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSyncSnapshot {
    pub change_id: String,
    pub repository_id: String,
    pub rad_home: String,
    pub control_socket: String,
    pub local_actor: String,
    pub peer_id: String,
    pub peer_address: String,
    pub before: ChangeSyncLocalState,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "action", rename_all = "snake_case")]
pub enum ChangeSyncAction {
    Connect {
        timeout_seconds: u64,
        persistent: bool,
    },
    Fetch {
        timeout_seconds: u64,
    },
    ReloadTypedChange,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChangeSyncPlan {
    pub change_id: String,
    pub repository_id: String,
    pub rad_home: String,
    pub control_socket: String,
    pub local_actor: String,
    pub peer_id: String,
    pub peer_address: String,
    pub before: ChangeSyncLocalState,
    pub actions: Vec<ChangeSyncAction>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChangeSyncRefUpdate {
    pub name: String,
    pub kind: String,
    pub old_oid: Option<String>,
    pub new_oid: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeSyncConnectObservation {
    NotAttempted,
    Connected,
    Failed { reason: String },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChangeSyncFetchObservation {
    NotAttempted,
    Succeeded {
        ref_updates: Vec<ChangeSyncRefUpdate>,
        namespaces: Vec<String>,
    },
    Failed {
        reason: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSyncExecutionObservation {
    pub guarded_before: Option<ChangeSyncLocalState>,
    pub local_node_id: Option<String>,
    pub connect: ChangeSyncConnectObservation,
    pub fetch: ChangeSyncFetchObservation,
    pub after: Option<ChangeSyncLocalState>,
    pub peer_change_ref_present: Option<bool>,
    pub observation_error: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct ChangeSyncExecutionOutcome {
    pub local_node_id: Option<String>,
    pub connect_status: String,
    pub fetch_status: String,
    pub ref_updates: Vec<ChangeSyncRefUpdate>,
    pub namespaces: Vec<String>,
    pub after: Option<ChangeSyncLocalState>,
    pub peer_change_ref_present: Option<bool>,
    pub blockers: Vec<String>,
    pub network_attempted: bool,
    pub storage_changed: Option<bool>,
    pub noop: bool,
    pub success: bool,
}

#[derive(Debug, Error)]
pub enum ChangeSyncError {
    #[error("invalid change name: {reason}")]
    InvalidChangeName { reason: String },
    #[error("invalid local actor {actor}: {reason}")]
    InvalidLocalActor { actor: String, reason: String },
    #[error("invalid peer node ID {peer_id}: {reason}")]
    InvalidPeerId { peer_id: String, reason: String },
    #[error("invalid peer protocol address {address}: {reason}")]
    InvalidPeerAddress { address: String, reason: String },
    #[error("sync peer must differ from local profile actor {actor}")]
    SelfPeer { actor: String },
    #[error("sync requires a non-empty explicit Radicle home")]
    MissingRadicleHome,
    #[error("sync requires an absolute explicit node control socket path")]
    InvalidControlSocket,
    #[error("sync plan no longer matches its canonical snapshot")]
    InvalidPlan,
    #[error("execution Radicle home {current} does not match planned home {planned}")]
    RadicleHomeDrift { planned: String, current: String },
    #[error("explicit Radicle profile actor changed from {planned} to {current}")]
    ProfileActorDrift { planned: String, current: String },
    #[error("sync is blocked by a cooperating local operation: {reason}")]
    LockBlocked { reason: String },
    #[error("sync preparation failed and lock cleanup also failed: {reason}")]
    LockCleanup { reason: String },
    #[error(transparent)]
    Adapter(#[from] ChangeImportError),
    #[error("Radicle repository inspection failed: {0}")]
    Repository(#[from] RepositoryError),
    #[error("Radicle peer-ref inspection failed: {0}")]
    Git(#[from] radicle::git::raw::Error),
    #[error("Cairn rejected synchronized change state: {0}")]
    Core(#[from] ChangeCobError),
}

pub fn plan_change_sync(snapshot: ChangeSyncSnapshot) -> Result<ChangeSyncPlan, ChangeSyncError> {
    validate_change_name(&snapshot.change_id)
        .map_err(|reason| ChangeSyncError::InvalidChangeName { reason })?;
    let _repository_id = parse_repository_id(&snapshot.repository_id)?;
    let local_actor = PublicKey::from_str(&snapshot.local_actor).map_err(|error| {
        ChangeSyncError::InvalidLocalActor {
            actor: snapshot.local_actor.clone(),
            reason: error.to_string(),
        }
    })?;
    let peer_id =
        PublicKey::from_str(&snapshot.peer_id).map_err(|error| ChangeSyncError::InvalidPeerId {
            peer_id: snapshot.peer_id.clone(),
            reason: error.to_string(),
        })?;
    let _peer_address = Address::from_str(&snapshot.peer_address).map_err(|error| {
        ChangeSyncError::InvalidPeerAddress {
            address: snapshot.peer_address.clone(),
            reason: error.to_string(),
        }
    })?;
    if local_actor == peer_id {
        return Err(ChangeSyncError::SelfPeer {
            actor: snapshot.local_actor,
        });
    }
    if snapshot.rad_home.is_empty() {
        return Err(ChangeSyncError::MissingRadicleHome);
    }
    if snapshot.control_socket.is_empty() || !Path::new(&snapshot.control_socket).is_absolute() {
        return Err(ChangeSyncError::InvalidControlSocket);
    }
    Ok(ChangeSyncPlan {
        change_id: snapshot.change_id,
        repository_id: snapshot.repository_id,
        rad_home: snapshot.rad_home,
        control_socket: snapshot.control_socket,
        local_actor: snapshot.local_actor,
        peer_id: snapshot.peer_id,
        peer_address: snapshot.peer_address,
        before: snapshot.before,
        actions: vec![
            ChangeSyncAction::Connect {
                timeout_seconds: SYNC_CONNECT_TIMEOUT_SECONDS,
                persistent: false,
            },
            ChangeSyncAction::Fetch {
                timeout_seconds: SYNC_FETCH_TIMEOUT_SECONDS,
            },
            ChangeSyncAction::ReloadTypedChange,
        ],
    })
}

// r[impl cairn.collaboration_sync.execution]
pub fn execute_profile_sync(
    rad_home: &Path,
    plan: &ChangeSyncPlan,
) -> Result<ChangeSyncExecutionOutcome, ChangeSyncError> {
    validate_sync_plan(plan)?;
    let current_rad_home = rad_home.display().to_string();
    if current_rad_home != plan.rad_home {
        return Err(ChangeSyncError::RadicleHomeDrift {
            planned: plan.rad_home.clone(),
            current: current_rad_home,
        });
    }
    let profile = load_profile_at(rad_home)?;
    let current_actor = profile.public_key.to_string();
    if current_actor != plan.local_actor {
        return Err(ChangeSyncError::ProfileActorDrift {
            planned: plan.local_actor.clone(),
            current: current_actor,
        });
    }
    let repository_id = parse_repository_id(&plan.repository_id)?;
    let locks = acquire_sync_locks(profile.home.path(), &plan.repository_id, &plan.change_id)?;
    let result = execute_guarded_sync(&profile, repository_id, plan);
    let release_blockers = locks.release();
    match result {
        Ok(mut outcome) => {
            if !release_blockers.is_empty() {
                outcome.blockers.extend(release_blockers);
                outcome.blockers.sort();
                outcome.blockers.dedup();
                outcome.success = false;
                outcome.noop = false;
            }
            Ok(outcome)
        }
        Err(error) if release_blockers.is_empty() => Err(error),
        Err(error) => Err(ChangeSyncError::LockCleanup {
            reason: format!("{error}; {}", release_blockers.join("; ")),
        }),
    }
}

fn validate_sync_plan(plan: &ChangeSyncPlan) -> Result<(), ChangeSyncError> {
    let canonical = plan_change_sync(ChangeSyncSnapshot {
        change_id: plan.change_id.clone(),
        repository_id: plan.repository_id.clone(),
        rad_home: plan.rad_home.clone(),
        control_socket: plan.control_socket.clone(),
        local_actor: plan.local_actor.clone(),
        peer_id: plan.peer_id.clone(),
        peer_address: plan.peer_address.clone(),
        before: plan.before.clone(),
    })?;
    if canonical != *plan {
        return Err(ChangeSyncError::InvalidPlan);
    }
    Ok(())
}

fn execute_guarded_sync(
    profile: &radicle::profile::Profile,
    repository_id: radicle::identity::RepoId,
    plan: &ChangeSyncPlan,
) -> Result<ChangeSyncExecutionOutcome, ChangeSyncError> {
    let guarded_before = inspect_sync_state(profile, repository_id, &plan.change_id)?;
    if guarded_before != plan.before {
        return Ok(evaluate_change_sync_observation(
            plan,
            ChangeSyncExecutionObservation {
                guarded_before: Some(guarded_before.clone()),
                local_node_id: None,
                connect: ChangeSyncConnectObservation::NotAttempted,
                fetch: ChangeSyncFetchObservation::NotAttempted,
                after: Some(guarded_before),
                peer_change_ref_present: None,
                observation_error: None,
            },
        ));
    }

    let mut node = Node::new(&plan.control_socket);
    let local_node_id = match node.nid() {
        Ok(node_id) => node_id,
        Err(error) => {
            return Ok(evaluate_change_sync_observation(
                plan,
                ChangeSyncExecutionObservation {
                    guarded_before: Some(guarded_before.clone()),
                    local_node_id: None,
                    connect: ChangeSyncConnectObservation::NotAttempted,
                    fetch: ChangeSyncFetchObservation::NotAttempted,
                    after: Some(guarded_before),
                    peer_change_ref_present: None,
                    observation_error: Some(format!(
                        "failed to read local node identity from explicit control socket: {error}"
                    )),
                },
            ));
        }
    };
    if local_node_id.to_string() != plan.local_actor {
        return Ok(evaluate_change_sync_observation(
            plan,
            ChangeSyncExecutionObservation {
                guarded_before: Some(guarded_before.clone()),
                local_node_id: Some(local_node_id.to_string()),
                connect: ChangeSyncConnectObservation::NotAttempted,
                fetch: ChangeSyncFetchObservation::NotAttempted,
                after: Some(guarded_before),
                peer_change_ref_present: None,
                observation_error: None,
            },
        ));
    }

    let peer_id =
        PublicKey::from_str(&plan.peer_id).map_err(|error| ChangeSyncError::InvalidPeerId {
            peer_id: plan.peer_id.clone(),
            reason: error.to_string(),
        })?;
    let peer_address = Address::from_str(&plan.peer_address).map_err(|error| {
        ChangeSyncError::InvalidPeerAddress {
            address: plan.peer_address.clone(),
            reason: error.to_string(),
        }
    })?;
    let connect = match node.connect(
        peer_id,
        peer_address,
        ConnectOptions {
            persistent: false,
            timeout: Duration::from_secs(SYNC_CONNECT_TIMEOUT_SECONDS),
        },
    ) {
        Ok(ConnectResult::Connected) => ChangeSyncConnectObservation::Connected,
        Ok(ConnectResult::Disconnected { reason }) => {
            ChangeSyncConnectObservation::Failed { reason }
        }
        Err(error) => ChangeSyncConnectObservation::Failed {
            reason: error.to_string(),
        },
    };
    if !matches!(connect, ChangeSyncConnectObservation::Connected) {
        return Ok(evaluate_change_sync_observation(
            plan,
            ChangeSyncExecutionObservation {
                guarded_before: Some(guarded_before.clone()),
                local_node_id: Some(local_node_id.to_string()),
                connect,
                fetch: ChangeSyncFetchObservation::NotAttempted,
                after: Some(guarded_before),
                peer_change_ref_present: None,
                observation_error: None,
            },
        ));
    }

    let fetch = match node.fetch(
        repository_id,
        peer_id,
        Duration::from_secs(SYNC_FETCH_TIMEOUT_SECONDS),
        None,
    ) {
        Ok(FetchResult::Success {
            updated,
            namespaces,
            ..
        }) => ChangeSyncFetchObservation::Succeeded {
            ref_updates: updated.into_iter().map(ref_update_record).collect(),
            namespaces: namespaces
                .into_iter()
                .map(|namespace| namespace.to_string())
                .collect(),
        },
        Ok(FetchResult::Failed { reason }) => ChangeSyncFetchObservation::Failed { reason },
        Err(error) => ChangeSyncFetchObservation::Failed {
            reason: error.to_string(),
        },
    };
    let after = inspect_sync_state(profile, repository_id, &plan.change_id);
    let (after, peer_change_ref_present, observation_error) = match after {
        Ok(after) if matches!(fetch, ChangeSyncFetchObservation::Succeeded { .. }) => {
            let peer_ref =
                exact_peer_change_ref_exists(profile, repository_id, peer_id, after.object_id());
            match peer_ref {
                Ok(present) => (Some(after), Some(present), None),
                Err(error) => (Some(after), None, Some(error.to_string())),
            }
        }
        Ok(after) => (Some(after), None, None),
        Err(error) => (None, None, Some(error.to_string())),
    };
    Ok(evaluate_change_sync_observation(
        plan,
        ChangeSyncExecutionObservation {
            guarded_before: Some(guarded_before),
            local_node_id: Some(local_node_id.to_string()),
            connect,
            fetch,
            after,
            peer_change_ref_present,
            observation_error,
        },
    ))
}

// r[impl cairn.collaboration_sync.blocked]
pub fn evaluate_change_sync_observation(
    plan: &ChangeSyncPlan,
    observation: ChangeSyncExecutionObservation,
) -> ChangeSyncExecutionOutcome {
    let network_attempted = !matches!(
        observation.connect,
        ChangeSyncConnectObservation::NotAttempted
    );
    let mut blockers = Vec::new();
    let mut ref_updates = Vec::new();
    let mut namespaces = Vec::new();

    if let Some(error) = &observation.observation_error {
        blockers.push(error.clone());
    } else if observation.guarded_before.as_ref() != Some(&plan.before) {
        if observation.guarded_before.is_some() {
            blockers.push(BLOCKER_LOCAL_STATE_DRIFT.to_string());
        } else {
            blockers.push(BLOCKER_LOCAL_STATE_UNAVAILABLE.to_string());
        }
    } else if observation.local_node_id.as_deref() != Some(plan.local_actor.as_str()) {
        match &observation.local_node_id {
            Some(node_id) => blockers.push(format!(
                "control socket node {node_id} does not match explicit profile actor {}",
                plan.local_actor
            )),
            None => blockers.push(BLOCKER_LOCAL_NODE_UNAVAILABLE.to_string()),
        }
    } else {
        match &observation.connect {
            ChangeSyncConnectObservation::Connected => match &observation.fetch {
                ChangeSyncFetchObservation::Succeeded {
                    ref_updates: observed_updates,
                    namespaces: observed_namespaces,
                } => {
                    ref_updates = sort_ref_updates(observed_updates.clone());
                    namespaces = sort_namespaces(observed_namespaces.clone());
                    validate_post_fetch(plan, &observation, &mut blockers);
                }
                ChangeSyncFetchObservation::Failed { reason } => {
                    blockers.push(format!("peer fetch failed: {reason}"));
                }
                ChangeSyncFetchObservation::NotAttempted => {
                    blockers.push(BLOCKER_FETCH_NOT_ATTEMPTED.to_string());
                }
            },
            ChangeSyncConnectObservation::Failed { reason } => {
                blockers.push(format!("peer connection failed: {reason}"));
            }
            ChangeSyncConnectObservation::NotAttempted => {
                blockers.push(BLOCKER_CONNECT_NOT_ATTEMPTED.to_string());
            }
        }
    }

    let connect_status = connect_status(&observation.connect);
    let fetch_status = fetch_status(&observation.fetch);
    let storage_changed = storage_changed(plan, &observation, &ref_updates);
    let success = blockers.is_empty();
    let noop = success && storage_changed == Some(false);
    ChangeSyncExecutionOutcome {
        local_node_id: observation.local_node_id,
        connect_status,
        fetch_status,
        ref_updates,
        namespaces,
        after: observation.after,
        peer_change_ref_present: observation.peer_change_ref_present,
        blockers,
        network_attempted,
        storage_changed,
        noop,
        success,
    }
}

fn validate_post_fetch(
    plan: &ChangeSyncPlan,
    observation: &ChangeSyncExecutionObservation,
    blockers: &mut Vec<String>,
) {
    match &observation.fetch {
        ChangeSyncFetchObservation::Succeeded { namespaces, .. }
            if namespaces
                .iter()
                .any(|namespace| namespace == &plan.peer_id) => {}
        ChangeSyncFetchObservation::Succeeded { .. } => {
            blockers.push(BLOCKER_PEER_NAMESPACE_MISSING.to_string());
        }
        ChangeSyncFetchObservation::NotAttempted | ChangeSyncFetchObservation::Failed { .. } => {}
    }
    match &observation.after {
        Some(state) if state.has_change() => {}
        Some(_) => blockers.push(BLOCKER_POST_CHANGE_MISSING.to_string()),
        None => blockers.push(BLOCKER_POST_STATE_UNAVAILABLE.to_string()),
    }
    match observation.peer_change_ref_present {
        Some(true) => {}
        Some(false) => blockers.push(BLOCKER_PEER_REF_MISSING.to_string()),
        None => blockers.push(BLOCKER_PEER_REF_UNAVAILABLE.to_string()),
    }
}

fn sort_ref_updates(mut updates: Vec<ChangeSyncRefUpdate>) -> Vec<ChangeSyncRefUpdate> {
    updates.sort_by(|left, right| {
        (
            left.name.as_str(),
            left.kind.as_str(),
            left.old_oid.as_deref(),
            left.new_oid.as_deref(),
        )
            .cmp(&(
                right.name.as_str(),
                right.kind.as_str(),
                right.old_oid.as_deref(),
                right.new_oid.as_deref(),
            ))
    });
    updates
}

fn sort_namespaces(mut namespaces: Vec<String>) -> Vec<String> {
    namespaces.sort();
    namespaces.dedup();
    namespaces
}

fn connect_status(observation: &ChangeSyncConnectObservation) -> String {
    match observation {
        ChangeSyncConnectObservation::NotAttempted => "not_attempted",
        ChangeSyncConnectObservation::Connected => "connected",
        ChangeSyncConnectObservation::Failed { .. } => "failed",
    }
    .to_string()
}

fn fetch_status(observation: &ChangeSyncFetchObservation) -> String {
    match observation {
        ChangeSyncFetchObservation::NotAttempted => "not_attempted",
        ChangeSyncFetchObservation::Succeeded { .. } => "succeeded",
        ChangeSyncFetchObservation::Failed { .. } => "failed",
    }
    .to_string()
}

fn storage_changed(
    plan: &ChangeSyncPlan,
    observation: &ChangeSyncExecutionObservation,
    updates: &[ChangeSyncRefUpdate],
) -> Option<bool> {
    if !matches!(
        observation.fetch,
        ChangeSyncFetchObservation::Succeeded { .. }
    ) {
        return (!observation
            .connect
            .eq(&ChangeSyncConnectObservation::Connected))
        .then_some(false);
    }
    observation
        .after
        .as_ref()
        .map(|after| after != &plan.before || updates.iter().any(|update| update.kind != "skipped"))
}

pub fn inspect_profile_sync(
    rad_home: &Path,
    repository_id: &str,
    change_id: &str,
) -> Result<ChangeSyncInspection, ChangeSyncError> {
    validate_change_name(change_id)
        .map_err(|reason| ChangeSyncError::InvalidChangeName { reason })?;
    let profile = load_profile_at(rad_home)?;
    let repository_id = parse_repository_id(repository_id)?;
    let state = inspect_sync_state(&profile, repository_id, change_id)?;
    Ok(ChangeSyncInspection {
        local_actor: profile.public_key.to_string(),
        repository_id: repository_id.to_string(),
        state,
    })
}

fn inspect_sync_state(
    profile: &radicle::profile::Profile,
    repository_id: radicle::identity::RepoId,
    change_id: &str,
) -> Result<ChangeSyncLocalState, ChangeSyncError> {
    if !profile.storage.contains(&repository_id)? {
        return Ok(ChangeSyncLocalState::RepositoryMissing);
    }
    let repo = profile.storage.repository(repository_id)?;
    let import = inspect_change_import(&repo, &profile.public_key, change_id)?;
    if import.existing_object_id.is_none() {
        return Ok(ChangeSyncLocalState::ChangeMissing {
            repository_head_oid: import.base_git_oid,
            current_identity_oid: import.authority.identity_oid,
            local_is_delegate: import.authority.is_delegate,
        });
    }
    let status = inspect_change_status(&repo, &profile.public_key, change_id)?;
    let conflicts = inspect_change_cob_conflicts(&status.state)?;
    let artifact_conflict_count = conflicts.artifact_conflicts.len();
    let conflict_head_count = conflicts
        .artifact_conflicts
        .iter()
        .map(|conflict| conflict.heads.len())
        .sum();
    let lifecycle_conflict_head_count = conflicts.lifecycle_heads.len();
    let content_root = if conflicts.has_conflicts() {
        None
    } else {
        Some(change_cob_content_root(&status.state)?)
    };
    Ok(ChangeSyncLocalState::Present {
        repository_head_oid: status.repository_head_oid,
        current_identity_oid: status.authority.identity_oid,
        local_is_delegate: status.authority.is_delegate,
        object_id: status.object_id,
        state_hash: change_cob_state_hash(&status.state)?,
        content_root,
        artifact_conflict_count,
        conflict_head_count,
        lifecycle_conflict_head_count,
        artifact_conflicts: conflicts.artifact_conflicts,
        lifecycle_conflicts: conflicts.lifecycle_heads,
    })
}

fn acquire_sync_locks(
    rad_home: &Path,
    repository_id: &str,
    change_id: &str,
) -> Result<ChangeSyncLocks, ChangeSyncError> {
    let import = acquire_import_lock(rad_home, repository_id, change_id).map_err(|error| {
        ChangeSyncError::LockBlocked {
            reason: error.to_string(),
        }
    })?;
    let update = match acquire_update_lock(rad_home, repository_id, change_id) {
        Ok(update) => update,
        Err(error) => {
            let reason = error.to_string();
            return match import.release() {
                Ok(()) => Err(ChangeSyncError::LockBlocked { reason }),
                Err(cleanup) => Err(ChangeSyncError::LockCleanup {
                    reason: format!("{reason}; failed to release import lock: {cleanup}"),
                }),
            };
        }
    };
    let checkout = match acquire_checkout_lock(rad_home, repository_id, change_id) {
        Ok(checkout) => checkout,
        Err(error) => {
            let mut reasons = vec![error.to_string()];
            if let Err(cleanup) = update.release() {
                reasons.push(format!("failed to release update lock: {cleanup}"));
            }
            if let Err(cleanup) = import.release() {
                reasons.push(format!("failed to release import lock: {cleanup}"));
            }
            return if reasons.len() == 1 {
                Err(ChangeSyncError::LockBlocked {
                    reason: reasons.remove(0),
                })
            } else {
                Err(ChangeSyncError::LockCleanup {
                    reason: reasons.join("; "),
                })
            };
        }
    };
    Ok(ChangeSyncLocks {
        import,
        update,
        checkout,
    })
}

impl ChangeSyncLocks {
    fn release(self) -> Vec<String> {
        let mut blockers = Vec::new();
        if let Err(error) = self.checkout.release() {
            blockers.push(format!("failed to release checkout lock: {error}"));
        }
        if let Err(error) = self.update.release() {
            blockers.push(format!("failed to release update lock: {error}"));
        }
        if let Err(error) = self.import.release() {
            blockers.push(format!("failed to release import lock: {error}"));
        }
        blockers
    }
}

fn exact_peer_change_ref_exists(
    profile: &radicle::profile::Profile,
    repository_id: radicle::identity::RepoId,
    peer_id: PublicKey,
    object_id: Option<&str>,
) -> Result<bool, ChangeSyncError> {
    let Some(object_id) = object_id else {
        return Ok(false);
    };
    let repo = profile.storage.repository(repository_id)?;
    exact_peer_change_ref_exists_in(&repo, peer_id, object_id)
}

fn exact_peer_change_ref_exists_in(
    repo: &Repository,
    peer_id: PublicKey,
    object_id: &str,
) -> Result<bool, ChangeSyncError> {
    let expected = format!(
        "refs/namespaces/{peer_id}/refs/cobs/{}/{object_id}",
        &*TYPENAME
    );
    for reference in repo.backend.references()? {
        let reference = reference?;
        if reference.name() == Some(expected.as_str()) {
            return Ok(true);
        }
    }
    Ok(false)
}

fn ref_update_record(update: RefUpdate) -> ChangeSyncRefUpdate {
    match update {
        RefUpdate::Updated { name, old, new } => ChangeSyncRefUpdate {
            name: name.to_string(),
            kind: "updated".to_string(),
            old_oid: Some(old.to_string()),
            new_oid: Some(new.to_string()),
        },
        RefUpdate::Created { name, oid } => ChangeSyncRefUpdate {
            name: name.to_string(),
            kind: "created".to_string(),
            old_oid: None,
            new_oid: Some(oid.to_string()),
        },
        RefUpdate::Deleted { name, oid } => ChangeSyncRefUpdate {
            name: name.to_string(),
            kind: "deleted".to_string(),
            old_oid: Some(oid.to_string()),
            new_oid: None,
        },
        RefUpdate::Skipped { name, oid } => ChangeSyncRefUpdate {
            name: name.to_string(),
            kind: "skipped".to_string(),
            old_oid: Some(oid.to_string()),
            new_oid: Some(oid.to_string()),
        },
    }
}

#[cfg(test)]
mod tests {
    use radicle::crypto::{test::signer::MockSigner, Signer as _};

    use super::*;

    const CHANGE_ID: &str = "collaboration-sync";
    const REPOSITORY_ID: &str = "rad:z3gqcJUoA1n9HaHKufZs5FCSGazv5";
    const RAD_HOME: &str = "/tmp/cairn-sync-profile";
    const CONTROL_SOCKET: &str = "/tmp/cairn-sync-profile/node/control.sock";
    const PEER_ADDRESS: &str = "127.0.0.1:8776";
    const LOCAL_SEED_BYTE: u8 = 1;
    const PEER_SEED_BYTE: u8 = 2;
    const KEY_BYTES: usize = 32;
    const SYNC_ACTION_COUNT: usize = 3;

    fn actor(seed_byte: u8) -> String {
        MockSigner::from_seed([seed_byte; KEY_BYTES])
            .public_key()
            .to_string()
    }

    fn missing_state() -> ChangeSyncLocalState {
        ChangeSyncLocalState::RepositoryMissing
    }

    fn present_state(state_hash: &str) -> ChangeSyncLocalState {
        ChangeSyncLocalState::Present {
            repository_head_oid: "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
            current_identity_oid: "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string(),
            local_is_delegate: true,
            object_id: "cccccccccccccccccccccccccccccccccccccccc".to_string(),
            state_hash: state_hash.to_string(),
            content_root: Some(cairn_core::blake3_hex(b"content")),
            artifact_conflict_count: 0,
            conflict_head_count: 0,
            lifecycle_conflict_head_count: 0,
            artifact_conflicts: Vec::new(),
            lifecycle_conflicts: Vec::new(),
        }
    }

    fn snapshot(before: ChangeSyncLocalState) -> ChangeSyncSnapshot {
        ChangeSyncSnapshot {
            change_id: CHANGE_ID.to_string(),
            repository_id: REPOSITORY_ID.to_string(),
            rad_home: RAD_HOME.to_string(),
            control_socket: CONTROL_SOCKET.to_string(),
            local_actor: actor(LOCAL_SEED_BYTE),
            peer_id: actor(PEER_SEED_BYTE),
            peer_address: PEER_ADDRESS.to_string(),
            before,
        }
    }

    fn successful_observation(
        plan: &ChangeSyncPlan,
        after: ChangeSyncLocalState,
    ) -> ChangeSyncExecutionObservation {
        ChangeSyncExecutionObservation {
            guarded_before: Some(plan.before.clone()),
            local_node_id: Some(plan.local_actor.clone()),
            connect: ChangeSyncConnectObservation::Connected,
            fetch: ChangeSyncFetchObservation::Succeeded {
                ref_updates: Vec::new(),
                namespaces: vec![plan.peer_id.clone()],
            },
            after: Some(after),
            peer_change_ref_present: Some(true),
            observation_error: None,
        }
    }

    // r[verify cairn.collaboration_sync.planning.deterministic]
    #[test]
    fn equivalent_snapshots_produce_fixed_ordered_plans() -> Result<(), Box<dyn std::error::Error>>
    {
        let snapshot = snapshot(missing_state());
        let first = plan_change_sync(snapshot.clone())?;
        let second = plan_change_sync(snapshot)?;
        assert_eq!(first, second);
        assert_eq!(first.actions.len(), SYNC_ACTION_COUNT);
        assert!(matches!(
            first.actions.as_slice(),
            [
                ChangeSyncAction::Connect {
                    timeout_seconds: SYNC_CONNECT_TIMEOUT_SECONDS,
                    persistent: false
                },
                ChangeSyncAction::Fetch {
                    timeout_seconds: SYNC_FETCH_TIMEOUT_SECONDS
                },
                ChangeSyncAction::ReloadTypedChange
            ]
        ));
        Ok(())
    }

    // r[verify cairn.collaboration_sync.planning.negative]
    #[test]
    fn planner_rejects_self_peer_relative_socket_and_invalid_provider_values() {
        let mut self_peer = snapshot(missing_state());
        self_peer.peer_id = self_peer.local_actor.clone();
        assert!(matches!(
            plan_change_sync(self_peer),
            Err(ChangeSyncError::SelfPeer { .. })
        ));

        let mut relative_socket = snapshot(missing_state());
        relative_socket.control_socket = "node/control.sock".to_string();
        assert!(matches!(
            plan_change_sync(relative_socket),
            Err(ChangeSyncError::InvalidControlSocket)
        ));

        let mut invalid_peer = snapshot(missing_state());
        invalid_peer.peer_id = "not-a-node-id".to_string();
        assert!(matches!(
            plan_change_sync(invalid_peer),
            Err(ChangeSyncError::InvalidPeerId { .. })
        ));

        let mut invalid_address = snapshot(missing_state());
        invalid_address.peer_address = "missing-port".to_string();
        assert!(matches!(
            plan_change_sync(invalid_address),
            Err(ChangeSyncError::InvalidPeerAddress { .. })
        ));
    }

    // r[verify cairn.collaboration_sync.integration.noop]
    // r[verify cairn.collaboration_sync.receipts.sorted]
    #[test]
    fn evaluator_accepts_validated_noop_and_sorts_provider_evidence(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let before = present_state("state-one");
        let plan = plan_change_sync(snapshot(before.clone()))?;
        let mut observation = successful_observation(&plan, before);
        observation.fetch = ChangeSyncFetchObservation::Succeeded {
            ref_updates: vec![
                ChangeSyncRefUpdate {
                    name: "refs/z".to_string(),
                    kind: "skipped".to_string(),
                    old_oid: Some("z".to_string()),
                    new_oid: Some("z".to_string()),
                },
                ChangeSyncRefUpdate {
                    name: "refs/a".to_string(),
                    kind: "skipped".to_string(),
                    old_oid: Some("a".to_string()),
                    new_oid: Some("a".to_string()),
                },
            ],
            namespaces: vec![
                plan.peer_id.clone(),
                actor(LOCAL_SEED_BYTE),
                plan.peer_id.clone(),
            ],
        };
        let outcome = evaluate_change_sync_observation(&plan, observation);
        assert!(outcome.success);
        assert!(outcome.noop);
        assert_eq!(outcome.storage_changed, Some(false));
        assert_eq!(outcome.ref_updates[0].name, "refs/a");
        assert_eq!(outcome.namespaces.len(), 2);
        Ok(())
    }

    // r[verify cairn.collaboration_sync.planning.negative]
    // r[verify cairn.collaboration_sync.blocked.socket]
    // r[verify cairn.collaboration_sync.blocked.missing_ref]
    #[test]
    fn evaluator_fails_closed_for_drift_wrong_socket_and_missing_peer_ref(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let before = present_state("state-one");
        let plan = plan_change_sync(snapshot(before.clone()))?;

        let mut stale = successful_observation(&plan, before.clone());
        stale.guarded_before = Some(present_state("state-two"));
        stale.connect = ChangeSyncConnectObservation::NotAttempted;
        stale.fetch = ChangeSyncFetchObservation::NotAttempted;
        let stale = evaluate_change_sync_observation(&plan, stale);
        assert!(!stale.success);
        assert_eq!(stale.blockers, [BLOCKER_LOCAL_STATE_DRIFT]);
        assert!(!stale.network_attempted);
        assert_eq!(stale.storage_changed, Some(false));

        let mut wrong_socket = successful_observation(&plan, before.clone());
        wrong_socket.local_node_id = Some(plan.peer_id.clone());
        wrong_socket.connect = ChangeSyncConnectObservation::NotAttempted;
        wrong_socket.fetch = ChangeSyncFetchObservation::NotAttempted;
        let wrong_socket = evaluate_change_sync_observation(&plan, wrong_socket);
        assert!(!wrong_socket.success);
        assert!(wrong_socket.blockers[0].contains("does not match explicit profile actor"));

        let mut missing_namespace = successful_observation(&plan, before.clone());
        missing_namespace.fetch = ChangeSyncFetchObservation::Succeeded {
            ref_updates: Vec::new(),
            namespaces: vec![plan.local_actor.clone()],
        };
        let missing_namespace = evaluate_change_sync_observation(&plan, missing_namespace);
        assert!(!missing_namespace.success);
        assert_eq!(missing_namespace.blockers, [BLOCKER_PEER_NAMESPACE_MISSING]);

        let mut missing_ref = successful_observation(&plan, before);
        missing_ref.peer_change_ref_present = Some(false);
        let missing_ref = evaluate_change_sync_observation(&plan, missing_ref);
        assert!(!missing_ref.success);
        assert_eq!(missing_ref.blockers, [BLOCKER_PEER_REF_MISSING]);
        Ok(())
    }

    #[test]
    fn ref_update_records_preserve_provider_change_kind() -> Result<(), Box<dyn std::error::Error>>
    {
        let oid = radicle::git::Oid::from_str("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa")?;
        let update = RefUpdate::Skipped {
            name: radicle::git::fmt::RefString::try_from("refs/heads/main".to_string())?,
            oid,
        };
        let record = ref_update_record(update);
        assert_eq!(record.kind, "skipped");
        assert_eq!(record.old_oid, record.new_oid);
        Ok(())
    }

    #[test]
    fn execution_rejects_a_different_radicle_home_before_profile_access(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let plan = plan_change_sync(snapshot(missing_state()))?;
        let result = execute_profile_sync(Path::new("/tmp/different-cairn-sync-profile"), &plan);
        assert!(matches!(
            result,
            Err(ChangeSyncError::RadicleHomeDrift { .. })
        ));
        Ok(())
    }

    // r[verify cairn.collaboration_sync.execution.locked]
    #[test]
    fn sync_locks_serialize_and_release_every_cooperating_path(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let root = std::env::temp_dir().join(format!(
            "cairn-collaboration-sync-locks-{}",
            std::process::id()
        ));
        match std::fs::remove_dir_all(&root) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => return Err(error.into()),
        }
        std::fs::create_dir_all(&root)?;
        let first = acquire_sync_locks(&root, REPOSITORY_ID, CHANGE_ID)?;
        assert!(matches!(
            acquire_sync_locks(&root, REPOSITORY_ID, CHANGE_ID),
            Err(ChangeSyncError::LockBlocked { .. })
        ));
        assert!(first.release().is_empty());
        let second = acquire_sync_locks(&root, REPOSITORY_ID, CHANGE_ID)?;
        assert!(second.release().is_empty());
        std::fs::remove_dir_all(root)?;
        Ok(())
    }
}
