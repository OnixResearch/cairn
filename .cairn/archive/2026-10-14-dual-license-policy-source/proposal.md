## Why

Terrane copies an immutable Cairn policy projection and its Nickel authoring
inputs. The original revision's AGPL default leaves an avoidable distribution
question for consumers who need only the policy, not Cairn's operator CLI.

## What Changes

- Offer the five named policy files, both current and at the exact pinned
  historical revision, under `AGPL-3.0-or-later OR MIT OR Apache-2.0`.
- Keep the Cairn CLI and all other unmarked lifecycle material under their
  existing licenses; preserve the original Git object identities.
- Make the grant discoverable in Cairn's license map and machine-checked
  license inventory so Terrane can cite the published grant.

## Impact

- **Consumer:** Terrane can redistribute its exact Cairn policy copy under
  MIT without presenting its own MPL metadata as an upstream relicense.
- **Maintenance owner:** Cairn maintains the named policy licensing boundary.
- **Verification:** Check the grant, historical blobs, license inventory, and
  native lifecycle gates; these checks do not establish copyright authority.
