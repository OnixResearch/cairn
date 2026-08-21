# Gate interchange contract

`cairn-gate-contract` publishes a versioned boundary for consumers that run Cairn gates through their own process shells.

The contract is `no_std + alloc`. It contains no filesystem, process, network, clock, credential, persistence, or provider behavior.

<!-- r[impl cairn.gate_interchange.boundary] -->

## Contract parts

<!-- r[verify cairn.gate_interchange.contract.scenario.pinned] -->

Version one defines:

- `cairn.gate-request.v1`;
- `cairn.gate-result.v1`;
- `cairn.native-gate-observation.v1`;
- `cairn.cli-machine.v1` as the selected transport envelope;
- `cairn.cli-machine-diagnostic.v1` as the selected diagnostic shape;
- `cairn.policy-schema.v1` as the policy cohort;
- `cairn.gate-receipt.v1` as the native receipt cohort.

The Rust source is in `crates/cairn-gate-contract`. The reviewable shape contract is in `contracts/cairn-gate-contract.ncl`.

## Request binding

A request identity uses tagged, length-prefixed BLAKE3 framing. It binds:

- the candidate identity;
- the candidate-root identity;
- the pipeline-revision identity;
- the policy identity;
- the sorted unique evidence identities;
- the provider-profile identity;
- the effect identity;
- the idempotency key;
- the change and gate stage;
- the Cairn source revision;
- the executable BLAKE3 identity;
- the machine, diagnostic, policy, and native-receipt schemas.

Changing any field requires a new request identity.

## Result binding

A result repeats the applicability facts and tool cohort. It also binds:

- the exact request identity;
- the native change and stage;
- the native verdict and validity;
- the native gate-input identity;
- the native policy identity;
- the native receipt identity;
- each bounded finding;
- the required non-claims.

Admission recomputes request and result identities. It rejects applicability, target, policy, tool-cohort, schema, or identity drift.

A pass requires `valid=true` and no findings. A block requires `valid=false` and at least one finding.

## Shell workflow

A consumer shell uses this sequence:

1. Pin one immutable Cairn revision and contract revision.
2. Verify the executable BLAKE3 identity before execution.
3. Build and admit one gate request.
4. Run `cairn --machine gate <stage> <change> --root <path> --policy <path>` with explicit limits.
5. Check the machine schema, process status, transport outcome, and diagnostic codes.
6. Map one complete native gate receipt into `NativeGateObservation`.
7. Build and admit the bound gate result.
8. Preserve the native receipt and outer result identities.

The consumer must apply deadlines, output limits, cancellation, and process teardown. Cairn's machine envelope does not supervise the host process.

## Unknown outcomes

<!-- r[verify cairn.gate_interchange.reconciliation.scenario.unknown] -->

Timeout, cancellation, missing output, malformed output, ambiguous termination, or a failed executable preflight does not produce a gate result.

The consumer keeps the outcome as timeout, cancellation, provider failure, or unknown. It must not invent a pass or block. A retry needs the consumer's explicit deterministic rerun policy.

## Authority boundary

Cairn owns native lifecycle gate meaning. The contract owns only external request and result binding.

The consumer owns:

- the truth of candidate and evidence facts;
- repository and policy path authority;
- executable verification;
- process limits and supervision;
- persistence and idempotency;
- retry and reconciliation policy;
- gate composition;
- merge, publication, and release authority.

## Non-claims

<!-- r[verify cairn.gate_interchange.boundary.scenario.scope] -->

An admitted result does not prove that candidate facts describe the evaluated filesystem. It does not prove source correctness, test completeness, process reliability, deployment success, or future applicability.

A passing result does not authorize merge, publication, lifecycle mutation, or release execution.

## Fixtures and checks

`fixtures/cairn-gate-contract/producer/fixtures-v1.json` contains one request, one passing result, and one blocked result.

Run focused checks with:

```bash
cargo test -p cairn-gate-contract --all-targets
cargo clippy -p cairn-gate-contract --all-targets -- -D warnings
cargo check -p cairn-gate-contract --lib --target wasm32-unknown-unknown
nickel export --format json fixtures/cairn-gate-contract/positive/request.ncl
```

The Nix checks also regenerate the producer fixture and reject drift. Negative Nickel fixtures reject an unsupported schema, malformed identity, and non-canonical evidence order.
