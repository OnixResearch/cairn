## Tasks

- [x] [serial] Add canonical Kamacite Preserves receipt identity fields to function-address readiness inputs. r[cairn.stack_provenance_gates.function_address_preserves_readiness.contract]
- [x] [serial] Validate Preserves receipt schema metadata, Valence/Mantle linkage, roles, claim scopes, digest spelling, and non-claims in the pure readiness evaluator. r[cairn.stack_provenance_gates.function_address_preserves_readiness.core]
- [x] [serial] Keep JSON wrapper support compatibility-only and bound to canonical Preserves identity. r[cairn.stack_provenance_gates.function_address_preserves_readiness.json_projection]
- [x] [serial] Add positive fixtures for optional absent, optional present, and required complete Preserves-backed evidence. r[cairn.stack_provenance_gates.function_address_preserves_readiness.positive]
- [x] [serial] Add negative fixtures for missing canonical hash, stale linkage, unsupported schema, malformed digest, projection drift, wrong scope/role, and overclaims. r[cairn.stack_provenance_gates.function_address_preserves_readiness.negative]
- [x] [serial] Run cargo tests, policy export check, lifecycle validation, and Preserves-backed release-readiness smoke. r[cairn.stack_provenance_gates.function_address_preserves_readiness.validation]
  - Core function-address tests pass 22/22, release-receipt tests pass 5/5, complete core tests pass 295/295, workspace tests pass, policy export is fresh, lifecycle validation passes, and Tracey reports 1298/1298 referenced requirements.
