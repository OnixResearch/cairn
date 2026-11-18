# Classify semantic requirement and policy revisions

## Why

Cairn binds requirements and policy to exact content revisions, but it cannot explain whether a later revision means the same thing, strengthens an obligation, weakens it, or makes comparison unsupported.
Without that distinction, reviewers can reuse evidence across a strengthening change, miss a weakening, or treat a changed finite structure as an editorial edit.
Issue #9 and Roadmap #4 require an advisory comparison that binds both revisions and keeps every original applicability check authoritative.

## What Changes

- Add a pure comparison of two normalized typed requirement contracts under an explicit common domain, value and observable correspondence.
- Enumerate the declared finite observation worlds and check both implication directions, retaining replayable distinguishing witnesses.
- Classify the new obligation as `editorial`, `clarifying`, `strengthening`, `weakening`, `incompatible`, or `undetermined`.
- Report assumptions, activation and guarantee changes separately from the overall obligation relation.
- Add one supported acceptance-policy fragment over named required checks, admissible evidence roles and allowed assumption sets, with accepted-evidence universe, containment, feasibility and witnesses.
- Emit one `reusable` or `not_reusable` decision per retained evidence claim, with the old receipt marked stale and transfer disallowed.
- Define a future evidence-transfer record shape while keeping automated transfer disabled.
- Add read-only `cairn revision classify` inspection over bounded files.
- Put every admitted bound in a standalone Nickel policy export.

## Impact

Current consumers: maintainers reviewing a requirement or selected acceptance-policy change, and the #10 support-maintenance workstream that consumes revision facts without automatic transfer.
Immediate outcome: one exact pair yields a deterministic classification, witness basis and evidence-reuse decisions.
Durable contribution: a bounded revision comparison surface that later derivation and qualification work can cite.

Cairn owns typed meaning, comparison semantics and policy bounds. Existing native applicability validators, verification roles, readiness and gate decisions keep their meanings.

## Out of Scope

- Prose similarity, natural-language inference or arbitrary Nickel program comparison
- Unbounded theorem proving, reactive realizability or whole-system correctness
- Automatic evidence transfer, receipt promotion, release authorization or obligation discharge
- Replacing the native requirement registry or any existing completion decision

Dependencies: typed requirement contracts [#5], scoped claim derivations [#6], and the query/witness discipline from consistency analysis [#8]. Implementation remains open for parent verification.

## Acceptance

- The finite corpus distinguishes equivalent, stronger, weaker, incomparable, unsupported and inconclusive requirement changes.
- Tightened and widened policy acceptance are distinct, and every failed containment direction retains a replayable candidate.
- Assumption changes are reported separately from guarantee and overall obligation changes.
- Every policy comparison names its supported fragment, accepted-evidence universe, feasibility and empty-set status.
- Old receipts remain not current after every revision, including an equivalent edit.
- Advisory output never discharges an obligation, omits a required check, authorizes release, transfers evidence or changes a lifecycle decision.
- Automated transfer remains refused until separately qualified.
