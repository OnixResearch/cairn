# Validation: Radicle Import CLI

## Result

The preview-first local import vertical slice is validated on `x86_64-linux`. Cairn can plan exact native active-change bytes, require current canonical delegate authority, serialize same-home imports, create a signed typed change COB, reload it, and verify the previewed BLAKE3 content root.

## Deterministic Evidence

- Baseline focused Radicle reducer and adapter tests passed before implementation.
- Pure import planner tests passed for exact proposal/design/tasks/metadata/delta-spec bytes and for malformed IDs and protocol-limit rejection.
- Radicle adapter tests passed for canonical current delegate/non-delegate authority, signed Git-backed typed create/reload, duplicate import rejection, unauthorized no-mutation behavior, and atomic local import locking.
- CLI unit tests passed for explicit argument requirements, exact capability-bounded active-change reads, unsupported specs entry rejection, BLAKE3 receipt binding, blockers, mutation state, and required non-claims.
- Full `cargo test --workspace` passed after the implementation stabilized.
- Full `cargo clippy --workspace --all-targets -- -D warnings` and `cargo fmt --check` passed.
- Policy freshness, Cairn validation, proposal/design/tasks gates, and the verified-core purity search passed.
- Traceability coverage passed with `1611/1611` accepted requirement and scenario IDs referenced.
- `nix flake check path:$PWD -L` completed with `all checks passed` on `x86_64-linux`.

## Defects Found During Validation

Radicle 0.24's generic `Store::all()` silently omitted a newly created valid custom object in the local test topology even though typed `Store::get(object_id)` loaded it. Import inventory now discovers exact namespaced custom-object refs through the in-process Git backend, deduplicates their object IDs, and typed-loads every candidate before comparing Cairn change IDs.

Adversarial review found that separate local processes could both observe no existing object and create duplicates. Production profile import now performs an initial current-authority check, atomically acquires a BLAKE3-RID/per-change lock under the explicit Radicle home, and repeats authority and duplicate checks while serialized. The negative test proves a second lock acquisition fails before signing and that explicit release permits a later import.

## Bounded Claims and Remaining Environment Gap

The passing tests establish pure planning, point-in-time canonical authority evaluation, local serialization, and signed typed Git-backed storage behavior. The concrete explicit-profile loader and signer path compile in the full workspace and Nix rails, while signed mutation is exercised with Radicle's real Git-backed test repository and device. No external `rad` CLI or live node was available, so this evidence does not establish network publication, multi-peer replication, remote availability, credential-agent behavior on an operator machine, or remote trust.

A passing import still does not prove gate validity, evidence truth, dependency readiness, future authority, source correctness, archive execution, or release eligibility. A process crash can leave a stale local lock; Cairn reports the path relative to `--rad-home` and requires deliberate operator recovery rather than guessing that no importer remains.
