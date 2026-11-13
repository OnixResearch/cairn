use super::*;

fn profile(include: &[&str], exclude: &[&str]) -> Profile {
    Profile {
        name: "test".to_string(),
        include_rules: include.iter().map(|value| value.to_string()).collect(),
        exclude_rules: exclude.iter().map(|value| value.to_string()).collect(),
    }
}

fn agreement(source: &str, enhanced: bool, maximum: usize) -> Result<Vec<Value>, String> {
    diagnostics(
        source,
        &profile(&["E0001"], &[]),
        enhanced,
        maximum,
        "notes.md",
    )
}

fn keys(value: &Value) -> Vec<&str> {
    let mut keys: Vec<_> = value
        .as_object()
        .unwrap()
        .keys()
        .map(String::as_str)
        .collect();
    keys.sort_unstable();
    keys
}

#[test]
fn legacy_shape_retains_only_the_original_diagnostic_fields() {
    let rows = agreement("He are late.\n", false, 1).unwrap();
    assert_eq!(rows.len(), 1);
    let finding = &rows[0];
    assert_eq!(
        keys(finding),
        ["code", "helps", "labels", "level", "lint", "message", "notes"]
    );
    assert_eq!(finding["code"], "E0001");
    assert_eq!(finding["level"], "error");
    for label in finding["labels"].as_array().unwrap() {
        assert_eq!(
            keys(label),
            ["column", "end", "line", "message", "primary", "start"]
        );
    }
    let helps = finding["helps"].as_array().unwrap();
    assert!(helps.iter().any(|help| help["suggestion"]
        == json!({
            "start": 3, "end": 6, "replacement": "is", "review_required": true,
        })));
    for help in helps {
        assert_eq!(keys(help), ["message", "suggestion"]);
    }
}

#[test]
fn technical_document_projection_preserves_utf8_and_physical_positions() {
    let ignored = [
        "---\ntitle: He are late. 🙂\n#![allow(E0001)]\n---\n",
        "+++\ntitle = 'He are late. 🙂'\n#![allow(E0001)]\n+++\n",
        "~~~~text\nHe are late. 🙂\n#![allow(E0001)]\n<!-- cairn-prose-ignore-next-line BAD -->\n~~~~\n",
        "````text\r\nHe are late. 🙂\r\n```\r\n#![allow(E0001)]\r\n````\r\n",
        "<!--\nHe are late. 🙂\n#![allow(E0001)]\n-->\n",
        "[source,rust]\n----\nHe are late. 🙂\n#![allow(E0001)]\n----\n",
        "[listing]\n....\nHe are late. 🙂\n#![allow(E0001)]\n....\n",
    ];
    for prefix in ignored {
        let source = format!("{prefix}🙂 He are late.\n");
        let projected = projection::project(&source, "notes.md").unwrap();
        assert_eq!(projected.text.len(), source.len());
        assert_eq!(
            projected
                .text
                .match_indices('\n')
                .map(|(offset, _)| offset)
                .collect::<Vec<_>>(),
            source
                .match_indices('\n')
                .map(|(offset, _)| offset)
                .collect::<Vec<_>>()
        );
        let rows = agreement(&source, true, 1).unwrap();
        assert_eq!(rows.len(), 1, "{source}");
        let label = rows[0]["labels"]
            .as_array()
            .unwrap()
            .iter()
            .find(|label| label["primary"] == true)
            .unwrap();
        let start = source.rfind("are").unwrap();
        assert_eq!(label["start"], start);
        assert_eq!(label["end"], start + 3);
        assert_eq!(label["column"], 6);
        assert_eq!(label["end_column"], 9);
        assert_eq!(label["line"], prefix.matches('\n').count() + 1);
        assert_eq!(label["excerpt"], "are");
    }
}

#[test]
fn inline_code_urls_and_existing_upstream_attributes_keep_their_behavior() {
    let source = "`<!-- not a real comment`\n`He are late.` https://example.invalid/He-are\n#[allow(E0001)]\nHe are late.\nHe are late.\n";
    let rows = agreement(source, false, 1).unwrap();
    assert_eq!(rows.len(), 1);
    assert!(rows[0]["labels"]
        .as_array()
        .unwrap()
        .iter()
        .any(|label| { label["primary"] == true && label["line"] == 5 }));
    assert!(agreement("#![allow(E0001)]\nHe are late.\n", false, 0)
        .unwrap()
        .is_empty());
}

