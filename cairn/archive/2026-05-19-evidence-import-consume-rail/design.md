## Context

`cairn evidence export` creates a portable directory and `cairn evidence verify` validates it from the consumer side. The remaining gap is safe consumption: downstream CI should be able to materialize a verified copy in a stable local directory and keep one deterministic receipt proving what was imported.

## Goals

- Make imported evidence reproducible and bounded.
- Prevent undeclared or unsafe bundle content from entering the imported evidence directory.
- Compose the existing evidence verifier rather than duplicating trust logic.
- Keep receipts portable and secret-safe.

## Non-Goals

- No remote download or registry integration.
- No tar/zip/archive format support.
- No cryptographic trust decision beyond existing deterministic manifest/verification receipts.
- No garbage collection or cache lifecycle.

## Decisions

### 1. Import verifies first

**Choice:** `evidence import` invokes the existing bundle verification receipt before copying artifacts.

**Rationale:** Import is a consumption rail, not a parallel verifier. Reusing the verifier keeps validation semantics consistent.

**Implementation:** Build a verification receipt for `--bundle`; if invalid, emit an import failure receipt and do not copy declared artifacts.

### 2. Copy only declared artifacts

**Choice:** Import copies the artifact paths listed in `evidence-export-receipt.json`, plus writes `evidence-import-receipt.json` after copy.

**Rationale:** The export receipt is the canonical declaration. Copying only declared artifacts avoids accidental extra files becoming trusted evidence.

**Implementation:** Validate each artifact path is relative, normal, non-empty, and within the bundle/import directory. Rehash source and destination artifacts and record role/path/hash triples sorted by role/path.

### 3. Reject extra files

**Choice:** The importer fails if the bundle contains files beyond declared artifacts and `evidence-export-receipt.json`.

**Rationale:** Extra files are often how ad-hoc CI glue smuggles unreviewed state into evidence bundles.

**Implementation:** Recursively enumerate regular files under the bundle with a deterministic cap/order and compare to the allowed relative path set.

### 4. Stable receipt fields

**Choice:** Receipt display fields use normalized placeholders: `bundle = "<bundle>"`, `into = "<into>"`, `root` remains the normal root display used by Cairn receipts and portable hashing normalizes root-specific strings.

**Rationale:** Import receipts should be byte-stable across temp directories and CI roots.

## Risks / Trade-offs

- Rejecting extra files is strict, but keeps imports reviewable and deterministic.
- Copying JSON evidence as-is means stale host display fields remain in nested receipts, but those receipts have already been verified by portable hash semantics; the import receipt itself must avoid adding host-specific paths.
- A recursive bundle scan needs a bound to avoid unbounded traversal; use existing bounded traversal style and deterministic issue codes.
