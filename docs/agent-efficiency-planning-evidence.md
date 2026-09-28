# Agent efficiency planning evidence

## Scope

These receipts validate authored plans, not implemented product behavior.
The worktree starts from `origin/main` at `cb0be66e0e992c0f4070956bb9d451d8a8fe7573`.
The branch is `plan/cairn-agent-efficiency-20260906`.

The [roadmap](agent-efficiency-roadmap.md) lists the packages and their owners.
Each package retains proposal, design, and tasks receipts under `evidence/planning/`.
Shared receipts are in `evidence/agent-efficiency-planning/`.

## Commands and results

Cairn was built from the base checkout before the change skeletons were created:

```text
nix build path:$PWD#cairn --no-link --print-out-paths
```

The resulting CLI package is `/nix/store/3283rffavlm6in8s6l031rzpid52s95w-cairn-0.1.0`.
The build emitted an unknown `wasm-builtin` feature warning and completed successfully through the configured builder.
No flake inputs, lockfiles, product source, or policy changed.

With that package's `bin/cairn` as `CAIRN`, these commands ran from the worktree:

```text
"$CAIRN" validate --root .
"$CAIRN" validate --root . --strict
"$CAIRN" change graph --root .
"$CAIRN" policy export --check
"$CAIRN" gate proposal CHANGE --root .
"$CAIRN" gate design CHANGE --root .
"$CAIRN" gate tasks CHANGE --root .
git diff --check
```

`CHANGE` selected each package in the roadmap. Gate commands did not run implementation checks.

Observed planning results:

- The baseline repository validation passed before package creation.
- Final strict validation passed with eight active packages.
- The native dependency graph was valid.
- All proposal, design, and tasks receipts reported `valid: true`, `verdict: PASS`, and empty issues.
- The task receipts retained 88 open implementation tasks and no completed tasks.
- Generated policy remained fresh.
- The whitespace check passed.

A cached receipt checker admitted the top-level verdicts, exact change and stage identities, and open-task state.
Its controls reject a false top-level result even when a nested result is true.
The checker did not infer gate success from command exit status or receipt presence.
A separate cached prose check found no sentences above its descriptive word limit in the new proposals, designs, specifications, and roadmap.

## Source and audit limits

The initial MCP duplication probe used an older installed binary. Current source confirmed the same duplicated serialization path.
That probe supports the proposed mechanism, not a current-head token benchmark or improvement claim.
Two attempted independent agent reviews hit provider quotas during the initial audit.
The authored plans therefore received local source review and deterministic planning checks, not an independent semantic approval.

No implementation tests, full workspace test suite, Clippy run, or full Nix check are claimed for this planning-only commit.
Those checks remain explicit open tasks for the relevant implementations.
No package was synchronized, archived, pushed, or marked complete.
