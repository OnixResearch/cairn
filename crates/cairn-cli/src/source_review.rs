//! Read-only shell for source review. Never accepts private keys or signs.
// r[impl cairn.source_review.shell]
use std::{collections::BTreeMap, error::Error, io::Write, path::Path};

use cairn_core::source_review::{
    self, SignedSourceReview, SourceReviewAttachment, SourceReviewClaim, SourceReviewOutcome,
    SourceReviewSigningRequest, SourceReviewVerificationPolicy, MAX_INPUT_BYTES,
};

use crate::support::{print_json, CapabilityRoot, CapabilityRootKind};

pub(crate) fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    if args.is_empty() || args.len() > 160 {
        return Err("source-review requires signing-request, assemble, or verify with bounded explicit options".into());
    }
    let command = &args[0];
    let allowed: &[&str] = match command.as_str() {
        "signing-request" => &[
            "--root",
            "--claim",
            "--policy",
            "--producer",
            "--key-id",
            "--generation",
            "--public-key-hex",
            "--raw",
        ],
        "assemble" => &[
            "--root",
            "--request",
            "--signature-hex",
            "--policy",
            "--include",
        ],
        "verify" => &["--root", "--attachment", "--policy"],
        _ => return Err("unknown source-review command".into()),
    };
    let mut options = BTreeMap::<&str, &str>::new();
    let mut includes = Vec::new();
    let mut raw = false;
    let mut index = 1;
    while index < args.len() {
        let name = args[index].as_str();
        if !allowed.contains(&name) {
            return Err(format!("unsupported source-review option: {name}").into());
        }
        if name == "--raw" {
            if raw {
                return Err("duplicate --raw".into());
            }
            raw = true;
            index += 1;
            continue;
        }
        let value = args
            .get(index + 1)
            .filter(|value| !value.starts_with("--"))
            .ok_or("source-review option value missing")?;
        if name == "--include" {
            if includes.len() >= source_review::MAX_ROWS {
                return Err("include count exceeded".into());
            }
            includes.push(value.as_str());
        } else if options.insert(name, value.as_str()).is_some() {
            return Err(format!("duplicate source-review option: {name}").into());
        }
        index += 2;
    }
    let required = |name: &str| {
        options
            .get(name)
            .copied()
            .ok_or_else(|| format!("source-review requires {name}"))
    };
    let root = CapabilityRoot::open(
        CapabilityRootKind::LocalState,
        Path::new(options.get("--root").copied().unwrap_or(".")),
    )?;
    let load = |name: &str| root.read_bounded(Path::new(name), MAX_INPUT_BYTES as u64);
    let policy: SourceReviewVerificationPolicy =
        source_review::parse(&load(required("--policy")?)?)?;
    match command.as_str() {
        "signing-request" => {
            let claim: SourceReviewClaim = source_review::parse(&load(required("--claim")?)?)?;
            let request = source_review::signing_request(
                claim,
                &policy,
                required("--producer")?.to_owned(),
                required("--key-id")?.to_owned(),
                required("--generation")?.parse()?,
                required("--public-key-hex")?.to_owned(),
            )?;
            if raw {
                // Emit exactly the canonical bytes, without a trailing newline.
                let bytes = source_review::signing_request_bytes(&request)?;
                std::io::stdout().lock().write_all(&bytes)?;
            } else {
                print_json(serde_json::to_value(request)?)?;
            }
        }
        "assemble" => {
            let request: SourceReviewSigningRequest =
                source_review::parse(&load(required("--request")?)?)?;
            let row =
                source_review::assemble(request, required("--signature-hex")?.to_owned(), &policy)?;
            let mut reviews = vec![row];
            for path in includes {
                let row: SignedSourceReview = source_review::parse(&load(path)?)?;
                reviews.push(row);
            }
            let attachment = SourceReviewAttachment::new(reviews)?;
            // Insufficient attribution is still exportable (for example a rejection).
            source_review::verify(&attachment, &policy)?;
            print_json(serde_json::to_value(attachment)?)?;
        }
        "verify" => {
            let bytes = load(required("--attachment")?)?;
            let attachment: SourceReviewAttachment = source_review::parse(&bytes)?;
            let report = source_review::verify(&attachment, &policy)?;
            let accepted = report.outcome == SourceReviewOutcome::AcceptedManualReview;
            print_json(serde_json::json!({
                "report": report,
                "attachment_artifact_blake3": blake3::hash(&bytes).to_hex().to_string(),
            }))?;
            if !accepted {
                return Err("source-review approval threshold insufficient".into());
            }
        }
        _ => unreachable!("command checked above"),
    }
    Ok(())
}
