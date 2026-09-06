use super::*;
use std::fs;

#[test]
fn source_observation_binds_every_read_and_rejects_drift() -> Result<(), Box<dyn Error>> {
    let root = std::env::temp_dir().join(format!(
        "cairn-bootstrap-source-observation-{}",
        std::process::id()
    ));
    fs::create_dir(&root)?;
    let mut sources = Sources {
        root: open_root(&root)?,
        remaining: MAX_TOTAL_BYTES,
        identities: BTreeMap::new(),
    };
    fs::write(root.join("source"), b"first")?;
    assert_eq!(sources.read("source", MAX_ARTIFACT_BYTES)?, b"first");
    assert_eq!(sources.read("source", MAX_ARTIFACT_BYTES)?, b"first");
    assert_eq!(sources.identities.len(), 1);
    assert_eq!(
        sources.identities["source"],
        BootstrapIdentity::from_bytes(b"first")
    );
    fs::write(root.join("source"), b"changed")?;
    assert_eq!(
        sources
            .read("source", MAX_ARTIFACT_BYTES)
            .unwrap_err()
            .to_string(),
        "bootstrap.source.changed_during_preview"
    );
    let mut bounded = Sources {
        root: open_root(&root)?,
        remaining: 0,
        identities: BTreeMap::new(),
    };
    assert!(bounded.read("source", MAX_ARTIFACT_BYTES).is_err());
    assert!(bounded.identities.is_empty());
    fs::remove_dir_all(root)?;
    Ok(())
}
