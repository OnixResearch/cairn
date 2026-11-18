# Maintain alternative support through invalidation and changed assumptions

## Why

Cairn can already replay one bounded scoped derivation, but it cannot explain what happens when one alternative is withdrawn while another survives. Current output omits a derivation that is no longer usable, cannot distinguish a stale receipt from a refutation, and does not show which dependent claims or lifecycle obligations require reevaluation after an assumption, policy or context change.

Issue #10 and Roadmap #4 require one deterministic argument that keeps alternative support branches, refuting branches, residual assumptions and discharge paths together across evidence invalidation, assumption withdrawal, claim revision, policy/context changes and newly admitted counterevidence.

## What Changes

- Add an optional versioned support-maintenance case over immutable scoped claims, support/refuting branches and admitted evidence records.
- Keep each alternative as a conjunction of exact premises with its own assumptions and discharge paths; a shared assumption remains shared.
- Apply a bounded ordered change log for evidence staleness/withdrawal, assumption withdrawal, admitted evidence, claim revision, and policy/context changes.
- Recompute from immutable records and explain surviving alternatives, withdrawn alternatives with their historical cause, newly unresolved claims, affected claims and affected lifecycle obligations.
- Require an explicit admitted relation for any cross-revision branch; revision classifications never transfer evidence.
- Preserve applicable counterevidence separately from missing evidence, including when another support alternative succeeds.
- Add a read-only `cairn support-maintenance explain --case FILE [--policy FILE]` command.
- Put bounds and the admitted change vocabulary in a standalone Nickel export; keep the core pure and the shell limited to bounded reads.

## Impact

Current consumers: maintainers reviewing why a claim loses or retains support after changed evidence, assumptions or policy selection, and later workflow/kernel work that needs a stable invalidation surface.
Immediate outcome: one claim with `(A AND B) OR C` keeps C after B is withdrawn, while a shared assumption withdrawal is visible on every dependent branch.
Durable contribution: a replayable support-maintenance report with historical branch reasons, without becoming a second gate, an automatic belief-repair engine or an evidence-transfer authority.
Maintenance owner: Cairn verified-core maintainers own branch/change semantics; CLI shell maintainers own bounded reads and presentation.

Native `Pass | Block`, readiness states, verification roles and applicability validators keep their current meanings and authority.
Support maintenance is advisory: it does not mutate lifecycle decisions, authorize release, or make an old observation false.

## Out of Scope

- Incremental caching or incremental evaluation; the first deliverable is deterministic full recomputation
- Automatic belief repair, automatic evidence transfer between revisions, or probabilistic confidence
- Scheduling, work selection, merge authority or lifecycle mutation
- Treating a revision classification or a surviving alternative as proof of a composite judgment

Dependency: `add-scoped-claim-derivations`. Coordinates with requirement refinement and semantic revision analysis, whose classifications remain scoped inputs. Implementation remains open.
