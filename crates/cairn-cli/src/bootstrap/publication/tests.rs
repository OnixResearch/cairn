use super::*;
use std::os::unix::fs::PermissionsExt;
use std::{
    fs,
    path::Path,
    process::Command,
    time::{Duration, Instant},
};
const WAIT_LIMIT: Duration = Duration::from_secs(20);
const POLL: Duration = Duration::from_millis(10);
const CHILD_ROOT: &str = "CAIRN_BOOTSTRAP_KILL_FIXTURE_ROOT";
const READ_ONLY_DIRECTORY: u32 = 0o500;
const WRITABLE_DIRECTORY: u32 = 0o700;
fn write(destination: &str, bytes: &[u8]) -> WriteIntent {
    WriteIntent {
        destination: destination.into(),
        prior: None,
        bytes: bytes.to_vec(),
    }
}
fn prepared() -> Prepared {
    Prepared {
        record: json!({"schema": "fixture", "plan_hash": "fixture", "valid": false}),
        writes: vec![write("artifact", b"payload")],
        complete: write(COMPLETE, b"not yet selected"),
        selection: write(SELECTION, b"not yet selected"),
    }
}

// r[verify cairn.consumer_bootstrap.publication]
#[test]
fn interruption_child() -> Result<(), Box<dyn Error>> {
    let Some(path) = std::env::var_os(CHILD_ROOT) else {
        return Ok(());
    };
    let root = open_root(Path::new(&path))?;
    let _lock = Lock::acquire(&root)?;
    execute_with(&root, &prepared(), |_| {
        root.write("checkpoint", b"artifact published")?;
        std::thread::sleep(WAIT_LIMIT);
        Err("fixture parent did not kill child".into())
    })
}

// r[verify cairn.consumer_bootstrap.publication]
#[test]
fn kill_between_artifact_and_manifest_preserves_recovery_state() -> Result<(), Box<dyn Error>> {
    let path = std::env::temp_dir().join(format!("cairn-bootstrap-kill-{}", std::process::id()));
    fs::create_dir(&path)?;
    let mut child = Command::new(std::env::current_exe()?)
        .args([
            "--exact",
            "bootstrap::publication::tests::interruption_child",
            "--nocapture",
        ])
        .env(CHILD_ROOT, &path)
        .spawn()?;
    let deadline = Instant::now() + WAIT_LIMIT;
    while !path.join("checkpoint").exists() && Instant::now() < deadline {
        if child.try_wait()?.is_some() {
            break;
        }
        std::thread::sleep(POLL);
    }
    let reached = path.join("checkpoint").exists();
    let _ = child.kill();
    let status = child.wait()?;
    assert!(
        reached,
        "child must reach the production artifact/manifest boundary"
    );
    assert!(!status.success());
    assert_eq!(fs::read(path.join("artifact"))?, b"payload");
    assert!(path.join(PREPARED).is_file());
    assert!(path.join(LOCK).is_file());
    assert!(!path.join(COMPLETE).exists());
    assert!(!path.join(SELECTION).exists());
    let root = open_root(&path)?;
    assert!(Lock::acquire(&root).is_err());
    assert_eq!(
        read_required(&root, PREPARED, MAX_BUNDLE_BYTES)?,
        serde_json::to_vec(&prepared().record)?
    );
    fs::remove_dir_all(path)?;
    Ok(())
}

// r[verify cairn.consumer_bootstrap.publication]
#[test]
fn permission_failure_and_unknown_durability_do_not_complete() -> Result<(), Box<dyn Error>> {
    assert!(require_durable(&PublicationDisposition::CommittedDurabilityUnknown).is_err());
    assert!(require_durable(&PublicationDisposition::CommittedAndParentSynchronized).is_ok());
    let path =
        std::env::temp_dir().join(format!("cairn-bootstrap-permission-{}", std::process::id()));
    fs::create_dir(&path)?;
    let root = open_root(&path)?;
    let mut transaction = prepared();
    directory(&root, "denied")?;
    fs::set_permissions(
        path.join("denied"),
        fs::Permissions::from_mode(READ_ONLY_DIRECTORY),
    )?;
    transaction.writes[0].destination = "denied/artifact".into();
    let result = execute(&root, &transaction);
    fs::set_permissions(
        path.join("denied"),
        fs::Permissions::from_mode(WRITABLE_DIRECTORY),
    )?;
    assert!(result.is_err());
    assert!(path.join(PREPARED).exists());
    assert!(!path.join(COMPLETE).exists());
    assert!(!path.join(SELECTION).exists());
    assert!(!path.join("denied/artifact").exists());
    // A failed retry/recovery does not erase the original prepared record.
    let before = fs::read(path.join(PREPARED))?;
    assert!(execute(&root, &transaction).is_err());
    assert_eq!(fs::read(path.join(PREPARED))?, before);
    fs::remove_dir_all(path)?;
    Ok(())
}

// r[verify cairn.consumer_bootstrap.publication]
#[test]
fn returned_uncertainty_after_visible_artifact_blocks_final_selection() -> Result<(), Box<dyn Error>>
{
    let path =
        std::env::temp_dir().join(format!("cairn-bootstrap-uncertain-{}", std::process::id()));
    fs::create_dir(&path)?;
    let root = open_root(&path)?;
    let result = execute_with(&root, &prepared(), |_| {
        require_durable(&PublicationDisposition::CommittedDurabilityUnknown)
    });
    assert!(result
        .unwrap_err()
        .to_string()
        .contains("durability_unknown"));
    assert_eq!(fs::read(path.join("artifact"))?, b"payload");
    assert!(path.join(PREPARED).is_file());
    assert!(!path.join(COMPLETE).exists());
    assert!(!path.join(SELECTION).exists());
    fs::remove_dir_all(path)?;
    Ok(())
}
