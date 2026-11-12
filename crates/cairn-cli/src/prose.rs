//! Opt-in prose diagnostics. Never rewrites documents or participates in gates.
use std::{error::Error, path::Path};

use serde_json::{json, Value};

use crate::support::{print_json, reject_unknown_options, CapabilityRoot, CapabilityRootKind};

const MAX_FILES: usize = 64;
const MAX_DOCUMENT_BYTES: u64 = 1_048_576;
const ENGINE_REVISION: &str = "336ed696eec73e5b9858c5d7952b54c8e93b9df9";
const NON_CLAIMS: &[&str] = &[
    "prose diagnostics are advisory and do not determine lifecycle gate outcomes",
    "grammar observations do not prove requirement validity, consistency, completeness, or satisfaction",
    "prose diagnostics are not implementation, verification, readiness, or release evidence",
    "all suggestions require author review and are never applied by this command",
];

// r[impl cairn.prose_check.advisory] r[impl cairn.prose_check.inputs]
// r[impl cairn.prose_check.diagnostics] r[impl cairn.prose_check.read_only]
pub(crate) fn run(args: Vec<String>) -> Result<(), Box<dyn Error>> {
    let Some((command, options)) = args.split_first() else {
        return Err("prose requires check [--root DIR] FILE...".into());
    };
    if command != "check" {
        return Err("unknown prose command; use prose check [--root DIR] FILE...".into());
    }
    reject_unknown_options(options, &["--root"])?;
    let mut root_path = Path::new(".");
    let mut paths = Vec::new();
    let mut tokens = options.iter();
    while let Some(token) = tokens.next() {
        if token == "--root" {
            root_path = Path::new(tokens.next().ok_or("--root requires a value")?);
        } else if token.is_empty() || token.starts_with('-') {
            return Err("prose check requires explicit root-relative file paths, not flags".into());
        } else {
            if paths.len() == MAX_FILES {
                return Err("prose check accepts at most 64 files".into());
            }
            paths.push(Path::new(token));
        }
    }
    if paths.is_empty() {
        return Err("prose check requires at least one explicit file path".into());
    }
    let root = CapabilityRoot::open(CapabilityRootKind::Repository, root_path)?;
    let mut files = Vec::with_capacity(paths.len());
    for path in paths {
        let relative = root.relative(path)?;
        let source = String::from_utf8(root.read_bounded(&relative, MAX_DOCUMENT_BYTES)?)?;
        files.push(json!({
            "path": relative,
            "diagnostics": diagnostics(&source),
        }));
    }
    print_json(json!({
        "schema_version": "cairn.prose-check.v1",
        "advisory": true,
        "mutated": false,
        "engine": { "name": "englishc", "revision": ENGINE_REVISION },
        "files": files,
        "non_claims": NON_CLAIMS,
    }))
}

fn diagnostics(source: &str) -> Vec<Value> {
    let diagnostics = englishc::check(source, &englishc::Config::default());
    if diagnostics.is_empty() {
        return Vec::new();
    }
    let line_starts: Vec<_> = std::iter::once(0)
        .chain(source.match_indices('\n').map(|(offset, _)| offset + 1))
        .collect();
    diagnostics
        .into_iter()
        .map(|diagnostic| {
            let labels: Vec<_> = diagnostic
                .labels
                .into_iter()
                .map(|label| {
                    let start = label.span.start;
                    let line = line_starts.partition_point(|&offset| offset <= start);
                    let column = source[line_starts[line - 1]..start].chars().count() + 1;
                    json!({
                        "start": start,
                        "end": label.span.end,
                        "line": line,
                        "column": column,
                        "message": label.message,
                        "primary": label.primary,
                    })
                })
                .collect();
            let helps: Vec<_> = diagnostic
                .helps
                .into_iter()
                .map(|help| {
                    json!({
                        "message": help.message,
                        "suggestion": help.suggestion.map(|suggestion| json!({
                            "start": suggestion.span.start,
                            "end": suggestion.span.end,
                            "replacement": suggestion.replacement,
                            "review_required": true,
                        })),
                    })
                })
                .collect();
            json!({
                "code": diagnostic.code,
                "lint": diagnostic.lint,
                "level": match diagnostic.level {
                    englishc::Level::Error => "error",
                    englishc::Level::Warning => "warning",
                },
                "message": diagnostic.message,
                "labels": labels,
                "notes": diagnostic.notes,
                "helps": helps,
            })
        })
        .collect()
}
