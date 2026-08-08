# Validation: Generated agent workflows

Date: 2026-08-04

## Baseline

The change branch started at `e2a3b2e482b883a2310e074abfb796a39c313327`.

That commit had no `agent workflow` command family. It had passing policy, lifecycle, traceability, release-readiness, Slotcar, and Nix evidence from the completed guidance change.

The baseline Hermes BLAKE3 identity was `834efe19f34115f4b456b255f3310fd0a1ed46746c1955de1717d6831cb148e4`.

The final Hermes identity is the same. The maintained Hermes source did not change.

Baseline task `8034` overlapped the first source edits. It stopped after the checked policy became stale, so this task is not clean baseline evidence.

## Implemented surface

The checked policy now owns six typed workflow descriptors and three versioned adapters.

The pure core admits descriptors, enforces command permissions, renders files, classifies freshness, and builds BLAKE3 manifests and plans.

The CLI owns explicit project roots, capability-relative reads, locks, expected-prior rechecks, atomic replacement, post-write verification, receipts, and exits.

The first slice reports obsolete files as removal candidates. It does not delete them.

The generated policy field-set digest is `dcf499d154b906a153039768157f57eca6868b205524f5f23a27b570c3b53257`.

The generated policy hash is `794e65e05f3ffe7a872581e7d2cddae45bcea5a6d38cd9a8892a2079050679f2`.

## Focused verification

The focused core suite covers positive and negative descriptor, adapter, explore, manifest, freshness, conflict, version, obsolete-path, and source-change behavior.

The focused CLI suite covers all adapters, fresh installation, managed update, freshness checks, stale plans, modified files, unsafe paths, symlinks, privacy, and checkout independence.

A shell unit test confirms that a changed target fails before output replacement and leaves no partial generated files.

Adapter projection tests reject authority, network-hook, and model-hook fields. The pure core contains no filesystem, environment, process, clock, network, async, or output effects.

## Commands and results

- `cargo fmt --all -- --check`: passed.
- `cargo test --workspace`: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- `nix run path:$PWD#cairn -- policy export --check`: passed.
- `nix run path:$PWD#cairn -- validate --root $PWD`: `valid: true`.
- Proposal gate: `valid: true`, `verdict: PASS`.
- Design gate: `valid: true`, `verdict: PASS`.
- Tasks gate: `valid: true`, `verdict: PASS`.
- Traceability: `1928/1928`, with no missing or dangling references.
- MCP smoke: `valid: true`, `verdict: pass`.
- Release readiness: `valid: true`, `verdict: pass`.
- Optional Onix release disposition: `onix_release_ready: false`.
- Focused core purity search: no matches.
- Slotcar: passed through `nix build path:$PWD#checks.x86_64-linux.slotcar --no-link -L`.
- `nix flake check path:$PWD -L`: passed with `all checks passed!`.

The repository-wide historical purity pattern still finds pre-existing `to_json` helpers outside this change. The new generated-workflow core has no match.

The noncanonical Octet dependency-path findings remain pre-existing and outside this package. They did not fail Slotcar or the full Nix rail.

## Lifecycle evidence

Accepted-spec sync preview receipt: `8b6e639ad72c25d51a5c94dee2185dc772dd496bbc6135f43ae50e24c8a274ff`.

Accepted-spec sync execution receipt: `10f937564bb8cb0177914d56f8ae62acd338d7e548a21fda5f5c20e822604a75`.

Archive preview receipt: `67e350ee2baeaa51e1f5ce663003c45adb4fdbd99cf087e5d7358655fd0303b4`.

Archive execution receipt: `71a1d802fea0f9d95321fe755c24d73d5f60072c12ce70a275062a52484fa490`.

Post-archive validation, traceability, and release readiness passed.

The accepted specification is `cairn/specs/generated-agent-workflows/spec.md`.

## Claim boundary

Generated workflows are deterministic guidance over checked policy values. They do not prove agent compliance, gate success, evidence truth, implementation correctness, or release eligibility.