#[test]
fn next_line_directives_are_narrow_and_shared_by_report_versions() {
    let directive = "<!-- cairn-prose-ignore-next-line E0001,indefinite_article -- Intentional technical subject. -->\n";
    let source = format!("{directive}He are late.\nHe are late.\n");
    for enhanced in [false, true] {
        let rows = agreement(&source, enhanced, 1).unwrap();
        assert_eq!(rows.len(), 1);
        assert!(rows[0]["labels"]
            .as_array()
            .unwrap()
            .iter()
            .any(|label| { label["primary"] == true && label["line"] == 3 }));
        let blank_next_line = format!("{directive}\nHe are late.\n");
        assert_eq!(agreement(&blank_next_line, enhanced, 1).unwrap().len(), 1);
    }
}

#[test]
fn malformed_unknown_and_unbounded_directives_fail_without_findings() {
    let invalid = [
        "<!-- cairn-prose-ignore-next-line E0001 -->".to_string(),
        "<!-- cairn-prose-ignore-next-line UNKNOWN -- Intentional. -->".to_string(),
        "<!-- cairn-prose-ignore-next-line E0001 --  -->".to_string(),
        "<!--  cairn-prose-ignore-next-line E0001 -- Intentional. -->".to_string(),
        "<!-- cairn-prose-ignore-next-line E0001 -- Intentional. --> trailing".to_string(),
        format!(
            "<!-- cairn-prose-ignore-next-line E0001 -- {} -->",
            "é".repeat(129)
        ),
        format!(
            "<!-- cairn-prose-ignore-next-line {} -- Intentional. -->",
            vec!["E0001"; 17].join(",")
        ),
        "<!-- cairn-prose-ignore-next-line E0001 -- Intentional. -->\n".repeat(65),
    ];
    for source in invalid {
        assert!(agreement(&source, false, 10).is_err(), "{source}");
    }
    let boundary = format!(
        "<!-- cairn-prose-ignore-next-line E0001 -- {} -->\n",
        "é".repeat(128)
    );
    assert!(agreement(&boundary.repeat(64), true, 0).unwrap().is_empty());
    let source =
        "<!--\n<!-- cairn-prose-ignore-next-line UNKNOWN -- Not active. -->\nHe are late.\n";
    assert_eq!(agreement(source, false, 1).unwrap().len(), 1);
}

#[test]
fn profile_selection_preserves_severity_and_limits_only_selected_findings() {
    let source = "He are late.\nA apple.\nThe the cat is here.\n";
    let rows = diagnostics(
        source,
        &profile(&["subject_verb_agreement"], &[]),
        false,
        1,
        "notes.md",
    )
    .unwrap();
    assert_eq!(rows.len(), 1);
    assert_eq!(rows[0]["code"], "E0001");
    assert!(diagnostics(
        source,
        &profile(&["E0001"], &["subject_verb_agreement"]),
        false,
        0,
        "notes.md"
    )
    .unwrap()
    .is_empty());
    let warnings =
        diagnostics(source, &profile(&["warnings"], &[]), false, 10, "notes.md").unwrap();
    assert!(warnings
        .iter()
        .any(|row| row["lint"] == "repeated_word" && row["level"] == "warning"));
    assert!(warnings
        .iter()
        .all(|row| row["level"] == "warning" && row["code"].is_null()));
    assert!(diagnostics(
        source,
        &profile(&[], &["E0001", "E0002", "warnings"]),
        false,
        0,
        "notes.md"
    )
    .unwrap()
    .is_empty());
    assert!(agreement("He are late.\nHe are late.\n", false, 1).is_err());
    assert_eq!(
        agreement("He are late.\nHe are late.\n", true, 2)
            .unwrap()
            .len(),
        2
    );
}

#[test]
fn requirement_context_is_contained_and_does_not_use_examples_or_references() {
    let source = "### Requirement: First r[demo.first]\nHe are late.\n#### Scenario: Child\nHe are late.\n### Other heading\nHe are late.\n### Requirement: Body identity\nSee r[demo.reference].\n`r[demo.example]`\nr[demo.body] The system MUST keep this text.\nHe are late.\n### Requirement: Ambiguous r[demo.one] r[demo.two]\nHe are late.\n## Outside\nHe are late.\n";
    let rows = agreement(source, true, 6).unwrap();
    assert_eq!(rows.len(), 6);
    let ids: Vec<_> = rows
        .iter()
        .map(|row| row["requirement_id"].clone())
        .collect();
    assert_eq!(
        ids,
        vec![
            json!("demo.first"),
            json!("demo.first"),
            Value::Null,
            json!("demo.body"),
            Value::Null,
            Value::Null
        ]
    );
    let source = "~~~markdown\n### Requirement: Example r[demo.fake]\n~~~\nHe are late.\n### Requirement: No declaration\nHe are late.\n#### Scenario: Not a declaration\nr[demo.scenario]\nHe are late.\n";
    let rows = agreement(source, true, 3).unwrap();
    assert_eq!(rows.len(), 3);
    assert!(rows.iter().all(|row| row["requirement_id"].is_null()));
}

