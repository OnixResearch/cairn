# Release evidence index for verified code commit a5d3928

This page records a checked-in operator index for the verified code commit `a5d3928` on branch `main`. The evidence-recording commit is the commit that carries this page; do not treat this page as a request to move any readiness tag. r[impl release_evidence.current_head_evidence_index.focused_modules.visible_boundaries]

## Scope boundary

- Scope: local/trusted Cairn release-readiness and evidence-publication receipts generated from commit `a5d3928`.
- Non-claim: this is not a public unattended production-readiness claim and it does not assert remote mirror, credential, transport, or external trust validation.
- Tag state at capture: no local Git tags pointed at `a5d3928`, and no local tags were listed by `git tag --list --sort=-creatordate`; this index therefore binds the commit and generated receipts directly rather than moving or inventing a readiness tag.
- Worktree state at capture: `## main...origin/main` plus this evidence-index/docs slice.

## Generated local evidence bundle

Generated artifacts are intentionally under `target/roi-05-current-head-evidence/` and are not checked in. The checked-in contract is this bounded index plus the commands/hashes below. r[impl release_evidence.current_head_evidence_index.focused_modules.narrow_bridges]

| Artifact role | Relative path in export bundle | BLAKE3 hash |
| --- | --- | --- |
| `agent_evidence_bundle` | `agent-evidence-bundle.json` | `fbe92e70508e8b9627f45b66965b229e109109751647de2cc5cdcf858270f30d` |
| `agent_evidence_bundle_verification` | `agent-evidence-bundle-verification.json` | `78de3bc2c94211312229ace34df5a6ea6b3b0854ceb563e894e449d9a17bdfea` |
| `agent_receipt` | `agent-receipt.json` | `950931d6307c5da05f1eff5e392310b1cac9959cd3efb2a4435a21d0b6c3c4b8` |
| `coverage_audit` | `coverage-audit.json` | `639afd286becf0f9ea531fc205ad58876476c16f555e73f325674779fa881e22` |
| `evidence_manifest` | `evidence-manifest.json` | `e5aa9b9b7631dde06dbd671393f7517070651c2d6152b91e871910c1bc9198d3` |
| `evidence_manifest_verification` | `evidence-manifest-verification.json` | `d0d67a84c7458cb3a5f8cdd60ddab1736e7f65584ee1ba41bb932e6245bc3b56` |
| `release_readiness` | `release-readiness.json` | `08cf20f2911f25098fb96b2d024376593f024817507fbb8c8e7db64eee3cca92` |
| `replay_receipt` | `replay-agent.json` | `f9779af0927f50a74f6cf24a701d65115c3b3d5214866f39016f72cc13e39001` |
| `replay_verification` | `replay-verification.json` | `c1ba8b8b34136fd5a52dbc4e011a5f54ac9541324b62051181f87a3da7bcc2c3` |

## Release-readiness receipt summary

Command:

```sh
nix develop -c cargo run -p cairn-cli --bin cairn -- release-readiness --root . > target/roi-05-current-head-evidence/release-readiness.json
```

- Receipt verdict: `pass` (`valid=true`)
- Release-readiness receipt hash: `ce6f2cfab90df04978d1eae15115ff20793b6ecc31bd2115503673d31d70b506`
- Cairn validation hash: `d0926f76352ed19998c6f7978bd69ab9ab34906e71a3caae9a6d033f0864a7c8`
- Tracey coverage hash: `437e9e0a7ab019eb7b32813090e62a3644a772d17e4e5c329f5bfbddc495ed17`
- MCP smoke receipt hash: `bd5ca2862a4ab2705fc578d0bca4b0c687b404d32c2a75fde4913b5fb6dcfb16`
- Coverage-audit receipt hash: `c5266598f6f7d6b8384e32f77a01c24e8ca9451f9ccc2d03e1d7f0e4c3ba76ae`

