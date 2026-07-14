## Why

Cairn combines an embeddable pure lifecycle core with a complete operator CLI and MCP shell. Applying one permissive license to both surfaces gives hosted or distributed shell forks no reciprocity requirement, while applying AGPL to the whole workspace would unnecessarily constrain `cairn-core` consumers and copied lifecycle templates.

## What Changes

- Keep `cairn-core`, lifecycle schemas, examples, and copyable templates under `MIT OR Apache-2.0`.
- License `cairn-cli` under `AGPL-3.0-or-later`.
- Add complete license texts and an explicit repository boundary document.
- Document that tool output is not automatically relicensed, while copied template content retains its stated permissive terms.

## Impact

The CLI gains network/distribution reciprocity without changing the embedding contract for the pure core or generated project content.
