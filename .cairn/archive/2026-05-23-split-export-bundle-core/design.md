# Design: Split Export Bundle Core

## Boundaries

The split keeps `content_addressed::export_bundle` as the private module family behind the existing `verified::content_addressed` facade:

- `mod.rs`: public facade, validation orchestration, Tracey implementation references.
- `types.rs`: public `ExportBundleIssue` and `ExportBundleReport` DTOs.
- `constants.rs`: schema string, required-field tables, and sizing constants.
- `hash.rs`: canonical bundle hash and hash-material construction.
- `schema.rs`: top-level required-field and schema checks.
- `objects.rs`: object membership validation and sorted object-hash collection.
- `pointers.rs`: name-pointer sorting and object-reference validation.
- `replay.rs`: replay metadata validation.
- `compiler_ir.rs`: HIR/MIR compiler IR checks and overclaim rejection.
- `manifest.rs`: mutation-manifest touched-file validation.
- `caveats.rs`: explicit non-claim/overclaim validation.
- `entropy.rs`: ambient entropy checks over canonical hash material.
- `common.rs`: private issue construction helper.

## Invariants

- Public API names stay unchanged through the existing `verified::content_addressed` re-exports.
- Hash material continues to exclude the same non-hash fields.
- Issue codes/messages remain unchanged.
- The split remains in `cairn-core` pure deterministic code; no filesystem, environment, clocks, process, or JSON presentation shell behavior is introduced.
- Verification covers focused content-addressed core behavior and CLI fixture behavior before archive.
