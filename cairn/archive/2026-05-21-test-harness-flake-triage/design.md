## Context

This is a narrow local harness hardening slice.

## Goals

- Deterministic JSON receipts.
- Clear failure classes.
- BLAKE3 artifact integrity.

## Non-Goals

- Remote publication or signing.

## Decisions

- Implement a focused `cairn test-harness` command.
- Keep generated artifacts under `target/cairn-test-harness`.
