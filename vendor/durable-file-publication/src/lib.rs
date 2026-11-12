#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]
//! Bounded capability-relative publication of one regular file.
//!
//! The crate classifies mechanical publication facts. It grants no product
//! authority and makes no filesystem, device, or power-loss correctness claim.

extern crate alloc;

// r[impl durable_publication.core.classification]
pub mod core;
// r[impl durable_publication.validation.corpus]
pub mod corpus;
#[cfg(feature = "std")]
// r[impl durable_publication.scope.one_file]
// r[impl durable_publication.platform.support]
pub mod shell;
