# Admit owner-scoped producer jobs

## Why

Cairn's assurance policy admits one producer-job schema, Octet's, and
`cairn convergence inspect` requires every handoff `job.json` to carry it. A
producer job owned by another repo with its own owner-authored schema therefore
cannot bind, even when Mantle realized and observed it and Octet judged it.

WP8 turns Terrane's `bounds.clamp` Verus proof into a Trellis-owned Trellis
producer job. Its `job.json` declares `owner = "Trellis"` and
`schema_version = "trellis-mantle-producer-job/v1"`. Presenting that job as an
Octet job would misstate its owner, and a local `--policy` override would
bypass the reviewed policy. The reviewed default has to admit the Trellis job
next to the existing Octet job.

## What Changes

- Replace the single `schemas.octet_producer_job` value in
  `cairn-policy/assurance-targets.ncl` with the owner-scoped map
  `schemas.producer_jobs = { octet = "octet-mantle-producer-job/v1", trellis =
  "trellis-mantle-producer-job/v1" }`, and export it the repository way.
- Keep the map typed: at least one owner must be admitted, every owner key and
  schema value must be non-empty, and no two keys may name the same owner
  without regard to ASCII case.
- Admit the handoff `job.json` by its own `owner`: the job must declare a
  non-empty owner, the policy must admit a schema for that owner, and the job's
  `schema_version` must equal it. A missing owner, an unknown owner and a
  schema/owner mismatch each fail closed with a stable issue code, in the CLI's
  bounded read and in the pure evaluator.
- Keep the existing `job_id` binding to the target's `evidence_job.job_id`
  unchanged.
- Retain the exercised Octet pilot corpus unchanged as the positive Octet case,
  and add a synthetic Trellis corpus plus one negative per new rejection.

## Impact

- **Current consumers**: repositories that author assurance targets and run
  `cairn convergence inspect`, including the WP8 Terrane pilot and every
  existing Octet job.
- **Immediate outcome**: the retained Octet pilot row stays `current`; a
  Trellis-owned synthetic job binds; an unknown owner, a schema/owner mismatch
  and a missing owner are rejected.
- **Durable contribution**: an owner-scoped producer-job admission that later
  producer owners extend by one reviewed policy entry rather than a schema
  comparison change.
- **Maintenance owner**: Cairn verified-core maintainers own owner admission and
  the closed issue codes; Cairn CLI maintainers own the bounded read; each
  producer owner (Octet, Terrane/Trellis) owns its job schema and contract.
- **Files**: `cairn-policy/assurance-targets.ncl`,
  `cairn-policy/generated/assurance-targets.json`,
  `crates/cairn-core/src/verified/assurance_targets.rs`,
  `crates/cairn-core/src/verified/convergence.rs`,
  `crates/cairn-cli/src/convergence.rs`, their tests,
  `fixtures/convergence/pilot/` (unchanged positive corpus),
  `docs/convergence.md`, `CHANGELOG.md`.
- **Testing**: positive and negative core tests for the owner map and the three
  rejection codes; CLI tests that invoke the real binary against the synthetic
  corpus and the unchanged pilot; `policy export --check`, `validate`, the
  gates, `validate` and coverage after archive, and the full `nix flake check`.

Native `Pass | Block`, readiness states, verification roles and
`implementation-ready` output keep their meanings and bytes. This change adds
no gate, scheduler, dispatch, claim or lifecycle mutation, and no runtime
command starts Nickel.
