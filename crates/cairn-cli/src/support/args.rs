use std::collections::BTreeSet;
use std::error::Error;

use super::*;

/// Reject options outside the command's documented set and options without a
/// following value. Positional arguments are ignored.
pub(crate) fn reject_unknown_options(
    args: &[String],
    allowed: &[&str],
) -> Result<(), Box<dyn Error>> {
    let mut seen = BTreeSet::new();
    let mut index = 0;
    while index < args.len() {
        let argument = &args[index];
        if argument.starts_with("--") {
            if !allowed.contains(&argument.as_str()) {
                return Err(format!("unknown option: {argument}").into());
            }
            if args.get(index + 1).is_none() {
                return Err(format!("{argument} requires a value").into());
            }
            if !seen.insert(argument.as_str()) {
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
