mod support;

use std::{
    collections::BTreeSet,
    env,
    error::Error,
    fs::{self, File},
    io,
    net::{SocketAddr, TcpListener},
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    sync::atomic::{AtomicU64, Ordering},
    thread,
    time::{Duration, Instant},
};

use cairn_core::{
    change_cob_content_root, inspect_change_cob_conflicts, plan_change_cob_resolution,
    ChangeArtifactId, ChangeCobAction, ChangeCollaborationResolutionSnapshot,
};
use onix_cairn_radicle::{create_change_resolution, inspect_change_status};
use radicle::{
    crypto::{PublicKey, Seed},
    node::{policy::Scope, ConnectOptions, ConnectResult, FetchResult, Handle as _, Node},
    profile::{Home, Profile},
    rad,
    storage::{ReadStorage, SignRepository},
    test::fixtures,
};

use support::{
    add_delegate, assert_same_typed_state, assert_signed_change_ref, import_change,
    resolution_selection, update_proposal, validate_distinct_peers, PeerDescriptor, ALICE_PROPOSAL,
    BOB_PROPOSAL, CHANGE_ID, EXPECTED_CONFLICT_HEADS, POLICY_HASH,
};

const TEST_DIRECTORY_PREFIX: &str = "cairn-live-radicle-convergence";
const RADICLE_NODE_BINARY_ENV: &str = "CAIRN_RADICLE_NODE_BIN";
const SECRET_KEY_FILE: &str = "radicle";
const LOOPBACK_HOST: &str = "127.0.0.1";
const NODE_LOG_FILE: &str = "cairn-live-node.log";
const NODE_READY_TIMEOUT: Duration = Duration::from_secs(10);
const CONNECT_TIMEOUT: Duration = Duration::from_secs(3);
const FETCH_TIMEOUT: Duration = Duration::from_secs(5);
const POLL_INTERVAL: Duration = Duration::from_millis(50);
static TEST_SEQUENCE: AtomicU64 = AtomicU64::new(0);

#[derive(Debug)]
struct TestRoot {
    path: PathBuf,
}

impl TestRoot {
    fn create() -> Result<Self, io::Error> {
        let sequence = TEST_SEQUENCE.fetch_add(1, Ordering::Relaxed);
        let path = env::temp_dir().join(format!(
            "{TEST_DIRECTORY_PREFIX}-{}-{sequence}",
            std::process::id()
        ));
        match fs::remove_dir_all(&path) {
            Ok(()) => {}
            Err(error) if error.kind() == io::ErrorKind::NotFound => {}
            Err(error) => return Err(error),
        }
        fs::create_dir_all(&path)?;
        Ok(Self { path })
    }
}

impl Drop for TestRoot {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[derive(Debug)]
struct LiveNode {
    child: Child,
    log_path: PathBuf,
}

impl LiveNode {
    fn spawn(binary: &Path, profile: &Profile, listen: SocketAddr) -> Result<Self, Box<dyn Error>> {
        let log_path = profile.home.node().join(NODE_LOG_FILE);
        fs::create_dir_all(profile.home.node())?;
        let log = File::create(&log_path)?;
        let stderr = log.try_clone()?;
        let child = Command::new(binary)
            .arg("--config")
            .arg(profile.home.config())
            .arg("--secret")
            .arg(profile.home.keys().join(SECRET_KEY_FILE))
            .arg("--listen")
            .arg(listen.to_string())
            .arg("--log-level")
            .arg("error")
            .env("RAD_HOME", profile.home.path())
            .stdin(Stdio::null())
            .stdout(Stdio::from(log))
            .stderr(Stdio::from(stderr))
            .spawn()?;
        Ok(Self { child, log_path })
    }

    fn wait_ready(&mut self, socket: &Path) -> Result<PublicKey, Box<dyn Error>> {
        let deadline = Instant::now() + NODE_READY_TIMEOUT;
        loop {
            if let Some(status) = self.child.try_wait()? {
                return Err(format!(
                    "radicle-node exited before readiness with {status}: {}",
                    self.diagnostics()
                )
                .into());
            }
            let handle = Node::new(socket);
            if let Ok(node_id) = handle.nid() {
                return Ok(node_id);
            }
            if Instant::now() >= deadline {
                return Err(format!(
                    "radicle-node readiness deadline expired: {}",
                    self.diagnostics()
                )
                .into());
            }
            thread::sleep(POLL_INTERVAL);
        }
    }

