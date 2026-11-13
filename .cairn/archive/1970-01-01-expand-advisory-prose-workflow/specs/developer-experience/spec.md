## ADDED Requirements

### Requirement: Explicit advisory presentation formats
r[cairn.prose_workflow.presentation] WHEN an operator selects text, GitHub annotations, or SARIF presentation, Cairn MUST emit deterministic review-only feedback without source mutation, automatic fixes, publication, or grammar-based lifecycle failures, while preserving default JSON v1 and existing machine JSON semantics.

#### Scenario: Human and CI consumers share the same findings
- GIVEN admitted prose containing an upstream error
- WHEN text, GitHub, SARIF, and opt-in JSON v2 are requested
- THEN source locations, rule identities, suggestions, and review requirements SHALL agree
- AND GitHub and SARIF findings SHALL remain non-blocking warnings or notes
- AND untrusted paths and messages SHALL NOT inject terminal controls or additional annotations

### Requirement: Confined change-scoped authoring selection
r[cairn.prose_workflow.selection] WHEN `--change NAME` is supplied instead of explicit files, Cairn MUST select only existing proposal, design, tasks, and sorted delta `spec.md` files from that active native change, with confined no-follow bounded traversal and the existing file-count and per-file limits.

#### Scenario: Selection excludes unrelated content
- GIVEN active change artifacts, delta specs, archived changes, vendored content, and evidence
- WHEN the active change is selected
- THEN only its authoring documents SHALL be checked in deterministic order
- AND unsafe names, links, traversal exhaustion, and incompatible explicit inputs SHALL fail without a successful report

### Requirement: Exported profiles and narrow suppressions
r[cairn.prose_workflow.configuration] WHEN an operator configures prose rules, Cairn MUST admit a bounded closed Nickel-exported profile collection, validate pinned upstream selectors, record effective configuration in v2, and permit only reasoned bounded next-physical-line suppressions without inserting suppression text.

#### Scenario: Rule selection and suppression remain reviewable
- GIVEN a profile selecting agreement findings and a valid next-line directive with a reason
- WHEN the document is checked
- THEN selected findings SHALL be filtered only by the declared profile and directive scope
- AND unknown rules, malformed directives, duplicate or unbounded configuration, and stale exports SHALL fail
- AND directives in ignored code SHALL NOT suppress ordinary prose

### Requirement: Content-bound reports and current-input rechecking
r[cairn.prose_workflow.binding] WHEN JSON v2 is requested, Cairn MUST bind original file bytes, engine cohort, effective profile, selection, bounds, and complete report through BLAKE3 identities, and MUST independently recheck current confined inputs before declaring a saved report current.

#### Scenario: Stale or changed suggestions are rejected
- GIVEN a saved content-bound report
- WHEN input bytes, selected files, effective configuration, engine identity, or report findings differ from current recomputation
- THEN rechecking SHALL fail without claiming current suggestions
- WHEN all bound facts and recomputed findings match
- THEN rechecking SHALL succeed regardless of grammar findings without claiming authenticity, approval, or lifecycle evidence

### Requirement: Original positions survive technical-document projection
r[cairn.prose_workflow.documents] WHEN technical documents contain supported fences, leading front matter, comments, or AsciiDoc listing blocks, Cairn MUST exclude opaque regions while preserving original UTF-8 byte spans, Unicode-scalar positions, and line endings for findings in prose.

#### Scenario: Prose after opaque Unicode blocks keeps its position
- GIVEN YAML or TOML front matter, backtick or tilde fences, AsciiDoc source blocks, CRLF, Unicode, and a prose defect outside those regions
- WHEN the check completes
- THEN opaque examples SHALL NOT produce findings or active suppression effects
- AND finding and replacement positions SHALL address the unchanged original document

### Requirement: Bounded complete reports
r[cairn.prose_workflow.bounds] WHEN prose inputs or output exceed declared aggregate byte, diagnostic, traversal, or report bounds, Cairn MUST fail without a successful partial report, and operator-selected bounds MUST NOT exceed documented hard ceilings.

#### Scenario: A bound is an admission decision, not silent truncation
- GIVEN documents or findings exactly at an admitted limit
- WHEN the check runs
- THEN the complete report SHALL be emitted
- GIVEN the corresponding limit is exceeded
- WHEN the check runs
- THEN the command SHALL fail before emitting a successful report

### Requirement: Requirement identities and meaning-change review reasons
r[cairn.prose_workflow.requirement_context] WHEN enriched findings occur within an identifiable native requirement, Cairn MUST reuse existing traceability marker admission to attach the containing requirement ID and flag changes involving obligation words, negation, numbers, units, conditions, quantifiers, or requirement IDs as review reasons, not semantic verdicts.

#### Scenario: Context never leaks across sibling sections
- GIVEN requirement declarations, sibling sections, code examples, and grammar suggestions
- WHEN enriched findings are produced
- THEN only unambiguous containing requirement IDs SHALL be attached
- AND potential meaning-change reasons SHALL concern changed sensitive tokens rather than unrelated unchanged section text
- AND all suggestions SHALL still require author review

### Requirement: Repeatable bounded integration evidence
r[cairn.prose_workflow.verification] WHEN the expanded feature is integrated, Cairn maintainers MUST retain consumer-visible positive and negative regression coverage, live CLI observations, generated-profile freshness evidence, and repository-required lifecycle verification while preserving explicit non-claims.

#### Scenario: Completion is tied to checked behavior
- GIVEN implemented authoring, configuration, content binding, and export paths
- WHEN focused checks and required gates pass
- THEN the change SHALL be synchronized and archived with observed evidence
- AND no grammar result SHALL be promoted to requirement satisfaction or release eligibility

## MODIFIED Requirements

### Requirement: Confined explicit inputs
r[cairn.prose_check.inputs] WHEN prose inputs are supplied explicitly or through an admitted active `--change` selection, Cairn MUST admit only 1 through 64 root-relative regular UTF-8 files of at most 1 MiB each and reject invalid arguments, symlinks, path escapes, unreadable files, and oversized files without emitting a successful report.

#### Scenario: Read only admitted selected files
- GIVEN a root with ordinary documents, active change artifacts, and a symlink outside the root
- WHEN an operator supplies explicit files or selects an active change
- THEN Cairn SHALL report only the admitted selected documents
- AND explicit-file mode SHALL NOT traverse directories
- WHEN an operator selects a symlink, an invalid document, or an unsafe change
- THEN Cairn SHALL fail without a success report
