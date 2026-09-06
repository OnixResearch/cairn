//! Cairn-owned evidence production decisions over supplied facts.
//! Admission is not execution, authority, freshness observation, or publication.
mod admission;
mod cargo_output;
pub use cargo_output::{cargo_coverage, CargoCoverage};
mod normalization;
mod types;

pub use admission::{admit_plan, authorize_attempt, profile_command};
pub use normalization::{normalize_attempt, publication_decision};
pub use types::*;

#[cfg(test)]
mod tests;
