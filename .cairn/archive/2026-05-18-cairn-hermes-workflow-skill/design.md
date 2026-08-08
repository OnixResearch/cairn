# Design: Canonical Cairn Hermes workflow skill

## Context

Hermes skills are durable agent workflow memory. Cairn now owns the native lifecycle, validation, gate, MCP, receipt, and release-readiness surfaces, while older OpenSpec/Ironspec references are compatibility or migration context. The active skill namespace should reflect that reality so agents discover the right workflow from the project name.

## Decisions

### 1. Canonical skill name is `cairn-workflow`

**Choice:** Maintain a live Hermes skill named `cairn-workflow` and a repository source file named `docs/hermes/cairn-workflow-skill.md`.

**Rationale:** The name matches the repo, CLI, package, and user language. It prevents future agents from treating OpenSpec or Ironspec as the primary workstream.

### 2. Older `ironspec-workflow` becomes a redirect for Cairn

**Choice:** Keep the old local skill as a compatibility shim that points Cairn work to `cairn-workflow` rather than deleting it immediately.

**Rationale:** Historical prompts and references may still load `ironspec-workflow`; a redirect avoids silent loss of guidance while making the intended skill explicit.

### 3. Repo source remains copyable and audit-friendly

**Choice:** The repo source uses normal Hermes `SKILL.md` frontmatter, describes stable contracts, includes no credentials, and avoids transient session progress.

**Rationale:** It can be reviewed and copied into local Hermes skill storage without requiring Cairn runtime integration with Hermes internals.

### 4. Verification is docs/spec focused

**Choice:** Verify with Cairn validation, proposal/design/tasks gates, Tracey coverage, diff checks, and live `skill_view` for the new skill.

**Rationale:** This slice changes workflow/docs and live skill metadata, not Rust runtime behavior.
