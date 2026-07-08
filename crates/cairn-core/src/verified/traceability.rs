use std::collections::{BTreeMap, BTreeSet};

pub const TRACEABILITY_PROFILE_CAIRN_DEFAULT: &str = "cairn-default";
pub const TRACEABILITY_PROFILE_TRELLIS_RUST: &str = "trellis-rust";
pub const TRACEABILITY_VERB_IMPL: &str = "impl";
pub const TRACEABILITY_VERB_VERIFY: &str = "verify";
pub const TRACEABILITY_VERB_DEPENDS: &str = "depends";
pub const TRACEABILITY_VERB_RELATED: &str = "related";
pub const TRACEABILITY_COVERAGE_VERBS: &[&str] =
    &[TRACEABILITY_VERB_IMPL, TRACEABILITY_VERB_VERIFY];

const TRACEABILITY_MARKER_PREFIX: &str = "r[";
const TRACEABILITY_MARKER_SUFFIX: char = ']';
const TRACEABILITY_ID_EXTENSION_SEPARATOR: char = '+';
const TRACEABILITY_LINE_NUMBER_ORIGIN: usize = 1;
const TRACEABILITY_PLACEHOLDER_ID: &str = "...";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceabilityProfile {
    pub id: String,
    pub requirement_sources: Vec<TraceabilitySource>,
    pub evidence_sources: Vec<TraceabilitySource>,
    pub marker_verbs: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceabilitySource {
    pub root: String,
    pub source_kind: String,
    pub extensions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceabilityFile {
    pub path: String,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceabilityRequirementOccurrence {
    pub path: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceabilityReferenceOccurrence {
    pub verb: String,
    pub path: String,
    pub line: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceabilityReferenceMarker {
    pub verb: String,
    pub requirement_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraceabilityCoverageReport {
    pub requirements: BTreeMap<String, Vec<TraceabilityRequirementOccurrence>>,
    pub coverage_references: BTreeMap<String, Vec<TraceabilityReferenceOccurrence>>,
    pub all_references: BTreeMap<String, Vec<TraceabilityReferenceOccurrence>>,
    pub missing: Vec<String>,
    pub dangling: Vec<String>,
    pub requirement_count: usize,
    pub referenced_count: usize,
    pub valid: bool,
}

pub fn classify_traceability_coverage(
    profile: &TraceabilityProfile,
    requirement_files: &[TraceabilityFile],
    evidence_files: &[TraceabilityFile],
) -> TraceabilityCoverageReport {
    // r[impl cairn.supersede.traceability_ownership.authoritative_checker]
    let requirements = collect_requirement_markers(requirement_files);
    let all_references = collect_reference_markers(evidence_files, &profile.marker_verbs);
    let coverage_references = coverage_references(&all_references);
    let missing = missing_requirements(&requirements, &coverage_references);
    let dangling = dangling_references(&requirements, &all_references);
    let requirement_count = requirements.len();
    let referenced_count = requirements
        .keys()
        .filter(|requirement_id| coverage_references.contains_key(*requirement_id))
        .count();
    let valid = missing.is_empty() && dangling.is_empty();
    TraceabilityCoverageReport {
        requirements,
        coverage_references,
        all_references,
        missing,
        dangling,
        requirement_count,
        referenced_count,
        valid,
    }
}

pub fn traceability_spec_marker(line: &str) -> Option<String> {
    let rest = line.strip_prefix(TRACEABILITY_MARKER_PREFIX)?;
    let end = rest.find(TRACEABILITY_MARKER_SUFFIX)?;
    let marker = &rest[..end];
    let id = marker
        .split(TRACEABILITY_ID_EXTENSION_SEPARATOR)
        .next()
        .unwrap_or(marker);
    if traceability_id_is_valid(id) {
        Some(id.to_string())
    } else {
        None
    }
}

pub fn traceability_ref_markers(
    line: &str,
    allowed_verbs: &[String],
) -> Vec<TraceabilityReferenceMarker> {
    let mut markers = Vec::new();
    let mut start = 0usize;
    while let Some(offset) = line[start..].find(TRACEABILITY_MARKER_PREFIX) {
        let marker_start = start + offset + TRACEABILITY_MARKER_PREFIX.len();
        let Some(end_offset) = line[marker_start..].find(TRACEABILITY_MARKER_SUFFIX) else {
            break;
        };
        let marker = &line[marker_start..marker_start + end_offset];
        let mut parts = marker.split_whitespace();
        let verb = parts.next().unwrap_or_default();
        let raw_id = parts.next().unwrap_or_default();
        let requirement_id = raw_id
            .split(TRACEABILITY_ID_EXTENSION_SEPARATOR)
            .next()
            .unwrap_or(raw_id);
        if allowed_verbs.iter().any(|allowed| allowed == verb)
            && requirement_id != TRACEABILITY_PLACEHOLDER_ID
            && traceability_id_is_valid(requirement_id)
        {
            markers.push(TraceabilityReferenceMarker {
                verb: verb.to_string(),
                requirement_id: requirement_id.to_string(),
            });
        }
        start = marker_start + end_offset + TRACEABILITY_MARKER_SUFFIX.len_utf8();
    }
    markers
}

pub fn traceability_id_is_valid(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'-'))
}

fn collect_requirement_markers(
    files: &[TraceabilityFile],
) -> BTreeMap<String, Vec<TraceabilityRequirementOccurrence>> {
    let mut requirements = BTreeMap::new();
    for file in files {
        for (line_index, line) in file.text.lines().enumerate() {
            if let Some(requirement_id) = traceability_spec_marker(line.trim()) {
                requirements
                    .entry(requirement_id)
                    .or_insert_with(Vec::new)
                    .push(TraceabilityRequirementOccurrence {
                        path: file.path.clone(),
                        line: line_index + TRACEABILITY_LINE_NUMBER_ORIGIN,
                    });
            }
        }
    }
    requirements
}

fn collect_reference_markers(
    files: &[TraceabilityFile],
    allowed_verbs: &[String],
) -> BTreeMap<String, Vec<TraceabilityReferenceOccurrence>> {
    let mut references = BTreeMap::new();
    for file in files {
        for (line_index, line) in file.text.lines().enumerate() {
            for marker in traceability_ref_markers(line, allowed_verbs) {
                references
                    .entry(marker.requirement_id)
                    .or_insert_with(Vec::new)
                    .push(TraceabilityReferenceOccurrence {
                        verb: marker.verb,
                        path: file.path.clone(),
                        line: line_index + TRACEABILITY_LINE_NUMBER_ORIGIN,
                    });
            }
        }
    }
    references
}

fn coverage_references(
    all_references: &BTreeMap<String, Vec<TraceabilityReferenceOccurrence>>,
) -> BTreeMap<String, Vec<TraceabilityReferenceOccurrence>> {
    all_references
        .iter()
        .filter_map(|(requirement_id, references)| {
            let covering = references
                .iter()
                .filter(|reference| {
                    TRACEABILITY_COVERAGE_VERBS
                        .iter()
                        .any(|verb| *verb == reference.verb)
                })
                .cloned()
                .collect::<Vec<_>>();
            (!covering.is_empty()).then(|| (requirement_id.clone(), covering))
        })
        .collect()
}

fn missing_requirements(
    requirements: &BTreeMap<String, Vec<TraceabilityRequirementOccurrence>>,
    coverage_references: &BTreeMap<String, Vec<TraceabilityReferenceOccurrence>>,
) -> Vec<String> {
    requirements
        .keys()
        .filter(|requirement_id| !coverage_references.contains_key(*requirement_id))
        .cloned()
        .collect()
}

fn dangling_references(
    requirements: &BTreeMap<String, Vec<TraceabilityRequirementOccurrence>>,
    all_references: &BTreeMap<String, Vec<TraceabilityReferenceOccurrence>>,
) -> Vec<String> {
    all_references
        .keys()
        .filter(|requirement_id| !requirements.contains_key(*requirement_id))
        .cloned()
        .collect()
}

pub fn unique_sorted_strings(values: &[String]) -> Vec<String> {
    values
        .iter()
        .cloned()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn profile() -> TraceabilityProfile {
        TraceabilityProfile {
            id: TRACEABILITY_PROFILE_CAIRN_DEFAULT.to_string(),
            requirement_sources: vec![TraceabilitySource {
                root: "cairn/specs".to_string(),
                source_kind: "accepted_specs".to_string(),
                extensions: vec!["md".to_string()],
            }],
            evidence_sources: vec![TraceabilitySource {
                root: "crates".to_string(),
                source_kind: "rust_source".to_string(),
                extensions: vec!["rs".to_string()],
            }],
            marker_verbs: vec![
                TRACEABILITY_VERB_IMPL.to_string(),
                TRACEABILITY_VERB_VERIFY.to_string(),
                TRACEABILITY_VERB_DEPENDS.to_string(),
                TRACEABILITY_VERB_RELATED.to_string(),
            ],
        }
    }

    fn file(path: &str, text: &str) -> TraceabilityFile {
        TraceabilityFile {
            path: path.to_string(),
            text: text.to_string(),
        }
    }

    fn ref_marker(verb: &str, requirement_id: &str) -> String {
        format!("// r[{verb} {requirement_id}]")
    }

    #[test]
    fn complete_impl_or_verify_coverage_passes() {
        // r[verify cairn.supersede.traceability_ownership.authoritative_checker.complete]
        let report = classify_traceability_coverage(
            &profile(),
            &[file(
                "cairn/specs/demo/spec.md",
                "r[cairn.demo.alpha] The system MUST work.\nr[cairn.demo.beta] The system MUST also work.",
            )],
            &[file(
                "crates/demo/src/lib.rs",
                &format!(
                    "{}\n{}",
                    ref_marker(TRACEABILITY_VERB_IMPL, "cairn.demo.alpha"),
                    ref_marker(TRACEABILITY_VERB_VERIFY, "cairn.demo.beta")
                ),
            )],
        );

        assert!(report.valid);
        assert_eq!(report.requirement_count, report.referenced_count);
        assert!(report.missing.is_empty());
        assert!(report.dangling.is_empty());
    }

    #[test]
    fn depends_or_related_does_not_satisfy_required_coverage() {
        // r[verify cairn.supersede.traceability_ownership.authoritative_checker.missing]
        let report = classify_traceability_coverage(
            &profile(),
            &[file(
                "cairn/specs/demo/spec.md",
                "r[cairn.demo.alpha] The system MUST work.",
            )],
            &[file(
                "crates/demo/src/lib.rs",
                &ref_marker(TRACEABILITY_VERB_DEPENDS, "cairn.demo.alpha"),
            )],
        );

        assert!(!report.valid);
        assert_eq!(report.missing, vec!["cairn.demo.alpha".to_string()]);
        assert!(report.dangling.is_empty());
    }

    #[test]
    fn unknown_reference_is_reported_as_dangling() {
        // r[verify cairn.supersede.traceability_ownership.authoritative_checker.dangling]
        let report = classify_traceability_coverage(
            &profile(),
            &[file(
                "cairn/specs/demo/spec.md",
                "r[cairn.demo.alpha] The system MUST work.",
            )],
            &[file(
                "crates/demo/src/lib.rs",
                &format!(
                    "{}\n{}",
                    ref_marker(TRACEABILITY_VERB_IMPL, "cairn.demo.alpha"),
                    ref_marker(TRACEABILITY_VERB_RELATED, "cairn.demo.missing")
                ),
            )],
        );

        assert!(!report.valid);
        assert!(report.missing.is_empty());
        assert_eq!(report.dangling, vec!["cairn.demo.missing".to_string()]);
    }

    #[test]
    fn marker_parser_rejects_placeholders_and_invalid_ids() {
        let verbs = profile().marker_verbs;

        assert_eq!(
            traceability_spec_marker("r[cairn.demo.alpha+meta] text"),
            Some("cairn.demo.alpha".to_string())
        );
        assert_eq!(
            traceability_spec_marker(&format!(
                "r[{} {}]",
                TRACEABILITY_VERB_IMPL, "cairn.demo.alpha"
            )),
            None
        );
        assert!(
            traceability_ref_markers(&ref_marker(TRACEABILITY_VERB_IMPL, "..."), &verbs).is_empty()
        );
        assert!(traceability_ref_markers(
            &ref_marker(TRACEABILITY_VERB_IMPL, "cairn.demo/alpha"),
            &verbs
        )
        .is_empty());
    }
}
