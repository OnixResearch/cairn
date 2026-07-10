## Why

The first stack smoke lived entirely under `/tmp/onix-function-address-smoke`, so the passing evidence path is not reproducible from the repos. The stack needs committed positive and negative smoke fixtures that prove Octet, Kamacite, Valence, Mantle, and Cairn can exchange function-address evidence without bespoke temporary adapters.

## What Changes

- Add a small committed function-address smoke fixture and expected artifact set.
- Replace temporary adapter behavior with first-class Octet, Kamacite, Valence, Mantle, and Cairn commands as those changes land.
- Include negative fixtures for stale function address, stale receipt hash, malformed digest, missing source span, and overclaiming non-claims.
- Document the one-command smoke validation path and expected evidence hashes.

## Impact

Future changes can rerun the stack-wide function-address integration path and catch regressions in artifact shape, digest spelling, non-claim boundaries, and readiness gates.
