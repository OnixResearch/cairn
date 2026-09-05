# Drain preflight: blocked toolchain integrity

## Scope and disposition

The requested drain covers the five changes on `plan/cairn-utility-stack-20260905` at `8d2a99410b331ad045f185645e5c2402de6b15f4`.
The observed origin main revision is `5e1fe08d62149b286ff583eb056e655fe92e3cf1`.

No utility change is complete. All implementation tasks remain open.
Product Rust, Nickel policy, Cargo manifests, Cargo.lock, flake.nix, and flake.lock remain unchanged.
No sync, archive, remote push, or main integration occurred.

The readiness baseline ran after its reviewed package entered the dedicated readiness worktree at `54c3122b8e91c4bbd9d43148a870f02cb4e98841`.
The tests therefore cover existing product code, not an implementation of unified readiness.

## Results

| Check | Observed result |
|---|---|
| Proposal, design, and tasks gates for readiness | Passed before implementation |
| Readiness-worktree lifecycle validation | Valid before implementation |
| Focused Cargo baseline through the development shell | Did not reach Cargo. Nix rejected a modified Rust source input |
| `checks.x86_64-linux.rust-tests` | Passed, including existing positive and negative Rust tests |
| `checks.x86_64-linux.slotcar` | Failed before consumer analysis on the same Rust source hash mismatch |
| Direct Nix content verification | Reported the Rust source path as modified |
| `nix store repair` | Failed because the daemon store does not support `repairPath` |
| Exact derivation repair followed by content verification | The Rust source path still failed content verification |
| Non-interactive administrator availability | `sudo -n true` failed because a password is required |

The successful Rust test derivation does not replace the mandatory Slotcar check or establish implementation acceptance.

## Exact blocking input

The selected input is:

```text
/nix/store/3r2nwafkx9xha0y6xsd0w0557ba0c294-rust-src-1.96.0-nightly-2026-03-21-x86_64-unknown-linux-gnu
```

Its derivation is:

```text
/nix/store/jyn051mm96nk1ml1432vqmngdk3fnbd0-rust-src-1.96.0-nightly-2026-03-21-x86_64-unknown-linux-gnu.drv
```

The import diagnostic reports these exact NAR identities:

```text
specified: sha256-q/gu/3mAuLgNfJlxV/Sw1jttbi4PIBjN+XH0bGmB5NQ=
got:       sha256-WTRv7eyiu+VOfb8+90cALNJrUa3uLwRFIaeEr+tAIjQ=
```

SHA-256 is the Nix protocol identity here. New log identities use BLAKE3.
The content verification independently reports the selected path as modified.
No pin or expected hash changed to accept those bytes.

## Retained local evidence

Raw logs remain in the primary checkout under `.pi/cairn-utility-drain-20260905/`, not inside a disposable worktree.
The directory is owner-only. These paths are local evidence locators, not published product dependencies.

| Log | BLAKE3 |
|---|---|
| `dev-shell-baseline.log` | `6e6eb9e755ec92e77febffec7f40c2083241a74cf98001fd687eda7bc305c117` |
| `rust-tests-baseline.log` | `b4f89038cc54cf9a0bd15ec7aa08d5eaaf74eaca7f931d864756df5d9953af61` |
| `slotcar-baseline.log` | `6559ace1e5acb95abf1510702bc19d176e70573fabff91f0101fa2a7526263a6` |

## Retained branches

- `drain/utility-bootstrap-20260905` contains the reviewed bootstrap package at preparation commit `95dbaa8`.
- `drain/utility-readiness-20260905` contains the reviewed readiness package, baseline results, and this blocker.
- `drain/utility-evidence-20260905` contains the reviewed producer package at preparation commit `f23a5b0`.

The three worktrees remain available. Their product source matches the same origin main base.
Bootstrap and evidence production require the same mandatory toolchain check before completion.
Verification guidance and adoption reporting also retain their declared local prerequisites.
Their implementation worktrees did not start because the prerequisite changes remain active.

## Resume conditions

An administrator must restore the exact selected Rust source input from a trusted source.
A changed expected hash or disabled gate is not an acceptable repair.

After restoration, run content verification against the exact path:

```text
nix store verify --no-trust /nix/store/3r2nwafkx9xha0y6xsd0w0557ba0c294-rust-src-1.96.0-nightly-2026-03-21-x86_64-unknown-linux-gnu
```

Then rerun the development-shell baseline and the standalone Slotcar check in the readiness worktree.
When reliable verification is available, resume implementation with the baseline failure recorded.
The normal drain still requires implementation, acceptance evidence, sync, archive, change-branch publication, and fresh origin main integration.

## Independent review limit

The parallel worker probes did not provide implementation or review evidence.
Pi reported the Codex usage limit. The other CLI probes reported unavailable or expired authentication.
Non-interactive AI commands under Pueue also need closed stdin to avoid an unrelated input wait.
These worker failures are separate from the verified toolchain-integrity blocker.
