## Why

Cairn binds every accepted requirement to exact Markdown bytes in the content-bound registry, but no requirement says which assurance it needs or whether current evidence discharges it.
Mantle can realize an evidence job into a signed, content-addressed output, and Octet can judge a requirement contract over that output, yet nothing binds a registry row to its declared target, the realized output and Octet's decision.
Phase 1 of the declarative codebase loop needs one read-only answer for one committed revision: which requirements lack current evidence at their declared assurance level.

## What Changes

- Add `contracts/assurance-targets.ncl`, a consumer-facing Nickel contract for per-requirement assurance targets. A target binds one exact registry row and declares `reviewed`, `bounded_check` or `checked_proof`, a bounded claim, its informal remainder, one evidence job, the admissible evidence and the negative controls. Targets are grouped into one contract per specification and carry no source, tool, artifact or policy hash.
- Add a Nickel `project` function that turns targets plus a separately generated source record into an `octet-requirement-contract/v1` contract. `reviewed` targets are never projected.
- Add `cairn-policy/assurance-targets.ncl`, exported to `cairn-policy/generated/assurance-targets.json`, holding the admitted assurance values, the assurance-to-disposition mapping, the admitted Cairn, Mantle and Octet schema identifiers and every bound.
- Add pure target admission and a pure convergence evaluator in `cairn_core`. The evaluator recomputes the clause mapping from the targets, so Nickel projects and Rust verifies.
- Add a read-only `cairn convergence inspect --case FILE [--policy FILE] [--root DIR]` command that prints a `cairn.convergence.v1` receipt listing every registry row with one state and the `targets_current`, `coverage_complete` and `complete` flags.
- Retain an exercised pilot: Octet's `octet.aeneas.external-consumer.mantle.v1` producer job realized through Mantle, observed by Mantle's verification bridge and judged by Octet, plus one negative case per binding check.

## Impact

- **Current consumers**: maintainers who need to know which requirements still lack current evidence, and the phase 2 Kiln campaign intake that will consume the receipt.
- **Immediate outcome**: for one Octet revision, the pilot row `octet.aeneas_consumer_contract.compatibility` reports `current`, every other row reports `untargeted`, and `complete` is false.
- **Durable contribution**: a reusable targets contract, projection and convergence receipt that later phases reuse without a new gate.
- **Maintenance owner**: Cairn verified-core maintainers own target admission and convergence semantics; Cairn CLI maintainers own bounded reads and presentation; Octet owns the pilot targets, evidence job and decisions; Mantle owns realization records.
- **Files**: `contracts/assurance-targets.ncl`, `cairn-policy/assurance-targets.ncl`, `cairn-policy/generated/assurance-targets.json`, `crates/cairn-core/src/verified/assurance_targets.rs`, `crates/cairn-core/src/verified/convergence.rs`, `crates/cairn-cli/src/convergence.rs`, `crates/cairn-cli/tests/convergence.rs`, `fixtures/assurance-targets/`, `fixtures/convergence/`, `flake.nix`, `docs/convergence.md`, `README.md`, `CHANGELOG.md`.
- **Testing**: positive and negative core tests for admission, projection and every binding check; CLI tests that invoke the real binary; a flake check that re-exports the projection fixtures and requires every negative fixture to fail; `policy export --check`, `validate`, `traceability coverage` and `nix flake check`.

Native `Pass | Block`, readiness states, verification roles and `implementation-ready` output keep their meanings and bytes.
This change adds no gate, scheduler, dispatch, claim or lifecycle mutation, and no runtime command starts Nickel.

## Out of Scope

- Any Kiln change, scheduling, dispatch or automated landing
- Moving specifications from Markdown to Nickel or moving requirement row identity
- Octet's `requirements freshness`, and transfer of evidence between requirement revisions
- Any change to Octet's Rust, contract, evidence or decision schemas, or to its producer job contract
- Any Mantle proof registry, theorem policy or acceptance logic
- The Terrane pilot, deferred until this change lands
