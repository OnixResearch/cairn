# Design: Release-readiness receipt bundle

## Context

Cairn's recent agent and MCP work created the pieces needed for strong release evidence:

- `cairn validate --root .` proves lifecycle/spec validity.
- `tools/check-tracey-coverage.py --root .` proves accepted requirements have implementation or verification references.
- `agent-receipt smoke-mcp --root .` proves the MCP catalog, strict argument rejection, agent receipt validation, and default mutation denial compose into a stable receipt.
- Workspace tests, Clippy, Slotcar, and flake checks prove the repository implementation and packaged checks are healthy.

Today those checks are independent. A maintainer can run them, but no single Cairn artifact says which readiness checks were evaluated, which hashes were produced, or whether the checkout is ready for release/operator handoff.

## Decisions

### 1. Additive operator command

Add a public command such as:

```sh
nix run .#cairn -- release-readiness --root .
```

The exact command name may be refined during implementation, but the surface must be a local operator command that emits JSON. It should avoid network dependencies and should not require a long-running MCP client.

### 2. Bundle existing proof surfaces instead of inventing a new gate

The bundle should compose existing proof surfaces rather than duplicating their validation rules. It may invoke or embed receipts for Cairn validation and MCP smoke behavior, and it should record externally-run repository gates as explicit check entries when fully running them inside the command would create recursion or high cost.

### 3. Deterministic receipt shape

The bundle receipt should include stable fields such as:

- `command`
- `root`
- `valid`
- `verdict`
- `checks`
- `cairn_validation_hash`
- `tracey_coverage_hash` or structured Tracey summary hash
- `mcp_smoke_receipt_hash`
- `repository_gate_summary`
- `input_hash` or checkout state identity where available
- `receipt_hash`

Per-check entries should include a stable name, pass/fail status, source command or artifact, and any cited receipt hash. The bundle must not depend on human prose summaries as the only evidence.

### 4. Failure reporting stays transparent

If a component check fails, the bundle should emit `valid=false` / failing verdict with a deterministic issue/check entry that names the failed component. It should not discard underlying receipt hashes or force operators to scrape stderr to understand what failed.

### 5. FCIS boundary

Pure readiness summarization should live in the deterministic core where practical. Filesystem access, child command execution, environment handling, stdout/stderr, and JSON rendering remain in the CLI shell.

## Risks and Mitigations

- **Long-running recursive gates**: keep the bundle focused on local receipt evidence and record heavyweight gate status explicitly; avoid invoking `nix flake check` from inside a flake check path unless implementation proves it safe.
- **Receipt drift**: assert exact top-level fields and hash-shaped values in tests.
- **False readiness claims**: failing or missing required check evidence must produce `valid=false` and a deterministic issue.
- **Over-coupling to prose output**: consume structured receipts and hashes wherever possible.
