# Design

## Context

The content-bound trace index owns exact marker linkage for Cairn.
Buoy owns a second content-addressed binding protocol with immutable objects and `current`/`stale`/`historical` trace classifications.
The two producers use different schemas and digest namespaces even when both use BLAKE3.
Cross-schema digest equality is therefore not evidence of agreement.

## Decisions

### Decision: Pin identity without runtime resolution

**Choice:** Record Buoy revision `01096fbdd0b9bf39b2fec468f90e7b1db958f615` and recursive source content hash `sha256-1+/x0hY3+YMYgLVXs0uoUmA1VPPlg5i/7qt19NrBruc=`, the `nightly-2026-04-16` toolchain, the `blake3-256` feature, and only `static`, `trace`, and `package-identity` commands.

**Rationale:** The comparison check must require no network and no private upstream full-acceptance setup after its pinned build inputs are realized.
Keeping upstream identity data in the manifest makes a source, tool, configuration, or command change visible in the receipt identity instead of silently reusing a prior observation.

### Decision: Declare blocked upstream observations

**Choice:** Mark every upstream observation `declared_not_executed` and the upstream artifact unavailable with a stated reason.
The manifest validator rejects an `executed` claim while no artifact is available.

**Rationale:** The pinned consumer package sets `doCheck = false`; building it would not execute comparison cases.
Full Devenv acceptance needs private Atoll access, and a source Cargo build downloads public registry dependencies on first use.
Without a realized pinned artifact, no upstream result exists to report.
The reachable substitute is the declared comparison fixture and executed native mutation qualification, not a manufactured upstream result.

### Decision: Execute native fixtures on temporary copies

**Choice:** `traceability compare` copies each saved fixture into a private temporary directory, exports a requirement registry and trace index, and verifies either the current fixture or a saved base index.
Originals are hashed before and after the check.
All commands are read-only trace-index commands; the comparator never invokes upstream binding or source rewriting.

**Rationale:** Mutation fixtures must exercise the real native code path rather than restating declared JSON.
A copied marker, moved byte span, changed requirement body, and changed index schema must each change the native classification.
Raw stdout, stderr, and exit disposition are retained for review.

### Decision: One semantic subset, separate producer identities

**Choice:** Compare observable obligations (`current`, `stale_requirement`, `stale_item`, `missing_object`, `ambiguous`, `unsupported`, `invalid_output`) rather than digest equality.
Native and declared upstream report identities use separate BLAKE3 domains and retain their producer schema.

**Rationale:** Equal BLAKE3 values across schemas do not establish cross-schema equality, and requirement/source identities remain Cairn's.
The projection is not a second production authority.

### Decision: Reject restated mutation qualifications

**Choice:** The manifest validator requires each mutation to change both its declared upstream semantic class and its native expectation from the base case.
The evaluator additionally re-validates a restated-buoy clone and a restated-native clone and reports whether both were rejected.

**Rationale:** A comparison that accepts a mutated fixture with the base result is restating current output rather than detecting planted drift.
Ordinary output changes never regenerate these expectations.

### Decision: Keep the comparison advisory and unfinished upstream work explicit

**Choice:** Retain the bounded comparator and its fixtures, set `runtime_adoption = false`, and record unsupported cases (`schema_version_drift`, Unicode/CRLF exact-span identity, and cfg scope) explicitly.

**Rationale:** The executed evidence qualifies Cairn's own mutation detection and the declared mapping.
It does not prove Buoy behavior, cross-implementation agreement, or requirement satisfaction.
Permanent runtime adoption is not assumed.

## Risks / Trade-offs

- The declared upstream observations may drift from real Buoy behavior; they are pinned to a reviewed revision and marked not executed until a realized artifact exists.
- The semantic subset is intentionally small. Cases outside it remain unsupported rather than being inferred.
- Unicode/CRLF and cfg dimensions are named expected divergences, not waived native checks; native acceptance still runs independently.
- The comparator adds a CLI surface. It stays opt-in and does not alter existing trace-index gate meanings.
