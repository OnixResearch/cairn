## Why

Cairn evidence, registry, release-readiness, replay, and scan commands work over repository-local trees and generated artifact directories. Those are exactly the places where ambient `std::fs` path handling can accidentally rely on absolute paths, parent traversal checks, or symlink behavior that is hard to audit.

## What Changes

- Adopt `cap-std` for Cairn CLI and support-code filesystem boundaries that operate under an explicit repository, evidence bundle, registry, release, replay, or scan root.
- Pass typed capability roots into evidence, registry, release evidence, support scanning, and replay execution shells.
- Keep `cairn-core` pure and free of capability-filesystem dependencies.
- Add positive and negative fixtures for valid in-root paths, `../`, absolute paths, and symlink escapes.

## Impact

- Cairn artifact and lifecycle evidence paths become root-confined at the shell.
- Existing accepted lifecycle semantics and receipt schemas remain unchanged for valid inputs.
- Invalid path states fail before artifact reads, writes, or replay execution.
