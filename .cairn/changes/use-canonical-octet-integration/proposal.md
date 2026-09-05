# Use canonical Octet integration

## Why

Cairn still selects the former Slotcar name in its Nix input, check attribute, Cargo metadata, lint config, and operator commands.
The locked source already supplies Octet and its canonical interfaces.
Current guidance must name that tool without a repository redirect or legacy CLI alias.

## What Changes

- Use the canonical Octet repository, Nix input, check attribute, and Cargo executable.
- Keep the existing immutable producer revision and consumer check scope.
- Use Octet config tables and list the canonical check in release-readiness reports.
- Update current documentation and add positive and negative regression controls.
- Preserve archived commands and recorded receipt bytes as historical evidence.

## Impact

Cairn maintainers own this consumer integration.
The immediate consumer is the utility-change drain and its repository verification commands.
The durable capability is a tested canonical integration that does not silently return to legacy names.
Octet retains ownership of lint semantics, its compiler inputs, and producer implementation.
No new shared component or runtime port is necessary.
This change does not repair modified Nix store bytes or relax a failed check.
