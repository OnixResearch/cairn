# Producer fixture provenance

Cairn producer-adapter maintainers own these imports and translations.
These fixtures test metadata admission. They do not record real upstream test execution.
No producer pin or producer implementation changes.

| File | Original source | Revision | License |
|---|---|---|---|
| `lattice-producer-fixtures.json` | Lattice `crates/lattice-workflow-contract/fixtures/producer-fixtures-v1.json`, unchanged | `70496e67c7fd4a8b05914161a8e09de2759bebc8` | MIT, as declared by the original crate manifest |
| `lattice-request.json` | Nickel export of Lattice `tests/fixtures/kiln-workflow-contract/request-valid.ncl` | Same Lattice revision | MIT |
| `lattice-cohort.json` | Cairn's native-library replay over `lattice-request.json`; identities and verdicts use the original Lattice library | Same Lattice revision | Cairn-owned fixture construction; native source retains MIT |
| `mantle-producer-fixtures.json` | Mantle `fixtures/mantle-build-contract/producer/fixtures-v1.json`, unchanged | `0bbab77e23ab95caa4e4184b9945cfaa5c8040e2` | AGPL-3.0-or-later |
| `octet-producer-fixture.json` | Nickel export of Octet `standards/receipts/fixtures/valid/acceptance_receipt.ncl` | `890f424cd7b1b2015f39c0eb5747f1bd1e576028` | AGPL-3.0-or-later |
| `valence-sidecar.json` | Nickel export of Valence `contracts/stack-provenance/fixtures/pass/sidecar.ncl` | `e40c76b4d2070a29636e00c85c0dff93f03dba2f` | AGPL-3.0-or-later |
| `kiln-producer-fixtures.json` | Export of the original private `plan()` and `completed()` fixtures in Kiln `crates/kiln-core/src/workflow/tests.rs`; values unchanged | `5d99f4e9bb954743443e340baece31256b4c53f1` | MIT OR Apache-2.0; original source stays in its pinned distribution |
| `kiln-workflow.json` | Export of Cairn-owned `kiln-workflow.ncl`; original Kiln nominal constructors and workflow validator admit the values | Kiln `5d99f4e9bb954743443e340baece31256b4c53f1` | Cairn fixture; native Kiln dependency retains MIT OR Apache-2.0 |

Lattice and Valence are published at `https://github.com/OnixResearch/lattice` and `https://github.com/OnixResearch/valence`.
Mantle is published at `https://github.com/OnixResearch/mantle`.
Kiln is published as `rad:z2wsvXm5S2sJGvuV1k5JHiwi1PbKE` through `https://seed.radicle.garden/z2wsvXm5S2sJGvuV1k5JHiwi1PbKE.git`.
The runtime dependencies retain their original source notices. Valence's original license text is retained in `LICENSES/valence-producer-fixtures-LICENSE`.
Octet's original license text is retained in `LICENSES/octet-producer-fixtures-LICENSE`.
The Lattice source snapshot supplies MIT package metadata but no separate root license file.
The owner-authorized reuse grant does not change producer licenses or third-party rights.

## Repeatable native replay

Run these commands from the Cairn repository in its development shell.
Use explicit source roots at the revisions above.

```console
rustc tools/check-producer-fixture-sources.rs -o target/check-producer-fixture-sources
target/check-producer-fixture-sources LATTICE_ROOT MANTLE_ROOT VALENCE_ROOT OCTET_ROOT
KILN_FIXTURE_SOURCE=KILN_ROOT cargo run --locked --manifest-path tools/check-evidence-kiln-fixtures/Cargo.toml -- KILN_ROOT > target/kiln-producer-replay.json
cmp target/kiln-producer-replay.json fixtures/check-evidence/kiln-producer-fixtures.json
KILN_FIXTURE_SOURCE=KILN_ROOT cargo test --locked --manifest-path tools/check-evidence-kiln-fixtures/Cargo.toml
cargo run --locked --manifest-path tools/check-evidence-lattice-fixtures/Cargo.toml -- \
  fixtures/check-evidence/lattice-request.json fixtures/check-evidence/lattice-producer-fixtures.json \
  > target/lattice-cohort-replay.json
cmp target/lattice-cohort-replay.json fixtures/check-evidence/lattice-cohort.json
nickel export --format json fixtures/check-evidence/kiln-workflow.ncl > target/kiln-workflow-replay.json
cmp target/kiln-workflow-replay.json fixtures/check-evidence/kiln-workflow.json
cargo test -p cairn-core check_evidence
cargo test -p cairn-cli --test check_evidence_stack
```

The source checker rejects swapped source roots or changed fixture bytes.
It also checks the Valence and Octet licenses, reexports their pinned fixtures, and rejects five original invalid Nickel exports.
Valence export requires unchanged tracked contract and license inputs.
The Kiln helper requires an explicit source root and the exact published revision.
It reads the original test source through `git show`, not the working file, and compiles it only in Cargo's output directory.
The helper rechecks that source before export. A substituted revision or fixture object fails closed.
Its five tests are the original Kiln tests. Its output contains the original positive fixture plus stale-attempt and false-paused-success controls.
This is fixture-only source reuse, authorized by the owner and covered by Kiln's retained MIT OR Apache-2.0 notices.
No producer implementation is copied into Cairn's tracked source, and the product does not load a sibling working tree.
The native Lattice helper also checks the producer's original positive, cyclic, false-success, and stale-execution fixtures.
Its separate Cargo graph avoids the incompatible Lattice/Mantle BLAKE3 constraints without a producer patch.
The product translator agrees with those native fixtures and rejects rehashed semantic substitutions.
Kiln's admitted workflow DTO provides no public executed-test coverage fixture. The Cairn-owned native DTO fixture is not a substitute for a Kiln completion replay.
Valence admission preserves only the reviewed stack and chain non-claim texts; extra unreviewed text fails closed.
