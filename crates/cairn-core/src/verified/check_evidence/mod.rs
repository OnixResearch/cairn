//! Cairn-owned evidence production decisions over supplied facts.
//! Admission is not execution, authority, freshness observation, or publication.
mod admission;
mod cargo_output;
pub use cargo_output::{cargo_coverage, CargoCoverage};
mod kiln;
pub use kiln::{admit_kiln_json, KilnAdmission, KILN_REVISION};
mod lattice;
mod manifest;
mod normalization;
pub use lattice::{admit_lattice_json, LatticeAdmission, LATTICE_REVISION};
pub use manifest::{
    admit_manifest, recovery_decision, RecoveryDecision, MAX_BUNDLE_ARTIFACTS,
    MAX_BUNDLE_ARTIFACT_BYTES, MAX_BUNDLE_BYTES,
};
mod octet;
mod types;
mod valence;
pub use octet::{admit_octet_json, OctetAdmission, OctetStatus, OCTET_REVISION};
pub use valence::{admit_valence_json, ValenceAdmission, VALENCE_REVISION};

pub use admission::{admit_plan, authorize_attempt, profile_command};
pub use normalization::{normalize_attempt, publication_decision};
pub use types::*;

#[cfg(test)]
mod kiln_tests;
#[cfg(test)]
mod manifest_tests;
#[cfg(test)]
mod tests;
