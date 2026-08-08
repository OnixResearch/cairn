## Overview

Introduce explicit, deterministic metadata for active Cairn change packages. Metadata is optional for backward compatibility, policy-validated when present, and surfaced through existing read-side receipts.

## Metadata model

Each active change MAY declare `metadata.json` with these fields:

- `groups`: sorted, unique group labels such as `current`, `future`, or `feature:<name>`.
- `target_date`: optional `YYYY-MM-DD` date for scheduled or future-dated work.
- `status`: optional policy-owned planning status such as `current`, `planned`, `blocked`, or `deferred`.

Invalid labels, duplicate groups, malformed dates, unknown status values, invalid JSON, or non-object metadata produce validation issues.

## CLI behavior

`cairn change list --root <path>` and `cairn change show <name> --root <path>` include normalized metadata in their JSON receipts. Missing metadata is represented by empty groups and absent optional fields.

`cairn change list` supports deterministic filters:

- `--group <label>` returns active changes containing that group.
- `--target-before <YYYY-MM-DD>` returns active changes with target dates before the bound.
- `--target-after <YYYY-MM-DD>` returns active changes with target dates after the bound.

Filters compose by intersection and preserve stable sorted output.

## Boundaries

Metadata parsing, normalization, filtering, and validation are pure core operations over caller-provided artifacts and policy. Filesystem traversal, argument parsing, and JSON presentation remain in the CLI shell. This feature does not change archive semantics or evidence-registry compatibility classes.
