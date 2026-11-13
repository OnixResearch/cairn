//! Pure advisory analysis over an original-byte-preserving document projection.
use std::ops::Range;

use englishc::{Diagnostic, Level};
use serde_json::{json, Value};

use super::profiles::Profile;

mod context;
mod projection;
mod risks;

pub(super) fn diagnostics(
    source: &str,
    profile: &Profile,
    enhanced: bool,
    maximum: usize,
    document_path: &str,
) -> Result<Vec<Value>, String> {
    let projection = projection::project(source, document_path)?;
    let mut findings = englishc::check(&projection.text, &englishc::Config::default());
    // Filtering before JSON construction also makes the limit apply to findings
    // actually selected by this profile and the next-physical-line directives.
    findings.retain(|finding| {
        (profile.include_rules.is_empty()
            || profile
                .include_rules
                .iter()
                .any(|selector| matches_selector(finding, selector)))
            && !profile
                .exclude_rules
                .iter()
                .any(|selector| matches_selector(finding, selector))
            && !projection.suppressions.iter().any(|suppression| {
                suppression.range.contains(&finding.span().start)
                    && suppression
                        .selectors
                        .iter()
                        .any(|selector| matches_selector(finding, selector))
            })
    });
    if findings.len() > maximum {
        return Err(format!(
            "prose diagnostic limit exceeded: {} selected findings exceeds {maximum}",
            findings.len()
        ));
    }
    if findings.is_empty() {
        return Ok(Vec::new());
    }
    let positions = Positions::new(source);
    let requirements = if enhanced {
        context::requirements(&projection.text)
    } else {
        Vec::new()
    };
    findings
        .into_iter()
        .map(|finding| diagnostic_json(finding, source, &positions, &requirements, enhanced))
        .collect()
}

fn matches_selector(finding: &Diagnostic, selector: &str) -> bool {
    selector == finding.lint
        || finding.code == Some(selector)
        || (selector == "warnings" && finding.level == Level::Warning)
}

struct Positions {
    starts: Vec<usize>,
}

impl Positions {
    fn new(source: &str) -> Self {
        Self {
            starts: std::iter::once(0)
                .chain(source.match_indices('\n').map(|(offset, _)| offset + 1))
                .collect(),
        }
    }

    fn at(&self, source: &str, offset: usize) -> (usize, usize) {
        let line = self.starts.partition_point(|&start| start <= offset);
        let column = source[self.starts[line - 1]..offset].chars().count() + 1;
        (line, column)
    }
}

fn checked_span<'a>(source: &'a str, span: &Range<usize>) -> Result<&'a str, String> {
    source.get(span.clone()).ok_or_else(|| {
        format!(
            "englishc returned an invalid UTF-8 source span {}..{}",
            span.start, span.end
        )
    })
}

fn diagnostic_json(
    finding: Diagnostic,
    source: &str,
    positions: &Positions,
    requirements: &[context::Requirement],
    enhanced: bool,
) -> Result<Value, String> {
    let primary = finding.span();
    checked_span(source, &primary)?;
    let requirement_id = requirements
        .iter()
        .find(|requirement| {
            requirement.range.contains(&primary.start) && primary.end <= requirement.range.end
        })
        .and_then(|requirement| requirement.id.as_deref());
    let mut labels = Vec::with_capacity(finding.labels.len());
    for label in finding.labels {
        let excerpt = checked_span(source, &label.span)?;
        let (line, column) = positions.at(source, label.span.start);
        let mut value = json!({
            "start": label.span.start,
            "end": label.span.end,
            "line": line,
            "column": column,
            "message": label.message,
            "primary": label.primary,
        });
        if enhanced {
            let (end_line, end_column) = positions.at(source, label.span.end);
            value["end_line"] = json!(end_line);
            value["end_column"] = json!(end_column);
            value["excerpt"] = json!(excerpt.chars().take(200).collect::<String>());
        }
        labels.push(value);
    }
    let mut helps = Vec::with_capacity(finding.helps.len());
    let mut review_risks = Vec::new();
    for help in finding.helps {
        let suggestion = if let Some(suggestion) = help.suggestion {
            let original = checked_span(source, &suggestion.span)?;
            let mut value = json!({
                "start": suggestion.span.start,
                "end": suggestion.span.end,
                "replacement": suggestion.replacement,
                "review_required": true,
            });
            if enhanced {
                let (line, column) = positions.at(source, suggestion.span.start);
                let (end_line, end_column) = positions.at(source, suggestion.span.end);
                let change_risks = risks::review_risks(original, &suggestion.replacement);
                for risk in &change_risks {
                    if !review_risks.contains(risk) {
                        review_risks.push(*risk);
                    }
                }
                value["line"] = json!(line);
                value["column"] = json!(column);
                value["end_line"] = json!(end_line);
                value["end_column"] = json!(end_column);
                value["review_risks"] = json!(change_risks);
            }
            value
        } else {
            Value::Null
        };
        helps.push(json!({"message": help.message, "suggestion": suggestion}));
    }
    let mut value = json!({
        "code": finding.code,
        "lint": finding.lint,
        "level": match finding.level {
            Level::Error => "error",
            Level::Warning => "warning",
        },
        "message": finding.message,
        "labels": labels,
        "notes": finding.notes,
        "helps": helps,
    });
    if enhanced {
        // Keep category order independent of the engine's order of alternatives.
        review_risks.sort_by_key(|risk| risks::CODES.iter().position(|code| code == risk));
        value["requirement_id"] = json!(requirement_id);
        value["review_risks"] = json!(review_risks);
    }
    Ok(value)
}

#[cfg(test)]
mod tests;
