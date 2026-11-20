# Assurance targets and convergence

`cairn convergence inspect` answers one read-only question for one committed
revision: which requirements lack current evidence at their declared assurance
level. Nickel authors the targets, Mantle realizes the evidence job, Octet judges
the evidence, and Cairn binds those saved records together. The command starts no
tool and changes no gate.

## Declare targets

A consumer repository authors one targets file in Nickel with
[`contracts/assurance-targets.ncl`](../contracts/assurance-targets.ncl) and checks
in its JSON export (`cairn.assurance-targets.v1`). Targets are grouped into one
contract per specification file. Each target binds one row of
`cairn requirement-registry export` by all six row fields and declares:

| Field | Meaning |
| --- | --- |
| `assurance` | `reviewed`, `bounded_check` or `checked_proof`. `unassessed` is not a target. |
| `claim` | The bounded statement the evidence supports. It becomes the Octet clause statement. |
| `informal_remainder` | What the claim leaves uncovered. |
| `evidence_job` | The job id and the path of its owner-authored definition. Required unless `reviewed`. |
| `checked_scope`, `evidence_ids`, `evidence_classes`, `provider`, `negative_controls` | The evidence that may discharge the claim. |

Targets carry no source, tool, artifact or policy hash. Source identities come
from a separately generated source record (`cairn.assurance-target-sources.v1`)
that a producer writes from the realized output. The registry row copied into
each target is the one hand-maintained lock.

The generated record also carries `specifications`, one entry per reviewed
specification file: its path and the **plain** BLAKE3 of that file's bytes at the
revision, produced by a tool (for example `b3sum`). It is never typed into a
target and it is not the registry's domain-separated `specification_blake3`,
because Octet verifies the plain digest itself. The projection copies that plain
digest into the contract's `reviewed_specification`; a missing or duplicated
entry for the contract's specification path fails the projection.

The Nickel `project` function turns a targets contract and the generated record
into an `octet-requirement-contract/v1` contract. `checked_proof` maps to
`formal_proof`, `bounded_check` to `empirical_check`, and `reviewed` targets
produce no clause. `cairn_core::assurance_targets` implements the same mapping
and convergence compares it with the supplied contract file, so Nickel projects
and Rust verifies. Bounds, admitted values and admitted foreign schema
identifiers live in `cairn-policy/assurance-targets.ncl`, exported to
`cairn-policy/generated/assurance-targets.json`.

## Inspect convergence

```console
cairn convergence inspect --case case.json [--policy FILE] [--root DIR]
```

`--root` is a checkout of the consumer at the registry revision. The case file
(`cairn.convergence-case.v1`) names the registry, the exported targets, the
generated source record, the trusted Mantle signer key names and, per contract,
the Octet contract, decision and evidence bundle, the bridge handoff directory,
the producer output directory the bridge consumed, and the Mantle realization
receipt and store-info record for the job's root output. Relative paths resolve
against the case file's directory.

Octet judges the contract with the consumer checkout at the registry revision as
`--source-root`, so the reviewed specification is read from that checkout and
its plain digest is recomputed there; Cairn does the same from the accepted
specifications under `--root` and marks the target `stale` when the contract's
`reviewed_specification` does not carry that digest. The proof deck, by
contrast, lives in the realized producer output: the evidence bundle may name it
by a path relative to `producer_output` or by an absolute path that starts with
the case's resolved `producer_output` directory followed by `/`. Cairn reads only
the remaining relative path inside that directory. When the retained fixture
keeps those files elsewhere, the case may also name
`producer_output_locator`: the absolute path the evidence recorded for the
realized output. Its final component must equal `producer_output`'s, the bytes
are still read inside the case's `producer_output` directory, and absolute deck
paths must start with the locator followed by `/`. No canonicalization or link
resolution widens the directory, a `..` component is refused, and any other
absolute locator is rejected with `convergence.proof_deck_outside_output`.

Every read is bounded, refuses symbolic links and stays inside its directory. An
unknown option, a missing `--case`, a malformed or oversized input, or a schema
identifier the policy does not admit is rejected with exit status 1. Otherwise
the command prints one `cairn.convergence.v1` receipt and exits 0, whatever the
states. It spawns no process, uses no network, starts neither Nickel, Mantle nor
Octet, and writes nothing under the root.

