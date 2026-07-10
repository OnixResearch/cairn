## Tasks

- [ ] [serial] Define supported function-address digest spellings for Valence, Mantle, Kamacite, and graph readiness fields. r[cairn.stack_provenance_gates.function_address_digest_normalization.contract]
- [ ] [serial] Normalize allowed digest spellings in CLI parsing before pure readiness evaluation. r[cairn.stack_provenance_gates.function_address_digest_normalization.shell]
- [ ] [serial] Keep role/domain validation separate from digest spelling normalization. r[cairn.stack_provenance_gates.function_address_digest_normalization.domain]
- [ ] [serial] Add positive fixtures for every supported producer spelling. r[cairn.stack_provenance_gates.function_address_digest_normalization.positive]
- [ ] [serial] Add negative fixtures for malformed hashes, unsupported prefixes, cross-field substitution, and optional-present invalid evidence. r[cairn.stack_provenance_gates.function_address_digest_normalization.negative]
- [ ] [serial] Run cargo tests and release-readiness smoke with normalized digest inputs. r[cairn.stack_provenance_gates.function_address_digest_normalization.validation]
