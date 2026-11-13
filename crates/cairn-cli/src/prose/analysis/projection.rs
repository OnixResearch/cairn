use std::ops::Range;

pub(super) struct Projection {
    pub text: String,
    pub suppressions: Vec<Suppression>,
}

pub(super) struct Suppression {
    pub range: Range<usize>,
    pub selectors: Vec<String>,
}

#[derive(Clone, Copy)]
enum Block {
    FrontMatter(u8),
    Fence(u8, usize),
    Listing(u8, usize),
}

// Spaces replace every ignored byte, including every byte of a multibyte
// scalar. CR/LF remain untouched: the engine and original document therefore
// share byte offsets and physical line numbers, without a remapping table.
fn mask(bytes: &mut [u8]) {
    for byte in bytes {
        if !matches!(*byte, b'\r' | b'\n') {
            *byte = b' ';
        }
    }
}

fn delimiter(line: &str, byte: u8, minimum: usize) -> Option<usize> {
    let count = line
        .bytes()
        .take_while(|candidate| *candidate == byte)
        .count();
    (count >= minimum && line[count..].trim().is_empty()).then_some(count)
}

fn fence(line: &str) -> Option<(u8, usize)> {
    let indentation = line.bytes().take_while(|byte| *byte == b' ').count();
    if indentation > 3 {
        return None;
    }
    let rest = &line[indentation..];
    let byte = *rest.as_bytes().first()?;
    if !matches!(byte, b'`' | b'~') {
        return None;
    }
    let count = rest
        .bytes()
        .take_while(|candidate| *candidate == byte)
        .count();
    if count < 3 || (byte == b'`' && rest[count..].contains('`')) {
        return None;
    }
    Some((byte, count))
}

pub(super) fn project(source: &str, document_path: &str) -> Result<Projection, String> {
    let mut bytes = source.as_bytes().to_vec();
    let asciidoc = document_path
        .rsplit_once('.')
        .is_some_and(|(_, extension)| {
            extension.eq_ignore_ascii_case("adoc") || extension.eq_ignore_ascii_case("asciidoc")
        });
    let mut listing_attribute = false;
    let mut block = None;
    let mut comment = false;
    let mut inline_ticks = None;
    let mut suppressions = Vec::new();
    let mut offset = 0;
    for (index, segment) in source.split_inclusive('\n').enumerate() {
        let end = offset + segment.len();
        let line = segment.trim_end_matches(['\r', '\n']);
        let trimmed = line.trim();
        if let Some(active) = block {
            mask(&mut bytes[offset..end]);
            let closes = match active {
                Block::FrontMatter(byte) => {
                    trimmed == if byte == b'-' { "---" } else { "+++" }
                        || (byte == b'-' && trimmed == "...")
                }
                Block::Fence(byte, count) => {
                    let indentation = line.bytes().take_while(|byte| *byte == b' ').count();
                    indentation <= 3 && delimiter(&line[indentation..], byte, count).is_some()
                }
                Block::Listing(byte, count) => delimiter(trimmed, byte, count) == Some(count),
            };
            if closes {
                block = None;
            }
        } else if !comment && inline_ticks.is_none() {
            let first = if index == 0 {
                trimmed.strip_prefix('\u{feff}').unwrap_or(trimmed)
            } else {
                trimmed
            };
            let front_matter = index == 0
                && matches!(first, "---" | "+++")
                && source[end..].lines().any(|line| {
                    let delimiter = line.trim();
                    delimiter == first || (first == "---" && delimiter == "...")
                });
            if front_matter {
                block = Some(Block::FrontMatter(first.as_bytes()[0]));
                mask(&mut bytes[offset..end]);
            } else if let Some((byte, count)) = fence(line) {
                listing_attribute = false;
                block = Some(Block::Fence(byte, count));
                mask(&mut bytes[offset..end]);
            } else if let Some((byte, count)) = [b'-', b'.'].into_iter().find_map(|byte| {
                (asciidoc || listing_attribute)
                    .then(|| delimiter(trimmed, byte, 4).map(|count| (byte, count)))
                    .flatten()
            }) {
                block = Some(Block::Listing(byte, count));
                listing_attribute = false;
                mask(&mut bytes[offset..end]);
            } else if is_listing_attribute(trimmed) {
                listing_attribute = true;
                mask(&mut bytes[offset..end]);
            } else {
                if !trimmed.is_empty() {
                    listing_attribute = false;
                }
                if directive_candidate(trimmed) {
                    if suppressions.len() >= 64 {
                        return Err("prose suppression directive limit exceeded (64)".into());
                    }
                    let selectors = directive(trimmed).map_err(|error| {
                        format!("invalid prose suppression on line {}: {error}", index + 1)
                    })?;
                    let next_end = source[end..]
                        .find('\n')
                        .map_or(source.len(), |length| end + length + 1);
                    suppressions.push(Suppression {
                        range: end..next_end,
                        selectors,
                    });
                }
                mask_comments(
                    line,
                    &mut bytes[offset..offset + line.len()],
                    &mut comment,
                    &mut inline_ticks,
                );
            }
        } else {
            mask_comments(
                line,
                &mut bytes[offset..offset + line.len()],
                &mut comment,
                &mut inline_ticks,
            );
        }
        offset = end;
    }
    let text = String::from_utf8(bytes)
        .map_err(|_| "prose document projection did not preserve UTF-8".to_string())?;
    Ok(Projection { text, suppressions })
}

