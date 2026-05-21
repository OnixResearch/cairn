use crate::verified::repo::SpecSource;

use super::markdown::{
    has_requirement_strength, has_scenario_clause, markdown_heading, next_heading_at_or_above,
};

pub fn validate_spec_sources(sources: &[SpecSource]) -> Vec<String> {
    let mut ordered = sources.to_vec();
    ordered.sort_by(|left, right| left.path.cmp(&right.path).then(left.id.cmp(&right.id)));
    let mut issues = Vec::with_capacity(ordered.len().saturating_mul(2));
    for source in &ordered {
        issues.extend(validate_spec_source(source));
    }
    issues
}

pub fn validate_spec_source(source: &SpecSource) -> Vec<String> {
    // r[impl cairn.supersede.validate_ears] r[impl cairn.supersede.validate_ears.accept]
    // r[impl cairn.supersede.validate_ears.weak_requirement]
    // r[impl cairn.supersede.validate_ears.incomplete_scenario]
    // r[impl cairn.supersede.validate_ears.boundary]
    let lines = source.markdown.lines().collect::<Vec<_>>();
    let mut issues = Vec::with_capacity(lines.len().saturating_div(4));
    for (index, line) in lines.iter().enumerate() {
        let Some((level, heading)) = markdown_heading(line) else {
            continue;
        };
        if level == 3 && heading.starts_with("Requirement:") {
            issues.extend(validate_requirement_block(source, &lines, index, heading));
        }
        if level == 4 && heading.starts_with("Scenario:") {
            issues.extend(validate_scenario_block(source, &lines, index, heading));
        }
    }
    issues
}

fn validate_requirement_block(
    source: &SpecSource,
    lines: &[&str],
    index: usize,
    heading: &str,
) -> Option<String> {
    let end = next_heading_at_or_above(lines, index + 1, 3);
    if has_requirement_strength(&lines[index + 1..end]) {
        return None;
    }
    Some(format!(
        "{}: {} missing requirement strength (MUST/SHALL/SHOULD/MAY)",
        source.path, heading
    ))
}

fn validate_scenario_block(
    source: &SpecSource,
    lines: &[&str],
    index: usize,
    heading: &str,
) -> Option<String> {
    let end = next_heading_at_or_above(lines, index + 1, 4);
    let block = &lines[index + 1..end];
    let mut missing = Vec::with_capacity(3);
    if !has_scenario_clause(block, "GIVEN") {
        missing.push("GIVEN");
    }
    if !has_scenario_clause(block, "WHEN") {
        missing.push("WHEN");
    }
    if !has_scenario_clause(block, "THEN") {
        missing.push("THEN");
    }
    if missing.is_empty() {
        return None;
    }
    Some(format!(
        "{}: {} missing {} clause(s)",
        source.path,
        heading,
        missing.join("/")
    ))
}
