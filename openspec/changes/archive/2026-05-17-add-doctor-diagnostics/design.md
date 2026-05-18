## Context

`cairn validate` reports validation failures after layout and policy selection succeeds. Operators still lack a single read-only receipt that explains what layout was selected, whether expected directories exist, whether the policy file loaded, and how many specs/changes were inspected.

## Decisions

### 1. Add pure doctor snapshot/report types

**Choice:** Add a core `DoctorSnapshot` input and `DoctorReport` output that classify shell-supplied booleans, counts, and policy status.
**Rationale:** This preserves the FCIS boundary and makes diagnostics unit-testable without filesystem fixtures.
**Alternative:** Build the JSON receipt entirely in the CLI; rejected because policy for valid/failing diagnostics would become presentation logic.

### 2. Keep doctor read-only and bounded

**Choice:** `doctor` reads the selected layout, policy file, repo status, and validation inputs but performs no mutation or external process execution.
**Rationale:** Operators should be able to run it safely before deciding whether to initialize, migrate, or repair a repo.

### 3. Use validate-compatible issue counts

**Choice:** The CLI reuses existing validation functions for change/spec issue totals and includes counts in the doctor snapshot.
**Rationale:** `doctor` should summarize validation health while `validate` remains the detailed issue receipt.

## Risks / Trade-offs

**Receipt overlap with validate** → Keep `doctor` as a health summary with checks and counts, not a replacement for detailed validation.

**Policy read failure currently aborts other commands** → `doctor` catches policy load/parse errors and reports them in JSON instead of returning an error.
