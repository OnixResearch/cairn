use super::*;

pub(crate) struct EvidenceExportOptions {
    pub(crate) root: PathBuf,
    pub(crate) out: Option<PathBuf>,
    pub(crate) agent_receipt: Option<PathBuf>,
}

pub(super) fn evidence_export_options(
    args: &[String],
) -> Result<EvidenceExportOptions, Box<dyn Error>> {
    let root = read_option(args.to_vec(), "--root", ".");
    let out = read_string_option(args, "--out").map(PathBuf::from);
    let agent_receipt = read_string_option(args, "--agent-receipt").map(PathBuf::from);
    if out.is_none() {
        return Err("evidence export requires --out <dir>".into());
    }
    Ok(EvidenceExportOptions {
        root,
        out,
        agent_receipt,
    })
}
