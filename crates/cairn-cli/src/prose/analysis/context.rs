use std::ops::Range;

use cairn_core::{traceability_id_is_valid, traceability_spec_marker};

pub(super) struct Requirement {
    pub range: Range<usize>,
    pub id: Option<String>,
}

struct OpenRequirement {
    range: Range<usize>,
    id: Option<String>,
    declaration_seen: bool,
    body_open: bool,
}

pub(super) fn requirements(projected: &str) -> Vec<Requirement> {
    let mut result = Vec::new();
    let mut current: Option<OpenRequirement> = None;
    let mut offset = 0;
    let mut inline_ticks = None;
    for segment in projected.split_inclusive('\n') {
        let line = segment.trim_end_matches(['\r', '\n']);
        let heading = if inline_ticks.is_none() {
            heading(line)
        } else {
            None
        };
        if let Some((level, title)) = heading {
            if level <= 3 {
                if let Some(mut previous) = current.take() {
                    previous.range.end = offset;
                    result.push(Requirement {
                        range: previous.range,
                        id: previous.id,
                    });
                }
                if level == 3 && title.starts_with("Requirement:") {
                    current = Some(OpenRequirement {
                        range: offset..projected.len(),
                        id: None,
                        declaration_seen: false,
                        body_open: true,
                    });
                }
            } else if let Some(requirement) = &mut current {
                // A scenario or other subheading is not a declaration source.
                requirement.body_open = false;
            }
        }
        let on_requirement_heading =
            heading.is_some_and(|(level, title)| level == 3 && title.starts_with("Requirement:"));
        let eligible = current.as_ref().is_some_and(|requirement| {
            !requirement.declaration_seen
                && requirement.body_open
                && (on_requirement_heading || declaration_line(line))
        });
        let ids = markers(line, &mut inline_ticks, eligible);
        if !ids.is_empty() {
            if let Some(requirement) = &mut current {
                requirement.declaration_seen = true;
                requirement.id = if ids.len() == 1 {
                    ids.into_iter().next()
                } else {
                    None
                };
            }
        }
        offset += segment.len();
    }
    if let Some(requirement) = current {
        result.push(Requirement {
            range: requirement.range,
            id: requirement.id,
        });
    }
    result
}

fn heading(line: &str) -> Option<(usize, &str)> {
    let indent = line.bytes().take_while(|byte| *byte == b' ').count();
    if indent > 3 {
        return None;
    }
    let line = &line[indent..];
    let level = line.bytes().take_while(|byte| *byte == b'#').count();
    let title = line.get(level..)?;
    (level > 0 && level <= 6 && title.starts_with(' ')).then(|| (level, title.trim_start()))
}

fn declaration_line(line: &str) -> bool {
    let indent = line.bytes().take_while(|byte| *byte == b' ').count();
    if indent > 3 {
        return false;
    }
    let line = line.trim_start();
    if line.starts_with('>')
        || ["- ", "* ", "+ ", "["]
            .iter()
            .any(|prefix| line.starts_with(prefix))
    {
        return false;
    }
    // Locate a declaration, not an EARS/requirement-strength verdict. Bare
    // markers may also trail ordinary body prose. Explicit reference prose
    // and code/link examples must not manufacture declaration ownership.
    let first = line
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .trim_end_matches(':');
    ![
        "see",
        "refer",
        "reference",
        "references",
        "example",
        "examples",
    ]
    .iter()
    .any(|word| first.eq_ignore_ascii_case(word))
}

fn markers(line: &str, inline_ticks: &mut Option<usize>, collect: bool) -> Vec<String> {
    let mut result = Vec::new();
    let bytes = line.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() {
        if bytes[cursor] == b'`' {
            let count = bytes[cursor..]
                .iter()
                .take_while(|byte| **byte == b'`')
                .count();
            match *inline_ticks {
                Some(open) if open == count => *inline_ticks = None,
                None => *inline_ticks = Some(count),
                _ => {}
            }
            cursor += count;
        } else if collect && inline_ticks.is_none() && bytes[cursor..].starts_with(b"r[") {
            let previous = cursor
                .checked_sub(1)
                .and_then(|index| bytes.get(index))
                .copied();
            // A marker embedded in a URL, word or link target is a reference.
            let boundary = previous.is_none_or(|byte| {
                byte.is_ascii_whitespace() || matches!(byte, b'(' | b':' | b';')
            });
            if boundary {
                if let Some(id) = traceability_spec_marker(&line[cursor..]) {
                    if traceability_id_is_valid(&id) && !result.contains(&id) {
                        result.push(id);
                    }
                }
            }
            cursor += 2;
        } else {
            cursor += 1;
        }
    }
    result
}
