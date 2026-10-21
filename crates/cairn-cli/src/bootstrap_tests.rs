use super::*;
use std::fs;
mod material {
    include!(env!("CAIRN_BOOTSTRAP_TEST_MATERIAL"));
}

// r[verify cairn.consumer_bootstrap.publication]
// r[verify cairn.consumer_bootstrap.boundary]
#[test]
fn locked_recheck_rejects_substitution_before_publication() -> Result<(), Box<dyn Error>> {
    let temp =
        std::env::temp_dir().join(format!("cairn-bootstrap-lock-drift-{}", std::process::id()));
    fs::create_dir(&temp)?;
    let sources = temp.join("sources");
    fs::create_dir(&sources)?;
    let policy = fs::read(
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../cairn-policy/generated/cairn-policy.json"),
    )?;
    let bundle = material::bundle_material(&sources, policy, &fs::read(std::env::current_exe()?)?)?;
    let original_bundle = serde_json::to_vec(&bundle)?;
    for scenario in ["unchanged", "source", "destination", "symlink", "root"] {
        let root = temp.join(scenario);
        fs::create_dir(&root)?;
        fs::write(sources.join("bundle.json"), &original_bundle)?;
        let args = vec![
            "apply".into(),
            "--root".into(),
            root.display().to_string(),
            "--source-root".into(),
            sources.display().to_string(),
            "--bundle".into(),
            "bundle.json".into(),
        ];
        let preview = prepare(&open_root(&root)?, &options(&args)?)?;
        let mut execute = args;
        execute.extend([
            "--execute".into(),
            "--plan-hash".into(),
            preview.record["plan_hash"]
                .as_str()
                .ok_or("plan hash")?
                .into(),
        ]);
        let displaced = temp.join("displaced");
        let outcome = run_with(execute, || {
            assert!(root.join(publication::LOCK).is_file());
            assert!(!root.join(PREPARED).exists());
            match scenario {
                "source" => fs::write(
                    sources.join("bundle.json"),
                    serde_json::to_vec_pretty(&bundle)?,
                )?,
                "destination" | "symlink" => {
                    fs::create_dir_all(root.join(".cairn/bootstrap"))?;
                    if scenario == "destination" {
                        fs::write(
                            root.join(material::POLICY_DESTINATION),
                            b"unowned substitution",
                        )?;
                    } else {
                        std::os::unix::fs::symlink(
                            sources.join("policy.json"),
                            root.join(material::POLICY_DESTINATION),
                        )?;
                    }
                }
                "root" => {
                    fs::rename(&root, &displaced)?;
                    fs::create_dir(&root)?;
                }
                _ => {}
            }
            Ok(())
        });
        if scenario == "unchanged" {
            outcome?;
            assert!(state::load(&open_root(&root)?)?.is_some());
            assert!(!root.join(PREPARED).exists());
            assert!(!root.join(publication::LOCK).exists());
            continue;
        }
        let error = outcome.expect_err("substitution must block").to_string();
        let expected = match scenario {
            "source" => "bootstrap.setup.stale_plan",
            "destination" => "Unmanaged",
            "symlink" => "bootstrap.path.invalid_file",
            "root" => "bootstrap.setup.root_substituted",
            _ => unreachable!(),
        };
        assert!(error.contains(expected), "{scenario}: {error}");
        let retained = if scenario == "root" {
            &displaced
        } else {
            &root
        };
        assert!(retained.join(publication::LOCK).is_file());
        for locator in [PREPARED, COMPLETE, SELECTION] {
            assert!(!root.join(locator).exists(), "{scenario}: {locator}");
            assert!(!retained.join(locator).exists(), "{scenario}: {locator}");
        }
        if scenario == "destination" {
            assert_eq!(
                fs::read(root.join(material::POLICY_DESTINATION))?,
                b"unowned substitution"
            );
        }
        if scenario == "symlink" {
            assert!(fs::symlink_metadata(root.join(material::POLICY_DESTINATION))?.is_symlink());
        }
        if scenario == "root" {
            assert_eq!(fs::read_dir(&root)?.count(), 0);
        }
    }
    fs::remove_dir_all(temp)?;
    Ok(())
}

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
