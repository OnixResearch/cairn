# Post-tag remote install smoke: v0.1.0

## Payload

- Tag: `v0.1.0`
- Peeled commit: `2e6ca1f`
- Remote flake reference: `github:OnixResearch/cairn/v0.1.0#cairn`

## Commands

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
