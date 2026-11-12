use super::*;

pub(crate) fn resolve_under_root(root: &Path, path: std::path::PathBuf) -> std::path::PathBuf {
    if path.is_absolute() {
        path
    } else {
        root.join(path)
    }
}

pub(crate) fn is_safe_relative_reference(path: &str) -> bool {
    let path = Path::new(path);
    !path.as_os_str().is_empty()
        && !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, std::path::Component::Normal(_)))
}

pub(crate) fn safe_relative_path(root: &Path, path: &Path) -> String {
    path.strip_prefix(root)
        .unwrap_or(path)
        .to_string_lossy()
        .replace('\\', "/")
}

pub(crate) fn collect_files(root: &Path) -> std::io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    let mut dirs = vec![root.to_path_buf()];
    while let Some(current_dir) = dirs.pop() {
        let entries = fs::read_dir(&current_dir)?.collect::<Result<Vec<_>, _>>()?;
        files.reserve(entries.len());
        dirs.reserve(entries.len());
        for entry in entries {
            let path = entry.path();
            if entry.file_type()?.is_dir() {
                dirs.push(path);
            } else {
                files.push(path);
            }
        }
    }
    files.sort();
    Ok(files)
}

pub(crate) fn portable_json_hash(root: &Path, value: &Value) -> String {
    canonical_json_hash(&portable_json(root, value))
}

pub(crate) fn portable_json(root: &Path, value: &Value) -> Value {
    match value {
        Value::Array(items) => {
            Value::Array(items.iter().map(|item| portable_json(root, item)).collect())
        }
        Value::Object(object) => Value::Object(
            object
                .iter()
                .map(|(key, value)| (key.clone(), portable_json(root, value)))
                .collect(),
        ),
        Value::String(text) => Value::String(portable_text(root, text)),
        other => other.clone(),
    }
}

pub(crate) fn portable_text(root: &Path, text: &str) -> String {
    let mut normalized = text.to_string();
    let root_text = path_string(root);
    if !root_text.is_empty() && root_text != "." {
        normalized = normalized.replace(&root_text, ".");
    }
    if let Ok(canonical_root) = root.canonicalize() {
        let canonical_text = path_string(canonical_root);
        if !canonical_text.is_empty() && canonical_text != "." {
            normalized = normalized.replace(&canonical_text, ".");
        }
    }
    normalized
}

pub(crate) fn file_content_manifest(root: &Path, paths: &[PathBuf]) -> Value {
    let mut unique = paths.to_vec();
    unique.sort();
    unique.dedup();
    let entries = unique
        .iter()
        .map(|path| {
            let exists = path.exists();
            let content_hash = if exists {
                fs::read(path).ok().map(|bytes| blake3_hex(&bytes))
            } else {
                None
            };
            json!({
                "path": portable_text(root, &path_string(path)),
                "exists": exists,
                "content_hash": content_hash,
            })
        })
        .collect::<Vec<_>>();
    let manifest = json!({"entries": entries});
    json!({
        "entries": manifest["entries"].clone(),
        "manifest_hash": portable_json_hash(root, &manifest),
    })
}

pub(crate) fn tree_content_paths(root: &Path, dir: &Path) -> std::io::Result<Vec<PathBuf>> {
    if !dir.exists() {
        return Ok(vec![dir.to_path_buf()]);
    }
    let mut files = collect_files(dir)?;
    if files.is_empty() {
        files.push(dir.to_path_buf());
    }
    Ok(files
        .into_iter()
        .map(|path| root.join(path.strip_prefix(root).unwrap_or(&path)))
        .collect())
}

pub(crate) fn read_bounded_text_no_symlink(
    path: &Path,
    max_bytes: u64,
    label: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    let metadata = std::fs::symlink_metadata(path)?;
    if metadata.file_type().is_symlink() {
        return Err(format!("{label} {} must not be a symlink", path.display()).into());
    }
    if metadata.len() > max_bytes {
        return Err(format!("{label} exceeds the reviewed byte bound").into());
    }
    Ok(std::fs::read_to_string(path)?)
}

pub(crate) fn read_existing_bounded_text_no_symlink(
    path: &Path,
    max_bytes: u64,
    label: &str,
) -> Result<String, Box<dyn std::error::Error>> {
    match std::fs::symlink_metadata(path) {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(String::new()),
        Err(error) => Err(error.into()),
        Ok(_) => read_bounded_text_no_symlink(path, max_bytes, label),
    }
}

pub(crate) fn read_bounded_json_no_symlink(
    path: &Path,
    max_bytes: u64,
    label: &str,
) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let text = read_bounded_text_no_symlink(path, max_bytes, label)?;
    Ok(serde_json::from_str(&text)?)
}

pub(crate) fn path_string(path: impl Into<PathBuf>) -> String {
    path.into().display().to_string()
}

#[cfg(test)]
mod tests {
    use super::{read_bounded_json_no_symlink, resolve_under_root};
    use std::path::{Path, PathBuf};

    #[test]
    fn resolves_relative_paths_under_root() {
        assert_eq!(
            resolve_under_root(Path::new("/repo"), PathBuf::from("ledger.json")),
            PathBuf::from("/repo/ledger.json")
        );
    }

    #[test]
    fn keeps_absolute_paths() {
        assert_eq!(
            resolve_under_root(Path::new("/repo"), PathBuf::from("/tmp/ledger.json")),
            PathBuf::from("/tmp/ledger.json")
        );
    }

    fn test_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "cairn-json-reader-{}-{name}.json",
            std::process::id()
        ))
    }

    #[test]
    fn parses_bounded_json() {
        let path = test_path("valid");
        std::fs::write(&path, br#"{"a":1}"#).unwrap();
        let value = read_bounded_json_no_symlink(&path, 1024, "test input").unwrap();
        assert_eq!(value["a"], 1);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn rejects_oversized_json() {
        let path = test_path("oversized");
        let file = std::fs::File::create(&path).unwrap();
        file.set_len(11).unwrap();
        let error = read_bounded_json_no_symlink(&path, 10, "test input").unwrap_err();
        assert!(error
            .to_string()
            .contains("exceeds the reviewed byte bound"));
        std::fs::remove_file(path).unwrap();
    }

    #[cfg(unix)]
    #[test]
    fn rejects_symlinked_json() {
        use std::os::unix::fs::symlink;

        let outside = test_path("outside");
        let link = test_path("symlink");
        std::fs::write(&outside, br#"{"a":1}"#).unwrap();
        symlink(&outside, &link).unwrap();
        let error = read_bounded_json_no_symlink(&link, 1024, "test input").unwrap_err();
        assert!(error.to_string().contains("must not be a symlink"));
        std::fs::remove_file(link).unwrap();
        std::fs::remove_file(outside).unwrap();
    }
}
