# Design: Repo-Maintained Hermes Skill Source

## Context

Hermes skills are operational knowledge, not runtime code. The `cairn` repo should carry a canonical skill artifact so future agents can load/update a workflow that matches the repo's current contracts.

## Decisions

### 1. Store a copyable skill artifact under docs

**Choice:** Add `docs/hermes/cairn-workflow-skill.md` containing YAML frontmatter and a concise `SKILL.md` body.

**Rationale:** The artifact is reviewable with code changes, easy to copy into `~/.hermes/skills/`, and does not require the `cairn` CLI to know about Hermes internals.

**Alternative:** Only update live Hermes memory/skills. Rejected because that is not repo-auditable and cannot be validated by repo gates.

### 2. Keep the skill procedural, not session-log-shaped

**Choice:** Include stable triggers, FCIS rules, OpenSpec workflow, verification gates, and known pitfalls. Exclude transient session history.

**Rationale:** Hermes skills should guide future work without bloating prompt context or encoding stale task progress.

### 3. Validate via existing docs/spec gates

**Choice:** Use existing `cairn validate`, Rust checks, Slotcar, and Nix flake checks.

**Rationale:** This is a docs/spec artifact, so no new runtime test harness is needed. The EARS-style validation gate ensures the OpenSpec change remains well-formed.

## Risks

- Skill drift: mitigated by keeping the skill artifact in the repo and updating it when workflow contracts change.
- Over-specific instructions: mitigated by writing class-level workflow rules and avoiding one-off session details.
