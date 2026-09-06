//! Policy selection retains the CLI's cwd-relative/absolute path meaning.
//! Only the producer read boundary changes; validation remains Cairn's original
//! policy validator. Paths, contents and parser diagnostics never enter errors.
use cairn_core::Policy;
use std::{error::Error, path::Path};

const MAX_POLICY_BYTES: u64 = 4_194_304;
const POLICY_REJECTED: &str =
    "check-evidence policy is unavailable, malformed, or exceeds its bound";

pub(super) fn load(path: &Path) -> Result<Policy, Box<dyn Error>> {
    #[cfg(target_os = "linux")]
    {
        use cap_std::{ambient_authority, fs::Dir};
        let (anchor, relative) = if path.is_absolute() {
            (
                Path::new("/"),
                path.strip_prefix("/").map_err(|_| POLICY_REJECTED)?,
            )
        } else {
            (Path::new("."), path)
        };
        let root =
            Dir::open_ambient_dir(anchor, ambient_authority()).map_err(|_| POLICY_REJECTED)?;
        let bytes =
            super::reads::read(&root, relative, MAX_POLICY_BYTES).map_err(|_| POLICY_REJECTED)?;
        let value = serde_json::from_slice(&bytes).map_err(|_| POLICY_REJECTED)?;
        cairn_core::validate_policy_json(value).map_err(|_| POLICY_REJECTED.into())
    }
    #[cfg(not(target_os = "linux"))]
    {
        let _ = (path, MAX_POLICY_BYTES);
        Err(POLICY_REJECTED.into())
    }
}

#[cfg(all(test, target_os = "linux"))]
mod tests {
    use super::*;
    use std::{fs, os::unix::fs::symlink};

    // r[verify cairn.check_evidence_producer.local_run]
    // r[verify cairn.check_evidence_producer.publication]
    #[test]
    fn policy_meaning_is_unchanged_and_rejections_are_redacted() -> Result<(), Box<dyn Error>> {
        let source = include_bytes!("../../../../cairn-policy/generated/cairn-policy.json");
        let expected = cairn_core::validate_policy_json(serde_json::from_slice(source)?)?;
        let relative = std::path::PathBuf::from(format!(
            "target/cairn-producer-policy-{}",
            std::process::id()
        ));
        let root = std::env::current_dir()?.join(&relative);
        fs::create_dir_all(root.join("nested"))?;
        fs::write(root.join("nested/policy"), source)?;
        assert_eq!(load(&root.join("nested/policy"))?.raw, expected.raw);
        assert_eq!(load(&relative.join("nested/policy"))?.raw, expected.raw);
        symlink("nested/policy", root.join("secret-leaf"))?;
        symlink("nested", root.join("secret-parent"))?;
        fs::write(root.join("invalid-json"), b"credential-secret")?;
        fs::write(root.join("invalid-utf8"), [0xff])?;
        fs::write(
            root.join("invalid-policy"),
            br#"{"schema":"credential-secret"}"#,
        )?;
        fs::File::create(root.join("oversized"))?.set_len(MAX_POLICY_BYTES + 1)?;
        for name in [
            "secret-leaf",
            "secret-parent/policy",
            "invalid-json",
            "invalid-utf8",
            "invalid-policy",
            "oversized",
            "missing-secret",
            "nested",
            "nested/../nested/policy",
        ] {
            let error = match load(&root.join(name)) {
                Ok(_) => panic!("admitted invalid policy"),
                Err(error) => error,
            };
            assert_eq!(error.to_string(), POLICY_REJECTED);
        }
        fs::remove_dir_all(root)?;
        Ok(())
    }
}
