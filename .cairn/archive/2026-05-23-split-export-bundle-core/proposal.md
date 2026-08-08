# Proposal: Split Export Bundle Core

## Summary

Split `crates/cairn-core/src/verified/content_addressed/export_bundle.rs` into a focused module tree while preserving the existing `verified::content_addressed` public API and export-bundle receipt behavior.

## Rationale

The export-bundle validator is now the largest non-test Rust source in Cairn. It mixes schema checks, hash material, object membership, name-pointer references, replay metadata, compiler IR checks, mutation manifests, caveats, entropy scanning, and issue construction in one review surface. A behavior-preserving split lowers review cost and keeps the content-addressed evidence family easier to maintain.

## Compatibility

This change does not intentionally alter public constants, DTOs, hash material, issue codes/messages, CLI behavior, fixtures, or receipt validation semantics. `verified::content_addressed` and crate-root re-exports remain stable.
