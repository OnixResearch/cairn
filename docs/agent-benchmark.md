# Offline agent benchmark observations

<!-- r[impl cairn.agent_efficiency.cohort] r[impl cairn.agent_efficiency.accounting] -->
<!-- r[impl cairn.agent_efficiency.outcomes] r[impl cairn.agent_efficiency.import] -->
<!-- r[impl cairn.agent_efficiency.conformance] -->

Cairn admits explicitly saved observations and compares declared cohorts. It does
not call a model, select credentials, collect transcripts, run an oracle, or read
a clock. The host supplies those observations; Cairn checks their shape,
completeness, identities, accounting, and comparison scope.

```sh
cairn agent-benchmark validate --root . --input observations.json
cairn agent-benchmark compare --root . --input comparison.json
cairn --machine agent-benchmark compare --root . --input comparison.json
```

Both commands are read-only. `--input` is relative to the explicit `--root`
(default `.`). Traversal, linked path components, non-regular files, oversized
inputs, unknown or duplicate fields, and malformed schemas are refused. No
undeclared file or network source is searched. Direct failures expose bounded
error classes, not raw input or paths. Machine mode retains its existing
`exit_class` and diagnostic-code contract.

## Authored inputs and provenance

`contracts/agent-benchmark.ncl` defines the typed scenario, accounting, run,
comparison, and report shapes. `cairn-policy/agent-benchmark.ncl` owns the named
bounds and non-claims. Runtime code reads its bundled generated JSON; it does not
evaluate Nickel or change the main lifecycle policy.

An input has schema `cairn.agent-benchmark.input.v1`, one scenario, and exactly
its declared number of run observations. A scenario names a bounded scope,
`representation_replay` or `live_host` mode, repetitions, and maximum calls per
run. The mode describes supplied observations; selecting `live_host` does not
authorize or start a host run.

Every cohort binds source projection, product revision, policy, fixture, oracle,
model, agent configuration, catalog, presentation, and accounting identities.
Identities are lowercase 64-digit BLAKE3 values. Hosts retain the exact identified
manifests, model IDs, and source revisions; a digest alone does not authenticate
an observation. All runs within one input have the same cohort. Run and call IDs
are unique within that input, and call sequences are contiguous from zero.

The supported accounting sources are:

- `provider_usage`: an immutable `usage_source` identity and explicit counters.
- `exact_tokenizer`: immutable `implementation_revision` and `encoding`
  identities, plus each call's exact exposed `content_identity`.

No tokenizer is bundled or executed. An external tokenizer observation must
identify its actual implementation, encoding, and exposed content. Character or
byte ratios are not token observations. Missing usage remains unavailable.

## Accounting and outcomes

Uncached input, cached input, output, and optional reasoning counts remain
separate. One reasoning-inclusion convention applies throughout the input:

- `included_in_output`: output already includes reasoning; it is not added again.
- `additional_to_output`: known reasoning is added exactly once.

Missing reasoning is not zero. When output is inclusive, billed output can remain
known while `reasoning_complete` is false. Missing additional reasoning makes the
complete token cost unavailable. Reports retain known subtotals, but a subtotal
is not a complete cost. Arithmetic uses checked integer operations.

Retry, tool-call, repeated-read, and duration observations remain explicit,
bounded counters. Duration is a host observation, not a clock sampled by the core.

| Supplied repository-oracle observation | Agent claims completion | Reported outcome |
|---|---|---|
| Completed | Either | `completed` |
| Correctly blocked | No | `correctly_blocked` |
| Correctly blocked or rejected | Yes | `false_completion` |
| Rejected | No | `failed` |
| Inconclusive | Either | `incomplete` |

A host must derive the oracle observation from its declared repository-owned
assertions. A checked task box, successful process exit, transport success, or an
agent's statement is not that oracle. Cairn cannot establish that supplied oracle
or usage observations are truthful.

All attempted-run costs remain in the totals, including failed and falsely
completed attempts. `tokens_per_correct_completion` is an exact numerator and
nonzero completion denominator, not a success-only average. Correct blocking has
a separately named denominator. Missing complete accounting or a zero denominator
leaves the corresponding ratio absent.

## Comparisons

`cairn.agent-benchmark.comparison.v1` contains baseline and candidate inputs and
an explicit `allowed_candidate_differences` list. A changed product revision must
be declared. Source projection, agent configuration, catalog, and presentation
may differ only when explicitly admitted for that comparison.

Policy, fixture, oracle, model, accounting source, scope, mode, repetitions, and
call budget must match. Changing completion criteria or a run budget is not a
product improvement. Malformed declarations fail admission; a well-formed but
incomparable pair returns `comparable: false` and a withheld verdict with drift
fields.

A comparison with unavailable costs, no correctly completed runs, or worse
correctness withholds an improvement verdict. Representation replay always
withholds an end-to-end efficiency verdict. Its scoped counts describe the
selected representation only. Live-host results describe the supplied cohort,
not general productivity, independent sampling, or code correctness. The report
makes no statistical-confidence claim.

Neither command grants execution, gate, completion, archive, or release authority.
The report's schema, cohort bindings, completeness, outcome classes, and verdict
must be inspected independently of machine transport success.

## Bounds and repeatability

The bundled policy admits at most 16 runs, 128 calls per run, 1 MiB of input,
96 bytes of scope text, 1,000,000 events per counter, one hour per duration
observation, and 1,000,000,000,000,000,000 tokens per supplied token counter.
Requests do not raise these limits. Core admission additionally enforces named
hard ceilings and validates nested identities before canonicalizing input.

Regenerate the standalone policy and example projections using the pinned cohort:

```sh
nix develop -c nickel export --format json cairn-policy/agent-benchmark.ncl \
  | jq -S . > cairn-policy/generated/agent-benchmark.json
nix develop -c nickel export --format json fixtures/agent-benchmark/positive-input.ncl \
  | jq -S . > fixtures/agent-benchmark/positive-input.json
nix develop -c nickel export --format json fixtures/agent-benchmark/positive-comparison.ncl \
  | jq -S . > fixtures/agent-benchmark/positive-comparison.json
```

The `positive-*` examples are synthetic schema/accounting controls, **not measured
provider usage or live-agent runs**. Their representation-only pair cannot claim
end-to-end savings.

`checks.<system>.agent-benchmark-contracts` checks policy and fixture freshness,
positive and negative Nickel admission, and deliberate policy drift. Core and CLI
controls cover accounting, stale cohorts, false success, incomplete traces,
overflow, and unsafe reads. The fresh-root oracle control invokes native
completion, gates, and readiness and keeps missing or stale evidence from becoming
success. Retained pre-change command receipts record bytes and BLAKE3 identities;
without actual usage they explicitly leave tokens unavailable.

Cairn verification maintainers own scenario/oracle and accounting review.
Lifecycle-core maintainers own pure admission; CLI maintainers own bounded local
import. Agent hosts own authorization, observations, raw-data privacy, provider
quotas, and any live-run budget. The durable capability is repeatable scoped
comparison over saved observations, not a new agent runner.
