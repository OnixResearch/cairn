# Evidence type limits and owner qualification

## Change

`crates/cairn-core/src/verified/check_evidence/types.rs` now names the digest
width, label limit, and adjacent-pair width. Their values did not change.
The file qualifies the collection and receipt owners without import aliases.
No public type, Serde name, field, dependency, or pin changed.

Six tests cover accepted and rejected digest widths, hexadecimal alphabets,
label limits, label bytes, empty sequences, duplicates, and descending order.
The negative fixture arithmetic uses checked operations.

Cairn owns these private admission rules and their maintenance. The durable
contribution is an executable boundary matrix for later catalog repairs.
No new shared component or external effect is involved.

## Validation

The baseline used the fresh `90e710f` source snapshot described in
`strict-rail-recovery.md`. Core tests passed before this change.
The operator copied only the changed source file into that snapshot.
A byte comparison confirmed that the tested source matches the local file.
Its BLAKE3 digest is
`cdbed79deae0b6ed9b0c4283507dfcce1c29cae07fa1cfa889824b1b9bb3a636`.

- Formatting passed.
- Core tests passed before and after the change across all targets and features.
- CLI compilation passed across all targets and features after the production change.
- Core Clippy passed across all targets and features with `-D warnings`.
- The official pinned deny-all hook reported 3,637 library errors and 4,186 library-test errors.

The full-catalog total decreased from 7,829 to 7,823 overlapping findings.
The new tests have no catalog finding. The source still has existing Serde
import and repeated public-name findings.

## Evidence and limits

The primary checkout keeps the operator logs under `.pi/cairn-rest-1295/`:
`types-limits-before.log`, `types-limits-after.log`, `types-limits-final.log`,
`types-limits-strict.log`, `types-limits-status.json`, and
`types-limits-summary.txt`.

This batch does not establish full Octet acceptance. The Nix route still has
the private-input SSH blocker. No archive or push occurred.
