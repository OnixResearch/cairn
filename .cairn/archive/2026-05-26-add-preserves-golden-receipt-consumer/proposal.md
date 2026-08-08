# Add Preserves golden receipt consumer

## Why

Cairn now has an internal Preserves packed-byte/hash seam, but the seam is only proven by helper-local tests. A narrow first consumer should bind an existing receipt-shaped artifact to the Preserves hash path so future changes can prove the seam on real Cairn evidence without changing public JSON receipt surfaces.

## What Changes

- Add an internal agent-run receipt Preserves hash helper that computes a BLAKE3 hash over Preserves packed bytes.
- Keep the existing JSON `receipt_hash` contract unchanged.
- Add a golden fixture assertion for the existing positive agent-run receipt.
- Add object-order and content-drift tests at the consumer boundary.

## Non-Goals

- No public receipt schema change.
- No CLI output change.
- No replacement of JSON/Nickel/TOML surfaces.
- No new signing, publication, or remote evidence behavior.
