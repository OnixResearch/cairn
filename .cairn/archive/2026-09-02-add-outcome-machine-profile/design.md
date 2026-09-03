# Design: Add an outcome-machine workflow profile

## Context

Cairn workflow profiles define immutable artifact graphs and profile-specific gates. The current profiles cover ordinary specification work, assurance evidence, and no-spec changes.

Repositories need a stronger review route for significant work. The route must connect immediate outcomes to repeatable capability without turning Cairn into a product-value judge.

## Decisions

### 1. Add an opt-in profile

**Choice:** Add `outcome-machine` as a fourth policy-owned workflow profile. Keep `spec-driven` as the compatibility default.

**Rationale:** Cairn remains product-neutral. A consuming repository can select this profile or require it through repository-owned policy.

### 2. Use one typed JSON artifact

**Choice:** The profile requires `outcome-machine.json` after the proposal and before the design.

The record uses `cairn.outcome-machine-review.v1`. Unknown fields, unsupported routes, empty required values, placeholders, and exceeded bounds fail closed.

**Rationale:** Runtime Cairn consumes JSON without evaluating Nickel. The generated profile and contracts remain Nickel-owned.

### 3. Admit three routes

**Choice:** The record selects one route:

- `delivery` names an immediate outcome, current consumer, durable capability contribution, and repeatability plan.
- `foundation` names a current consumer, target outcome, adoption path, and repeatability plan.
- `bounded_exception` names an immediate outcome, exception reason, recurrence decision, revisit trigger, and capability non-claim.

Every route names a maintenance owner, tracked inputs, deterministic checks, AI-state facts, and non-claims.

**Rationale:** Delivery-only policy rewards short-term output. Foundation-only policy rewards architecture theater. The bounded exception preserves urgent work without promoting its claims.

### 4. Separate design admission from completion evidence

**Choice:** Route and common-field failures apply at the design stage. Completion evidence failures use an internal completion stage.

The proposal, design, and tasks gates ignore completion-stage findings. Sync, archive, repository validation, and drain planning retain them as completion blockers.

**Rationale:** A change needs an approved plan before implementation. It needs observed evidence before lifecycle completion.

### 5. Bind fresh-state disposition

**Choice:** The record declares fresh-state reproduction as `planned`, `observed`, or `not_applicable`.

`observed` requires evidence references. `not_applicable` requires a bounded rationale. `planned` blocks sync and archive.

**Rationale:** Not every change has an executable reproduction path. An explicit rationale is stronger than an invented run.

### 6. Keep evidence claims narrow

**Choice:** Completion evidence uses typed references with a kind, safe change-local path, and exact BLAKE3 identity.

The shell reads each referenced file through the change-directory capability. The core compares observed and declared identities.

Cairn validates shape, bounds, route consistency, and exact local bytes. Cairn does not verify evidence truth, product value, semantic correctness, or maintenance quality.

**Rationale:** These decisions remain with the repository owner and its domain checks.

## Architecture

The imperative shell reads bounded artifact bytes and parses JSON. It also observes referenced evidence files through a capability root.

The functional core validates schema, route, fields, bounds, completion state, AI state, evidence identities, and non-claims. It returns stable profile issues with design or completion scope.

Profile gate projection includes design issues at design and tasks gates. Lifecycle completion uses the complete issue set before sync or archive.

## Risks and Trade-offs

- Authors can write plausible but false statements. Cairn records structure and identity, not truth.
- A strict profile adds review work. Repository policy controls where the profile is required.
- Completion-stage issues add a new internal stage. Stable gate stages and receipt meanings remain unchanged.
- JSON is less suitable than Nickel for human-authored policy. This file is a change-local runtime artifact, not shared policy.
- Repositories can ignore the profile. Stack adoption requires repository-owned policy and CI enforcement.

## Verification

- Export and admit the updated Nickel policy.
- Exercise each valid route in pure core tests.
- Reject malformed, empty, placeholder, hidden-state, and unsupported records.
- Verify that design issues block design and tasks gates.
- Verify that planned completion passes planning gates but blocks sync and archive.
- Verify that observed completion permits lifecycle plans when all other requirements pass.
- Verify that existing profiles and unprofiled changes retain compatibility.
