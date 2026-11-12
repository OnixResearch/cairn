# Advisory prose checks

`cairn prose check` offers optional local grammar feedback on specifications,
proposals, and public documentation. It uses the upstream
[`englishc` library](https://github.com/Aaryan-Kapoor/english-rs), pinned to
`336ed696eec73e5b9858c5d7952b54c8e93b9df9`. No additional executable,
network call, or model is needed at runtime.

```bash
cairn prose check --root . README.md docs/prose-check.md
cairn prose check --root . .cairn/changes/CHANGE/proposal.md
cairn --machine prose check --root . README.md
```

The checker runs only when requested. It is not part of `validate`, lifecycle
gates, readiness, archive, or completion decisions. Cairn CLI maintainers own the
integration; english-rs owns the grammar rules. Cargo and Nix pin the upstream
source and its packaging hash for repeatable rule selection.

## Inputs and failure behavior

Supply 1 through 64 explicit file paths relative to `--root` (the current directory
by default). Paths normalize `.` components; absolute paths, `..` components,
symlinks, and directories are rejected. Each file must be regular UTF-8 text of
at most 1 MiB. There is no directory walk or implicit file selection. A `.cairn/`
layout is not required for this authoring command.

Argument, path, encoding, and read failures return a nonzero exit status without
a successful report. Output is emitted only after every requested file has been
read and checked. Grammar findings, including upstream `error` findings, return
exit status zero. Success means the requested check completed, not that the prose
is correct or the lifecycle is ready.

`--fix` and other undocumented options are rejected. Cairn never rewrites an input.

## Report

Default output is one JSON object with schema `cairn.prose-check.v1`:

- `advisory: true` and `mutated: false` state the command boundary.
- `engine` identifies englishc and its pinned source revision.
- `files` preserves the explicit input order. Each row has the normalized `path`
  and `diagnostics` in upstream source order.
- Each diagnostic retains `code` (nullable), `lint`, upstream `level`, `message`,
  `labels`, `notes`, and `helps`.
- Labels contain `start` and `end` byte offsets into the original UTF-8 text,
  zero-based with an exclusive end. `line` and `column` identify the label start,
  both one-based; columns count Unicode scalar values, not terminal display cells.
- Help may contain a `suggestion` with a byte range, `replacement`, and
  `review_required: true`. Upstream automatic-fix applicability is not permission
  to rewrite a requirement.
- `non_claims` states that grammar findings establish neither requirement validity
  or satisfaction nor implementation, verification, readiness, or release evidence.

For example, a file containing only the following plain text produces an `E0001`
subject-verb-agreement diagnostic over the bytes of `are` and a suggestion of
`is`; the file remains unchanged:

```text
He are late.
```

The report is not a content-bound lifecycle receipt and has no `valid` verdict.
With `--machine`, the existing CLI machine envelope nests the report. Its `ok`
and `exit_class` describe transport completion, not prose or lifecycle validity.
Machine output remains subject to the existing capture bounds; split very large
reports into smaller invocations if necessary. See
[the machine contract](cli-machine-contract.md).

## Markdown and review

The pinned upstream lexer treats backtick code spans, backtick-fenced code blocks,
and URLs as opaque. It recognizes Markdown headings and lists; it is not a full
Markdown or AsciiDoc parser. In-source upstream suppressions are respected, for
example `#![allow(E0001)]` on its own line suppresses agreement findings for that
file. Cairn does not insert suppression text.

False positives and missed defects remain possible, especially in technical
language. Review suggestions before changing normative prose. The checker does
not validate EARS structure, resolve requirement contradictions, inspect
implementation, or prove satisfaction. Cairn's existing requirement and lifecycle
checks remain separate.

Diagnostics can contain document excerpts in labels or suggestions. They remain
local unless an operator exports the report; treat the output with the same
confidentiality as the selected documents.

<!-- r[impl cairn.prose_check.advisory] r[impl cairn.prose_check.inputs] -->
<!-- r[impl cairn.prose_check.diagnostics] r[impl cairn.prose_check.read_only] -->