## States

The receipt lists every registry row in registry order with one state:

| State | Condition |
| --- | --- |
| `untargeted` | The row has no target. |
| `not_projected` | The target is `reviewed` and its row binding holds. |
| `current` | Every binding below holds and Octet's decision passes the clause. |
| `unrealized` | No complete, trusted-signer Mantle realization is supplied for the job. |
| `stale` | A binding does not match. |
| `failed` | Octet's decision reports failure for the clause. |
| `unsupported` | Octet reports `unsupported`, or a value outside the admitted vocabulary appears. |
| `incomplete` | An input is missing or Octet reports an incomplete result. |

When several conditions hold, the first of `incomplete`, `unsupported`,
`unrealized`, `stale`, `failed` wins. A row is `current` only when none hold.

A target is `current` only when all of these bind:

- the registry verifies against `--root`, its revision kind is `git`, and its
  revision equals the bridge observation's `packaging.source_revision` and the
  handoff `source-revision`;
- the target's row equals the registry row and its contract's specification
  equals the row's specification;
- the recomputed projection equals the contract file;
- the contract's `reviewed_specification` names an accepted specification under
  `--root` and its `blake3` equals the plain BLAKE3 of that file's bytes;
- the decision's `contract_blake3`, `evidence_blake3` and
  `reviewed_specification` bind the contract and evidence files, and the evidence
  names the same contract;
- the target's job id equals the handoff `job.json`, and the observation's
  `producer_job_identity`, `tool_graph_identity` and `receipt_identity` equal the
  BLAKE3 identities of the handoff bytes;
- the producer output's receipt and profile equal the handoff copies, and the
  proof card Octet accepted lists that receipt with the same BLAKE3;
- the realization receipt is complete, its unit whose `node_id` is `nix:` plus
  the handoff root derivation's file name produced the directory the bridge
  consumed (`target_derivation` is Mantle's translated derivation), and the
  store-info record for that path has the same NAR hash and a signature by a
  trusted key name;
- Octet's aggregate result passes and its clause for the target is `passed`.

For a `current` row, `bound.root_derivation` is the frozen foreign root from the
handoff, `bound.node_id` is the matched unit's foreign identity (`nix:` plus that
root's file name), and `bound.target_derivation` is Mantle's translated
derivation, which differs from the frozen root even on the preserve route.
`bound.realization_route` names the route Mantle recorded in the realization
receipt's `cache_policy.realization_route` (null when absent or not a string).
`cache-only-preserve-v1` means Mantle preserved a prebuilt output and did not
build or execute it; the build route records no route.

Only Octet's decision is acceptance. A Mantle success, a cache hit, the
observation's `proof_acceptance` and its `externally_reported_outcome` never
are.

## Completeness

`targets_current` is true when there is at least one target and every targeted
row is `current` or `not_projected`. `coverage_complete` is true when no row is
`untargeted`. `complete` is true only when both are, so declaring two targets
cannot make a repository look converged.

The receipt also carries `unbound_target_issues`: the binding issues that no
evaluated row consumed, sorted by code and field path. A target whose
requirement_id is absent from the registry, or a contract-level specification
mismatch on a contract with no located target, appears there instead of being
dropped. `targets_current` additionally requires that list to be empty.

Identical inputs give byte-identical receipts. `receipt_blake3` is the BLAKE3 of
the canonical receipt without that field.

## Claim boundary

A passing receipt shows only that, for one revision, the supplied Mantle records
and Octet decision bind to the declared targets. It does not show:

- that any code is correct, or that the evidence is true;
- that a reused output was re-verified; a cache hit preserves an earlier
  acceptance and is not fresh proof work;
- that the supplied records were honestly produced; Cairn re-runs and
  authenticates none of them and does not verify signatures, only signer names;
- that a verifier, a builder, the sandbox or the Nickel exporter is sound;
- that Mantle built the tool graph from source, unless the recorded route says
  so;
- that a `claim` captures its requirement's full meaning, which is why
  `informal_remainder` exists;
- release eligibility.

The `Pass | Block` gate, readiness states, verification roles and
`implementation-ready` output keep their meanings and bytes. The receipt is
advisory.
