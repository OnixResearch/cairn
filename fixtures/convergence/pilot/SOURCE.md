# Retained convergence pilot corpus: provenance

This directory is the retained positive case for `cairn.convergence.conformance`.
The records are copied as exact raw bytes; none is edited or regenerated. The
only file authored here is `case.json` and this note.

## Provenance

- Octet root tree: Octet repository `onixresearch/octet`, reviewed revision
  `d5b34c2d64b1227417a46df8ed8427f2d10ed3c8` (registry `revision.value`).
- Octet pilot records: Octet branch commit
  `e5086dca9b7feabfad37df3769390cc5005811f5`
  ("Add the Cairn convergence pilot contract over the realized producer output").
- Realization: Mantle branch `raise-foreign-import-graph-bounds` at
  `b99ed8637`; the route is `prebuilt closure preserved (Mantle
  preserve-cache-paths-v1 / cache-only-preserve-v1); Nix executed the producer
  job and its proof; Mantle verified the pilot-key signature and preserved the
  output; Mantle did not build the tool graph`.
- Handoff: Mantle branch `admit-octet-producer-layout-in-verification-bridge`;
  `b14db98cb` wrote `handoff/` and the final `7fdb6b861` reproduces it byte for
  byte.
- Signer: the pilot-only key `convergence-pilot-20261119-1`.

## What the case binds

`case.json` names `requirement-registry.json`, `targets.json`, `generated.json`,
the Octet `contract.json`, `decision.json` and `evidence.json`, the bridge
`handoff/`, the consumed `producer-output/` tree, and the Mantle
`realization-receipt.json` and `store-info.json`. `producer_output_locator` is
the absolute path the evidence's deck names; the evidence deck is read inside
the retained `producer-output/` copy.

The route label's non-claims: the preserved output was not built or executed by
Mantle, a cache hit is not fresh proof work, and the receipt says nothing about
whether the tool graph was built from source. The proof itself was executed by
Nix in the producer job.

## Copy commands

```console
git -C /home/brittonr/git/OnixResearch/octet archive \
  d5b34c2d64b1227417a46df8ed8427f2d10ed3c8 \
  .cairn/specs cairn-policy/generated/cairn-policy.json |
  tar -x -C fixtures/convergence/pilot/root

for f in \
  standards/requirements/fixtures/cairn-pilot/requirement-registry.json \
  standards/requirements/cairn-pilot/targets.json \
  standards/requirements/cairn-pilot/generated.json \
  standards/requirements/cairn-pilot/contract.json \
  standards/requirements/fixtures/cairn-pilot/evidence.json \
  standards/requirements/fixtures/cairn-pilot/decision.json; do
  git -C /home/brittonr/wt/octet-convergence show e5086dca:"$f" \
    > fixtures/convergence/pilot/"$(basename "$f")"
done

cp -r /home/brittonr/wt/convergence-evidence/realization/handoff \
  fixtures/convergence/pilot/handoff

cp -r /datapool/mantle-store/convergence-pilot-20261119/store/\
zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1 \
  fixtures/convergence/pilot/producer-output/
chmod -R u+w fixtures/convergence/pilot/producer-output

cp /home/brittonr/wt/convergence-evidence/realization/realization-receipt.json \
  /home/brittonr/wt/convergence-evidence/realization/store-info.json \
  fixtures/convergence/pilot/
```

## Sorted BLAKE3 of every retained file

`b3sum` over `find . -type f ! -name SOURCE.md | sort`, path relative to this
directory. `SOURCE.md` is excluded because it contains this listing.

