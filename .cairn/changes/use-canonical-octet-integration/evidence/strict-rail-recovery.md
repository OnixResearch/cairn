# Restore comparable strict catalog evidence

## Correction

The direct runs for `c99fa39` and `90e710f` used `DYLINT_RUSTFLAGS=-D warnings`.
They did not use the official deny-all hook. Their counts of 4,589 and 4,583
are not full-catalog counts and cannot replace the earlier strict baseline.
Those runs also returned `integration-failure`, not acceptance.

The wrapper matches the pinned remote flake output. The difference is the
hook's explicit list of deny flags, which includes default-allow lints.

## Fresh replay

The operator exported commit `90e710f` with `git archive` into a fresh remote
directory. The archive BLAKE3 digest is
`cfb392dd8a0cfac17cbd599029b0adae8100e917c9fa04c19e9730652303bb34`.
The source does not inherit changes from the old remote sandbox.

The existing remote dev shell supplies the toolchain. The pinned vendor
configuration supplies dependencies. The checker runs from the fresh source
root with these arguments:

```sh
export OCTET_PRECOMMIT_USE_INSTALLED=true
export DYLINT_RUSTFLAGS='-D warnings'
"$producer/hooks/octet-deny-all.sh" \
  --artifact-dir "$artifacts" -- --package cairn-core --all-targets --all-features
```

`producer` is `/nix/store/xrly6lsrlc8h8bbpwy1ahf80lsr4dgz9-source`.
The PATH selects the pinned runner at
`/nix/store/k5r3sywcngl4376h2njxcdzcv7wn4xg5-cargo-octet-0.1.0/bin`.
These are the resolved outputs of the existing Octet pin, not new dependencies.

## Result

The official hook reached 3,640 library errors and 4,189 library-test errors.
The overlapping total is 7,829, compared with the earlier total of 7,839.
The two naming batches removed ten full-catalog findings.

Operator evidence is under the primary checkout's `.pi/cairn-rest-1295/`:

- `strict-90e710f.log`
- `strict-90e710f-status.json`
- `strict-90e710f-command.txt`
- `strict-90e710f-summary.txt`

The Nix evaluation route still needs private-input SSH access. This direct
hook replay avoids that fetch but does not replace the full Nix acceptance
check. The full strict catalog still fails. No completion, archive, release,
or push claim follows from this result.
