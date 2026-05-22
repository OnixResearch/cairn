## Why

Cairn's deterministic lifecycle and evidence model is stronger than competitor onboarding, but new repositories still require too much Cairn/Nix/Nickel context before the first useful change can be reviewed. Competitors win the first-ten-minutes experience with templates, simple bootstrap guidance, and agent-facing instructions.

## What Changes

- Add first-class onboarding templates that can initialize or preview a small set of reviewed repository profiles.
- Emit deterministic template receipts that list created files, selected profile, policy source, and follow-up commands.
- Provide operator-facing quickstart guidance without weakening Cairn's native layout, policy, or FCIS boundaries.

## Impact

- **Files**: CLI template/init surfaces, policy-owned template metadata, fixtures, README/docs quickstart examples.
- **Testing**: Positive and negative CLI tests for template selection, safe file creation, idempotency, and receipt stability.
