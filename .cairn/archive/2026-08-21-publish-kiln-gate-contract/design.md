# Design: versioned Cairn gate interchange

## Context

Cairn owns lifecycle gate meaning and native gate receipts. A consumer such as Kiln owns its CI candidate identity, provider selection, process supervision, idempotency, reconciliation, and merge or release authority.

The current machine contract gives bounded JSON transport and stable diagnostics. The native gate receipt binds Cairn policy and lifecycle inputs. Neither contract names an external CI candidate or evidence set. Adding those facts to the native receipt would mix consumer domain meaning into Cairn's gate engine.

## Goal and Success Contract

Publish one narrow component that lets a consumer bind a captured native gate observation to an exact external request.

Completion requires:

- versioned typed Rust and Nickel schemas;
- deterministic request and result identities;
- exact candidate, pipeline, policy, evidence, profile, effect, idempotency, target, and tool-cohort bindings;
- fail-closed positive and negative admission;
- passing and blocked producer fixtures;
- no filesystem, process, environment, clock, network, credential, or authority access in the contract core.

False completion includes changing native gate semantics, accepting unsorted or duplicate evidence, trusting a supplied identity without recomputation, treating a machine transport success as a gate pass, or claiming that a bound receipt proves candidate truth.

## Decisions

### Decision: Publish a standalone functional-core component

**Choice:** Add `cairn-gate-contract` as a `no_std + alloc` workspace crate with only pure types, framing, validation, and admission.

**Rationale:** Consumers can pin one immutable Cairn revision and depend on the contract without importing `cairn-cli`, filesystem behavior, or provider integrations.

### Decision: Keep execution in consumer-owned shells

**Choice:** Keep `cairn gate` and `cairn --machine gate` unchanged. The consumer shell verifies the executable, applies process bounds, selects explicit root and policy paths, captures the machine envelope, and maps its native receipt into a typed observation.

**Rationale:** Cairn must not absorb Kiln process policy, candidate storage, timeout handling, or retry authority. The existing machine envelope already owns controlled diagnostics and transport completion.

### Decision: Bind instead of echoing unverified applicability

**Choice:** The request identity uses tagged, length-prefixed BLAKE3 framing over every applicability and cohort field. The result identity binds the exact request identity, repeated applicability, exact tool cohort, native observation, findings, and native receipt identity. Admission recomputes both identities and compares every repeated fact.

**Rationale:** An unchanged native Cairn receipt can be safely scoped to an external request without pretending that Cairn observed or authenticated the external candidate.

### Decision: Preserve native and consumer verdict meanings

**Choice:** The contract accepts only a coherent captured native observation. A pass requires `valid=true` and no findings. A block requires `valid=false` and at least one finding. Missing output, timeout, cancellation, malformed output, and ambiguous termination do not produce a contract result.

**Rationale:** Those failures belong to the process shell and must remain provider errors, cancellation, timeout, or unknown. They must not become fabricated gate blocks or passes.

### Decision: Publish reviewable configuration contracts and fixtures

**Choice:** Add a typed Nickel contract for request and result wire values. Generate deterministic passing and blocked JSON fixtures from the Rust producer example. Add negative Nickel fixtures for malformed identity, unsupported schema, and unordered evidence.

**Rationale:** Rust owns identity and admission semantics. Nickel gives operators a reviewable shape contract. Fixtures prove the producer and independent consumers can agree on bytes and rejection behavior.

## Authority and Claim Boundary

- Cairn owns native lifecycle gate meaning and the gate-receipt contract.
- The contract owns external binding and admission semantics only.
- The consumer owns executable verification, repository-path authority, policy-path authority, process bounds, persistence, retries, and reconciliation.
- A passing result can satisfy one consumer-selected gate condition only after consumer admission.
- The result does not authorize merge, publish, release, lifecycle mutation, or provider mutation.

## Risks / Trade-offs

- The outer result binds consumer-supplied candidate facts. It does not prove those facts describe the evaluated filesystem. The consumer must establish that mapping.
- Version one supports native change gates only. Release-readiness can use a later explicit schema instead of overloading the first contract.
- The contract retains bounded native finding messages for diagnostics. Consumers must apply output-size limits before parsing.
