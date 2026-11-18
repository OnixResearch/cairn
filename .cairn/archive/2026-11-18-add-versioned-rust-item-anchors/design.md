## Context

The v1 index binds requirement rows, exact marker spans and source-file content but does not resolve declarations. The requirement registry remains authoritative; Rust interpretation is derived evidence, not a new requirement authority.

## Decisions

### Decision: Preserve v1 serialization and add a closed v2 structural payload

**Choice:** Omit structural fields for exact-marker v1 and retain its original hash domain/material. A `rust_item_v1` profile emits v2, with independently hashed bindings and full-index self-binding.

**Rationale:** Old exact-marker bytes stay stable; a saved v2 index can reject changed interpretation and retain prior bindings without mutating them.

### Decision: Parse at the shell using bounded saved bytes

**Choice:** Cap each `.rs` source at 1 MiB, parse through pinned `syn 2.0.117` and `proc-macro2` byte ranges, and pass immutable facts into the pure core. Configuration is a safe logical path whose exact bytes are hashed. The supported subset is named file/module/trait/impl declarations and members. `cfg(all/any/not)` is interpreted structurally for explicit target/feature/cfg values; unresolved macros, cfg_attr, unknown predicates, malformed syntax and nondeclarations fail closed.

**Rationale:** `Span::byte_range()` avoids character-column/UTF-8 conversion ambiguity; no Buoy implementation is copied or required at runtime.

### Decision: Keep current attachments and immutable history apart

**Choice:** Current bindings name exact marker links and parsed item facts. `--previous-index` imports valid prior v2 bindings into a sorted historical list. Verification always recomputes current facts from the supplied root and selected policy; history alone never passes freshness.

**Rationale:** A moved/copied declaration, changed requirement row, or changed source/configuration cannot borrow a prior receipt. Current linkage does not establish callee behavior, test results or proof freshness.

## Risks / Trade-offs

- Macro expansion, generated code, cfg_attr and custom target predicates remain unsupported; explicit diagnostics are preferable to pretending those items are resolved.
- Normalized token identity ignores ordinary comments/formatting, while exact source receipts still become stale. The selected source snapshot includes helpers/callees and changes invalidate freshness even if caller syntax is unchanged; no behavioral or proof claim follows.
