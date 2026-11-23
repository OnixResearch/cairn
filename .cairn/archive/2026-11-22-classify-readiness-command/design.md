# Design

## Context

`generated_agent_workflow_policy` declares `command_classes` (each `read` or
`mutation`), a `command_catalog` that maps command IDs to an invocation and one
class, and six descriptors with their allowed `command_classes`. Each rendered
workflow file lists the allowed classes under "Allowed Cairn command classes",
and every descriptor's constraints say "Use only explicit project roots and
policy-known Cairn command classes." Admission in
`crates/cairn-core/src/verified/generated_agent_workflow/admission.rs` checked
that catalog and descriptor classes exist, but never related the commands a
descriptor's text names to the catalog. The policy parser runs the same
admission on every policy load.

At `9082e1ae` the catalog had no entry in the `readiness` class, and
`cairn readiness` was in `gate`. The descriptors that name it are `apply`
(allows `readiness`, not `gate`), `verify` and `archive` (allow both). `drain`
names `cairn drain-plan --view readiness`, which the catalog omitted.

## Decisions

### Decision: `cairn readiness` belongs to the `readiness` class

**Choice:** Move the catalog entry to `readiness` rather than adding `gate` to
`apply`.

**Rationale:** The policy defines `readiness` as its own read class and gives
it to every workflow whose required evidence includes a "Current readiness
observation"; `gate` is the class of `cairn gate tasks`, which checks one
lifecycle stage. `apply` does not list `gate`: it is the implementation
workflow, and its success evidence is mapped code and recorded checks, not gate
passage. Adding `gate` to `apply` would widen its allowed commands to stage
gates to fix a misfiled entry. The history agrees: the catalog was written
before the `readiness` class existed, and the readiness adoption that added the
class never refiled the entry. `verify`, `archive` and `drain` allow both
classes, so they keep every command they had.

### Decision: Catalog the drain readiness route

**Choice:** Add `drain-plan-readiness`,
`cairn drain-plan --view readiness --root PATH`, in the `readiness` class.

**Rationale:** The readiness adoption made `cairn drain-plan --view readiness`
the drain workflow's readiness route; it reads readiness and changes nothing.
Rewording `drain` to avoid the command would drop the route the
`cairn.agent_workflow_readiness.guidance` requirement asks for. The entry
guide's routes are unchanged.

### Decision: Admit named commands against the catalog

**Choice:** For each descriptor, scan the goal, success evidence, required
evidence, progress rule, immediate action, output shape, constraints and
non-claims. Each token has Markdown emphasis, code spans, quotes, brackets and
parentheses (`*`, `_`, backticks, straight and curly quotes, `[]`, `()`)
stripped from both ends and `.`, `,`, `:` or `;` from its end; stripping at
the end closes the command at that token. A mention is a token equal to
`cairn` followed by at least one command word: a token of lowercase ASCII
letters, digits and hyphens that starts with a letter, up to the first other
token or closing token. Code fence lines (three or more backticks or tildes)
are skipped, so a fence info string such as `cairn` is never a mention and a
mention never runs across a fence; the lines inside a fence are scanned. Each
catalog invocation yields its command words the same way. The mention
resolves to the catalog entries with the longest command-word prefix of its
words; none is an issue, and each resolved entry whose class the descriptor
does not allow is an issue.

The text is tokenized once. Catalog entries are keyed by their command words,
and each mention reads at most the catalog's largest command-word count of
following tokens, then looks up its prefixes from longest to shortest. Work is
at most `tokens * (1 + depth)` token inspections per field. Each field keeps
only the `max_issues` smallest distinct messages, the ones the sorted,
truncated issue list can report.

**Rationale:** The defect is a relation between three policy fields, so the
check belongs in admission beside the existing class checks, where every
policy load and every consumer policy meets it. Scanning every rendered field
keeps the rule generic instead of special-casing `immediate_action`. Longest
prefix makes `cairn workflow status` resolve to `workflow-status`, not
`status`. Requiring lowercase command words keeps placeholders (`CHANGE`,
`<change>`) and options out of the match, and requiring the lowercase token
`cairn` keeps "Cairn command classes" prose out. Stripping wrapping keeps
`**cairn gate tasks**` from escaping the scan; skipping fence lines keeps a
` ```cairn ` label from reading as the command `cairn cairn`. An uncatalogued
command has no class a descriptor could allow, so it is refused too. Bounding
the lookahead by catalog depth keeps a field of repeated `cairn` tokens linear
instead of rescanning the rest of the field at each one.

### Decision: Diagnostics name the field and the command words

**Choice:** Issues carry the descriptor field path (for example
`descriptors[0].immediate_action` or `constraints[2]`), the matched command
words, the catalog ID and its class; an uncatalogued mention names only its
first command word. Any catalog ID, command class or command word longer than
64 bytes is copied as its first 64 bytes followed by
`...(truncated, N bytes)`, as are the identifiers in the other admission and
lookup diagnostics.

**Rationale:** An author can locate and fix the text from the issue alone.
Catalog IDs and classes have no length bound of their own, so a diagnostic
must not copy them whole; the field path and the prefix still identify them.

## Risks / Trade-offs

- A policy that still carries either defect, including an unmodified copy of
  the 9082e1ae default, fails to load with this binary. That is the intended
  fail-closed outcome; consumers re-vendor the default with the binary.
- Prose that uses lowercase `cairn` followed by an ordinary lowercase word,
  wrapped or not, reads as a command and fails admission. Descriptor authors
  can capitalize the product name or quote an exact catalog command.
- Scanning is linear in descriptor text, with a factor of the catalog's
  largest command-word count; resolution is a map lookup per prefix length.
