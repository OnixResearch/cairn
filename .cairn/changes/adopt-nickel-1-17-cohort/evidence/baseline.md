# Baseline evidence

Base revision: `b6759012db4dab0158936e995b7bad39ac10465b`.

Before this change, `flake.nix` selected ambient `pkgs.nickel`. The checked evaluator descriptor and documentation named `nickel-lang-cli-1.16.0` with identity `nixpkgs:nickel`.

The initial `nix develop` policy and focused-test commands did not reach Nickel or the test binary. The configured remote builder rejected its copied Rust source path with this exact mismatch:

```text
specified: sha256-q/gu/3mAuLgNfJlxV/Sw1jttbi4PIBjN+XH0bGmB5NQ=
got:       sha256-WTRv7eyiu+VOfb8+90cALNJrUa3uLwRFIaeEr+tAIjQ=
```

Post-change Nix checks disable remote builders so that this unrelated remote-store fault cannot change the cohort result.
