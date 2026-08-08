## Design

### CLI surface

Add a read-only subcommand under existing determinism tooling:

```bash
cairn determinism verify-receipt <receipt.json> --root .
```

The command reads a saved replay receipt and emits JSON. It does not run replay cases or mutate the workspace.

### Verification checks

The verification receipt contains stable top-level fields:

- `command`
- `path`
- `root`
- `valid`
- `verdict`
- `issues`
- `receipt_hash`
- `replay_receipt_hash`
- `stated_replay_receipt_hash`
- `current_replay_policy_hash`
- `replay_policy_hash`
- `selected_cases`
- `selected_groups`
- `environment_attestation_hash`

Issue codes are deterministic, including malformed JSON, missing schema, invalid receipt hash, stale policy hash, invalid selected case/group, invalid entropy policy hash, missing/malformed environment attestation, and invalid attestation hash.

### Hashing

Receipt hash verification recomputes the canonical replay receipt hash using the same self-reference exclusion as replay emission. Environment attestation hash verification recomputes the hash over the attestation object excluding `attestation_hash`.

### Policy ownership

The command uses generated policy for current replay policy hash, known case IDs, known group IDs/memberships, entropy policy hash, and the reviewed environment attestation fact list.

### Compatibility

The command is intentionally strict for the saved replay receipt: missing modern fields fail with stable issue codes. It does not change replay generation behavior or make agent receipts incompatible.

### Documentation/tests

Tests cover a positive saved receipt, tampered receipt hash, stale policy hash, malformed selected groups/cases, malformed attestation hash, and public docs/schema fields.