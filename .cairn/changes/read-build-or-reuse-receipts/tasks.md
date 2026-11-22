# Tasks

- [ ] [serial] [task:policy] Admit realization receipt v1 and v2 and the root dispositions `built` and `reused`, then regenerate the policy JSON. r[cairn.convergence.realization.policy_admission]
- [ ] [serial] [task:core] [after:policy] Read `trusted_builder_keys` and the original receipt; check the version 2 root disposition, builder role and fingerprint, and report `bound.root_disposition` and `bound.original_receipt_blake3`. r[cairn.convergence.realization.root_disposition]
- [ ] [serial] [task:link] [after:core] Implement the pure reuse-link check and the `convergence.reuse_link_invalid` refusal. r[cairn.convergence.realization.reuse_link]
- [ ] [serial] [task:unit] [after:link] Add positive and negative unit tests for the classification, signer, disposition and link decisions. r[cairn.convergence.realization.root_disposition] r[cairn.convergence.realization.reuse_link]
- [ ] [serial] [task:fixtures] [after:unit] Retain version 2 built and reused fixtures beside their missing, tampered, mismatched, untrusted and failed-decision records, and run them through the CLI. r[cairn.convergence.realization.root_disposition] r[cairn.convergence.realization.reuse_link]
- [ ] [serial] [task:docs] [after:fixtures] Update `docs/convergence.md` and the non-claims for the recorded-fingerprint and recorded-link boundary. r[cairn.convergence.realization.reuse_link]
- [ ] [serial] [task:checks] [after:docs] Run the workspace tests, clippy, fmt, policy export check, `cairn validate` and the change gates. r[cairn.convergence.realization.policy_admission] r[cairn.convergence.realization.root_disposition] r[cairn.convergence.realization.reuse_link]

## Validation and completion

- [ ] [serial] [task:lifecycle] [after:checks] Record the completed change and archive only after every required check passes. r[cairn.convergence.realization.policy_admission]