    fn diagnostics(&self) -> String {
        fs::read_to_string(&self.log_path)
            .unwrap_or_else(|error| format!("unable to read node log: {error}"))
    }
}

impl Drop for LiveNode {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

fn available_loopback_address() -> Result<SocketAddr, io::Error> {
    let listener = TcpListener::bind((LOOPBACK_HOST, 0))?;
    listener.local_addr()
}

fn profile_at(path: &Path, alias: &str) -> Result<Profile, Box<dyn Error>> {
    let home = Home::new(path)?;
    Ok(Profile::init(home, alias.parse()?, None, Seed::generate())?)
}

fn peer_descriptor(profile: &Profile, repository_id: &str) -> PeerDescriptor {
    PeerDescriptor {
        home: profile.home.path().display().to_string(),
        storage: profile.storage.path().display().to_string(),
        public_key: profile.public_key.to_string(),
        repository_id: repository_id.to_string(),
    }
}

fn successful_fetch(
    handle: &mut Node,
    repository_id: radicle::prelude::RepoId,
    remote: PublicKey,
) -> Result<Vec<radicle::storage::RefUpdate>, Box<dyn Error>> {
    match handle.fetch(repository_id, remote, FETCH_TIMEOUT, None)? {
        FetchResult::Success { updated, .. } => Ok(updated),
        FetchResult::Failed { reason } => {
            Err(format!("live Radicle fetch failed: {reason}").into())
        }
    }
}

fn connect_options() -> ConnectOptions {
    ConnectOptions {
        persistent: true,
        timeout: CONNECT_TIMEOUT,
    }
}

// r[impl cairn.radicle_two_peer_convergence.live]
// r[verify cairn.radicle_two_peer_convergence.live.success]
// r[verify cairn.radicle_two_peer_convergence.live.explicit]
// r[verify cairn.radicle_two_peer_convergence.blocked.unavailable]
#[test]
#[ignore = "requires CAIRN_RADICLE_NODE_BIN and the dedicated Nix live-node gate"]
fn live_nodes_transfer_conflict_and_resolution_without_copying_storage(
) -> Result<(), Box<dyn Error>> {
    let binary = env::var_os(RADICLE_NODE_BINARY_ENV)
        .map(PathBuf::from)
        .ok_or("CAIRN_RADICLE_NODE_BIN is required")?;
    if !binary.is_file() {
        return Err(format!("radicle-node binary does not exist: {}", binary.display()).into());
    }

    let root = TestRoot::create()?;
    let alice_profile = profile_at(&root.path.join("alice"), "alice")?;
    let bob_profile = profile_at(&root.path.join("bob"), "bob")?;
    let unavailable_profile = profile_at(&root.path.join("unavailable"), "unavailable")?;
    let alice_signer = alice_profile.signer()?;
    let bob_signer = bob_profile.signer()?;
    let (repository_id, _, _, _) = fixtures::project(
        root.path.join("working"),
        &alice_profile.storage,
        alice_signer.as_ref(),
    )?;
    let alice_repo = alice_profile.storage.repository(repository_id)?;
    add_delegate(&alice_repo, &alice_signer, bob_profile.public_key)?;
    let alice_imported = import_change(&alice_repo, &alice_signer)?;
    validate_distinct_peers(
        &peer_descriptor(&alice_profile, &repository_id.to_string()),
        &peer_descriptor(&bob_profile, &repository_id.to_string()),
    )?;

    let alice_address = available_loopback_address()?;
    let bob_address = available_loopback_address()?;
    assert_ne!(alice_address, bob_address);
    let mut alice_node = LiveNode::spawn(&binary, &alice_profile, alice_address)?;
    let mut bob_node = LiveNode::spawn(&binary, &bob_profile, bob_address)?;
    let alice_socket = alice_profile.home.socket_default();
    let bob_socket = bob_profile.home.socket_default();
    let alice_node_id = alice_node.wait_ready(&alice_socket)?;
    let bob_node_id = bob_node.wait_ready(&bob_socket)?;
    assert_eq!(alice_node_id, alice_profile.public_key);
    assert_eq!(bob_node_id, bob_profile.public_key);
    assert_ne!(alice_node_id, bob_node_id);

    let mut alice_handle = Node::new(&alice_socket);
    let mut bob_handle = Node::new(&bob_socket);
    assert!(alice_handle.seed(repository_id, Scope::All)?);
    assert!(alice_handle.add_inventory(repository_id)?);

    let unavailable_address = available_loopback_address()?;
    let unavailable_started = Instant::now();
    let unavailable = bob_handle.connect(
        unavailable_profile.public_key,
        unavailable_address.into(),
        connect_options(),
    );
    assert!(unavailable_started.elapsed() <= NODE_READY_TIMEOUT);
    assert!(matches!(
        unavailable,
        Ok(ConnectResult::Disconnected { .. }) | Err(_)
    ));
    assert!(bob_profile.storage.repository(repository_id).is_err());
    assert!(bob_handle.seed(repository_id, Scope::All)?);

    assert!(matches!(
        bob_handle.connect(alice_node_id, alice_address.into(), connect_options())?,
        ConnectResult::Connected
    ));
    let import_updates = successful_fetch(&mut bob_handle, repository_id, alice_node_id)?;
    assert!(!import_updates.is_empty());
    assert!(import_updates
        .iter()
        .any(|update| update.name().as_str().contains("refs/cobs/")));
    let bob_repo = bob_profile.storage.repository(repository_id)?;
    rad::fork(repository_id, bob_signer.as_ref(), &bob_profile.storage)?;
    bob_repo.sign_refs(&bob_signer)?;
    assert_signed_change_ref(&bob_repo, alice_node_id)?;
    let bob_imported = inspect_change_status(&bob_repo, &bob_profile.public_key, CHANGE_ID)?;
    assert_same_typed_state(&alice_imported, &bob_imported)?;

    assert!(matches!(
        alice_handle.connect(bob_node_id, bob_address.into(), connect_options())?,
        ConnectResult::Connected
    ));
    let alice_updated = update_proposal(&alice_repo, &alice_signer, ALICE_PROPOSAL)?;
    let bob_updated = update_proposal(&bob_repo, &bob_signer, BOB_PROPOSAL)?;
    assert!(!inspect_change_cob_conflicts(&alice_updated.state)?.has_conflicts());
    assert!(!inspect_change_cob_conflicts(&bob_updated.state)?.has_conflicts());
    alice_handle.announce_refs_for(repository_id, [alice_node_id])?;
    bob_handle.announce_refs_for(repository_id, [bob_node_id])?;

    let alice_updates = successful_fetch(&mut alice_handle, repository_id, bob_node_id)?;
    let bob_updates = successful_fetch(&mut bob_handle, repository_id, alice_node_id)?;
    assert!(!alice_updates.is_empty());
    assert!(!bob_updates.is_empty());
    assert_signed_change_ref(&alice_repo, bob_node_id)?;
    assert_signed_change_ref(&bob_repo, alice_node_id)?;
    let alice_conflicted =
        inspect_change_status(&alice_repo, &alice_profile.public_key, CHANGE_ID)?;
    let bob_conflicted = inspect_change_status(&bob_repo, &bob_profile.public_key, CHANGE_ID)?;
    assert_same_typed_state(&alice_conflicted, &bob_conflicted)?;
    let conflicts = inspect_change_cob_conflicts(&alice_conflicted.state)?;
    assert!(conflicts.has_conflicts());
    assert_eq!(conflicts.artifact_conflicts.len(), 1);
    let proposal_conflict = &conflicts.artifact_conflicts[0];
    assert_eq!(proposal_conflict.artifact, ChangeArtifactId::proposal());
    assert_eq!(proposal_conflict.heads.len(), EXPECTED_CONFLICT_HEADS);
    let expected_bases = proposal_conflict
        .heads
        .iter()
        .map(|head| head.revision_id.clone())
        .collect::<BTreeSet<_>>();

    let selection = resolution_selection(&alice_conflicted, ALICE_PROPOSAL)?;
    let resolution = plan_change_cob_resolution(
        &alice_conflicted.state,
        ChangeCollaborationResolutionSnapshot {
            change_id: CHANGE_ID.to_string(),
            policy_hash: POLICY_HASH.to_string(),
            actor: alice_conflicted.authority.actor.clone(),
            is_delegate: alice_conflicted.authority.is_delegate,
            artifact_selections: vec![selection],
            lifecycle_status: None,
        },
    )?;
    let resolution_bases = resolution
        .actions
        .iter()
        .find_map(|action| match action {
            ChangeCobAction::SetArtifact {
                artifact, basis, ..
            } if *artifact == ChangeArtifactId::proposal() => Some(basis),
            ChangeCobAction::Init { .. }
            | ChangeCobAction::SetArtifact { .. }
            | ChangeCobAction::AttachEvidence { .. }
            | ChangeCobAction::SetLifecycle { .. } => None,
        })
        .ok_or("resolution proposal action is missing")?;
    assert_eq!(resolution_bases, &expected_bases);
    create_change_resolution(&alice_repo, &alice_signer, &resolution, &alice_conflicted)?;
    alice_handle.announce_refs_for(repository_id, [alice_node_id])?;
    let resolution_updates = successful_fetch(&mut bob_handle, repository_id, alice_node_id)?;
    assert!(!resolution_updates.is_empty());
    assert_signed_change_ref(&bob_repo, alice_node_id)?;

    let alice_resolved = inspect_change_status(&alice_repo, &alice_profile.public_key, CHANGE_ID)?;
    let bob_resolved = inspect_change_status(&bob_repo, &bob_profile.public_key, CHANGE_ID)?;
    assert_same_typed_state(&alice_resolved, &bob_resolved)?;
    assert!(!inspect_change_cob_conflicts(&alice_resolved.state)?.has_conflicts());
    assert_eq!(
        change_cob_content_root(&alice_resolved.state)?,
        resolution.expected_content_root
    );
    assert_eq!(
        alice_resolved
            .state
            .artifacts
            .get(&ChangeArtifactId::proposal())
            .and_then(|revisions| revisions.revisions.values().find(|revision| {
                revision.value.as_deref() == Some(ALICE_PROPOSAL)
                    && revision.basis == expected_bases
            }))
            .and_then(|revision| revision.value.as_deref()),
        Some(ALICE_PROPOSAL)
    );
    Ok(())
}
