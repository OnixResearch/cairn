## Foundation

- [x] [serial] [task:foundation] Record the direct MCP correction scope and pass proposal, design and tasks gates before implementation. r[cairn.release_consumer.selection]

## Implementation and verification

- [ ] [serial] [task:selection] [after:foundation] Reuse typed consumer selection and exact policy propagation, preserving fail-closed behavior and self-host defaults. r[cairn.release_consumer.selection]
- [ ] [serial] [task:producer] [after:selection] Integrate the existing real dependency/native Nix producer mechanisms without weakening maintained gates. r[cairn.release_consumer.selection]
- [ ] [serial] [task:verification] [after:producer] Exercise real consumer acceptance and negative controls, run required repository gates, and retain exact-source evidence before reviewed sync/archive/publication. r[cairn.release_consumer.selection]
