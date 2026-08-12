# Design: Repository-scale trace-index proof

## Context

A requirement block can discuss marker syntax or name another requirement as an example. The current registry parser scans all requirement prose for bare markers. It rejects a valid requirement when prose contains marker examples in addition to the declaration marker.

## Goals

- Export Cairn's complete accepted requirement registry.
- Exercise registry export, trace-index export, freshness verification, and exact query in one maintained check.
- Retain output receipts as inspectable Nix artifacts.

## Non-Goals

- Do not infer requirement identity from arbitrary prose.
- Do not weaken duplicate accepted-ID rejection.
- Do not claim that a passing index proves implementation correctness.

## Decision: A declaration marker owns row identity

The parser will inspect the requirement heading and the first nonblank body line for declaration markers. Exactly one distinct marker must exist in that declaration region. Later prose remains part of the exact hashed requirement body, but marker-like examples in that prose do not define row identity.

This keeps identity explicit and prevents examples from becoming accidental authority. Missing declaration markers still fail. Distinct markers on the heading and first body line still fail. Duplicate accepted IDs across rows still fail.

## Decision: Nix owns the durable live proof

A new Nix check will run the packaged Cairn binary against the copied repository source. It will:

1. export a content-bound requirement registry;
2. export a content-bound trace index;
3. verify the saved index against the same source and policy;
4. query `cairn.trace_index.query`;
5. assert passing verdicts and a non-empty link set; and
6. install the registry, index, and four command receipts into the check output.

The output is deterministic for one source snapshot. Nix binds that snapshot independently. The receipt revision uses a fixed opaque source label because a flake source copy has no Git metadata.

## Architecture

The pure parser change remains in `cairn-core`. The Nix check and CLI invocations remain in the imperative verification shell. No filesystem or process behavior enters the core.

## Claim Boundary

The proof establishes that the packaged commands compose successfully over Cairn's accepted specifications and configured evidence roots. It does not prove requirement satisfaction, code correctness, test adequacy, or release eligibility.
