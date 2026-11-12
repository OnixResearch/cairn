# Post-tag remote install smoke

## v0.2.0

### Payload

- Tag: `v0.2.0`
- Peeled commit: `34d893d`
- Remote flake reference: `github:OnixResearch/cairn/v0.2.0#cairn`
- Release workflow run: `43262797374`
- aarch64 validation run: `43262828123`

### Result

- The release workflow built `.#cairn` from the tag, then ran
  `cairn --version`, `cairn init`, `cairn validate`, and `cairn status` from a
  temporary foreign working directory. All steps passed.
- The workflow published release `v0.2.0` with
  `cairn-0.2.0-x86_64-linux.closure.txt`.
- The aarch64 workflow built the same tag on a native `ubuntu-24.04-arm` runner
  and passed the same lifecycle smoke.
- A local repeat of `nix run github:OnixResearch/cairn/v0.2.0#cairn` was not
  recorded; local Nix store maintenance blocked it. The two workflow results
  above are the recorded evidence.

### Non-claims

- The workflow builds and smoke tests the package. They do not prove
  reproducibility, deployment safety, or release eligibility.
- No tag was created or moved by the smoke itself.

## v0.1.0

### Payload

- Tag: `v0.1.0`
- Peeled commit: `2e6ca1f`
- Remote flake reference: `github:OnixResearch/cairn/v0.1.0#cairn`

### Commands

```bash
nix build github:OnixResearch/cairn/v0.1.0#cairn -o <tmp>/cairn-out
<tmp>/cairn-out/bin/cairn init --root <tmp>/root
<tmp>/cairn-out/bin/cairn status --root <tmp>/root
<tmp>/cairn-out/bin/cairn validate --root <tmp>/root
```

## Result

- Remote flake unpacked `github:OnixResearch/cairn/2e6ca1fbb33f81e6242d7c800e33cd94fdfec464`.
- `init` reported native `cairn` layout and `mutated=true` for the isolated root.
- `status` reported native `cairn` layout.
- `validate` reported `valid=true` for the isolated root.

## Non-claims

- Local trusted smoke only; no credential, transport, mirror-trust, signature, or public unattended production-readiness claim.
- Temporary absolute paths and machine-specific Nix store details are intentionally not preserved.
- This smoke does not move or create release tags.

r[verify release_evidence.post_tag_remote_install_smoke.build]
r[verify release_evidence.post_tag_remote_install_smoke.lifecycle]
r[verify release_evidence.post_tag_remote_install_smoke.docs]
r[verify release_evidence.post_tag_remote_install_smoke.validation]