fn is_listing_attribute(line: &str) -> bool {
    let Some(body) = line
        .strip_prefix('[')
        .and_then(|line| line.strip_suffix(']'))
    else {
        return false;
    };
    matches!(
        body.split(',').next().unwrap_or_default().trim(),
        "source" | "listing"
    )
}

fn directive_candidate(line: &str) -> bool {
    line.strip_prefix("<!--")
        .is_some_and(|body| body.trim_start().starts_with("cairn-prose"))
        || line.starts_with("cairn-prose-ignore-next-line")
}

fn directive(line: &str) -> Result<Vec<String>, &'static str> {
    let body = line
        .strip_prefix("<!-- cairn-prose-ignore-next-line ")
        .and_then(|body| body.strip_suffix(" -->"))
        .ok_or("expected an own-line cairn-prose-ignore-next-line HTML comment")?;
    let (selectors, reason) = body
        .split_once(" -- ")
        .ok_or("a reason separated by ` -- ` is required")?;
    let reason = reason.trim();
    if reason.is_empty()
        || reason.len() > 256
        || reason.chars().any(char::is_control)
        || reason.contains("<!--")
        || reason.contains("-->")
    {
        return Err("reason must contain 1 through 256 bytes of plain text");
    }
    let mut result = Vec::new();
    for selector in selectors.split(',') {
        let selector = selector.trim();
        if result.len() >= 16 {
            return Err("at most 16 selectors are allowed");
        }
        if selector.is_empty()
            || (selector != "warnings" && englishc::lints::info(selector).is_none())
        {
            return Err("unknown or empty lint selector");
        }
        result.push(selector.to_string());
    }
    Ok(result)
}

fn mask_comments(
    line: &str,
    output: &mut [u8],
    comment: &mut bool,
    inline_ticks: &mut Option<usize>,
) {
    let bytes = line.as_bytes();
    let mut cursor = 0;
    while cursor < bytes.len() {
        if *comment {
            let end = line[cursor..].find("-->").map(|length| cursor + length + 3);
            let stop = end.unwrap_or(bytes.len());
            mask(&mut output[cursor..stop]);
            cursor = stop;
            if end.is_some() {
                *comment = false;
            }
        } else if bytes[cursor] == b'`' {
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
        } else if inline_ticks.is_none() && bytes[cursor..].starts_with(b"<!--") {
            *comment = true;
        } else {
            // Only ASCII delimiters matter; slicing occurs exclusively at their
            // boundaries, never at an arbitrary non-ASCII byte visited here.
            cursor += 1;
        }
    }
}
