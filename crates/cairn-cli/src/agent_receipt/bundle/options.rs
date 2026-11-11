use std::{error::Error, path::PathBuf};

use crate::support::{read_option, reject_unknown_options, resolve_under_root};

pub(super) struct AgentEvidenceBundleOptions {
    pub(super) root: PathBuf,
    pub(super) path: PathBuf,
}

pub(super) fn agent_evidence_bundle_options(
    args: &[String],
) -> Result<AgentEvidenceBundleOptions, Box<dyn Error>> {
    reject_unknown_options(args, &["--root"])?;
    let root = read_option(args.to_vec(), "--root", ".");
    let mut path = None;
    let mut index = 0;
    while index < args.len() {
        if args[index] == "--root" {
            index += 2;
            continue;
        }
        if args[index].is_empty() {
            return Err("agent-receipt verify-bundle requires <bundle.json>".into());
        }
        if path.replace(PathBuf::from(&args[index])).is_some() {
            return Err("agent-receipt verify-bundle accepts exactly one bundle path".into());
        }
        index += 1;
    }
    let path = path.ok_or("agent-receipt verify-bundle requires <bundle.json>")?;
    Ok(AgentEvidenceBundleOptions {
        root: root.clone(),
        path: resolve_under_root(&root, path),
    })
}
