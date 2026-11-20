# Registry-derived targets fixture

`targets.ncl` is the `../octet-example` targets file with each target's
`requirement` and the contract's `specification` derived by requirement id from
`requirement-registry.json` instead of copied. The flake check exports it and
compares it with `../octet-example/targets.json` byte for byte.

`requirement-registry.json` is a synthetic `cairn requirement-registry export`
that carries the same two synthetic rows as `../octet-example/targets.json` (see
its `SOURCE.md`). Its envelope identities, `registry_blake3` and receipt are
placeholders and are not produced by Cairn's registry algorithm; the Nickel
helpers validate only the schema identifier before selecting a row.
