## Context

`cairn change create` creates empty markdown files. Agents then fill them manually, and repeated failures usually come from unknown task refs, missing EARS clauses, or proposal/design sections that do not satisfy gates. A deterministic scaffold layer can remove this class of mistakes without embedding an LLM in Cairn.

## Decisions

### 1. Structured deterministic scaffolding, not built-in AI inference

**Choice:** Accept explicit request text, optional domain, and optional operator-provided bullet constraints; generate a deterministic draft package from templates and policy rules.

**Rationale:** Cairn should produce auditable scaffolds and input hashes. Agents may summarize intent before invoking the command, but Cairn must not depend on network LLM calls.

### 2. Pure planner plus thin filesystem shell

**Choice:** Put draft package planning, ID normalization, task-reference construction, and issue detection in `cairn-core`; keep file writes and JSON rendering in `cairn-cli`.

**Rationale:** This preserves FCIS while making the hard parts directly testable.

### 3. Gate-aware output

**Choice:** Scaffold proposal, design, tasks, and delta spec files with known requirement IDs and EARS-shaped scenario placeholders.

**Rationale:** The first generated draft should be reviewable and gateable before implementation, not just an empty folder.

## Risks / Trade-offs

- The command can only structure intent; it must not claim semantic completeness or implementation design correctness.
- Free-form prompts can be ambiguous; deterministic receipts should surface assumptions and missing-input issues rather than inventing hidden requirements.
