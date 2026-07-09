# Toolchain Profile Matrix

Cairn supports a declaration-only toolchain profile matrix for cross-repo validation lanes. Profiles distinguish `dev`, `ci`, `release`, `proof`, and `compatibility` purposes so receipts can say which lane and toolchain expectations they used.

Run validation with:

```bash
cairn toolchain-profile verify --input fixtures/toolchain-profile-matrix/positive/profile-classes.json
```

Each profile row records repo, profile ID, purpose, toolchain pins, validation commands, expected artifact classes, and non-claims.

## Evidence boundary

A profile declaration does not prove command success unless a separate execution receipt exists. It also does not prove release eligibility, proof soundness, or compatibility by itself. The matrix only checks that profile declarations are complete, unambiguous, and explicit about their evidence boundary.
