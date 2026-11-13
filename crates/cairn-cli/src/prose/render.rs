//! Offline presentation of advisory reports; never applies or publishes suggestions.
use std::{borrow::Cow, collections::HashMap, fmt::Write as _};

use serde_json::{json, Value};

pub(super) fn render(report: &Value, format: &str) -> Result<String, String> {
    if !matches!(format, "text" | "github" | "sarif") {
        return Err(format!("unsupported prose report format: {format}"));
    }
    if report.get("schema_version").and_then(Value::as_str) != Some("cairn.prose-check.v2")
        || report.get("advisory").and_then(Value::as_bool) != Some(true)
        || report.get("mutated").and_then(Value::as_bool) != Some(false)
    {
        return Err("presentation requires an advisory, unmutated prose v2 report".into());
    }
    // Validate once before constructing output, including positions used by all formats.
    for file in array(report, "files")? {
        string(file, "path")?;
        string(file, "content_blake3")?;
        for diagnostic in array(file, "diagnostics")? {
            string(diagnostic, "message")?;
            string(diagnostic, "level")?;
            for label in array(diagnostic, "labels")? {
                region(label)?;
            }
            for help in array(diagnostic, "helps")? {
                string(help, "message")?;
                if let Some(suggestion) = help.get("suggestion").filter(|v| !v.is_null()) {
                    region(suggestion)?;
                    string(suggestion, "replacement")?;
                    if suggestion.get("review_required").and_then(Value::as_bool) != Some(true) {
                        return Err("prose suggestions must explicitly require review".into());
                    }
                }
            }
        }
    }
    match format {
        "text" => text(report),
        "github" => github(report),
        "sarif" => sarif(report),
        _ => unreachable!(),
    }
}

fn string<'a>(value: &'a Value, key: &str) -> Result<&'a str, String> {
    value
        .get(key)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("invalid prose report: expected string {key}"))
}

fn array<'a>(value: &'a Value, key: &str) -> Result<&'a [Value], String> {
    value
        .get(key)
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .ok_or_else(|| format!("invalid prose report: expected array {key}"))
}

fn rule(diagnostic: &Value) -> &str {
    diagnostic
        .get("code")
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
        .or_else(|| {
            diagnostic
                .get("lint")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
        })
        .unwrap_or("prose")
}

fn primary(diagnostic: &Value) -> Option<&Value> {
    let labels = diagnostic.get("labels")?.as_array()?;
    labels
        .iter()
        .find(|label| label.get("primary").and_then(Value::as_bool) == Some(true))
        .or_else(|| labels.first())
}

struct Region {
    start: u64,
    end: u64,
    line: u64,
    column: u64,
    end_line: u64,
    end_column: u64,
}

impl Region {
    fn sarif(self) -> Value {
        json!({
            "startLine": self.line, "startColumn": self.column,
            "endLine": self.end_line, "endColumn": self.end_column,
            "byteOffset": self.start, "byteLength": self.end - self.start,
        })
    }
}

fn region(span: &Value) -> Result<Region, String> {
    let number = |key: &str| {
        span.get(key)
            .and_then(Value::as_u64)
            .ok_or_else(|| format!("invalid prose report: expected unsigned {key}"))
    };
    let start = number("start")?;
    let end = number("end")?;
    let line = number("line")?;
    let column = number("column")?;
    let end_line = number("end_line")?;
    let end_column = number("end_column")?;
    if end < start
        || line == 0
        || column == 0
        || end_column == 0
        || end_line < line
        || (end_line == line && end_column < column)
    {
        return Err("invalid prose report: inconsistent source region".into());
    }
    Ok(Region {
        start,
        end,
        line,
        column,
        end_line,
        end_column,
    })
}

// Render control characters visibly, including bidi controls and Unicode line separators.
// Newlines from report content must not create terminal lines or workflow commands.
fn unsafe_character(ch: char) -> bool {
    ch.is_control()
        || matches!(ch, '\u{061c}' | '\u{200e}' | '\u{200f}' | '\u{2028}'..='\u{202e}' | '\u{2066}'..='\u{2069}')
}

