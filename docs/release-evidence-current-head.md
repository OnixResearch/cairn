# Current-HEAD Release Evidence Index

## Payload

- Payload commit: `b253c9c`
- Branch: `main`
- Worktree clean: `true`
- Upstream: `origin/main`
- Ahead/behind: `0	0`

## Commands

- `release-readiness --root <root>`
- `evidence export --root <root> --out <artifact-dir>/exported`
- `evidence verify --root <root> --bundle <artifact-dir>/exported`
- `evidence import --root <root> --bundle <artifact-dir>/exported --into <artifact-dir>/imported`
- `evidence republish --root <root> --imported <artifact-dir>/imported --out <artifact-dir>/republished`
- `evidence index --root <root> --bundle <artifact-dir>/republished --out <artifact-dir>/release-evidence-index.json`
- `evidence index verify --root <root> --index <artifact-dir>/release-evidence-index.json`

## Receipt Hashes

| Role | Verdict | Receipt hash |
| --- | --- | --- |
| `evidence_verify` | `pass` | `50eb079731536f7f5336803802d938810c942e5760e5e66172c08fada23ba720` |
| `release_readiness` | `pass` | `a338363ced0b72d7567f40390132a0b5254234cd7af67533d0d2da4e47824a5c` |
| `evidence_republish` | `pass` | `f1e01ffe08dfe251b9f13c6a7a01e59fdfe9608cb916cb7d1b9fc243194d37cc` |
| `evidence_import` | `pass` | `420d98299aaa14e7bf2eaf5b264ceb37e2556ec760bcaf2e86f0101a1095893b` |
| `evidence_index` | `pass` | `e55d5b1cc683d6d0ab708218dc92eaa91584717e4cf3c0188aace16b4fe6a446` |
| `evidence_export` | `pass` | `e2633e19494fa54be775950fde71e604b948ed39d29a13b97f97b1b7225ae7b1` |
| `evidence_index_verify` | `pass` | `d008ae2df47077ec0928eb82693e2f0ad6acf4ce4d8dd987e5d2a2de617f6e1a` |

## Bundle Artifacts

| Role | Path | BLAKE3 |
| --- | --- | --- |
| `coverage_audit` | `coverage-audit.json` | `0de071933bf47b671d41d63547a121abae80eff9c5b24478b5a325dc77e9bfdc` |
| `replay_receipt` | `replay-agent.json` | `3da60ee798a2c82209ed4a90f192798859398a8f9fe110523ad618eee983cf97` |
| `agent_receipt` | `agent-receipt.json` | `c021b13a4af56965ac8fab57ad7eb65e8642c6b12b2addd6e0c3d4ff1ec8f113` |
| `release_readiness` | `release-readiness.json` | `c65442035e09483d2f91398f2c16e26d79c71842da870b29ab8b6fd5b95e168d` |
| `evidence_manifest` | `evidence-manifest.json` | `a23933af4cecd7a8574e5481fa86309a90c6f297b9b1bf725b7888c64210edc3` |
| `agent_evidence_bundle_verification` | `agent-evidence-bundle-verification.json` | `8c664b983369d01818422dba5df80eec26433fde1f8d3d836c7600bbc04c0e7a` |
| `evidence_manifest_verification` | `evidence-manifest-verification.json` | `bfc7b60000f8a1c3e40e0b7aabd3139c17d81639bc45f4b91dc36b27dbdd8d0b` |
| `replay_verification` | `replay-verification.json` | `e9f1e33e9fa06f1e47dbba2c853848cc71a783e218b99727cc0da545fe0c5ee9` |
| `agent_evidence_bundle` | `agent-evidence-bundle.json` | `d456839931d5c351468dbb1150766aea44747e1d4773ee1ccb27f529b47ceba1` |

## Non-claims

- Local trusted evidence only; no remote mirror trust claim.
- No credential, transport, or public unattended production-readiness validation.
- No tag creation or tag movement.
- The payload commit is the evaluated clean HEAD, not a later documentation commit carrying this index.
