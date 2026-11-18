# Concurrent change obligation tasks

## Implementation

- [x] [serial] [task:policy] Author the standalone Nickel policy for declaration and finite exploration bounds without changing the legacy default policy. r[cairn.concurrent_footprints.bounds]
- [x] [serial] [task:footprint] Add the versioned change footprint over exact typed requirement revisions, guarantees, assumptions, invariants, invalidations, interfaces, configurations and explicit unknown scope. r[cairn.concurrent_footprints.footprint]
- [x] [serial] [task:derivation] Derive changed-guarantee, invalidation, invariant-overlap and interface interactions with a bounded finite guarantee-versus-assumption check and retained witnesses. r[cairn.concurrent_footprints.derivation] r[cairn.concurrent_footprints.bounds]
- [x] [serial] [task:obligation] Emit compatibility obligations naming the combined candidate, participants, composition order, context, affected claims and required evidence with separate coexistence, preservation and commutativity propositions. r[cairn.concurrent_footprints.obligation]
- [x] [serial] [task:outcomes] Distinguish conflict, needs-joint-evidence, supported-within-declared-scope and unknown; refuse witness-scope preservation and individual-receipt discharge. r[cairn.concurrent_footprints.outcomes]
- [x] [serial] [task:explanation] Retain deterministic paths from changed guarantee to consumer assumption to resulting obligation. r[cairn.concurrent_footprints.explanation]
- [x] [serial] [task:invalidation] Invalidate prior obligation applicability on participant, dependency, configuration and composition drift. r[cairn.concurrent_footprints.invalidation]
- [x] [serial] [task:cli] Add read-only `cairn footprints inspect` with bounded reads and explicit advisory rejections. r[cairn.concurrent_footprints.inspection]
- [x] [serial] [task:core-tests] Author positive and negative core conformance cases for compatible, conflicting, order-sensitive, incomplete, unsupported, bounded and malformed declarations. r[cairn.concurrent_footprints.conformance] r[cairn.concurrent_footprints.independence]
- [x] [serial] [task:cli-tests] Author positive and negative CLI consumer cases for the advisory report, conflict path, supported joint evidence, order-sensitive propositions and rejections. r[cairn.concurrent_footprints.inspection] r[cairn.concurrent_footprints.conformance]

## Parent acceptance

- [x] [serial] [task:generate] [after:cli-tests] Generate `cairn-policy/generated/concurrent-change-footprints.json` from `cairn-policy/concurrent-change-footprints.ncl` and check policy freshness. r[cairn.concurrent_footprints.bounds]
- [x] [serial] [task:verify] [after:generate] Run focused core and CLI tests, formatting, workspace tests, Clippy, lifecycle validation, traceability and the maintained Nix checks. Retain current-source evidence. r[cairn.concurrent_footprints.conformance]
- [x] [serial] [task:handoff] [after:verify] Review the specification delta, refresh docs/changelog, run native lifecycle gates and sync or archive only after supported acceptance evidence is retained. r[cairn.concurrent_footprints.footprint] r[cairn.concurrent_footprints.inspection]
