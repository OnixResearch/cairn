# developer-experience Specification Delta

## ADDED Requirements

### Requirement: Advisory prose authoring
r[cairn.prose_check.advisory] WHEN an operator invokes `cairn prose check` with admitted files, Cairn MUST emit one advisory prose report and return success regardless of grammar findings without changing lifecycle validation, gate, readiness, or completion decisions.

#### Scenario: Findings are not lifecycle failures
- GIVEN a document with subject-verb disagreement
- WHEN the explicit prose check completes, including through `--machine`
- THEN the report retains the upstream error severity while the command succeeds
- AND the report states that grammar does not prove requirement validity or satisfaction

### Requirement: Confined explicit inputs
r[cairn.prose_check.inputs] WHEN prose inputs are supplied, Cairn MUST admit only 1 through 64 explicit root-relative regular UTF-8 files of at most 1 MiB each and reject invalid arguments, symlinks, path escapes, unreadable files, and oversized files without emitting a successful report.

#### Scenario: Read only the selected files
- GIVEN a root with ordinary documents and a symlink outside the root
- WHEN an operator selects ordinary documents
- THEN Cairn reports only those documents without traversing directories
- WHEN an operator selects the symlink or an invalid document
- THEN Cairn fails without a success report

### Requirement: Structured diagnostic positions
r[cairn.prose_check.diagnostics] WHEN englishc reports a finding, Cairn MUST retain its lint identity, severity, labels, notes and help, publish zero-based UTF-8 byte spans with exclusive ends and 1-based Unicode-scalar start positions, and identify the pinned engine revision.

#### Scenario: Preserve technical source locations
- GIVEN a Markdown document with Unicode text and a grammar defect outside code
- WHEN Cairn checks the document
- THEN the diagnostic spans identify the original source bytes
- AND code spans, fenced code and URLs follow upstream prose exclusions

### Requirement: Review-only suggestions
r[cairn.prose_check.read_only] WHEN a prose check runs, Cairn MUST leave source bytes unchanged, reject automatic-fix options, and mark all returned suggestions as requiring author review.

#### Scenario: Suggested edits are not applied
- GIVEN an upstream finding with a replacement suggestion
- WHEN an operator requests a prose report
- THEN the report includes the suggestion without changing the document
- AND an attempted `--fix` invocation fails without mutation
