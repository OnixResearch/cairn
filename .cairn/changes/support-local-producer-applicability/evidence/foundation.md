# Producer foundation evidence

Source baseline: Cairn `d2c1dd7867350b787cfffe0e311fd20115238339`, dedicated branch `change/chaoscontrol-producer-readiness-20261017`.
Current consumer and maintenance owner are stated in the proposal; this change is bounded to dependency applicability and native Nix build admission.

## Commands and observed verdicts

- `nix develop --no-update-lock-file -c cargo test -p cairn-core check_evidence`: 32 producer tests passed, zero failed; exit 0, no operational timeout. Local command receipt: `/home/brittonr/git/OnixResearch/cairn/target/cc-producer26/baseline/execution.json`.
- `cairn gate proposal support-local-producer-applicability --root .`: `valid=true`, `verdict=PASS`, no issues.
- `cairn gate design support-local-producer-applicability --root .`: `valid=true`, `verdict=PASS`, no issues; two substantive requirements and four scenarios.
- `cairn gate tasks support-local-producer-applicability --root .`: `valid=true`, `verdict=PASS`, no issues; dependency ordering valid.
- Raw gate reports and exact command receipts remain under `/home/brittonr/git/OnixResearch/cairn/target/cc-producer26/{proposal,design,tasks}-gate/`.

## Non-passing baseline observations

The local CLI baseline was also run before producer source edits: `nix develop --no-update-lock-file -c cargo test -p cairn-cli --test check_evidence_local --test check_evidence_nix`.
It exited 101 with eight failed local tests; Cargo stopped before the Nix integration binary.
Seven fixtures failed profile admission. A read-only environment probe measured the copied `PATH` at 5,414 bytes, exceeding the unchanged 4,096-byte profile value bound.
The live-repository developer-loop case emitted a non-passing receipt; its original bytes remain in `target/producer-loop-4154413/`.
The exact failed invocation and logs remain in `/home/brittonr/git/OnixResearch/cairn/target/cc-producer26/cli-baseline/`.
Subsequent verification uses an explicitly selected smaller invocation PATH rather than weakening profile admission or claiming the initial run passed.

## Native contract observation

`nix build --offline --no-link --json --file ./check.nix` successfully realized a disposable derivation and emitted native `drvPath` and named `outputs` fields.
The exact command, native payload and logs remain in `/home/brittonr/git/OnixResearch/cairn/target/cc-producer26/native-nix-observation/`.
The configured Nix builder route was used; this is not a fresh-local-builder or VM claim.
The native surface is grounded additionally in Nix 2.31 `src/nix/build.cc` and `src/libcmd/built-path.cc`.
Private-store portability experiments remain separate local observations and are not passing producer evidence.

## Boundary

Foundation gates establish structural policy conformance, not implementation correctness, complete consumer evidence, archive eligibility, or production readiness.
Only the foundation task is completed by this evidence. Implementation and validation tasks require their own integrated runtime results.
Historical receipts and other active changes are unchanged.