fn visible(value: &str) -> Cow<'_, str> {
    if !value.chars().any(unsafe_character) {
        return Cow::Borrowed(value);
    }
    let mut output = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '\n' => output.push_str("\\n"),
            '\r' => output.push_str("\\r"),
            '\t' => output.push_str("\\t"),
            ch if unsafe_character(ch) => {
                let _ = write!(output, "\\u{{{:04x}}}", ch as u32);
            }
            ch => output.push(ch),
        }
    }
    Cow::Owned(output)
}

fn summary(report: &Value) -> Result<String, String> {
    let files = array(report, "files")?;
    let findings: usize = files
        .iter()
        .map(|file| file["diagnostics"].as_array().map_or(0, Vec::len))
        .sum();
    Ok(format!(
        "Advisory prose check: {findings} finding(s) in {} file(s). All suggestions require author review; no changes applied. Grammar observations do not determine requirement validity or lifecycle outcomes.\n",
        files.len()
    ))
}

fn identities(report: &Value, output: &mut String) {
    if let Some(hash) = report.get("report_blake3").and_then(Value::as_str) {
        let _ = writeln!(output, "Report BLAKE3: {}", visible(hash));
    }
    if let Some(engine) = report.get("engine") {
        let _ = writeln!(
            output,
            "Engine: {} @ {}",
            visible(engine["name"].as_str().unwrap_or("englishc")),
            visible(engine["revision"].as_str().unwrap_or(""))
        );
    }
    if let Some(profile) = report.get("profile") {
        let _ = writeln!(
            output,
            "Profile: {} (BLAKE3: {})",
            visible(profile["name"].as_str().unwrap_or("")),
            visible(profile["blake3"].as_str().unwrap_or(""))
        );
    }
}

fn context(diagnostic: &Value, output: &mut String, terminal: bool) {
    if let Some(id) = diagnostic.get("requirement_id").and_then(Value::as_str) {
        let _ = write!(output, "\nRequirement: {}", presentation(id, terminal));
    }
    if let Some(risks) = diagnostic.get("review_risks").and_then(Value::as_array) {
        for risk in risks.iter().filter_map(Value::as_str) {
            let _ = write!(output, "\nReview risk: {}", presentation(risk, terminal));
        }
    }
    if let Some(notes) = diagnostic.get("notes").and_then(Value::as_array) {
        for note in notes.iter().filter_map(Value::as_str) {
            let _ = write!(output, "\nNote: {}", presentation(note, terminal));
        }
    }
    if let Some(helps) = diagnostic.get("helps").and_then(Value::as_array) {
        for help in helps {
            let _ = write!(
                output,
                "\nReview help (never applied): {}",
                presentation(help["message"].as_str().unwrap_or(""), terminal)
            );
            if let Some(suggestion) = help.get("suggestion").filter(|v| !v.is_null()) {
                let replacement =
                    presentation(suggestion["replacement"].as_str().unwrap_or(""), terminal);
                let _ = write!(output, "\nReview required: replace bytes {}..{} ({}:{}–{}:{}) with {replacement:?}; not a safe-applicability claim",
                    suggestion["start"], suggestion["end"], suggestion["line"], suggestion["column"], suggestion["end_line"], suggestion["end_column"]);
                append_risks(suggestion, output, terminal);
            }
            append_risks(help, output, terminal);
        }
    }
}

fn presentation(value: &str, terminal: bool) -> Cow<'_, str> {
    if terminal {
        visible(value)
    } else {
        Cow::Borrowed(value)
    }
}

fn append_risks(value: &Value, output: &mut String, terminal: bool) {
    if let Some(risks) = value.get("review_risks").and_then(Value::as_array) {
        for risk in risks.iter().filter_map(Value::as_str) {
            let _ = write!(
                output,
                "\nSuggestion review risk: {}",
                presentation(risk, terminal)
            );
        }
    }
}

