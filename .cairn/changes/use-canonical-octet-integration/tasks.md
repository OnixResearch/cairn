# Tasks

- [x] [serial] [task:baseline] Record the existing Rust baseline and verify the pinned producer interfaces. r[cairn.octet_integration.canonical] r[cairn.octet_integration.evidence]
- [x] [serial] [task:integration] [after:baseline] Migrate the direct Nix input, check, executable, and config tables without changing source identity or scope. r[cairn.octet_integration.canonical] r[cairn.octet_integration.evidence]
- [x] [serial] [task:guidance] [after:integration] Update current operator guidance and release-readiness commands while preserving historical receipts. r[cairn.octet_integration.guidance] r[cairn.octet_integration.evidence]
- [x] [serial] [task:sync] [after:guidance] Sync the reviewed requirements before the tests that enforce accepted-spec traceability. r[cairn.octet_integration.evidence]
- [x] [serial] [task:contract] [after:sync] Repair gate-contract source for the pinned deny-all hook while preserving public types and exact fixture bytes. r[cairn.octet_integration.evidence]
- [ ] [serial] [task:verification] [after:contract] Run positive and negative integration controls, Rust tests, formatting, lifecycle checks, and the full workspace Octet deny-all check. r[cairn.octet_integration.canonical] r[cairn.octet_integration.guidance] r[cairn.octet_integration.evidence]
- [ ] [serial] [task:completion] [after:verification] Record final acceptance evidence and archive only after the required checks pass. r[cairn.octet_integration.evidence]
