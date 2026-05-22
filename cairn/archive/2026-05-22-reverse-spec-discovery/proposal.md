## Why

Existing projects often start with code, tests, and docs but no formal Cairn specs. Competitors and ecosystem tools can help reverse-engineer draft specs from existing artifacts. Cairn currently imports OpenSpec but lacks a deterministic discovery rail for proposing Cairn specs from current repository evidence.

## What Changes

- Add a read-only reverse-spec discovery command that inventories selected source/docs/tests and emits candidate spec requirements with evidence links.
- Require explicit include paths and bounded size limits so discovery is deterministic and safe for agents.
- Keep output as reviewable draft evidence, not accepted truth, until an operator turns candidates into a normal Cairn change.

## Impact

- **Files**: CLI discovery command, pure evidence extraction/planning core, policy-owned limits and ignore rules, fixtures, docs.
- **Testing**: Deterministic inventory tests, redaction/limit tests, candidate receipt golden tests, and no-mutation tests.
