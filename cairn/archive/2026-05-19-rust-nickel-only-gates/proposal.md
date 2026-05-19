# Change: rust-nickel-only-gates

## Why

Cairn's deterministic rails should be self-contained in the project languages we trust for this repo: Rust/Rust CLI surfaces and Nickel policy. Checked-in Python helpers and Python-backed Nix checks create an avoidable second implementation language for policy freshness, Tracey bootstrap coverage, and rename classification.

## What Changes

- Move policy export/freshness validation behind `cairn policy export --check`, using Nickel for policy rendering and Rust for semantic validation.
- Move Tracey bootstrap coverage behind `cairn tracey coverage --root PATH`.
- Move rename classification behind `cairn rename-classification`.
- Remove checked-in Python helper scripts and Python from Nix/devshell check dependencies.
- Add strict validation exit behavior for CI via `cairn validate --strict`.

## Non-Goals

- Rewriting historical archive evidence that mentions old Python commands.
- Removing Nix builder command snippets; those remain Nix check definitions, not checked-in shell helper scripts.
