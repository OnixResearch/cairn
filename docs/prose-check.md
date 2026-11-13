# Advisory prose checks

`cairn prose check` offers offline grammar feedback on specifications, proposals,
and public documentation. It uses [`englishc`](https://github.com/Aaryan-Kapoor/english-rs),
pinned to `336ed696eec73e5b9858c5d7952b54c8e93b9df9`. Checking and rechecking need
no additional executable, network call, model, or runtime Nickel evaluation.

```bash
cairn prose check --root . README.md docs/prose-check.md
cairn prose check --root . --change add-friendly-start --format text
cairn --machine prose check --root . --report-version 2 README.md
```

Findings, including upstream `error` findings, return success. Success means the
requested check completed, not that the prose is correct. Argument, configuration,
path, encoding, read, analysis, and resource-limit failures return nonzero without
a successful partial report. No output is emitted until the whole check and
selected output fit the admitted bounds.

The command never rewrites selected documents. `--fix` is rejected. It is not part
of validation, lifecycle gates, readiness, archive, or completion decisions.
Cairn CLI maintainers own selection, configuration, reports and adapters;
english-rs owns grammar rules. Grammar is not a second EARS validator.

## Selection and hard bounds

Explicit paths are relative to `--root` (default `.`), preserve supplied order,
and normalize `.` components. Absolute paths, `..`, symlinks, directories and
non-UTF-8 documents are rejected. Explicit-file checking needs no `.cairn/` layout
and does not traverse directories.

Instead of explicit files, `--change NAME` selects existing `proposal.md`,
`design.md`, `tasks.md`, and recursively selected `specs/**/spec.md` from that
active native change. The final selection is path-sorted. It does not select
archives, evidence, or repository-wide vendored content. Names use existing Cairn
admission; traversal is no-follow and bounded. Explicit files and `--change`
cannot be combined.

| Resource | Hard ceiling | Operator lowering option |
|---|---:|---|
| Selected files | 64 | Select fewer files |
| One document | 1 MiB | Select smaller documents |
| Aggregate document bytes | 16 MiB | `--max-total-bytes N` |
| Selected diagnostics | 4096 | `--max-diagnostics N` |
| Rendered output, including final newline | 1 MiB | `--max-report-bytes N` |
| Change traversal entries | 16384 | Narrow the change package |
| Change traversal path components | 64 | Flatten the delta tree |
| Profile collection | 64 KiB, 16 profiles | Keep configuration small |

Numeric options must be positive and cannot raise their ceiling. Diagnostic
limits apply after profile filtering and suppressions. Exhaustion fails; it does
not silently truncate a successful report. Machine capture bounds additionally
apply: see [the machine contract](cli-machine-contract.md).

## Output formats and compatibility

`--format json` is the default. Its default report remains
`cairn.prose-check.v1`, including the original fields and machine behavior:

- `advisory: true`, `mutated: false`, and the pinned engine identity.
- `files` with normalized paths and upstream-ordered diagnostics.
- Diagnostic `code` (nullable), `lint`, upstream `level`, `message`, `labels`,
  `notes`, and `helps`.
- Zero-based UTF-8 byte spans with exclusive ends; one-based lines and columns.
  Columns count Unicode scalar values, not display cells or UTF-16 code units.
- Suggestions containing byte ranges, `replacement`, and `review_required: true`.
- Non-claims: grammar does not establish requirement validity, consistency,
  completeness, satisfaction, implementation, verification, readiness, or release.

Other explicit formats use the enriched diagnostic model:

```bash
cairn prose check --root . --format text README.md
cairn prose check --root . --format github README.md
cairn prose check --root . --format sarif README.md > prose.sarif
```

Text includes source locations, original severity, review-only suggestions, and
identities. Terminal controls and bidirectional formatting controls are escaped.
GitHub emits escaped `::warning` annotations, never grammar `::error` commands.
SARIF 2.1.0 carries warning/note findings, correctly encoded root-relative artifact
URIs, original byte regions, Unicode-code-point columns, content hashes and review
metadata. It does not expose automatic fixes. SARIF can be consumed by editors or
an operator-selected CI uploader; Cairn does not upload or publish anything.
Downstream CI policy may impose its own thresholds independently of Cairn.

`--machine` continues to accept only JSON presentation. It nests either JSON
version in the existing envelope; `ok` and `exit_class` describe command completion,
not prose validity. `--report-version` controls JSON output; non-JSON formats
always use enriched diagnostics.

## Rule profiles and scoped suppressions

The bundled generated collection comes from `cairn-policy/prose.ncl`. `default`
selects all upstream rules; `agreement` selects `E0001`. A profile contains a
bounded `name`, `include_rules`, and `exclude_rules`. Empty inclusion means all;
exclusions win. Selectors are pinned upstream codes, lint names, or `warnings`.
Unknown selectors, duplicate names/selectors, unsupported schemas, and unknown
fields fail admission. Each selector array permits at most 64 entries.

```bash
cairn prose check --root . --profile agreement README.md
cairn prose profiles export --check
cairn prose profiles export --source config/prose.ncl --output config/prose.json
cairn prose check --root . --profiles config/prose.json --profile technical README.md
```

Author custom collections in Nickel with schema `cairn.prose-profiles.v1` and a
`profiles` array using the same shape as the bundled source. Export runs the
existing pinned Nickel/nickel-export parity boundary and Rust admission. It
explicitly writes the named generated destination; its parent must already
exist. `--check` compares exact generated bytes without writing. Source and
destination use the existing portable, no-symlink export boundary. Checks read
custom profile JSON through the selected capability root; they do not evaluate
Nickel. Configuration selection never promotes warning severity into a gate.

Prefer a reasoned, narrow exception to disabling a rule across a document:

```markdown
<!-- cairn-prose-ignore-next-line E0001 -- Technical subject is intentional. -->
He are late.
```

The own-line directive affects only the next physical line. Comma-separated codes
or lint names are accepted; up to 16 selectors and a 1–256-byte plain-text reason
are required. At most 64 directives are admitted per document. Malformed or
unknown directives fail rather than silently suppressing findings. Directive text
nested inside other comments or opaque examples is not active. Existing upstream
attributes such as `#![allow(E0001)]` remain supported; Cairn never inserts either form.

## Content-bound v2 and stale-suggestion checks

```bash
cairn prose check --root . --report-version 2 README.md > prose-report.json
cairn prose verify --root . --report prose-report.json
```

`cairn.prose-check.v2` adds original-file `content_blake3`, effective profile
settings/name/identity/source, selection, bounds, and `report_blake3`. It adds end
positions and bounded excerpts to labels, start/end positions and review risks
to suggestions, and requirement context and review risks to diagnostics.

Profile identity uses Cairn canonical JSON hashing over the versioned profile
material. Report identity uses canonical JSON hashing over the entire report
except `report_blake3`. Input hashes cover exact original UTF-8 bytes, including
line endings, ignored examples and suppression reasons—not projected text.

`prose verify` reads a bounded root-relative v2 report, checks its cohort and
profile identity, loads the current selected profile (bundled or custom JSON),
reselects the current change or explicit file set, and recomputes the whole report.
Changed bytes, file membership, configuration, suggestions, metadata, unknown
fields, or hashes fail. Recomputing a tampered report's hash is insufficient.
V1 has no content binding and cannot be passed to this verifier.

Success emits `cairn.prose-freshness.v1` with `current: true`. This proves only that
the bounded observations and recomputed diagnostics match; it is not a signature,
approval, atomic filesystem snapshot, safe-application permission, lifecycle
receipt, or release evidence. All suggestions still require author review.

## Technical documents and requirement context

The byte-preserving projection excludes supported backtick/tilde fences, closed
leading YAML/TOML front matter, HTML comments, and AsciiDoc source/listing blocks.
Bare listing delimiters apply to `.adoc`/`.asciidoc` paths; explicit `[source]` or
`[listing]` attributes also identify listing blocks in mixed-format documents.
Markdown dividers remain prose boundaries rather than hiding later findings.
CR/LF positions are retained. Upstream inline backtick and URL handling is reused.
This is not a full Markdown or AsciiDoc parser; unsupported syntax remains input.

Enriched diagnostics associate unambiguous native Markdown `### Requirement:`
blocks with admitted bare requirement markers, reusing Cairn traceability marker
parsing. Ownership does not cross sibling/higher headings or originate in code
examples or references. This association is not requirement validation.

Changes to sensitive tokens may add review-risk codes `obligation`, `negation`,
`number`, `unit`, `condition`, `quantifier`, or `requirement_id`. Classification
compares the changed text and replacement, not unrelated keywords elsewhere in a
section. Ambiguous single-letter unit symbols need quantity context or an explicit
unit-to-unit edit. These are heuristics: false positives and missed risks remain
possible. They do not resolve contradictions or establish semantic equivalence.

Diagnostics can contain document excerpts. Treat reports and annotations with the
same confidentiality as their selected documents. No automatic fixes, grammar
readiness gates, quality scores, external agents, or model calls are introduced.

<!-- r[impl cairn.prose_check.advisory] r[impl cairn.prose_check.inputs] -->
<!-- r[impl cairn.prose_check.diagnostics] r[impl cairn.prose_check.read_only] -->
<!-- r[impl cairn.prose_workflow.presentation] r[impl cairn.prose_workflow.selection] -->
<!-- r[impl cairn.prose_workflow.configuration] r[impl cairn.prose_workflow.binding] -->
<!-- r[impl cairn.prose_workflow.documents] r[impl cairn.prose_workflow.bounds] -->
<!-- r[impl cairn.prose_workflow.requirement_context] r[impl cairn.prose_workflow.verification] -->
