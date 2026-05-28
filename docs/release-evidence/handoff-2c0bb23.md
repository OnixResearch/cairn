# Release Handoff Evidence Payload Snapshot: handoff-2c0bb23

## Payload

- Payload commit: `2c0bb23`
- Branch: `main`
- Worktree clean when evidence was generated: `true`
- Upstream: `origin/main`
- Ahead/behind: `0\t0`
- Local publication label: `stable` / `handoff-2c0bb23` / `local`

## Commands

- `release-evidence index --root . --out target/release-evidence-handoff-2c0bb23/index.md --artifact-dir target/release-evidence-handoff-2c0bb23/artifacts`
- `release-evidence check-index --root . --index target/release-evidence-handoff-2c0bb23/index.md --mode current-head`
- `release-evidence check-index --root . --index target/release-evidence-handoff-2c0bb23/index.md --mode audited-payload`
- `release-evidence publish --root . --index target/release-evidence-handoff-2c0bb23/index.md --artifact-dir target/release-evidence-handoff-2c0bb23/artifacts --publish-dir target/release-evidence-publish/handoff-2c0bb23 --channel stable --release handoff-2c0bb23 --retention local`

## Receipt Hashes

| Role | Verdict | Receipt hash |
| --- | --- | --- |
| `release_evidence_index_command` | `pass` | `c1dbb57170ca96b47c0b81674882aed41c4e2cddf6f26fa44fc1f0b8bf6beda1` |
| `check_index_current_head` | `pass` | `62683a1b579b91c399803fe64aa89611edd4e3d9e52ab97b95bcf56fd294c3f0` |
| `check_index_audited_payload` | `pass` | `a1887bf051acabde566e0051bdfd1ea86d5625e2b91194e28a115ff9c26af31c` |
| `publish_materialization` | `pass` | `fd28bc52111dc30699dac39292ec30aed96edbc5178393a680b057a90e5cfd51` |
| `evidence_index_verify` | `pass` | `1acd8e1f07e3f8d69b0c39d35c4bf6bd13f4973f6affce32ec2b8eeae9a2400f` |
| `evidence_export` | `pass` | `8e8e81bd6682540bef89d6a1a6aa0fc844d22bd83333e303ed4b3df0f0966239` |
| `evidence_index` | `pass` | `ce676628473b83cd541ea3addde3cc4fe2f961bd0e3022bd8dae7fdc95f417d3` |
| `evidence_verify` | `pass` | `79e2193c14e8aa442a435b6490a04fd7a7a1aa29b59cb4d83858f64c145dfa3f` |
| `evidence_republish` | `pass` | `278226b3ebea364e3b9ef6b651233ba70a62a00b09610366170d4a93458260c7` |
| `release_readiness` | `pass` | `03ea9780b44ed7ceef681adfa19b390609449b9d0a01ad003359e34e01770d7c` |
| `evidence_import` | `pass` | `14baa464f890d08300e74fedc62849ea3c50b43f9f70985a6426cdbe615b6ac1` |

## Bundle Artifacts

| Role | Path | BLAKE3 |
| --- | --- | --- |
| `coverage_audit` | `coverage-audit.json` | `09e4e402945de903d450834898c1e935a7ad79ad13d34672b1293cdfb7c86b6c` |
| `agent_evidence_bundle` | `agent-evidence-bundle.json` | `430d6fbd470ae3f22048adad0f58a31c30baf519f927afee4a2761f0d5a0cc32` |
| `agent_evidence_bundle_verification` | `agent-evidence-bundle-verification.json` | `8cee33bc55f58aeee72b431847d94b339b3e728f06e7554aad8c2ffcee8f6a93` |
| `evidence_manifest_verification` | `evidence-manifest-verification.json` | `5c1abe599ca03745a7594b698776d785900c5f42bee830c849ab6cf67b862bf1` |
| `evidence_manifest` | `evidence-manifest.json` | `576266b7e28ef2371dc883a74c059e239af1692d3393c92f69b3b111dc4f55eb` |
| `release_readiness` | `release-readiness.json` | `3a13a4fe98aee393d8e2088ff37b138cab885a00acb6d8a4dab28e7f753688bd` |
| `replay_verification` | `replay-verification.json` | `2a1a154a795dad8e2bcb89ea7fa5788395674a5b3d442f3be2c72117c40b2d57` |
| `replay_receipt` | `replay-agent.json` | `4301825648393887ec415f7097bc31b94fcdadcd34f099ed231b9836261a5b26` |
| `agent_receipt` | `agent-receipt.json` | `b23648d4f111d24982af5a4e7206442ceb88246b043503ab0420c406974f655d` |

## Published Local Tree

- Publish directory: `target/release-evidence-publish/handoff-2c0bb23`
- Manifest hash: `36b82ab2839159254f069816ae3f56f342ed1a72262c4af5e73045c33da49933`
- Source index hash: `7c684ceb669a3db218e261f8ffd2f534fe1e8afed2df708f3bcc616108ba292d`
- Materialized artifacts: `10`

## Non-claims and Notes

- Local trusted evidence only; no remote mirror trust, signature, credential, or public unattended production-readiness claim.
- No tag creation, tag movement, upload, or remote deletion occurred while generating this snapshot.
- Bulky generated receipts remain under `target/` and are not source-controlled.
- `aarch64-linux` is not validated by this handoff; use the documented aarch64 boundary workflow before claiming aarch64 release support.
- Nix attempted remote builder `ssh-ng://root@10.10.10.1`, failed to connect, and continued locally; this was non-blocking because all requested release-evidence receipts passed.
- The payload commit is the evaluated clean HEAD, not this later documentation commit carrying the handoff summary.