fn text(report: &Value) -> Result<String, String> {
    let mut output = summary(report)?;
    identities(report, &mut output);
    for file in array(report, "files")? {
        let path = visible(string(file, "path")?);
        let _ = writeln!(
            output,
            "\nFile: {path} (content BLAKE3: {})",
            visible(string(file, "content_blake3")?)
        );
        for diagnostic in array(file, "diagnostics")? {
            let _ = write!(output, "{path}");
            if let Some(label) = primary(diagnostic) {
                let _ = write!(output, ":{}:{}", label["line"], label["column"]);
            }
            let _ = writeln!(
                output,
                ": advisory {} (original severity: {}): {}",
                visible(rule(diagnostic)),
                visible(string(diagnostic, "level")?),
                visible(string(diagnostic, "message")?)
            );
            for label in array(diagnostic, "labels")? {
                let _ = writeln!(
                    output,
                    "  {}:{}–{}:{} [bytes {}..{}]{}: {}",
                    label["line"],
                    label["column"],
                    label["end_line"],
                    label["end_column"],
                    label["start"],
                    label["end"],
                    if label["primary"].as_bool() == Some(true) {
                        " primary"
                    } else {
                        ""
                    },
                    visible(label["message"].as_str().unwrap_or(""))
                );
                if let Some(excerpt) = label.get("excerpt").and_then(Value::as_str) {
                    let _ = writeln!(output, "    Excerpt: {}", visible(excerpt));
                }
            }
            let mut details = String::new();
            context(diagnostic, &mut details, true);
            for line in details.split('\n').filter(|line| !line.is_empty()) {
                // Context escapes each untrusted value before inserting its own separators.
                let _ = writeln!(output, "  {line}");
            }
        }
    }
    if let Some(non_claims) = report.get("non_claims").and_then(Value::as_array) {
        for claim in non_claims.iter().filter_map(Value::as_str) {
            let _ = writeln!(output, "Non-claim: {}", visible(claim));
        }
    }
    Ok(output)
}

fn github_escape(value: &str, property: bool) -> String {
    let mut output = String::with_capacity(value.len());
    for ch in value.chars() {
        match ch {
            '%' => output.push_str("%25"),
            '\r' => output.push_str("%0D"),
            '\n' => output.push_str("%0A"),
            ':' if property => output.push_str("%3A"),
            ',' if property => output.push_str("%2C"),
            ch if unsafe_character(ch) => {
                let _ = write!(output, "\\u{{{:04x}}}", ch as u32);
            }
            ch => output.push(ch),
        }
    }
    output
}

fn github(report: &Value) -> Result<String, String> {
    let mut output = summary(report)?;
    identities(report, &mut output);
    for file in array(report, "files")? {
        for diagnostic in array(file, "diagnostics")? {
            let _ = write!(
                output,
                "::warning file={}",
                github_escape(string(file, "path")?, true)
            );
            if let Some(label) = primary(diagnostic) {
                let _ = write!(
                    output,
                    ",line={},endLine={}",
                    label["line"], label["end_line"]
                );
                if label["line"] == label["end_line"] {
                    let column = label["column"].as_u64().unwrap_or(1);
                    // Workflow annotation endpoints are inclusive; source spans are exclusive.
                    let end_column = label["end_column"]
                        .as_u64()
                        .unwrap_or(column)
                        .saturating_sub(1)
                        .max(column);
                    let _ = write!(output, ",col={column},endColumn={end_column}");
                }
            }
            let _ = write!(
                output,
                ",title={}::",
                github_escape(&format!("Advisory prose: {}", rule(diagnostic)), true)
            );
            let mut message = format!("{} [advisory; original severity: {}; author review required; never applied]\nContent BLAKE3: {}", string(diagnostic, "message")?, string(diagnostic, "level")?, string(file, "content_blake3")?);
            for label in array(diagnostic, "labels")? {
                if let Some(description) = label
                    .get("message")
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty())
                {
                    let _ = write!(
                        message,
                        "\nLabel {}:{}–{}:{}: {description}",
                        label["line"], label["column"], label["end_line"], label["end_column"]
                    );
                }
                if let Some(excerpt) = label.get("excerpt").and_then(Value::as_str) {
                    let _ = write!(message, "\nExcerpt: {excerpt}");
                }
            }
            context(diagnostic, &mut message, false);
            let _ = writeln!(output, "{}", github_escape(&message, false));
        }
    }
    Ok(output)
}