```text
d9f4719251b9e1f7e53656ec7103f67c76c97a3e5811609b860302ae818f4c50  case.json
56d7f459cc59ddfcdc31d42b86d6c0cef9ae3c03d8a9673ba6d6fb0a66e8eb3e  contract.json
ca047f54add015d84784694c8ccdaa18c74c1f58bf3a835d0eb2df5c7dec91c7  decision.json
5c620f8cd64cd5872bdad7e5af7f82cef8e6f81426c46852c20d6440eb2d5293  evidence.json
a3c483b13fa560791b433991e4353b7fb0c1e9f6d1b2655d20cbe922ccd82ca7  generated.json
607476a979c269dc99d4341c626d7a28f62ce1fbc5390ead39b2d1c2a3b2a5d5  handoff/command.txt
86351a1b7a93d5fbcacc20aa97ad8fff4c9d15b670f47f55988e5ac2deb66291  handoff/external-profile.json
84d2e77717013a203a4daf6ce649de8e501c7386f142b7abc66f32d08c33420e  handoff/external-receipt.json
bc5f54b93bf5c2a2a38186abaaf9d9d6d673880a48bf7c898aa901a5ba981879  handoff/flake-metadata.json
2f4ef5cf2537f92cdfe4adfe5029d0b7d134c4c39454dbb716f8079c8d4e5cdc  handoff/flake.lock
a14ec403025db0330379bad1e1904fae23835ccd1ab95017a80e3e5ca715b318  handoff/job.json
585f5842f47168768b0b24fc5a2b71e7ed282bcb8fc7e935110895ae386e659a  handoff/mantle-observation.json
bfc3e95c55fae9aad7099183d2a2325319b26c37998c7267d158b86b545ab444  handoff/producer-artifact-verification.json
734a1e85a0d9b23fd03e3e514567155f858bb15c3611cab9e21a5c1baeee002e  handoff/proof-cards.json
417f6e10736a1267c42deac018402ee17384d0a433d35c524e11372c3be5beb3  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/Llbc.lean
7255d9366ad9340a5abcb82a970c8b8f461739e90274b2698db2f3b4ffb3880b  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/aeneas-command.json
c16344b6008749131ab6b5ba03881740fb8a75f4f3dde2adb1679aa8e073252d  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/aeneas-lean-library.json
3d546d9090d3331d821aa2f328c7323f9b2aca9bf1836f14d4804fa94b992aef  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/charon-command.json
fc5d991bc0f4af5dff84d99a139597f193a38419c3abbad53e18c672de30034e  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/charon-options.json
6f22675e95c98edcf9ace87cbc9f02ffba770c583b37e782d321c18f2863c952  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/checker-result.json
9fa8dc9570625be2be53d308f958332981ec8fb8137d3dd7ba0ae5da317eaa7d  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/external-models.json
130be6eabfed0fe39289c1ac21e2ec592be79a94f9e693bc5634817a0e524325  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/lean-command.json
4ff9ff39f3ec4e1015d318f317230c2a568a3aded1b49c07a9ab814af2cdbefa  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/lean-toolchain
a4d7eb6892d949ad0b5c50bb1a5a0d54deb7a81732a6d7e760326db6b49cd7b2  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/llbc.llbc
a90f0bc8c3ab6544519d582a901324b218242e006e4f53236a4f486b3ed07a7d  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/package-lock.json
cd11b49d1acede2c6a91fa26f0b7a6b4b28db4007b4601a65b3269706a0ce625  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/provider-profile.json
691e8de8b8f92665dd00b938b48b398b451c71a06c99dae10b38b38184d4dddd  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/source/src/lib.rs
dbda19205e0a5202032ed61b4dae184a68f00af38161324416ae2a4f1ecc5eb4  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/theorem-adequacy.json
776e9f85261080fbcfd75cb481c4f563cff50ffefaf8bdaae3d5fad010d8c77f  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/theorem-statement.lean
776e9f85261080fbcfd75cb481c4f563cff50ffefaf8bdaae3d5fad010d8c77f  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/theorem.lean
ac9089c17b063507da0515308a099eafe255e22605c0d9e633d6bb89b23b0abb  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/translation.json
021c033fb39bbf2ba3ef4d1e14a473e5988c1b464978881ec720c3b4e01d8379  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/trust-closure.json
84d2e77717013a203a4daf6ce649de8e501c7386f142b7abc66f32d08c33420e  handoff/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.receipt
426d7941e5c634fba40b1ee7d716ede5f661982d93c774d151986d82b5d4a0fc  handoff/root-derivation
c2b3ff97239a0a12efdd0e64f506165e0829887c22ca32a90c4d9b514426fe8c  handoff/source-revision
691e8de8b8f92665dd00b938b48b398b451c71a06c99dae10b38b38184d4dddd  handoff/src/lib.rs
f694e9a6b94bf4a9842b3171730035c8656e1fc71ecc09d7debff9d9af472068  handoff/status.json
1fa303ca5ecc02778baeb097a9d34e66e3c2c04fe9ea69ecb56be1d3eb39064b  handoff/summary.txt
d0ebfcbae2f8b44866a45df068cec42cf5be5b6379ff72cb91bddbe919697808  handoff/tool-closure.derivation-json.json
607476a979c269dc99d4341c626d7a28f62ce1fbc5390ead39b2d1c2a3b2a5d5  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/command.txt
188f032abc850c786660609dfe57c7198345be4f6d6fca07c1d3344301bfe723  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/evidence-log-manifest.json
734a1e85a0d9b23fd03e3e514567155f858bb15c3611cab9e21a5c1baeee002e  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proof-cards.json
417f6e10736a1267c42deac018402ee17384d0a433d35c524e11372c3be5beb3  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/Llbc.lean
7255d9366ad9340a5abcb82a970c8b8f461739e90274b2698db2f3b4ffb3880b  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/aeneas-command.json
c16344b6008749131ab6b5ba03881740fb8a75f4f3dde2adb1679aa8e073252d  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/aeneas-lean-library.json
3d546d9090d3331d821aa2f328c7323f9b2aca9bf1836f14d4804fa94b992aef  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/charon-command.json
fc5d991bc0f4af5dff84d99a139597f193a38419c3abbad53e18c672de30034e  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/charon-options.json
6f22675e95c98edcf9ace87cbc9f02ffba770c583b37e782d321c18f2863c952  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/checker-result.json
9fa8dc9570625be2be53d308f958332981ec8fb8137d3dd7ba0ae5da317eaa7d  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/external-models.json
130be6eabfed0fe39289c1ac21e2ec592be79a94f9e693bc5634817a0e524325  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/lean-command.json
4ff9ff39f3ec4e1015d318f317230c2a568a3aded1b49c07a9ab814af2cdbefa  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/lean-toolchain
a4d7eb6892d949ad0b5c50bb1a5a0d54deb7a81732a6d7e760326db6b49cd7b2  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/llbc.llbc
a90f0bc8c3ab6544519d582a901324b218242e006e4f53236a4f486b3ed07a7d  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/package-lock.json
cd11b49d1acede2c6a91fa26f0b7a6b4b28db4007b4601a65b3269706a0ce625  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/provider-profile.json
691e8de8b8f92665dd00b938b48b398b451c71a06c99dae10b38b38184d4dddd  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/source/src/lib.rs
dbda19205e0a5202032ed61b4dae184a68f00af38161324416ae2a4f1ecc5eb4  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/theorem-adequacy.json
776e9f85261080fbcfd75cb481c4f563cff50ffefaf8bdaae3d5fad010d8c77f  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/theorem-statement.lean
776e9f85261080fbcfd75cb481c4f563cff50ffefaf8bdaae3d5fad010d8c77f  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/theorem.lean
ac9089c17b063507da0515308a099eafe255e22605c0d9e633d6bb89b23b0abb  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/translation.json
021c033fb39bbf2ba3ef4d1e14a473e5988c1b464978881ec720c3b4e01d8379  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/trust-closure.json
84d2e77717013a203a4daf6ce649de8e501c7386f142b7abc66f32d08c33420e  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.receipt
97b164c4d081697a35abf5b2b1e7ace0b3fb8950625c9d4351344fc16ec6f707  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.stderr
20fe247f23727146eb84dd18b4e3923958c3d2197c96e0687f77c165beca9589  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.stdout
f9819cf81800d913044308bfafb3814b7304342f98bb4641365ddf376961bd88  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/provenance.jsonl
f694e9a6b94bf4a9842b3171730035c8656e1fc71ecc09d7debff9d9af472068  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/status.json
1fa303ca5ecc02778baeb097a9d34e66e3c2c04fe9ea69ecb56be1d3eb39064b  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/artifacts/summary.txt
607476a979c269dc99d4341c626d7a28f62ce1fbc5390ead39b2d1c2a3b2a5d5  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/command.txt
86351a1b7a93d5fbcacc20aa97ad8fff4c9d15b670f47f55988e5ac2deb66291  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/consumer-profile.json
188f032abc850c786660609dfe57c7198345be4f6d6fca07c1d3344301bfe723  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/evidence-log-manifest.json
86351a1b7a93d5fbcacc20aa97ad8fff4c9d15b670f47f55988e5ac2deb66291  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/external-profile.json
84d2e77717013a203a4daf6ce649de8e501c7386f142b7abc66f32d08c33420e  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/external-receipt.json
2f4ef5cf2537f92cdfe4adfe5029d0b7d134c4c39454dbb716f8079c8d4e5cdc  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/flake.lock
a14ec403025db0330379bad1e1904fae23835ccd1ab95017a80e3e5ca715b318  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/job.json
bfc3e95c55fae9aad7099183d2a2325319b26c37998c7267d158b86b545ab444  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/producer-artifact-verification.json
e7acf6d4bcefc4be2d0b90274ad1decd012af8d8b25ee347eb04bc4da36fd9e7  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/producer-prove.json
734a1e85a0d9b23fd03e3e514567155f858bb15c3611cab9e21a5c1baeee002e  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proof-cards.json
417f6e10736a1267c42deac018402ee17384d0a433d35c524e11372c3be5beb3  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/Llbc.lean
7255d9366ad9340a5abcb82a970c8b8f461739e90274b2698db2f3b4ffb3880b  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/aeneas-command.json
c16344b6008749131ab6b5ba03881740fb8a75f4f3dde2adb1679aa8e073252d  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/aeneas-lean-library.json
3d546d9090d3331d821aa2f328c7323f9b2aca9bf1836f14d4804fa94b992aef  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/charon-command.json
fc5d991bc0f4af5dff84d99a139597f193a38419c3abbad53e18c672de30034e  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/charon-options.json
6f22675e95c98edcf9ace87cbc9f02ffba770c583b37e782d321c18f2863c952  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/checker-result.json
9fa8dc9570625be2be53d308f958332981ec8fb8137d3dd7ba0ae5da317eaa7d  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/external-models.json
130be6eabfed0fe39289c1ac21e2ec592be79a94f9e693bc5634817a0e524325  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/lean-command.json
4ff9ff39f3ec4e1015d318f317230c2a568a3aded1b49c07a9ab814af2cdbefa  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/lean-toolchain
a4d7eb6892d949ad0b5c50bb1a5a0d54deb7a81732a6d7e760326db6b49cd7b2  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/llbc.llbc
a90f0bc8c3ab6544519d582a901324b218242e006e4f53236a4f486b3ed07a7d  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/package-lock.json
cd11b49d1acede2c6a91fa26f0b7a6b4b28db4007b4601a65b3269706a0ce625  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/provider-profile.json
691e8de8b8f92665dd00b938b48b398b451c71a06c99dae10b38b38184d4dddd  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/source/src/lib.rs
dbda19205e0a5202032ed61b4dae184a68f00af38161324416ae2a4f1ecc5eb4  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/theorem-adequacy.json
776e9f85261080fbcfd75cb481c4f563cff50ffefaf8bdaae3d5fad010d8c77f  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/theorem-statement.lean
776e9f85261080fbcfd75cb481c4f563cff50ffefaf8bdaae3d5fad010d8c77f  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/theorem.lean
ac9089c17b063507da0515308a099eafe255e22605c0d9e633d6bb89b23b0abb  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/translation.json
021c033fb39bbf2ba3ef4d1e14a473e5988c1b464978881ec720c3b4e01d8379  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.aeneas/trust-closure.json
84d2e77717013a203a4daf6ce649de8e501c7386f142b7abc66f32d08c33420e  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.receipt
97b164c4d081697a35abf5b2b1e7ace0b3fb8950625c9d4351344fc16ec6f707  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.stderr
20fe247f23727146eb84dd18b4e3923958c3d2197c96e0687f77c165beca9589  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/proofs/octet.aeneas_consumer_contract.external-fixture-rail-0.stdout
f9819cf81800d913044308bfafb3814b7304342f98bb4641365ddf376961bd88  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/provenance.jsonl
df6b3139fe05c69ee2331c3498fa362ff65ae5a41144706727892781a3401c04  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/provider-identity.json
ea943a34f613cb967e4c5bc9568b42d32e04d9d64fc0b1486776e8f787a0fc00  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/source-revision
28826659c5095a872ca9b17c747b96be1bc3afc147f2f383a7df5a527ab04370  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/source/Cargo.toml
776e9f85261080fbcfd75cb481c4f563cff50ffefaf8bdaae3d5fad010d8c77f  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/source/Theorem.lean
691e8de8b8f92665dd00b938b48b398b451c71a06c99dae10b38b38184d4dddd  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/source/src/lib.rs
691e8de8b8f92665dd00b938b48b398b451c71a06c99dae10b38b38184d4dddd  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/src/lib.rs
f694e9a6b94bf4a9842b3171730035c8656e1fc71ecc09d7debff9d9af472068  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/status.json
1fa303ca5ecc02778baeb097a9d34e66e3c2c04fe9ea69ecb56be1d3eb39064b  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/summary.txt
f5a5ae500f4b418a45918ef3115b8317c469752567236fe5a43df0d7cddafcdf  producer-output/zjq6whhf2c641vkz89kgwwpbajjq5cbq-octet-mantle-aeneas-consumer-job-v1/valence-provenance.json
872faacbae5b2212a0438106dcb6ea76702b1779f829c011bcba8a86adb8c1b6  realization-receipt.json
7982ceb5f4aad030876dcfd3e195206f512d0be6a989597a303547e936a3b4f3  requirement-registry.json
7230bc08fad44ebe0e04942d06463adc9d51d48812df10347e0e2a5e0f8bb94e  root/.cairn/specs/accumulating-validation/spec.md
2a34584872f9f8af4733830600e9ced7ed7377e93c3b28b81b27b2574f35fb93  root/.cairn/specs/adoption-story/spec.md
a9e9dd90ead57c1a984c2ffe15546ae2e9626a0658cade20b7a19af9fbbda932  root/.cairn/specs/advisory-lint-promotion/spec.md
7b6bbfdae779ada85f40f563c5d4db8c9afec9c00581f07794b4d907ec6f15c9  root/.cairn/specs/aeneas-consumer-contract/spec.md
fe11cfe79d48bce9dd8ac2e4593fc5cdae791f9b5f92e4c451baea3eb8f79563  root/.cairn/specs/aeneas-lean-proof-rail/spec.md
1d03e11974bde9858601e13cf3da2d1b93576649779636a17040a8226f4fff5e  root/.cairn/specs/api-stability-rail/spec.md
314617008ecc230cc7dc964c67379eea558eb8795af064f08e060f6206825bb9  root/.cairn/specs/artifact-bundle-verification/spec.md
9ce93105035d006b2cd71b4c75b15ca5fa0fa0acff14317211992c33a7638dc0  root/.cairn/specs/assurance-profiles/spec.md
3d77919ff2c8a970507d96fcd5ac99f5b49b97aded35ba3ca3bcad712136976e  root/.cairn/specs/ast-grep-structural-rails/spec.md
cb5de681ebf1f6e959519a0e3e5c5a6d22e9ed220730ca3967a09e99e44eaee0  root/.cairn/specs/authorized-effect-protocol/spec.md
a254ee5a499314aa41f8cf09828545dbaadf1c89f09eb09541db2c97356058f0  root/.cairn/specs/benchmark-evidence/spec.md
6d45a3aecc1926ee2ed64a66c0313ebd82a7de9f72f5ea1f98e5e165eb3ec834  root/.cairn/specs/build-hygiene/spec.md
beb79a2df4b3f029369d51b38d820db9d89d759ba363777bcf6ed1a6cd0c937c  root/.cairn/specs/capability-manifest-checker/spec.md
e5249e01a10dcde310eb0c71e79b001d87ef1831901b26f9a99f32dc0f2f3c28  root/.cairn/specs/charon-semantic-analysis/spec.md
a05257f36fa279a80f63872cee4ffa3c5391041cc651da0fdb7927e91337a4d0  root/.cairn/specs/checked-contract-ladder/spec.md
d556c8c177d967178a4969286f9a2e87a430c697841f877433b79643dc08e746  root/.cairn/specs/cob-collaboration-adoption/spec.md
48e0048ad5596e7bd625df38827a96103541e147c952948c37c00e78ed45342a  root/.cairn/specs/cob-collaboration-conflict-recovery/spec.md
9c2c5f0a5791cc722d0ca32c880078fcd61e08b9c4370db2401a8284ec21e0f0  root/.cairn/specs/compiler-architecture-ir/spec.md
e07611dbedf8580066c75a42b452192d0beae2a0056aa0c32af7e65a92bb1431  root/.cairn/specs/compiler-architecture-targets/spec.md
868358f1f6f8d6ee51c26f00a5ccec0508e184db697fb8388a340335b60d6937  root/.cairn/specs/compiler-policy-provider/spec.md
ef27fd4276f846894bbb3e17d4c8175573ce9bb926921a72bb27f10eaecc548f  root/.cairn/specs/concurrency-performance-lints/spec.md
4b6d8bf876eaa79e35e490fddb1de676c8458a0341ab6251d074c38eeef63173  root/.cairn/specs/controlled-deployment/spec.md
3d9740333fbba86405c18cda4c7b751def3add10d23e4d7163d427bca7a071fd  root/.cairn/specs/coverage-baseline-evidence/spec.md
4e2b24e9698b77cf18dace4c665d30f46b0291cc47278e0b30fdd69604a649bd  root/.cairn/specs/crate-boundary-config/spec.md
ec23e9727a8786a46a7a9682efcd6a4fdd8a4d1b81d792b9fd7a325ae5ce5afc  root/.cairn/specs/dependency-audit/spec.md
b5548150a8aef58516867f49d296deba518ee280d40f28f42003f9bf2dcaf127  root/.cairn/specs/dependency-direction/spec.md
3a4595b3ec9f5eb60b53acc5668810643e40953283be94ba3cdf9711c2bc770a  root/.cairn/specs/dependency-trust/spec.md
cb16fc25a34c3a8b42d233c333538df6c045ee17a7ff0371af23f407729968bb  root/.cairn/specs/dynamic-evidence-rails/spec.md
726547434dc67ad66c3589830d0e0b3846ea3af561b4b4b355ecab2dc3c05913  root/.cairn/specs/effect-closure-admission/spec.md
a40b4d41efd01a98a1720f33589a95f7bb76fc6c9d6204e38027fb2bb5a2d084  root/.cairn/specs/evidence-codebase-index/spec.md
a9429cc15b417b7082181f9f128060ed306354a55edf63d006ac693b9a591b64  root/.cairn/specs/evidence-contracts/spec.md
dc980cbe0c66616b149d4e5a5dedb7ce714a8ad0d3fb9c6953e4eac14ac7f3a2  root/.cairn/specs/evidence-log-storage/spec.md
84f67b937c8e5ec0108f358a4944ec222b6085a0c3a09d9134e08857173e00dd  root/.cairn/specs/evidence-provenance/spec.md
622fc25ef1d8c5c4c82c7810733cecef8f5189465e3b548d8ba1c061a9a97763  root/.cairn/specs/evidence-stack/spec.md
183737d9e10659eef0a18206acaf2a1b52acaa5edd7c2af9cb6189097c4a7ef7  root/.cairn/specs/evidence-tier-taxonomy/spec.md
070513dd34da0f8455eb4b3e89664f5ee287973010745e01bdd853dc0e11e275  root/.cairn/specs/experimental-solver-rails/spec.md
4f6f2ca65c7f3c756424680774f5e725557bb57259d155e5bb413a27162488fd  root/.cairn/specs/external-lint-reporting/spec.md
4dcb003a30681cf3a5ab5b50b5fb85679bfd78c316dc10f613d550ddf41f6025  root/.cairn/specs/function-address-export/spec.md
c86c0505739b89f4ea64bd8c982558f9b89b77603570372735ca6d5a5962d811  root/.cairn/specs/gate-posture/spec.md
ffe30bd1f84ae2be686d2f546539c84121bb673638525b04c494e5017ac0e5fa  root/.cairn/specs/hexagonal-ports/spec.md
a24f54620f4bd5a81cabc2b27de5f466b98df164c32c2bc8492aacfcb16dc894  root/.cairn/specs/kani-evidence-rail/spec.md
d8f2661519403373932dd802157b5a28c5d0c2326d8c9225c3da6a1bba31ea63  root/.cairn/specs/kani-panic-pilots/spec.md
cc2fb0ba5c9dc6e8c17a0a2f5f086a49f2eee84aaf19c4f78266563fe43e9c56  root/.cairn/specs/kani-property-checklist/spec.md
dfad47ff4995c1de1652966ca8263ea961c6c208db423f37290e261a2bd82d02  root/.cairn/specs/law-tested-deterministic-cores/spec.md
c9c57c4345448451f60b0c31cf12c614300d912fd9a1e0cf1cfd3d6e1713b7b7  root/.cairn/specs/lint-analysis-cores/spec.md
f0a98b618b5e496e7a485df634a1e6fd5409751cd70b4b3965b5767e24d1eb13  root/.cairn/specs/lint-autofix-quality/spec.md
40b3939ccb01718386dc4eb9a11a6f0866e35a928763e82f8e3316f9f0f1aaf7  root/.cairn/specs/lint-catalog-freshness/spec.md
e3ee51091b179eed2e337c9d22f8e75f4782937604cd0829e7cb1c54770f0476  root/.cairn/specs/lint-guardrail-fixtures/spec.md
9911f20f1d0877d7f421d28704f499de2e1a44a0fb9a1ab6db29b615f9dd3988  root/.cairn/specs/lint-guidance-freshness/spec.md
4749389d351801267c6fb9430119143aeb1aad7b52dca75c9dddd30bae377074  root/.cairn/specs/lint-metadata-catalog/spec.md
8a60536fe9c568ee9cad3a7615aa54cc39fda38843120868826e50196208080c  root/.cairn/specs/lint-signal-quality/spec.md
3c7cfd23e4829cceba5d793a3bd2b07d709a5b7ac48b2c3e8f39d1cffd094757  root/.cairn/specs/mantle-producer-handoff/spec.md
14df17f735e38e42d6b7f24e47751d29747f92759194176ea9e8a21519bd0191  root/.cairn/specs/mirai-semantic-pilot/spec.md
2c128be3482851751cdfdd7de9378ef644e6c9038b9b60740b01b787fe2c1668  root/.cairn/specs/mutation-testing-evidence/spec.md
681705bd7bb814be11a76ce4ff4eff27564fa90fe118a37421f686f07c229299  root/.cairn/specs/namespace-compatibility/spec.md
33a8000781bfa60e81f5fbe0874266f643eac3338d9d40ebe5befb6286489a80  root/.cairn/specs/nickel-export-helpers/spec.md
3b3ff461f73c3f196f4200bca3202a3e39e4552b5893516e8378a8f9fcc75b95  root/.cairn/specs/nickel-toolchain/spec.md
6c3197f74c3dae7771acbdc46ed2914d8b7f971e00d3769ffdc8e0ceabded504  root/.cairn/specs/object-corpus/spec.md
0ee1572acb15306cfbaf293fc8cdfcdf882c1c7fccdd26ef13cf830c6a08679a  root/.cairn/specs/ocp-enum-match-lints/spec.md
7256172a26db511ccb2c1fecf6e6983a78eb4e547a91856b9c5e009162126903  root/.cairn/specs/open-issue-resolution/spec.md
5d5d87b3ead9e8b46bdf1417168f8a697e7e6b26848165f36591f182bddc4433  root/.cairn/specs/policy-package-customs/spec.md
b4aa6731d530bc0c438811f1b48466b9b038eb18277e7e1d9ac321ee08e5cd28  root/.cairn/specs/product-positioning/spec.md
0646ed216da5f6dbf1582273415269a778598012f3f52b39375e84003716e734  root/.cairn/specs/project-architecture-gate/spec.md
fc2b978dd648b6254694bf369d4ddfbd7f66c859cd0f522340235e79bf3c692c  root/.cairn/specs/project-architecture-policy/spec.md
ee822f0eecfc2f90a8d339b0d13111b73f807abb94cc8f24898e07907f801381  root/.cairn/specs/proof-artifact-library/spec.md
ebbc34532abf5bda511366060954567c8bcd266cdaf1f5c226f1bf204b9b56f3  root/.cairn/specs/proof-claim-catalog/spec.md
001d2265905640f5d65847ba57f3da76599c7a11bab8c14eda9dc6e0e8716db1  root/.cairn/specs/proof-implementation-gap/spec.md
7a76845ad37897675b19af2e0ec67165847ab736fbc9cd84681ae39b7f14a9b9  root/.cairn/specs/purity-effect-lints/spec.md
d4c28d29b959721c0e7e805b2d94447b66c0196affb024e645092acc4880d706  root/.cairn/specs/release-supply-chain/spec.md
1aae7b8c50c41a71a2032a8a70e504ade9bc1b4c9b7e0c6cb3afb736e670cbf6  root/.cairn/specs/runner-core-architecture/spec.md
8cb83cff74aaebeb7e49a69e09f6217b370034c71c9cc7c0be16e2c37ec664be  root/.cairn/specs/runner-self-check/spec.md
97fc6611e7ee962c3a162e39d21e19111e0a92220bc6c7dd4d9ef444241d0aee  root/.cairn/specs/safety-lint-semantics/spec.md
b6c4bdc845aa1096e24f986155759ecffef5e33d8046ecc12d0544c6ceb43fc2  root/.cairn/specs/scope-boundary/spec.md
268e18ac4f5677facb578921d323624d84717762e2ca63b203beb7828990c44a  root/.cairn/specs/self-check/spec.md
03c5c0d1675bef1da6b426402ff2b0da499f57a6d63b9352008cf89e6d3d1353  root/.cairn/specs/semantic-merge-verification/spec.md
968464c0dd1b806c5dbc3d56da0fc5f1d746a9bbcd908059ac1fb62046b3ec37  root/.cairn/specs/small-checker-receipts/spec.md
87818f9a81feb7771f762848f108733c938a8e82d72365a7df6d8af3a3304fa8  root/.cairn/specs/solid-structure-lints/spec.md
c881568aaefda4f496782bbbef97970e29c7730497b4898ba91c1706f45f7e1b  root/.cairn/specs/spacewasm-mvp-artifact-profile/spec.md
224f91ea0e598e8e1853c5eb98abc0f1bd8f214125364ba02a36c474265d22d5  root/.cairn/specs/spacewasm-pilot-abi-admission/spec.md
1835a5fab162ebe29bd0dae3e88590b4336373373a0310c824bec39c51f7e701  root/.cairn/specs/spec-trace-parity/spec.md
0db64a0453c6377ca72a1540cf16cbbad18f04ecab8d6c4ab5f5fd710c282f68  root/.cairn/specs/state-machine-exhaustiveness/spec.md
2551d45ee8cb93be658e1e28b7f7b14f2c938eadf2069ee7b8eb3340dec4f82d  root/.cairn/specs/strict-rust-profiles/spec.md
cedd5bcceb8e411d019f5857fb2646537b284918cbe6b8d870116050a4cafab6  root/.cairn/specs/structured-migration-sessions/spec.md
b1fc8dbc18b4844d9c302e7bc254964801b8bf14a24a8eb99a4ec1d79f6e294a  root/.cairn/specs/trust-boundary-index/spec.md
a2a519ac9e7c77c1bc37fadc39f06757dc84a13749e30a15569cf05b77149b7b  root/.cairn/specs/type-driven-domain-modeling/spec.md
9d1afa77393d14aab586cf3fdd946d751a9d7dc88c2fa9f871ad4c8eeb7e4996  root/.cairn/specs/unsafe-trust-boundary/spec.md
3e74c8420d5f7687135d3a845c72a85aae46c72376eabe6b1fa1ae21a6695451  root/.cairn/specs/valence-evidence/spec.md
98fe83fbd8c2ba3159d6bf0a82153d680d904e710adf44c782a2a42dd8944704  root/.cairn/specs/valence-project-extraction/spec.md
c97b531a3d220404786e72b465466d8253f042cb92881322256d1a1627d05dd2  root/.cairn/specs/verifier-fixture-intake/spec.md
86191214280be5e196f317f5fd385d74e00d75adabfcb96de56b2716ab02cb48  root/.cairn/specs/vermilion-canary-rail/spec.md
74534660d8655d0a1cf9bd2747765b2be7cd534c7f979bc623364e8969e38f61  root/.cairn/specs/vermilion-proof-rail/spec.md
f399d502953b7c39c99cda2ae2eb22c9e0e8df6696094187c87c8e89ed93c6ed  root/.cairn/specs/verus-proof-rails/spec.md
24f28bdda552557e5c5b93e35ff49ea8f57d86145ec4a92bf491c2b9021d5a3f  root/.cairn/specs/wasm-component-artifact-rail/spec.md
728c4b47495c4469c2cdaa765908fd380732835f489f41a1270b60f36c4ae8e6  root/cairn-policy/generated/cairn-policy.json
df94c72baa071a69f7edd2b97c8eed61d37b9cfa7ea2b85270dd956e46634cfd  store-info.json
56c803468b0c0682f6d43dea11f1ebc20fdcda631b4cd52a8ae1970fa8f54dcd  targets.json
```
