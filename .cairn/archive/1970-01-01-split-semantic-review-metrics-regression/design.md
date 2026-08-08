# Design: Split semantic review metrics regression rail

Extract the current monolithic metrics regression rail into a directory module:

- `mod.rs`: public schema constants and validation entrypoints.
- `report.rs`: deterministic report building and receipt hash binding.
- `registry.rs`: category binding, allowed vocabulary, and uniqueness checks.
- `fixtures.rs`: fixture coverage parsing and positive/negative fixture-pair enforcement.
- `guidance.rs`: guidance/task/spec sync set extraction and category sync validation.
- `evidence_boundary.rs`: sanitized evidence ref checks, metadata matching, and secret-free traversal.
- `common.rs`: local non-empty string helper shared inside the module.

The split is intentionally behavior-preserving. No CLI arguments, JSON schemas, issue codes, or receipt hash construction should change except for Tracey refs added to prove module boundaries.