#[test]
fn meaning_change_risks_consider_the_specific_changed_tokens() {
    for (before, after, expected) in [
        ("MUST retain", "MAY retain", vec!["obligation"]),
        ("is not ready", "is ready", vec!["negation"]),
        ("10ms", "20ms", vec!["number"]),
        ("10ms", "10s", vec!["unit"]),
        ("10°C", "10°F", vec!["unit"]),
        ("when ready", "unless ready", vec!["condition"]),
        ("all inputs", "some inputs", vec!["quantifier"]),
        ("r[demo.old]", "r[demo.new]", vec!["requirement_id"]),
        ("REQ-123", "REQ-124", vec!["requirement_id"]),
        ("MUST retain", "must retain", vec!["obligation"]),
    ] {
        assert_eq!(
            risks::review_risks(before, after),
            expected,
            "{before} -> {after}"
        );
    }
    assert!(risks::review_risks(
        "If all 10 requests MUST retain r[demo.id], keep a apple and a orange.",
        "If all 10 requests MUST retain r[demo.id], keep an apple and an orange."
    )
    .is_empty());
    assert!(risks::review_risks(
        "A apple MUST remain with a orange.",
        "An apple MUST remain with an orange."
    )
    .is_empty());
}

#[test]
fn enhanced_suggestions_have_original_unicode_end_positions_and_specific_risks() {
    let source = "🙂 MUST\nnot change.\n";
    let finding = Diagnostic::error("E0001", "subject_verb_agreement", "Review this wording")
        .primary(5..9, "obligation")
        .suggest("Review the alternative", 5..13, "MAY\nnot");
    let value = diagnostic_json(finding, source, &Positions::new(source), &[], true).unwrap();
    assert_eq!(value["labels"][0]["line"], 1);
    assert_eq!(value["labels"][0]["column"], 3);
    assert_eq!(value["labels"][0]["end_column"], 7);
    let suggestion = &value["helps"][0]["suggestion"];
    assert_eq!(suggestion["line"], 1);
    assert_eq!(suggestion["column"], 3);
    assert_eq!(suggestion["end_line"], 2);
    assert_eq!(suggestion["end_column"], 4);
    assert_eq!(suggestion["review_required"], true);
    assert_eq!(suggestion["review_risks"], json!(["obligation"]));
    assert_eq!(value["review_risks"], json!(["obligation"]));
    assert!(value["requirement_id"].is_null());
}

#[test]
fn invalid_engine_spans_are_errors_and_excerpts_are_bounded() {
    let source = "é🙂";
    for span in [1..2, 0..99, std::ops::Range { start: 4, end: 2 }] {
        let finding = Diagnostic::error("E0001", "subject_verb_agreement", "Invalid span")
            .primary(span, "invalid");
        assert!(diagnostic_json(finding, source, &Positions::new(source), &[], false).is_err());
    }
    let finding = Diagnostic::error("E0001", "subject_verb_agreement", "Invalid suggestion")
        .primary(0..2, "valid")
        .suggest("Review", 2..3, "x");
    assert!(diagnostic_json(finding, source, &Positions::new(source), &[], true).is_err());
    let source = "🙂".repeat(201);
    let finding = Diagnostic::error("E0001", "subject_verb_agreement", "Long label")
        .primary(0..source.len(), "valid");
    let value = diagnostic_json(finding, &source, &Positions::new(&source), &[], true).unwrap();
    assert_eq!(
        value["labels"][0]["excerpt"]
            .as_str()
            .unwrap()
            .chars()
            .count(),
        200
    );
    assert_eq!(value["labels"][0]["end_column"], 202);
}

#[test]
fn body_declarations_do_not_require_an_independent_strength_validator() {
    let source = "### Requirement: Ordinary body\nHe are late. r[demo.body]\n#### Scenario: Child r[demo.reference]\nHe are late.\n";
    let rows = agreement(source, true, 2).unwrap();
    assert_eq!(rows.len(), 2);
    assert!(rows.iter().all(|row| row["requirement_id"] == "demo.body"));
}
