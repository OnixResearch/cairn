use super::*;
use std::{error::Error, fs, path::PathBuf};

fn fixture(name: &str) -> io::Result<PathBuf> {
    let root = std::env::temp_dir().join(format!("cairn-source-{name}-{}", std::process::id()));
    fs::create_dir_all(&root)?;
    Ok(root)
}

#[test]
fn source_reads_enforce_file_and_aggregate_limits() -> Result<(), Box<dyn Error>> {
    let root = fixture("bytes")?;
    fs::write(root.join("source"), vec![b'a'; MAX_SOURCE_FILE_BYTES])?;
    let mut reader = SourceReader::open(&root)?;
    assert_eq!(
        reader.read_to_string(Path::new("source"))?.len(),
        MAX_SOURCE_FILE_BYTES
    );
    reader.remaining_bytes = MAX_SOURCE_FILE_BYTES;
    assert!(reader.read_to_string(Path::new("source")).is_ok());
    assert!(reader.read_to_string(Path::new("source")).is_err());
    fs::write(root.join("source"), vec![b'a'; MAX_SOURCE_FILE_BYTES + 1])?;
    assert!(SourceReader::open(&root)?
        .read_to_string(Path::new("source"))
        .is_err());
    fs::write(root.join("source"), [0xff])?;
    assert!(SourceReader::open(&root)?
        .read_to_string(Path::new("source"))
        .is_err());
    fs::remove_dir_all(root)?;
    Ok(())
}

#[test]
fn source_traversal_enforces_aggregate_entries_and_depth() -> Result<(), Box<dyn Error>> {
    let root = fixture("entries")?;
    fs::create_dir_all(root.join("first"))?;
    fs::create_dir_all(root.join("second"))?;
    fs::write(root.join("first/spec.md"), "first")?;
    fs::write(root.join("second/spec.md"), "second")?;
    let mut reader = SourceReader::open(&root)?;
    reader.remaining_entries = 1;
    assert_eq!(reader.entries(Path::new("first"))?.len(), 1);
    assert!(reader.entries(Path::new("second")).is_err());
    let admitted = PathBuf::from(vec!["d"; MAX_SOURCE_DEPTH].join("/"));
    let denied = admitted.join("d");
    fs::create_dir_all(root.join(&denied))?;
    let mut reader = SourceReader::open(&root)?;
    assert!(reader.entries(&admitted).is_ok());
    assert!(reader.entries(&denied).is_err());
    fs::remove_dir_all(root)?;
    Ok(())
}

#[cfg(unix)]
#[test]
fn source_reads_and_traversal_reject_linked_ancestors_and_leaves() -> Result<(), Box<dyn Error>> {
    use std::os::unix::fs::symlink;
    let root = fixture("links")?;
    fs::create_dir_all(root.join("real/nested"))?;
    fs::write(root.join("real/nested/spec.md"), "safe")?;
    symlink("real", root.join("alias"))?;
    symlink("missing", root.join("dangling"))?;
    let mut reader = SourceReader::open(&root)?;
    assert_eq!(
        reader.read_to_string(Path::new("real/nested/spec.md"))?,
        "safe"
    );
    assert!(reader.entries(Path::new("real/nested")).is_ok());
    for path in ["alias/nested/spec.md", "dangling", "real/nested"] {
        assert!(reader.read_to_string(Path::new(path)).is_err(), "{path}");
        assert!(reader.metadata(Path::new(path)).is_err(), "{path}");
    }
    for path in ["alias", "alias/nested", "dangling", "."] {
        assert!(reader.entries(Path::new(path)).is_err(), "{path}");
    }
    fs::remove_dir_all(root)?;
    Ok(())
}
