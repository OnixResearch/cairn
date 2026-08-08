# Validation: Radicle Change COB

## Result

The library-level custom COB slice is validated on `x86_64-linux`. Cairn now has a lossless pure active-change reducer and a native Radicle adapter for `xyz.onixresearch.cairn-change`.

## Deterministic Evidence

- Baseline structured lifecycle core: `6/6` tests passed before implementation.
- Radicle change core: all focused positive and negative tests passed.
- Native adapter: `7/7` tests passed, including a Git-backed generic Radicle `Store` create/update/get test.
- Workspace tests and Clippy passed with all workspace members selected.
- Formatting, policy freshness, Cairn validation, proposal/design/tasks gates, and the verified-core purity search passed.
- Traceability coverage passed with `1596/1596` accepted requirement and scenario IDs referenced.
- `nix flake check path:$PWD -L` completed with `all checks passed` for `x86_64-linux`.

## Defects Found During Validation

The first full-workspace run exposed that Radicle enables `serde_json/preserve_order`. Cairn's prior `canonical_json` implementation implicitly depended on `serde_json::Map` being sorted, so positive receipt fixtures changed hashes only when the Radicle adapter participated in the build. Canonical JSON now recursively sorts object keys explicitly, and the existing positive fixtures pass under the unified feature graph.

The next full run exposed cross-root replay resource amplification: each replay root copied the repository's multi-gigabyte Cargo `target` tree. Replay copying now excludes derived `target` directories, focused replay tests pass, and temporary storage remains bounded by source inputs rather than build output size.

## Bounded Claims

The passing adapter test proves local signed Git-backed creation, update, retrieval, manifest typing, identity-document delegate mapping, and reducer rejection behavior under Radicle's test repository. It does not prove live-node transport, multi-peer replication, availability, current delegate authority at later Git mutation time, evidence truth, dependency readiness, archive execution, or release eligibility. CLI import, checkout, publish, and archive-execution integration remains separate future scope.
