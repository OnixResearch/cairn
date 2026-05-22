use super::*;

pub(crate) fn execute_sync_plan(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    plan: &SyncPlan,
) -> Result<(), Box<dyn Error>> {
    let delta_root = root.join(changes_dir(layout)).join(change).join("specs");
    let specs_root = root.join(specs_dir(layout));
    for action in &plan.actions {
        let source = PathBuf::from(&action.path);
        let relative = source.strip_prefix(&delta_root).map_err(|_| {
            format!(
                "delta spec {} is outside expected change specs root {}",
                source.display(),
                delta_root.display()
            )
        })?;
        let destination = specs_root.join(relative);
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        let delta_markdown = fs::read_to_string(&source).map_err(|source_error| {
            format!(
                "failed to read delta spec {}: {source_error}",
                source.display()
            )
        })?;
        let accepted_markdown = if destination.exists() {
            Some(fs::read_to_string(&destination).map_err(|source_error| {
                format!(
                    "failed to read accepted spec {}: {source_error}",
                    destination.display()
                )
            })?)
        } else {
            None
        };
        let spec_id = relative
            .parent()
            .map(path_string)
            .unwrap_or_else(|| path_string(relative));
        let merged = merge_delta_spec(accepted_markdown.as_deref(), &delta_markdown, &spec_id);
        fs::write(&destination, merged.markdown).map_err(|source_error| {
            format!(
                "failed to sync {} to {}: {source_error}",
                source.display(),
                destination.display()
            )
        })?;
    }
    Ok(())
}

pub(crate) fn execute_archive_plan(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    change_path: &str,
) -> Result<PathBuf, Box<dyn Error>> {
    let source = PathBuf::from(change_path);
    let archive_date = archive_date()?;
    let destination = root
        .join(archive_dir(layout))
        .join(format!("{archive_date}-{change}"));
    if destination.exists() {
        return Err(format!(
            "archive destination already exists: {}",
            destination.display()
        )
        .into());
    }
    if let Some(parent) = destination.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::rename(&source, &destination).map_err(|source_error| {
        format!(
            "failed to archive {} to {}: {source_error}",
            source.display(),
            destination.display()
        )
    })?;
    Ok(destination)
}

fn archive_date() -> Result<String, Box<dyn Error>> {
    let date = env::var("CAIRN_ARCHIVE_DATE")
        .or_else(|_| env::var("IRONSPEC_ARCHIVE_DATE"))
        .unwrap_or_else(|_| "1970-01-01".to_string());
    if is_archive_date(&date) {
        Ok(date)
    } else {
        Err(format!(
            "invalid archive date: expected YYYY-MM-DD in CAIRN_ARCHIVE_DATE or IRONSPEC_ARCHIVE_DATE, got {date:?}"
        )
        .into())
    }
}

fn is_archive_date(date: &str) -> bool {
    let bytes = date.as_bytes();
    bytes.len() == 10
        && bytes[4] == b'-'
        && bytes[7] == b'-'
        && bytes
            .iter()
            .enumerate()
            .all(|(index, byte)| index == 4 || index == 7 || byte.is_ascii_digit())
}

pub(crate) fn sync_mutation_paths(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    plan: &SyncPlan,
) -> Vec<PathBuf> {
    let delta_root = root.join(changes_dir(layout)).join(change).join("specs");
    let specs_root = root.join(specs_dir(layout));
    plan.actions
        .iter()
        .filter_map(|action| {
            let source = PathBuf::from(&action.path);
            source
                .strip_prefix(&delta_root)
                .ok()
                .map(|relative| specs_root.join(relative))
        })
        .collect()
}

pub(crate) fn sync_mutation_manifest(
    root: &Path,
    layout: LifecycleLayout,
    change: &str,
    plan: &SyncPlan,
    before: &Value,
) -> Value {
    let paths = sync_mutation_paths(root, layout, change, plan);
    let after = file_content_manifest(root, &paths);
    json!({
        "kind": "sync",
        "before": before,
        "after": after,
        "manifest_hash": portable_json_hash(root, &json!({"before": before, "after": after})),
    })
}

pub(crate) fn archive_mutation_manifest(
    root: &Path,
    source: &Path,
    destination: &Path,
    before: &Value,
) -> Value {
    let mut after_paths =
        tree_content_paths(root, destination).unwrap_or_else(|_| vec![destination.to_path_buf()]);
    after_paths.push(source.to_path_buf());
    let after = file_content_manifest(root, &after_paths);
    json!({
        "kind": "archive",
        "before": before,
        "after": after,
        "manifest_hash": portable_json_hash(root, &json!({"before": before, "after": after})),
    })
}
