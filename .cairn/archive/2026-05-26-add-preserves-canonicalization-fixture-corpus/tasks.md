# Tasks

- [x] [serial] I1 Add a checked-in Preserves canonicalization fixture corpus with stable case ids and expected BLAKE3 hashes. r[cairn.determinism.preserves_canonicalization_corpus.fixture_corpus]
- [x] [depends:1] I2 Add focused core tests that recompute every corpus hash and prove object-order-equivalent fixture material remains stable. r[cairn.determinism.preserves_canonicalization_corpus.golden_hashes]
- [x] [depends:2] I3 Add negative/content-sensitivity coverage without changing public/stored JSON receipt schemas. r[cairn.determinism.preserves_canonicalization_corpus.validation]
- [x] [depends:1,2,3] V1 Run focused Preserves core tests, formatting, Cairn validation/gates, Tracey coverage, clippy, and workspace tests before archive. r[cairn.determinism.preserves_canonicalization_corpus.validation]
