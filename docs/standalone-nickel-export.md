# Standalone Nickel export boundary

Cairn pins `OnixResearch/nickel-export` at revision `257fafc1c746f1faf156207043a4c826bfb16d49` in Cargo and Nix inputs. The supported evaluator cohort is `nickel-lang-cli-1.16.0` from the pinned Cairn Nix input.

`cairn policy export` and Nickel-backed `cairn lifecycle` reads use a Cairn-owned shell. The shell:

1. selects repository-relative source and destination identities;
2. discovers the declared static import closure;
3. executes the current Nickel command and the canonical observation path;
4. requires exact status, output-byte, and diagnostic-byte parity;
5. admits exact identities through `nickel-export-core`; and
6. passes the legacy bytes into Cairn's existing semantic core.

The wrapper rejects parent traversal, absolute paths, undeclared or unreadable imports, mutable source identities, evaluator-cohort drift, output drift, and diagnostic drift. Generated destinations are still written only by Cairn after semantic validation and freshness checks.

## Validation

```console
nix run path:$PWD#cairn --option secret-key-files '' --option builders '' -- policy export --check
nix develop --option secret-key-files '' --option builders '' -c \
  cairn lifecycle check --source fixtures/structured-lifecycle/positive-bundle.ncl
nix build path:$PWD#checks.x86_64-linux.nickel-export-cutover \
  --option secret-key-files '' --option builders '' --no-link -L
```

## Rollback and non-claims

The legacy command remains authoritative during the checked dual-run cohort. Any unexplained byte, status, diagnostic, identity, freshness, or semantic drift blocks generation. A retry requires paired positive and negative regression evidence.

Standalone admission proves only exact declared input/output identity under the recorded evaluator descriptor. It does not prove evaluator equivalence, Cairn policy correctness, gate soundness, replay correctness, deployability, or release eligibility.
