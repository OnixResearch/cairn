# Design

## Command

`cairn agent-receipt preserves-hash <path>` is a read-only operator command. It loads a bounded JSON file using the existing agent-receipt file reader, verifies the top-level value is an object, computes the internal `agent_run_preserves_receipt_hash`, and emits a deterministic receipt.

## Receipt Shape

The command emits stable JSON fields:

- `command`: `agent-receipt preserves-hash`
- `path`: display path from the CLI argument
- `input_kind`: `path`
- `valid`: boolean
- `verdict`: `PASS` or `FAIL`
- `issues`: deterministic issue array
- `agent_receipt_preserves_hash`: BLAKE3 hex on success, empty on failure
- `input_hash`: canonical JSON hash of the input material
- `max_content_bytes`: existing agent receipt bounded read limit
- `receipt_hash`: canonical JSON receipt hash

## Failure Behavior

Malformed JSON and missing files remain ordinary CLI errors from bounded file loading. JSON values that parse but are not receipt objects produce a structured failure receipt with `valid=false`, `verdict=FAIL`, and an issue code such as `receipt.not_object`.

## Public Surface Boundary

This command exposes the internal Preserves hash for inspection only. It does not add `preserves_receipt_hash` to stored receipts, does not require downstream consumers to use Preserves, and does not alter `agent-receipt validate`.
