//! Pure, deterministic core types for cairn.
//!
//! Filesystem access, environment reads, process control, and presentation live
//! in shell crates. Octet checks modules under `verified/` with its
//! `verified_purity` lint.
//! r[impl cairn.supersede.boundary.pure_core]
//! r[impl cairn.supersede.migrate_openspec.boundary]
//! r[impl cairn.supersede.tracey_coverage.boundary]
//! r[impl cairn.supersede.traceability_ownership.authoritative_checker]
//! r[impl cairn.supersede.traceability_ownership.configured_profiles]
//! r[impl cairn.supersede.gate_policy.references]
//! r[impl cairn.supersede.gate_policy.task_links]
//! r[impl cairn.supersede.gate_policy.exemptions]
//! r[impl cairn.supersede.doctor]
//! r[impl cairn.supersede.doctor.boundary]

pub mod verified;
pub use verified::source_review;

mod api;
pub use api::*;