| Check | Passed | Receipt/evidence hash | Source |
| --- | --- | --- | --- |
| `cairn_validate` | `true` | `d0926f76352ed19998c6f7978bd69ab9ab34906e71a3caae9a6d033f0864a7c8` | `cairn validate --root <root>` |
| `tracey_coverage` | `true` | `437e9e0a7ab019eb7b32813090e62a3644a772d17e4e5c329f5bfbddc495ed17` | `cairn tracey coverage --root <root>` |
| `mcp_agent_smoke` | `true` | `bd5ca2862a4ab2705fc578d0bca4b0c687b404d32c2a75fde4913b5fb6dcfb16` | `cairn agent-receipt smoke-mcp --root <root>` |
| `determinism_coverage_audit` | `true` | `c5266598f6f7d6b8384e32f77a01c24e8ca9451f9ccc2d03e1d7f0e4c3ba76ae` | `cairn determinism coverage-audit --root <root>` |
| `repository_gate_policy_listed` | `true` | `4cd0c10efbc5d0e78d3df6c26b31cf241d7d0e2ac49c41d7fe33092eab58f718` | `maintained local policy` |

## Evidence publication chain

Commands:

```sh
nix develop -c cargo run -p cairn-cli --bin cairn -- evidence export --root . --out target/roi-05-current-head-evidence/export
nix develop -c cargo run -p cairn-cli --bin cairn -- evidence verify --root . --bundle target/roi-05-current-head-evidence/export
nix develop -c cargo run -p cairn-cli --bin cairn -- evidence import --root . --bundle target/roi-05-current-head-evidence/export --into target/roi-05-current-head-evidence/imported
nix develop -c cargo run -p cairn-cli --bin cairn -- evidence republish --root . --imported target/roi-05-current-head-evidence/imported --out target/roi-05-current-head-evidence/republished
nix develop -c cargo run -p cairn-cli --bin cairn -- evidence index --root . --bundle target/roi-05-current-head-evidence/republished --out target/roi-05-current-head-evidence/cairn-evidence-index.json
nix develop -c cargo run -p cairn-cli --bin cairn -- evidence index verify --root . --index target/roi-05-current-head-evidence/cairn-evidence-index.json
```

| Step | Verdict | Receipt hash |
| --- | --- | --- |
| evidence export | `pass` (`valid=true`) | `6c003ce4d100eea126e6e450d9b3a63592c4f5a330fbab261f7a631d294020e8` |
| evidence verify | `pass` (`valid=true`) | `c42c2dd38cb78032234e7ecc5a72e3cea8b2507f07ff1cde1fbedb48ecb454ae` |
| evidence import | `pass` (`valid=true`) | `7b9afeab3c43be5af9f7612d4d2a97c2a26c975b7296a361d35ab9ef63390d1c` |
| evidence republish | `pass` (`valid=true`) | `a01db6dbf0a221596e487624765432b29079dc52ab418c9967aca2fe2f68baac` |
| evidence index | `pass` (`valid=true`) | `3dd76cc9ea7bc2c7c1c4819a6682d0473ff656cc7bf7422450c61fc41843bd28` |
| evidence index verify | `pass` (`valid=true`) | `3c1eefff6d888d1ed833b06f3bbe026f0bbeec0abfb6ddfad2de292ccc8d6b93` |

Index details:

- Bundle kind: `republished`
- Bundle receipt hash: `a01db6dbf0a221596e487624765432b29079dc52ab418c9967aca2fe2f68baac`
- Index verification receipt hash: `3c1eefff6d888d1ed833b06f3bbe026f0bbeec0abfb6ddfad2de292ccc8d6b93`
- Verified index receipt hash: `3dd76cc9ea7bc2c7c1c4819a6682d0473ff656cc7bf7422450c61fc41843bd28`

## Review notes

- The index uses stable relative artifact paths and receipt hashes; raw target artifacts remain generated local outputs.
- The release-readiness receipt records the heavyweight gate policy, while this page also documents the focused evidence-publication verification chain.
- The evidence-recording commit should be reported separately after commit/push so this file avoids self-referential final-HEAD claims. r[verify release_evidence.current_head_evidence_index.behavior_preserved.focused_checks]
