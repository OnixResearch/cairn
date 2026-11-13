use cairn_core::traceability_id_is_valid;

pub(super) const CODES: [&str; 7] = [
    "obligation",
    "negation",
    "number",
    "unit",
    "condition",
    "quantifier",
    "requirement_id",
];

/// These are review reasons, not judgments about equivalence or correctness.
/// Compare only touched tokens, then compare sensitive tokens within that edit.
/// Thus an engine suggestion covering a whole sentence does not flag a MUST,
/// number, or ID that it merely reproduces unchanged.
pub(super) fn review_risks(original: &str, replacement: &str) -> Vec<&'static str> {
    if original == replacement {
        return Vec::new();
    }
    let before: Vec<_> = Tokens::new(original).collect();
    let after: Vec<_> = Tokens::new(replacement).collect();
    let prefix = before
        .iter()
        .zip(&after)
        .take_while(|(left, right)| left == right)
        .count();
    let suffix = before[prefix..]
        .iter()
        .rev()
        .zip(after[prefix..].iter().rev())
        .take_while(|(left, right)| left == right)
        .count();
    let before_range = prefix..before.len() - suffix;
    let after_range = prefix..after.len() - suffix;
    let direct_unit_change = before_range.len() == 1
        && after_range.len() == 1
        && units(before[before_range.start])
        && units(after[after_range.start]);
    let mut result = Vec::new();
    for (category, code) in CODES.iter().enumerate() {
        let left = before_range
            .clone()
            .filter(|&index| {
                sensitive(before[index], category)
                    && (category != 3 || unit_context(&before, index, direct_unit_change))
            })
            .map(|index| before[index]);
        let right = after_range
            .clone()
            .filter(|&index| {
                sensitive(after[index], category)
                    && (category != 3 || unit_context(&after, index, direct_unit_change))
            })
            .map(|index| after[index]);
        if !left.eq(right) {
            result.push(*code);
        }
    }
    result
}

fn unit_context(tokens: &[&str], index: usize, direct_unit_change: bool) -> bool {
    // Single-letter symbols also occur as articles and ordinary words. Treat
    // them as units only in a quantity or an explicit unit-to-unit change.
    tokens[index].chars().count() > 1
        || direct_unit_change
        || index
            .checked_sub(1)
            .and_then(|previous| tokens.get(previous))
            .is_some_and(|token| token.parse::<f64>().is_ok())
        || tokens
            .get(index + 1)
            .is_some_and(|token| token.parse::<f64>().is_ok())
}

fn sensitive(token: &str, category: usize) -> bool {
    if category == 6 {
        return requirement_id(token);
    }
    if requirement_id(token) {
        return false;
    }
    match category {
        0 => words(
            token,
            &[
                "must",
                "shall",
                "should",
                "may",
                "required",
                "recommended",
                "optional",
            ],
        ),
        1 => {
            words(
                token,
                &[
                    "not",
                    "no",
                    "none",
                    "never",
                    "neither",
                    "nor",
                    "without",
                    "cannot",
                    "can't",
                    "won't",
                    "don't",
                    "doesn't",
                    "didn't",
                    "isn't",
                    "aren't",
                    "wasn't",
                    "weren't",
                    "hasn't",
                    "haven't",
                    "hadn't",
                    "mustn't",
                    "shouldn't",
                    "wouldn't",
                    "couldn't",
                    "needn't",
                ],
            ) || token.ends_with("n’t")
        }
        2 => {
            token.chars().any(|character| character.is_numeric())
                || words(
                    token,
                    &[
                        "zero",
                        "one",
                        "two",
                        "three",
                        "four",
                        "five",
                        "six",
                        "seven",
                        "eight",
                        "nine",
                        "ten",
                        "eleven",
                        "twelve",
                        "thirteen",
                        "fourteen",
                        "fifteen",
                        "sixteen",
                        "seventeen",
                        "eighteen",
                        "nineteen",
                        "twenty",
                        "thirty",
                        "forty",
                        "fifty",
                        "sixty",
                        "seventy",
                        "eighty",
                        "ninety",
                        "hundred",
                        "thousand",
                        "million",
                        "billion",
                        "trillion",
                        "half",
                        "quarter",
                        "twice",
                        "first",
                        "second",
                        "third",
                        "fourth",
                        "fifth",
                        "sixth",
                        "seventh",
                        "eighth",
                        "ninth",
                        "tenth",
                    ],
                )
        }
        3 => units(token),
        4 => words(
            token,
            &[
                "if",
                "when",
                "whenever",
                "unless",
                "until",
                "while",
                "before",
                "after",
                "once",
                "provided",
                "assuming",
                "where",
                "then",
                "otherwise",
                "except",
            ],
        ),
        5 => words(
            token,
            &[
                "all", "any", "every", "each", "no", "none", "some", "only", "exactly", "least",
                "most", "both", "either", "neither", "always", "never", "few", "many", "several",
                "multiple",
            ],
        ),
        _ => false,
    }
}

