use std::{error::Error, fmt};

#[derive(Debug)]
pub enum PolicyError {
    Parse { source: serde_json::Error },
    MissingField(&'static str),
    InvalidField(&'static str),
    UnsupportedVersion { found: u64, expected: u64 },
}

impl fmt::Display for PolicyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Parse { source } => write!(f, "failed to parse policy: {source}"),
            Self::MissingField(field) => write!(f, "policy missing field {field}"),
            Self::InvalidField(field) => write!(f, "policy has invalid field {field}"),
            Self::UnsupportedVersion { found, expected } => {
                write!(f, "unsupported policy version {found}; expected {expected}")
            }
        }
    }
}

impl Error for PolicyError {}
