# Design

## Context

This package records ROI rank 05 from the Cairn backlog sweep. The target is current clean `main` HEAD and existing release/readiness evidence artifacts.

## Goals

- Reduce future review and verification friction with a narrow, inspectable slice.
- Preserve existing public behavior, deterministic receipts, fixture expectations, and issue codes.
- Keep functional-core / imperative-shell boundaries intact.

## Decisions

- Treat this package as planning and acceptance criteria, not an implementation commit.
- Prefer cohesive module or test-target boundaries over broad shared helper rewrites.
- Keep bridge APIs narrow and local to the consuming module tree.
- Verify with focused checks first, then repo-level Cairn/Tracey/format/lint rails.

## Risks

- Over-broad helper extraction could create behavior drift or public API churn.
- Moved tests can accidentally weaken negative coverage if assertions are collapsed into generic helpers.
- Receipt or hash-bearing surfaces must not be regenerated unless the implementation intentionally changes the reviewed input.

## Rollout

Drain this package as one narrow implementation slice. Commit and push the verified slice before starting the next ROI-ranked package.
