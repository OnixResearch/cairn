# Advisory prose check design

## Context

englishc provides pure in-process diagnostics for a deliberately limited English grammar.
Its terminal output is not a stable machine contract and its automatic fixes can change normative meaning.

## Decisions

### Library boundary

Pin englishc at `336ed696eec73e5b9858c5d7952b54c8e93b9df9` and call `englishc::check` with its default configuration.
Keep grammar ownership upstream; do not copy rules, parse terminal text, spawn tools, or introduce a second requirement validator.
Expose reports only through an explicit CLI command; normal gates never invoke the checker.

### Input and output

Accept 1 through 64 explicit root-relative regular UTF-8 files of at most 1 MiB each.
Reuse CapabilityRoot bounded no-follow reads and existing argument admission.
Emit one `cairn.prose-check.v1` JSON report after all requested inputs succeed.
Retain upstream lint IDs, severity, labels, notes and help while mapping source spans to byte offsets and 1-based Unicode-scalar line/column coordinates.
Mark every suggestion as requiring author review regardless of upstream applicability.
Return success for findings, including upstream errors; invalid arguments or unreadable inputs remain command failures.
The existing machine envelope nests this advisory result unchanged.

## Risks / Trade-offs

- Heuristic grammar can miss defects and flag valid technical language; findings remain advisory.
- Documents and diagnostic excerpts may contain private text; output stays local unless an operator exports it.
- Reports are not signed or content-bound evidence and do not establish requirement satisfaction.
- Grammar rule upgrades require an explicit source/hash update and consumer check replay.
