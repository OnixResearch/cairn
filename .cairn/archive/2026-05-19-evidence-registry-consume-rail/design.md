# Design: Evidence Registry Consume Rail

## Context

Evidence registries aggregate already-built evidence index files into stable release/channel catalogs. The next consumer problem is deterministic selection: an agent should be able to fetch one registry, choose the exact matching index without guessing, and obtain a portable fetch plan for subsequent verification.

## Goals

- Consume registry files, not raw bundles.
- Verify registry structure and receipt data before producing consumer artifacts.
- Select entries using stable metadata only.
- Emit portable deterministic receipts with stable `receipt_hash` values.
- Reject ambiguous or unsafe input using deterministic issue codes.
- Preserve the Rust CLI / Nickel policy implementation boundary.

## Non-Goals

- Performing network fetches.
- Resolving credentials, secrets, mirrors, or URLs.
- Rebuilding indexes or bundles from registry selection commands.
- Adding Python or shell helper scripts.

## Decisions

### 1. Registry verification is part of consumption

**Choice:** `registry select` and `registry fetch-plan` parse and validate the registry before emitting their own artifacts.

**Rationale:** Consumers should not need to remember a separate verification command before using the registry. The resulting receipt binds the registry hash/receipt hash and exposes deterministic failure issues.

### 2. Selection uses stable metadata selectors

**Choice:** Start with selectors that already exist in registry entries: bundle kind, policy hash, index hash, index receipt hash, artifact role, artifact hash, and release/coverage receipt hashes.

**Rationale:** These fields are portable and already reviewable. Host paths, timestamps, env values, and credentials are not valid selectors.

### 3. Ambiguity is an error

**Choice:** `registry select` requires exactly one matching entry. Zero matches and multiple matches both fail with stable issue codes.

**Rationale:** Deterministic consumer automation should never silently choose between equally valid registry entries.

### 4. Fetch plan is metadata-only

**Choice:** `registry fetch-plan` emits sorted entries with safe relative index refs, index hashes, artifact roles/relative paths/hashes, and receipt-chain hashes.

**Rationale:** Cairn should describe what to fetch and verify, not perform network I/O or handle secrets.

### 5. Portable path placeholders remain mandatory

**Choice:** Receipts use placeholders such as `<registry>`, `<out>`, and safe relative refs from the registry. Absolute roots and raw output paths are excluded from hash-bearing fields.

**Rationale:** The receipt must compare across CI, agent sandboxes, and local workspaces.

## Risks / Mitigations

- **Registry schema drift**: update Nickel receipt schemas and deterministic surface inventory together.
- **False determinism from insertion order**: sort selected/fetch-plan output by stable metadata, not CLI argument order.
- **Path traversal**: reject unsafe `index_relative` or artifact relative paths before emitting consumer artifacts.
- **Dirty release evidence during development**: use deterministic fixtures or full clean-gate smoke after implementation.

## Validation Plan

- Add positive tests for registry select and fetch-plan after export/verify/import/republish/index/registry build.
- Add negative tests for no match, ambiguous match, tampered/invalid registry, and unsafe selector/path behavior where practical.
- Run policy export freshness, format, workspace tests, clippy, Tracey coverage, strict validation, diff check, and flake check.
