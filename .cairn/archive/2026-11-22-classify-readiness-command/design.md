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

### Decision: Check the shipped workflows' command routes in a test, not at admission

**Choice:** A test over Cairn's shipped default policy
(`cairn-policy/generated/cairn-policy.json`) reads every descriptor's goal,
success evidence, required evidence, progress rule, immediate action, output
shape, constraints and non-claims, and fails if a named `cairn` command
resolves to no catalog entry or to an entry whose class the descriptor does not
allow. The scanner lives in a `#[cfg(test)]` module
(`generated_agent_workflow/command_routes.rs`). Policy admission does not read
descriptor text.

**Rationale:** The defect was in Cairn's own shipped text, which Lattice and
Kiln embed unchanged, so a test over that text catches it where it is written.
A first draft (`8718b567`) put the scan in admission, which every policy load
runs. Two review rounds then found Markdown shapes it misread in both
directions (a fence label read as `cairn cairn`, `**cairn gate tasks**` missed,
then `**cairn** gate tasks`, link labels and `_cairn status`). A heuristic
reading of free text must not decide whether a consumer's policy loads; a miss
in a test over known text is fixed by the next test case, while a miss at
admission refuses or admits consumer policies Cairn has never seen. That
admission refusal is removed.

### Decision: Read descriptor text as Markdown

**Choice:** Each line is split into words and breaks. Code fence lines (three
or more backticks or tildes) are dropped, so an info string is never a word;
fenced lines are read, and a line break inside a paragraph is a space. Paired
emphasis runs of `*` or `_` (an opener before a non-space, a closer after one,
paired like CommonMark delimiters) are removed without a break, so
`**cairn** gate tasks` and `cairn **gate** tasks` read as `cairn gate tasks`.
An unpaired or intraword marker stays in its word, so `_cairn` and
`snake_cairn` are not `cairn`. A code span's content and a link label are read
and their edges are breaks; a link destination or reference label is skipped.
Parentheses, brackets, braces, quotes, and `.`, `,`, `:`, `;`, `!`, `?` before
a space or the line end are breaks. A mention is the word `cairn` followed by
at least one command word: lowercase ASCII letters, digits and hyphens starting
with a letter.

Catalog invocations are read the same way into a trie keyed by command word.
A mention walks the trie one word at a time and resolves to the deepest node
with entries, so `cairn workflow status` is `workflow-status`, not `status`;
each following word is looked up once, and no prefix is compared again. A
mention whose first command word has no entry is uncatalogued.

**Rationale:** Paired-delimiter emphasis and code and link edges follow how
Markdown renders the text an agent reads. Requiring lowercase command words
keeps placeholders (`CHANGE`, `<change>`) and options out of the match, and
requiring the lowercase word `cairn` keeps "Cairn command classes" prose out.
The trie keeps the cost per mention at the catalog depth.

### Decision: Report the field, the command words and the catalog entry

**Choice:** A violation carries the descriptor field path (for example
`descriptors[0].immediate_action` or `constraints[2]`), the workflow ID, the
matched command words, and the catalog ID and class; an uncatalogued mention
names its first command word.

**Rationale:** A failing test names the text to fix.

### Decision: Bound identifiers in admission and lookup diagnostics

**Choice:** Admission and lookup diagnostics copy at most 64 bytes of a catalog
ID, command class, route, workflow, adapter or profile name, followed by
`...(truncated, N bytes)`.

**Rationale:** Catalog IDs and classes have no length bound of their own, so a
diagnostic must not copy them whole; the field path and the prefix still
identify them.

## Risks / Trade-offs

- A consumer policy whose own workflow text names commands outside its classes
  loads; only Cairn's shipped text is checked. Consumers that edit workflow
  text own it.
- Prose that uses lowercase `cairn` followed by an ordinary lowercase word
  reads as a command and fails the test. Cairn's descriptor authors can
  capitalize the product name or quote an exact catalog command.
- The scanner follows CommonMark for the shapes it handles, not every rule
  (for example, delimiter-run length matching). A missed shape is a test gap,
  never a consumer load failure.