fn words(token: &str, candidates: &[&str]) -> bool {
    candidates
        .iter()
        .any(|candidate| token.eq_ignore_ascii_case(candidate))
}

fn units(token: &str) -> bool {
    // Symbols are case-sensitive (m, M, b and B are not interchangeable).
    matches!(
        token,
        "%" | "‰"
            | "°"
            | "ns"
            | "µs"
            | "μs"
            | "us"
            | "ms"
            | "s"
            | "min"
            | "h"
            | "d"
            | "mm"
            | "cm"
            | "m"
            | "km"
            | "mg"
            | "g"
            | "kg"
            | "ml"
            | "mL"
            | "l"
            | "L"
            | "Hz"
            | "kHz"
            | "MHz"
            | "GHz"
            | "B"
            | "kB"
            | "KB"
            | "MB"
            | "GB"
            | "TB"
            | "KiB"
            | "MiB"
            | "GiB"
            | "TiB"
            | "b"
            | "kb"
            | "Mb"
            | "Gb"
            | "bps"
            | "kbps"
            | "Mbps"
            | "Gbps"
            | "V"
            | "mV"
            | "A"
            | "mA"
            | "W"
            | "kW"
            | "J"
            | "Pa"
            | "K"
            | "C"
            | "F"
    ) || words(
        token,
        &[
            "nanosecond",
            "nanoseconds",
            "microsecond",
            "microseconds",
            "millisecond",
            "milliseconds",
            "second",
            "seconds",
            "minute",
            "minutes",
            "hour",
            "hours",
            "day",
            "days",
            "week",
            "weeks",
            "month",
            "months",
            "year",
            "years",
            "byte",
            "bytes",
            "bit",
            "bits",
            "kilobyte",
            "kilobytes",
            "megabyte",
            "megabytes",
            "gigabyte",
            "gigabytes",
            "meter",
            "meters",
            "metre",
            "metres",
            "kilometer",
            "kilometers",
            "kilometre",
            "kilometres",
            "gram",
            "grams",
            "kilogram",
            "kilograms",
            "percent",
            "percentage",
            "celsius",
            "fahrenheit",
            "kelvin",
            "hertz",
            "watt",
            "watts",
        ],
    )
}

fn requirement_id(token: &str) -> bool {
    if let Some(marker) = token
        .strip_prefix("r[")
        .and_then(|body| body.strip_suffix(']'))
    {
        return traceability_id_is_valid(marker.split('+').next().unwrap_or_default());
    }
    traceability_id_is_valid(token)
        && !words(token, &["e.g", "i.e"])
        && (token.contains('.')
            || (token.contains('-')
                && token.bytes().any(|byte| byte.is_ascii_digit())
                && token.bytes().any(|byte| byte.is_ascii_alphabetic())))
        && token.bytes().any(|byte| byte.is_ascii_alphabetic())
}

struct Tokens<'a> {
    source: &'a str,
    cursor: usize,
}

impl<'a> Tokens<'a> {
    fn new(source: &'a str) -> Self {
        Self { source, cursor: 0 }
    }
}

impl<'a> Iterator for Tokens<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let source = self.source;
        let mut cursor = self.cursor;
        while let Some(character) = source[cursor..].chars().next() {
            if !character.is_whitespace() {
                break;
            }
            cursor += character.len_utf8();
        }
        let start = cursor;
        let first = source[cursor..].chars().next()?;
        if source[cursor..].starts_with("r[") {
            if let Some(end) = source[cursor..].find(']') {
                let token = &source[cursor..cursor + end + 1];
                if requirement_id(token) {
                    self.cursor = cursor + token.len();
                    return Some(token);
                }
            }
        }
        cursor += first.len_utf8();
        if first.is_alphanumeric() || first == '_' {
            while let Some(character) = source[cursor..].chars().next() {
                if first.is_numeric() && !character.is_numeric() && character != '.' {
                    break;
                }
                let internal = matches!(character, '.' | '-' | '\'' | '’')
                    && source[cursor + character.len_utf8()..]
                        .chars()
                        .next()
                        .is_some_and(char::is_alphanumeric);
                if !character.is_alphanumeric() && character != '_' && !internal {
                    break;
                }
                cursor += character.len_utf8();
            }
        }
        self.cursor = cursor;
        Some(&source[start..cursor])
    }
}
