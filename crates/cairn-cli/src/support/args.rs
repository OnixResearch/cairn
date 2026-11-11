use std::collections::BTreeSet;
use std::error::Error;

use super::*;

/// Reject options outside the command's documented set and options without a
/// following value. Positional arguments are ignored.
pub(crate) fn reject_unknown_options(
    args: &[String],
    allowed: &[&str],
) -> Result<(), Box<dyn Error>> {
    reject_unknown_options_with_flags(args, allowed, &[])
}

pub(crate) fn reject_unknown_options_with_flags(
    args: &[String],
    allowed_values: &[&str],
    allowed_flags: &[&str],
) -> Result<(), Box<dyn Error>> {
    reject_unknown_options_with_repeats(args, allowed_values, allowed_flags, &[])
}

pub(crate) fn reject_unknown_options_with_repeats(
    args: &[String],
    allowed_values: &[&str],
    allowed_flags: &[&str],
    repeatable: &[&str],
) -> Result<(), Box<dyn Error>> {
    let mut seen = BTreeSet::new();
    let mut index = 0;
    while index < args.len() {
        let argument = &args[index];
        if argument.starts_with("--") {
            if allowed_flags.contains(&argument.as_str()) {
                if !repeatable.contains(&argument.as_str()) && !seen.insert(argument.as_str()) {
                    return Err(format!("duplicate option: {argument}").into());
                }
                index += 1;
                continue;
            }
            if !allowed_values.contains(&argument.as_str()) {
                return Err(format!("unknown option: {argument}").into());
            }
            if args
                .get(index + 1)
                .is_none_or(|value| value.is_empty() || value.starts_with("--"))
            {
                return Err(format!("{argument} requires a value").into());
            }
            if !repeatable.contains(&argument.as_str()) && !seen.insert(argument.as_str()) {
                return Err(format!("duplicate option: {argument}").into());
            }
            index += 2;
        } else {
            index += 1;
        }
    }
    Ok(())
}

pub(crate) fn read_string_option(args: &[String], name: &str) -> Option<String> {
    args.windows(2)
        .find_map(|pair| (pair[0] == name).then(|| pair[1].clone()))
}

pub(crate) fn read_repeated_path_options(args: &[String], name: &str) -> Vec<PathBuf> {
    args.windows(2)
        .filter(|pair| pair[0] == name)
        .map(|pair| PathBuf::from(&pair[1]))
        .collect()
}

pub(crate) fn read_option(args: Vec<String>, name: &str, default: &str) -> PathBuf {
    args.windows(2)
        .find_map(|pair| (pair[0] == name).then(|| PathBuf::from(&pair[1])))
        .unwrap_or_else(|| PathBuf::from(default))
}

pub(crate) fn has_flag(args: &[String], name: &str) -> bool {
    args.iter().any(|arg| arg == name)
}

/// Reject stray positional tokens for commands whose arguments are all
/// documented options. Call after option-name and value admission.
const BOOLEAN_OPTIONS: &[&str] = &[
    "--allow-mutations",
    "--check",
    "--execute",
    "--help",
    "--interactive",
    "--json",
    "--raw",
    "--run-probes",
    "--strict",
];

pub(crate) fn admit_arguments(
    args: &[String],
    values: &[&str],
    flags: &[&str],
) -> Result<(), Box<dyn Error>> {
    reject_unknown_options_with_flags(args, values, flags)?;
    reject_positional_arguments(args)?;
    Ok(())
}

pub(crate) fn reject_positional_arguments(args: &[String]) -> Result<(), Box<dyn Error>> {
    let mut index = 0;
    while index < args.len() {
        let argument = &args[index];
        if BOOLEAN_OPTIONS.contains(&argument.as_str()) {
            index += 1;
        } else if argument.starts_with("--") {
            match args.get(index + 1) {
                Some(value) if !value.is_empty() && !value.starts_with("--") => {
                    index += 2;
                }
                _ => return Err(format!("{argument} requires a value").into()),
            }
        } else {
            return Err(format!("unexpected argument: {argument}").into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{reject_positional_arguments, reject_unknown_options_with_flags};

    fn args(values: &[&str]) -> Vec<String> {
        values.iter().map(|value| (*value).to_string()).collect()
    }

    #[test]
    fn flags_do_not_consume_following_options() {
        reject_positional_arguments(&args(&[
            "--root",
            ".",
            "--json",
            "--profile",
            "trellis-rust",
        ]))
        .unwrap();
    }

    #[test]
    fn option_like_value_is_rejected() {
        let error = reject_positional_arguments(&args(&["--root", "--json"])).unwrap_err();
        assert_eq!(error.to_string(), "--root requires a value");
        let error = reject_positional_arguments(&args(&["--root", ""])).unwrap_err();
        assert_eq!(error.to_string(), "--root requires a value");
    }

    #[test]
    fn trailing_value_option_is_rejected() {
        let error = reject_positional_arguments(&args(&["--root"])).unwrap_err();
        assert_eq!(error.to_string(), "--root requires a value");
    }

    #[test]
    fn empty_values_are_rejected() {
        let error = reject_unknown_options_with_flags(&args(&["--root", ""]), &["--root"], &[])
            .unwrap_err();
        assert_eq!(error.to_string(), "--root requires a value");
    }

    #[test]
    fn value_options_consume_their_value() {
        reject_positional_arguments(&args(&["--format", "json", "--root", "."])).unwrap();
    }

    #[test]
    fn stray_positionals_are_rejected_after_flags() {
        let error = reject_positional_arguments(&args(&["--json", "stray"])).unwrap_err();
        assert_eq!(error.to_string(), "unexpected argument: stray");
    }
}
