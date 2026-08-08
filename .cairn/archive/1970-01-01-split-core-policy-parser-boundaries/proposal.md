# Split core policy parser boundaries

## Why

`crates/cairn-core/src/verified/policy.rs` is the largest current Rust hotspot and mixes the public policy DTOs, error formatting, generic JSON field extraction, Steel profile decoding, and parser assembly in one review surface.

## What

Split the module into a directory facade with focused helpers while preserving existing public APIs and behavior.

## Impact

No policy schema or runtime behavior changes are intended. This is a maintainability-only boundary split with focused policy tests plus Cairn validation and Tracey coverage.