fn artifact_uri(path: &str) -> String {
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut output = String::with_capacity(path.len());
    // Report paths are root-relative. Encoding bytewise also preserves literal backslashes,
    // percent signs, URI delimiters, spaces and every UTF-8 code unit without reinterpretation.
    for byte in path.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'.' | b'_' | b'~' | b'/') {
            output.push(char::from(byte));
        } else {
            output.push('%');
            output.push(char::from(HEX[usize::from(byte >> 4)]));
            output.push(char::from(HEX[usize::from(byte & 15)]));
        }
    }
    output
}

fn sarif(report: &Value) -> Result<String, String> {
    let mut artifacts = Vec::new();
    let mut results = Vec::new();
    let mut rules = Vec::new();
    let mut rule_indices = HashMap::new();
    for (artifact_index, file) in array(report, "files")?.iter().enumerate() {
        let uri = artifact_uri(string(file, "path")?);
        artifacts.push(json!({
            "location": {"uri": uri},
            "hashes": {"blake3": string(file, "content_blake3")?},
        }));
        for diagnostic in array(file, "diagnostics")? {
            let id = rule(diagnostic);
            let rule_index = *rule_indices.entry(id).or_insert_with(|| {
                let index = rules.len();
                rules.push(json!({
                    "id": id,
                    "defaultConfiguration": {"level": "warning"},
                    "properties": {"advisory": true, "reviewRequired": true},
                }));
                index
            });
            let mut locations = Vec::new();
            let preferred = primary(diagnostic);
            // Primary first, then remaining labels in report order; no map-order dependency.
            for label in preferred.into_iter().chain(
                array(diagnostic, "labels")?
                    .iter()
                    .filter(|label| !preferred.is_some_and(|p| std::ptr::eq(p, *label))),
            ) {
                let mut location = json!({
                    "physicalLocation": {
                        "artifactLocation": {"uri": uri, "index": artifact_index},
                        "region": region(label)?.sarif(),
                    },
                });
                if let Some(message) = label
                    .get("message")
                    .and_then(Value::as_str)
                    .filter(|s| !s.is_empty())
                {
                    location["message"] = json!({"text": message});
                }
                locations.push(location);
            }
            let mut review_helps = Vec::new();
            for help in array(diagnostic, "helps")? {
                let mut review_help = json!({
                    "message": string(help, "message")?,
                    "reviewRequired": true,
                    "reviewRisks": help.get("review_risks").cloned().unwrap_or_else(|| json!([])),
                });
                if let Some(suggestion) = help.get("suggestion").filter(|v| !v.is_null()) {
                    review_help["suggestion"] = json!({
                        "region": region(suggestion)?.sarif(),
                        "replacement": string(suggestion, "replacement")?,
                        "reviewRequired": true,
                        "reviewRisks": suggestion.get("review_risks").cloned().unwrap_or_else(|| json!([])),
                    });
                }
                review_helps.push(review_help);
            }
            let upstream_level = string(diagnostic, "level")?;
            results.push(json!({
                "ruleId": id,
                "ruleIndex": rule_index,
                "level": if matches!(upstream_level, "note" | "info" | "help") { "note" } else { "warning" },
                "message": {"text": string(diagnostic, "message")?},
                "locations": locations,
                "properties": {
                    "advisory": true,
                    "upstreamSeverity": upstream_level,
                    "reviewRequired": true,
                    "requirementId": diagnostic.get("requirement_id").cloned().unwrap_or(Value::Null),
                    "reviewRisks": diagnostic.get("review_risks").cloned().unwrap_or_else(|| json!([])),
                    "notes": diagnostic.get("notes").cloned().unwrap_or_else(|| json!([])),
                    "reviewHelps": review_helps,
                },
            }));
        }
    }
    let serialized = serde_json::to_string(&json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {"driver": {
                "name": report["engine"]["name"].as_str().unwrap_or("englishc"),
                "version": report["engine"]["revision"].as_str().unwrap_or(""),
                "rules": rules,
            }},
            "columnKind": "unicodeCodePoints",
            "artifacts": artifacts,
            "results": results,
            "properties": {
                "advisory": true,
                "mutated": false,
                "reviewRequired": true,
                "reportBlake3": report.get("report_blake3"),
                "profile": report.get("profile"),
                "nonClaims": report.get("non_claims"),
            },
        }],
    }))
    .map_err(|error| format!("cannot serialize advisory SARIF: {error}"))?;
    // JSON escaping preserves the original data while preventing raw bidi/C1 controls
    // in terminal consumers. serde_json already escapes ASCII controls and newlines.
    if !serialized.chars().any(unsafe_character) {
        return Ok(serialized);
    }
    let mut escaped = String::with_capacity(serialized.len());
    for ch in serialized.chars() {
        if unsafe_character(ch) {
            for unit in ch.encode_utf16(&mut [0; 2]) {
                let _ = write!(escaped, "\\u{unit:04x}");
            }
        } else {
            escaped.push(ch);
        }
    }
    Ok(escaped)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn report() -> Value {
        json!({
            "schema_version": "cairn.prose-check.v2", "advisory": true, "mutated": false,
            "engine": {"name": "englishc", "revision": "revision"},
            "profile": {"name": "all", "blake3": "profile-hash"},
            "report_blake3": "report-hash", "non_claims": ["No lifecycle authority"],
            "files": [{"path": "specs/é space%#?:,\\.md", "content_blake3": "content-hash", "diagnostics": [{
                "code": "E0001", "lint": "agreement", "level": "error", "message": "Check this wording",
                "labels": [{"start": 7, "end": 12, "line": 2, "column": 3, "end_line": 2, "end_column": 7, "message": "wording", "primary": true}],
                "notes": ["Advisory observation"], "requirement_id": "cairn.sample", "review_risks": ["obligation"],
                "helps": [{"message": "Consider a replacement", "suggestion": {
                    "start": 7, "end": 12, "line": 2, "column": 3, "end_line": 2, "end_column": 7,
                    "replacement": "MUST NOT", "review_required": true, "review_risks": ["negation"]
                }}]
            }]}]
        })
    }

    #[test]
    fn github_escapes_properties_and_untrusted_annotation_commands() {
        let mut report = report();
        report["files"][0]["path"] = json!("a%,:\r\n::error file=other::bad.md");
        report["files"][0]["diagnostics"][0]["message"] =
            json!("100%\r\n::error::injected\u{1b}[31m\u{202e}");
        let output = render(&report, "github").unwrap();
        let command = output
            .lines()
            .find(|line| line.starts_with("::warning "))
            .unwrap();
        assert!(command.contains("file=a%25%2C%3A%0D%0A%3A%3Aerror file=other%3A%3Abad.md"));
        assert!(command.contains("100%25%0D%0A::error::injected\\u{001b}[31m\\u{202e}"));
        assert_eq!(
            output.lines().filter(|line| line.starts_with("::")).count(),
            1
        );
        assert!(!output.contains('\u{1b}'));
        assert!(!output.contains('\r'));
        assert!(command.contains("original severity: error"));
        assert!(!output.lines().any(|line| line.starts_with("::error")));
    }

    #[test]
    fn sarif_preserves_unicode_columns_original_bytes_and_content_identity() {
        let output: Value = serde_json::from_str(&render(&report(), "sarif").unwrap()).unwrap();
        let run = &output["runs"][0];
        assert_eq!(output["version"], "2.1.0");
        assert_eq!(run["columnKind"], "unicodeCodePoints");
        assert_eq!(
            run["artifacts"][0]["location"]["uri"],
            "specs/%C3%A9%20space%25%23%3F%3A%2C%5C.md"
        );
        assert_eq!(run["artifacts"][0]["hashes"]["blake3"], "content-hash");
        assert_eq!(run["properties"]["reportBlake3"], "report-hash");
        let result = &run["results"][0];
        assert_eq!(result["ruleId"], "E0001");
        assert_eq!(result["level"], "warning");
        assert_eq!(result["properties"]["upstreamSeverity"], "error");
        let region = &result["locations"][0]["physicalLocation"]["region"];
        assert_eq!(region["startColumn"], 3);
        assert_eq!(region["endColumn"], 7);
        assert_eq!(region["byteOffset"], 7);
        assert_eq!(region["byteLength"], 5);
        // A consumer must recover the same original bytes and scalar columns,
        // rather than interpreting byte positions as character positions.
        let source = "é\nαβcafé";
        let offset = region["byteOffset"].as_u64().unwrap() as usize;
        let length = region["byteLength"].as_u64().unwrap() as usize;
        assert_eq!(&source[offset..offset + length], "café");
        let line_start = source[..offset].rfind('\n').unwrap() + 1;
        assert_eq!(
            source[line_start..offset].chars().count() + 1,
            region["startColumn"].as_u64().unwrap() as usize
        );
        assert_eq!(
            source[line_start..offset + length].chars().count() + 1,
            region["endColumn"].as_u64().unwrap() as usize
        );
        assert!(region.get("snippet").is_none());
        assert_eq!(result["properties"]["requirementId"], "cairn.sample");
        assert_eq!(result["properties"]["reviewRisks"], json!(["obligation"]));
    }

    #[test]
    fn all_formats_keep_suggestions_review_only() {
        let report = report();
        let text = render(&report, "text").unwrap();
        assert!(text.contains("Review required: replace bytes 7..12"));
        assert!(text.contains("MUST NOT"));
        assert!(text.contains("Requirement: cairn.sample"));
        assert!(text.contains("Suggestion review risk: negation"));
        let github = render(&report, "github").unwrap();
        assert!(github.contains("Review required: replace bytes 7..12"));
        let sarif: Value = serde_json::from_str(&render(&report, "sarif").unwrap()).unwrap();
        let result = &sarif["runs"][0]["results"][0];
        assert!(result.get("fixes").is_none());
        assert_eq!(result["properties"]["reviewRequired"], true);
        assert_eq!(
            result["properties"]["reviewHelps"][0]["suggestion"]["reviewRequired"],
            true
        );
        assert_eq!(
            result["properties"]["reviewHelps"][0]["suggestion"]["replacement"],
            "MUST NOT"
        );
        let mut invalid = report;
        invalid["files"][0]["diagnostics"][0]["helps"][0]["suggestion"]["review_required"] =
            json!(false);
        for format in ["text", "github", "sarif"] {
            assert!(render(&invalid, format).is_err());
        }
    }

    #[test]
    fn primary_location_is_preferred_without_losing_secondary_locations() {
        let mut report = report();
        let labels = report["files"][0]["diagnostics"][0]["labels"]
            .as_array_mut()
            .unwrap();
        let mut secondary = labels[0].clone();
        secondary["primary"] = json!(false);
        secondary["line"] = json!(1);
        secondary["end_line"] = json!(1);
        labels.insert(0, secondary);
        let github = render(&report, "github").unwrap();
        assert!(github.contains(",line=2,endLine=2,col=3,endColumn=6,"));
        let sarif: Value = serde_json::from_str(&render(&report, "sarif").unwrap()).unwrap();
        let locations = sarif["runs"][0]["results"][0]["locations"]
            .as_array()
            .unwrap();
        assert_eq!(locations.len(), 2);
        assert_eq!(locations[0]["physicalLocation"]["region"]["startLine"], 2);
        assert_eq!(locations[1]["physicalLocation"]["region"]["startLine"], 1);
    }

    #[test]
    fn zero_findings_remain_advisory_with_artifact_hashes() {
        let mut report = report();
        report["files"][0]["diagnostics"] = json!([]);
        for format in ["text", "github"] {
            let output = render(&report, format).unwrap();
            assert!(output.contains("0 finding(s) in 1 file(s)"));
            assert!(output.contains("author review"));
            assert!(!output.contains("::warning"));
        }
        let sarif: Value = serde_json::from_str(&render(&report, "sarif").unwrap()).unwrap();
        assert_eq!(sarif["runs"][0]["results"], json!([]));
        assert_eq!(sarif["runs"][0]["tool"]["driver"]["rules"], json!([]));
        assert_eq!(
            sarif["runs"][0]["artifacts"][0]["hashes"]["blake3"],
            "content-hash"
        );
        assert_eq!(sarif["runs"][0]["properties"]["advisory"], true);
    }

    #[test]
    fn text_renders_control_characters_visibly_and_rejects_invalid_positions() {
        let mut report = report();
        report["files"][0]["diagnostics"][0]["message"] =
            json!("word\u{1b}]0;title\u{7}\r\nnext\u{202e}");
        report["files"][0]["diagnostics"][0]["notes"] =
            json!(["note\nRequirement: spoofed\u{9b}31m"]);
        let output = render(&report, "text").unwrap();
        assert!(output.contains("word\\u{001b}]0;title\\u{0007}\\r\\nnext\\u{202e}"));
        assert!(!output.chars().any(|c| c.is_control() && c != '\n'));
        assert!(output.contains("Note: note\\nRequirement: spoofed\\u{009b}31m"));
        assert!(!output.lines().any(|line| line == "  Requirement: spoofed"));
        let sarif = render(&report, "sarif").unwrap();
        assert!(!sarif.chars().any(unsafe_character));
        let decoded: Value = serde_json::from_str(&sarif).unwrap();
        assert_eq!(
            decoded["runs"][0]["results"][0]["message"]["text"],
            report["files"][0]["diagnostics"][0]["message"]
        );
        report["files"][0]["diagnostics"][0]["labels"][0]["column"] = json!(0);
        for format in ["text", "github", "sarif"] {
            assert!(render(&report, format).is_err());
        }
    }

    #[test]
    fn multiline_annotations_omit_ambiguous_columns_and_empty_labels_are_supported() {
        let mut report = report();
        let label = &mut report["files"][0]["diagnostics"][0]["labels"][0];
        label["end_line"] = json!(3);
        label["end_column"] = json!(1);
        let output = render(&report, "github").unwrap();
        let command = output
            .lines()
            .find(|line| line.starts_with("::warning "))
            .unwrap();
        let properties = command
            .strip_prefix("::warning ")
            .unwrap()
            .split("::")
            .next()
            .unwrap();
        assert!(properties.contains(",line=2,endLine=3,"));
        assert!(!properties.contains(",col="));
        assert!(!properties.contains(",endColumn="));
        report["files"][0]["diagnostics"][0]["labels"] = json!([]);
        let output = render(&report, "github").unwrap();
        let command = output
            .lines()
            .find(|line| line.starts_with("::warning "))
            .unwrap();
        assert!(!command.split("::").nth(1).unwrap().contains(",line="));
        let sarif: Value = serde_json::from_str(&render(&report, "sarif").unwrap()).unwrap();
        assert_eq!(sarif["runs"][0]["results"][0]["locations"], json!([]));
    }

    #[test]
    fn sarif_rule_order_and_results_are_deterministic_and_notes_stay_nonblocking() {
        let mut report = report();
        let diagnostics = report["files"][0]["diagnostics"].as_array_mut().unwrap();
        let mut second = diagnostics[0].clone();
        second["code"] = Value::Null;
        second["lint"] = json!("another-lint");
        second["level"] = json!("note");
        diagnostics.push(second.clone());
        diagnostics.push(second);
        let first = render(&report, "sarif").unwrap();
        assert_eq!(first, render(&report, "sarif").unwrap());
        let sarif: Value = serde_json::from_str(&first).unwrap();
        let run = &sarif["runs"][0];
        assert_eq!(run["tool"]["driver"]["rules"].as_array().unwrap().len(), 2);
        assert_eq!(run["tool"]["driver"]["rules"][1]["id"], "another-lint");
        assert_eq!(run["results"][1]["level"], "note");
        assert_eq!(run["results"][2]["ruleIndex"], 1);
    }
}
