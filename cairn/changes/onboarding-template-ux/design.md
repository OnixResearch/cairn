## Context

`cairn init` creates the native lifecycle directories, but it does not yet give a new user a reviewed quickstart package, policy profile, or agent instruction set. A user must already understand the lifecycle layout and gate sequence.

## Decisions

### 1. Policy-owned template catalog

**Choice:** Store template identifiers, descriptions, default files, and profile constraints in generated Cairn policy rather than hard-coded CLI literals.

**Rationale:** Reviewed policy metadata keeps onboarding deterministic, auditable, and extensible without making the CLI guess at project conventions.

### 2. Preview-first template application

**Choice:** Add a read-only preview path and an explicit execute path for materializing template files, matching Cairn's sync/archive mutation pattern.

**Rationale:** Onboarding writes files, so it should produce dry-run and executed mutation receipts with BLAKE3-bound manifests.

### 3. Minimal initial profile set

**Choice:** Start with narrow profiles such as `rust-nix`, `generic-cli`, and `agent-workflow`, each generating only native Cairn lifecycle artifacts and documentation snippets.

**Rationale:** Small reviewed templates reduce support burden while closing the biggest first-use gap.

## Risks / Trade-offs

- Template sprawl can become policy debt; require reviewed metadata and deterministic fixture coverage before adding profiles.
- Template docs must not overclaim remote publishing, signing, hosted UI, or agent automation that Cairn does not provide.
