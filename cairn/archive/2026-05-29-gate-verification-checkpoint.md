# Archived Gate Verification Checkpoint

Date: 2026-05-29
Decision owner: repo agent checkpoint

## Question

The archived task files for `typed-lifecycle-probes` and `gate-probe-execution-boundary` mark proposal/design/tasks gate verification complete. Because the changes are archived, the normal gate CLI cannot target them in place: it reads active changes from `cairn/changes/`.

## Method

Reconstructed a temporary active root at `/tmp/cairn-archived-gate-check`:

1. copied current accepted specs into `/tmp/cairn-archived-gate-check/cairn/specs/`;
2. copied each archived change into `/tmp/cairn-archived-gate-check/cairn/changes/<change>`;
3. ran `./target/pi-check/debug/cairn gate <proposal|design|tasks> <change> --root /tmp/cairn-archived-gate-check`;
4. saved each JSON receipt under the archived change directory.

## Results

| Change | Stage | Receipt | Verdict | Receipt hash |
| --- | --- | --- | --- | --- |
| typed-lifecycle-probes | proposal | `cairn/archive/2026-05-29-typed-lifecycle-probes/gate-receipts/proposal.json` | PASS | `f6e25dcfbe29822d45d13eae4cc6b47a256bf435982b4d74c545c739ce37e9b8` |
| typed-lifecycle-probes | design | `cairn/archive/2026-05-29-typed-lifecycle-probes/gate-receipts/design.json` | PASS | `abbab7c73ae64db89b8ab81c69df9b508f2c8a053d48a5be3e7b067e01daf67a` |
| typed-lifecycle-probes | tasks | `cairn/archive/2026-05-29-typed-lifecycle-probes/gate-receipts/tasks.json` | PASS | `a3d10d376ded82346b5e9cbc049deddea5513db88dfe1dfbb89da8ce89f940bc` |
| gate-probe-execution-boundary | proposal | `cairn/archive/2026-05-29-gate-probe-execution-boundary/gate-receipts/proposal.json` | PASS | `36c53a872b86156e773df9b2780cfd67917b48ef6bd2724028ddd537cb2a55ff` |
| gate-probe-execution-boundary | design | `cairn/archive/2026-05-29-gate-probe-execution-boundary/gate-receipts/design.json` | PASS | `6de08dda3f37338c7b49f46ac370b0eda5f5bd13d918fa0af24c29d1ee815ff7` |
| gate-probe-execution-boundary | tasks | `cairn/archive/2026-05-29-gate-probe-execution-boundary/gate-receipts/tasks.json` | PASS | `1589a9936a663203b5b31627f2306c4ac3f9a00034f89e8ba7dec784f4fcc72e` |

## Repository Verification

Repo-wide verification evidence is saved under `cairn/archive/2026-05-29-repo-verification/`:

- `tracey-coverage.json`: `valid=true`, `referenced=902`, `requirements=902`.
- `validate.json`: `valid=true`, `specs_validated=14`, `changes=0`.

## Decision

The archived task-gate completion claims are accepted. The saved receipts provide deterministic evidence for all required proposal/design/tasks gates after replaying the archived changes as active changes in an isolated temporary root.

## Next Action

No further action required for these archived gate claims. Preserve the gate receipts and repo-wide verification receipts as audit evidence; future lifecycle/probe work should create fresh active-change gate receipts before archive.
