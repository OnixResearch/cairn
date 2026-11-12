# Security policy

## Supported versions

| Version | Supported |
|---|---|
| 0.2.x | Yes |
| 0.1.x | No (pre-public local tag) |

## Reporting a vulnerability

Do **not** open a public issue for a suspected vulnerability.

Use GitHub's private vulnerability reporting:

https://github.com/OnixResearch/cairn/security/advisories/new

Include:

- the affected version and platform;
- the exact command, policy locator, and input files;
- expected and observed behavior;
- a minimal reproduction, including whether a receipt, gate, or policy hash was
  involved; and
- any known impact on confidentiality, integrity, or availability.

Maintainers will acknowledge the report, reproduce it against a current
revision, and coordinate disclosure in the advisory. Do not include secrets or
third-party personal data beyond what the reproduction requires.

## Security-relevant surface

Cairn treats these as security boundaries:

- argument, option, and environment admission in the CLI shell;
- capability-confined file and directory input under an explicit root;
- lexical path admission, symlink rejection, and bounded reads;
- canonical JSON and receipt identity construction;
- gate interchange, authenticated evidence, and detached source-review
  verification;
- MCP and machine-output adapters that expose filesystem observations.

The pure core must not read files, inspect the environment, spawn processes,
print, access clocks, perform network I/O, or depend on ambient mutable state.
Reports that cross that boundary are in scope.

## Out of scope

- Trust in an external agent, model, or operator. Cairn does not grant ambient
  authority and does not prove agent intent.
- Policy decisions that admit a declared input on purpose. A passing gate proves
  conformance to selected policy and inputs, not code correctness, evidence
  truth, build reproducibility, deployment safety, or release eligibility.
- Vulnerabilities in third-party dependencies that are not reachable through a
  Cairn-owned boundary. Report those upstream and note the affected Cairn
  version if Cairn is exposed.
- Damage caused by a local operator who deliberately edits generated policy or
  receipts and also controls the verification inputs.

## Hardening expectations

- New file inputs must be capability-confined and bounded.
- New parsers and receipts need positive and negative fixtures.
- New adapters must fail closed and preserve documented non-claims.
- Generated policy, lockfiles, and source pins change only through their owning
  tools, with freshness checks in the same change.

See `docs/licensing.md` for the license boundary and `AGENTS.md` for repository
work rules.
