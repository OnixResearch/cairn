## Design

Archive compaction has two separate identities:

1. A **content identity** for the archived material being represented.
2. An **operation identity** for the ordered compaction event that changed local lifecycle storage.

The content identity should be Merkle-shaped so an operator can verify a single archived change, pack, or file without replaying every prior compaction. The operation identity should be chain-shaped so an operator can detect removed, reordered, or inserted compaction events.

### Decisions

1. **Merkle roots for content, chain hashes for history.** Each archive pack records sorted file entries with root-relative path, archive id, file role when known, byte length, and BLAKE3 content hash. Pack roots and archive roots are computed from canonical sorted JSON over those entries. The compaction ledger chains receipts with `prev_chain_hash` and `chain_hash`.

2. **Receipt hash and chain hash are different fields.** `receipt_hash` binds the current compaction receipt with self-hash fields excluded. `chain_hash` is domain-separated BLAKE3 material over the previous chain hash and the current receipt hash. Genesis uses an explicit policy-owned constant rather than an implicit empty string.

3. **Receipts bind before and after state.** A compaction receipt includes `pre_state_root`, `post_state_root`, `packed_archives_root`, `retained_manifest_root`, `policy_hash`, `plan_hash`, `mutation_manifest_hash`, `lossiness`, `receipt_hash`, and `chain_hash`. This proves only local representation integrity, not semantic correctness of the archived change.

4. **Lossless is the first executable profile.** The default profile writes a deterministic pack before replacing or pruning expanded archive files. The pack must be verified before source archive deletion or pointer replacement is considered successful. `compact unpack` restores exact bytes from the pack and refuses unsafe paths or hash mismatches.

5. **Lossy summaries are explicitly gated.** A lossy profile may be designed later, but it must carry tombstones with original paths, byte lengths, BLAKE3 hashes, reviewed summary hashes, and non-claims. Until policy enables such a profile, Cairn rejects lossy compaction requests.

6. **Functional core stays pure.** `cairn-core` owns snapshot normalization, plan construction, manifest root calculation, receipt hash material, chain append validation, and verification diagnostics over in-memory inputs. `cairn-cli` owns directory walking, pack/object writes, ledger writes, source pruning, unpacking, JSON presentation, and process status.

### Candidate command shape

```bash
cairn compact archive --root . --profile lossless-pack --out target/compact-plan.json
cairn compact archive --root . --plan target/compact-plan.json --execute
cairn compact verify --root .
cairn compact unpack --root . --archive 2026-05-18-example-change --execute
```

The first command is read-only. The execute command rechecks the plan, writes pack artifacts, verifies them, writes the receipt and ledger append, and only then applies the planned lifecycle mutation. Verification is read-only. Unpack is dry-run by default unless `--execute` is supplied.

### Failure modes

Negative coverage should include tampered pack objects, missing objects, stale plans, unsafe archive paths, path traversal inside manifests, duplicate manifest paths, wrong previous chain hash, removed ledger entries, unsupported lossy profiles, and interrupted executions that wrote a pack but did not complete mutation. All failures should emit deterministic diagnostics and avoid further mutation.
