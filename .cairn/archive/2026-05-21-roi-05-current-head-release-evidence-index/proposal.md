# Create current-HEAD release evidence index

## Problem

The repo is clean and drained with recent test-harness and readiness work, but operators need a small current-HEAD evidence index that binds the live commit to the relevant existing tag, harness, receipt, policy, Tracey, and release-readiness evidence without moving readiness tags.

## Proposed Change

Add a checked-in current-HEAD evidence index document and deterministic verification notes that point at existing evidence surfaces and clearly distinguish evidence-recording commits from readiness tags.

## Scope

This is an ROI-ranked planning package for current clean `main` HEAD and existing release/readiness evidence artifacts. The implementation should remain narrow and behavior-preserving unless a later change explicitly updates public contracts.

## Non-Goals

- No tag movement or release promotion unless explicitly requested.
- No mutation of generated receipts under `target/`.
- No claim that the evidence-index commit was itself part of earlier readiness receipts.

## Verification

- Run the focused checks named in the task list.
- Run native Cairn validation and Tracey coverage.
- Run formatting/linting/diff checks appropriate to the touched Rust/Nix files.
