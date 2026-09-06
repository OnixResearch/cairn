//! Cairn-owned evidence production decisions over supplied facts.
//! Admission is not execution, authority, freshness observation, or publication.
mod admission;
mod cargo_output;
pub use cargo_output::{cargo_coverage, CargoCoverage};
mod manifest;
mod normalization;
pub use manifest::{
    admit_manifest, recovery_decision, RecoveryDecision, MAX_BUNDLE_ARTIFACTS,
    MAX_BUNDLE_ARTIFACT_BYTES, MAX_BUNDLE_BYTES,
};
mod octet;
mod types;
pub use octet::{admit_octet_json, OctetAdmission, OctetStatus, OCTET_REVISION};

pub use admission::{admit_plan, authorize_attempt, profile_command};
pub use normalization::{normalize_attempt, publication_decision};
pub use types::*;

#[cfg(test)]
mod manifest_tests;
#[cfg(test)]
mod tests;
