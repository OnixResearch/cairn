# Octet integration verification

## Scope and disposition

Implementation commit: `564a855`.
Base: `5e1fe08d62149b286ff583eb056e655fe92e3cf1`.
Branch: `fix/cairn-octet-integration-20260905`.

The direct Nix input and check now use `octet`.
The development shell selects `cargo-octet`.
Cargo metadata, lint config, current operator guidance, and new release-readiness command lists use the canonical name.

The source rename is implemented, but the required full gate remains blocked.
The accepted spec was synced through an unblocked native Cairn plan.
That sync accepts requirements. It does not establish completed implementation, producer execution, or release readiness.
The change remains active. No archive, push, or main integration occurred.
The original utility changes remain separate and incomplete.

## Source and evidence preservation

The selected source remains `OnixResearch/octet` at `2f8213a88781d9d38bb81319d06a50805d48ccd3`.
Its NAR identity remains `sha256-OvqB00mDIsGDOcuK5jvPJUModVz9WbIBuM/l06TABHc=`.
Nix generated `flake.lock` after the direct input rename.
The lock diff changes the direct repository name, explicit revision declaration, and related node references without upgrading unrelated sources.

The producer already exports `cargo-octet` and reads canonical Octet tables at this revision.
The migration retains the workspace, all-target, and `redb-registry` consumer scope.
It introduces no sibling path or copied producer implementation.

Git comparison found no changes to `Cargo.lock`, existing archives, or recorded function-address smoke receipt files.
Transitive producer-owned lock inputs retain their historical names.
The maintained workflow document intentionally changed. Its reviewed BLAKE3 fixture now binds the new canonical guidance.
That fixture is not a historical producer receipt.

## Checks

| Check | Result |
|---|---|
| Rust suite before product edits | Passed |
| Rust suite after spec sync and workflow fixture refresh | Passed |
| New Rust config and identity controls | Passed all positive and negative cases |
| New release-readiness command controls | Passed canonical-list and legacy-exclusion cases |
| `checks.x86_64-linux.octet-integration` | Passed the actual config and all 17 negative mutations |
| Clippy and Rust formatting | Passed again after the final workflow fixture refresh |
| Nix formatting and whitespace | Passed |
| Proposal, design, and tasks gates | PASS with no issues |
| Lifecycle validation | Valid with no issues |
| Accepted-spec traceability | Valid with no missing or dangling references |
| Native sync | Executed an unblocked merge into the accepted specification |
| Archive preview | Blocked, dry-run, and no mutation |
| `checks.x86_64-linux.octet` | Blocked before consumer analysis by a modified Nix input |
| `nix flake check path:$PWD -L` | Blocked by the same Rust input mismatch |
| All-system evaluation without builds | Blocked by an invalid Nickel source derivation path, also with the evaluation cache disabled |

The saved `octet-integration.json` contains the positive result, each negative result, source identity, and non-claim.
The other JSON files retain the native lifecycle receipts.

## Test failures repaired during verification

The first post-change Rust run failed during evidence export.
The traceability receipt identified new Rust markers whose specification was still only a delta.
Cairn reads accepted specs for that traceability profile.
A native sync accepted the reviewed spec without archive or false task completion.
The repeated traceability check then reported no missing or dangling references.

The next run rejected the old maintained-workflow byte fixture.
The requested Octet guidance change changed the document hash from `71403945ca5ebb3a9871f4ee848db7b9d30885721d882c95d2bb23575c8a1d3d` to `079a6dabe0b1894058e1c16ad277ce15e83b0b55821cbe9f452bb413c48f5045`.
The fixture now binds that reviewed document. Its exact-byte assertion remains intact.
The final Rust run passed, including the workflow preservation test and the new Octet tests.
These diagnostic passes used this session. They do not constitute independent model review.

## Remaining environment blockers

The actual Octet check and full Nix check reject this input before consumer analysis:

```text
/nix/store/3r2nwafkx9xha0y6xsd0w0557ba0c294-rust-src-1.96.0-nightly-2026-03-21-x86_64-unknown-linux-gnu
specified: sha256-q/gu/3mAuLgNfJlxV/Sw1jttbi4PIBjN+XH0bGmB5NQ=
got:       sha256-WTRv7eyiu+VOfb8+90cALNJrUa3uLwRFIaeEr+tAIjQ=
```

All-system evaluation also stops at `packages.aarch64-linux.nickel-1-17-0` with:

```text
error: path '/nix/store/l0f2fvp0b7psxn0lx549kw3misk93gg4-source.drv' is not valid
```

No source pin, expected store hash, or gate changed to accept these errors.
An administrator must restore the modified Rust input before the required producer check can pass.
The invalid Nickel derivation also needs restoration before all-system evaluation can pass.
After restoration, rerun the Octet check, all-system evaluation, and full Nix check in this worktree.
Only then can the remaining verification and archive tasks complete.

## Retained raw logs

These logs remain under `.pi/` in the primary Cairn checkout, outside the implementation worktree.
Paths are local evidence locators, not product dependencies.

| Log | BLAKE3 |
|---|---|
| `cairn-octet-fix-rust-baseline.log` | `98f4c1f45d60bbf643c396bc47597f3a88e981ac85c628b674cbf09d833762e0` |
| `cairn-octet-fix-rust-verified.log` | `6c19ca21638177ac25508f847cfd63caae212ee40e5f76448baff98e89170483` |
| `cairn-octet-fix-lints-final.log` | `458684754dcdddfd6a0f28c79082a6854be5651a1c86b1efb53a7ea882960aa6` |
| `cairn-octet-fix-consumer.log` | `33dc80eb88da82238c8bad597edbf22b24901d8ae63c3ef071cdf33da73c06d6` |
| `cairn-octet-fix-flake-check.log` | `19f7f055aec1295a234c42fdc9604f45e98f1ec2374b075f786e4ab0d094ed3d` |
| `cairn-octet-fix-evaluation-fresh.log` | `8a9f35ca1d6273264b7ddff7ce4fd8d66a260f8280ee9b6fefd722b3a8d2488d` |
