# Cairn public launch plan

Status: prepared for the `v0.2.0` public release. This document is a plan and
message guide, not evidence that a tag, announcement, or registry publication
happened.

## Positioning

**Category:** deterministic lifecycle gates for repository work.

**One line:** Cairn turns a change into a bounded package of proposal, design,
tasks, and specs, then evaluates policy-owned gates and emits deterministic
receipts that people, CI, and coding agents can inspect.

**Problem:** coding agents can produce changes faster than review can verify
them. Prose checklists drift, agent transcripts are not evidence, and “the tests
passed” does not say what policy or inputs were checked.

**What Cairn is not:** an issue tracker, a scheduler, a model runtime, an agent
framework, a CI service, or a correctness proof. It does not call a model and
does not publish on its own.

### Message hierarchy

1. **Make the change explicit.** A native `.cairn/` package has a stable shape.
2. **Make the gate deterministic.** Nickel policy compiles to checked-in JSON;
   gate receipts bind policy and inputs.
3. **Make the result machine-readable.** One versioned machine outcome, MCP
   tools, gate interchange, and release evidence.
4. **Keep the claim honest.** A gate proves conformance to selected policy and
   declared inputs, not code correctness or release eligibility.

### Words to use and avoid

| Use | Avoid |
|---|---|
| lifecycle gates, receipts, policy-owned | “AI-powered”, “autonomous” |
| conformance to selected policy and inputs | “proves correctness”, “guarantees release” |
| deterministic, bounded, fail closed | “zero configuration”, “works everywhere” |
| native `.cairn/` layout | “yet another spec folder” |
| supported install is the Nix flake | “single static binary everywhere” |

## Audience and entry points

| Audience | Immediate job | Entry point |
|---|---|---|
| Maintainers using coding agents | Keep agent-produced changes reviewable | `cairn scaffold change`, `cairn gate`, `cairn readiness` |
| Rust and Nix platform teams | Standardize lifecycle rules across repositories | `cairn setup plan`, policy profiles, gate interchange |
| CI and release engineers | Bind a gate to an exact candidate and evidence set | `cairn --machine`, `release-evidence`, `release-run` |
| Tool builders | Consume a stable machine boundary | MCP shell, gate interchange, CLI machine contract |
| Reviewers and auditors | Inspect what was checked | receipts, authenticated evidence profiles |

## Proof assets

Use these instead of adjectives:

- `cairn --version` and `cairn --help` output.
- A 60-second demo transcript from a fresh temporary repository.
- One gate receipt showing `valid`, `issues`, `policy_hash`, `input_hash`, and
  `receipt_hash`.
- One `cairn --machine validate` result showing `schema_version` and
  `exit_class`.
- The gate interchange and release-evidence docs.
- `SECURITY.md`, `CONTRIBUTING.md`, and the license boundary.
- The aarch64 non-claim: packaged, not yet validated on a builder.

Do not publish performance numbers, adoption counts, or security certifications
that are not backed by a checked receipt.

## Launch sequence

### T-7 days: release candidate

- Freeze behavior changes on `main` except release blockers.
- Run the full gate (see below) and archive the result.
- Run `cairn release-readiness --root .` and review skipped or absent checks.
- Build the package on a clean machine and run the foreign-working-directory
  smoke from the README.
- Verify every dependency URL fetches anonymously over HTTPS.
- Review `README.md`, `CHANGELOG.md`, `SECURITY.md`, and `CONTRIBUTING.md`.
- Prepare the announcement draft and demo transcript.

### T-0: publish

- Tag `v0.2.0` on the verified commit.
- Let `.github/workflows/release.yml` verify the tag, build, smoke-test, and
  create the GitHub release with generated notes.
- Verify `nix run github:OnixResearch/cairn/v0.2.0#cairn -- --version` from a
  clean home directory.
- Publish the announcement with the demo transcript and the non-claims.
- Open a pinned “v0.2.0 feedback” issue.

### T+1 to T+30: adoption

- Triage every external report as a product signal, not a support ticket.
- Record which adoption stage failed: install, init, scaffold, gate, readiness,
  CI integration, or release evidence.
- Convert repeated friction into a named change package with a current consumer.
- Publish one follow-up note covering fixes and one real integration example.

### Channels

Priority order:

1. GitHub release and repository README.
2. Nix community channels, because the supported install is the flake.
3. Rust and developer-tooling communities with the technical demo.
4. Radicle, matching the repository’s collaboration model.
5. A Show HN-style post only after the install and demo work from a clean
   machine; lead with the receipt example, not the agent narrative.

Do not mass-post the same text. Adapt the first paragraph to each channel.

## Announcement draft

### Short

> Cairn is a deterministic lifecycle engine for repository work. It turns a
> change into a bounded package of proposal, design, tasks, and specs, then runs
> policy-owned gates and emits receipts that people, CI, and coding agents can
> inspect. It is written in Rust, uses a Nickel policy layer, and installs from
> Nix. A passing gate proves conformance to selected policy and inputs — not
> code correctness.

### Long

> Coding agents can produce changes faster than review can verify them. Most
> teams answer with more prose: a template, a checklist, a transcript, and a
> green CI run that does not say what policy or inputs were checked.
>
> Cairn makes the change explicit. `cairn init` creates a native `.cairn/`
> lifecycle. `cairn scaffold change` writes a bounded package. Policy-owned
> stages and gates evaluate it. Every result is deterministic JSON with policy,
> input, and receipt identities, and `cairn --machine` gives an agent one
> versioned outcome.
>
> The same boundary extends to CI and release work: gate interchange binds a
> native receipt to an exact candidate and evidence set; release evidence
> indexes and checks a payload; release-run ledgers record what happened.
>
> Cairn is deliberately conservative about claims. A gate does not prove code
> correctness, evidence truth, build reproducibility, deployment safety, or
> release eligibility. The docs state the boundary for each surface.
>
> Install with `nix profile install github:OnixResearch/cairn#cairn`, then run
> the 60-second demo in the README.

## Demo script

1. Create an empty repository and run `cairn init`.
2. Show the `.cairn/` layout.
3. Scaffold a change with `--execute` and open the four artifacts.
4. Run `cairn validate` and point at `valid`, `findings`, and `policy`.
5. Run `cairn gate proposal` and show the receipt identity fields.
6. Run `cairn readiness` and show the artifact states.
7. Run `cairn --machine validate` and show the versioned envelope.
8. State what the receipts do not prove.

Keep the recording to the terminal. Do not simulate a dashboard.

## Success metrics

Leading indicators, in order of signal quality:

1. External repositories with a committed `.cairn/` layout.
2. External issues that reach `gate` or `readiness` and name a concrete blocker.
3. Integration reports that bind a Cairn receipt to CI or release evidence.
4. MCP or machine-contract consumers outside the maintainer group.
5. Repeat use: a repository that runs Cairn on a second change.

Weak signals only: stars, clones, and social impressions. Do not use commit
counts, archive counts, or generated-code volume as launch evidence.

## Risks and mitigations

| Risk | Mitigation |
|---|---|
| “Another spec tool” | Lead with deterministic receipts and the machine contract, not templates. |
| Agent hype | State that Cairn does not call a model; show the local CLI. |
| Nix-only install | Document the Cargo path and its Nickel runtime requirement honestly. |
| AGPL CLI confusion | Link the license boundary and explain the permissive core. |
| aarch64 expectation | Keep the packaged-but-unclaimed boundary visible. |
| Overclaiming evidence | Put the non-claim in the announcement and every demo. |
| CI cannot fetch dependencies | Anonymous public HTTPS is a release blocker; recheck before tagging. |

## Release gate

Run from the tagged commit on a clean checkout:

```bash
cargo fmt --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
nix run path:$PWD#cairn -- policy export --check
nix run path:$PWD#cairn -- validate --root .
nix run path:$PWD#cairn -- release-readiness --root .
nix flake check path:$PWD -L
```

After tagging:

```bash
nix build github:OnixResearch/cairn/v0.2.0#cairn -o /tmp/cairn-release
cd "$(mktemp -d)"
/tmp/cairn-release/bin/cairn --version
/tmp/cairn-release/bin/cairn init --root repo
/tmp/cairn-release/bin/cairn validate --root repo
```

Record the commands, results, platform, and skipped checks. Label any local or
credentialed run as such. Do not claim aarch64 support without a
builder-backed receipt.

`release-evidence index` rejects a dirty worktree. Commit the release candidate
before generating the indexed payload; a local dirty-tree run is not release
evidence.

## Launch checklist

- [ ] `v0.2.0` version matches `Cargo.toml`, `Cargo.lock`, and the Nix package.
- [ ] GitHub repository description, homepage, and topics match the README
      positioning (`rust`, `nix`, `lifecycle`, `spec-driven`, `coding-agents`).
- [ ] GitHub private vulnerability reporting is enabled.
- [ ] `CHANGELOG.md` covers user-visible changes and breaking notes.
- [ ] `README.md` install and demo commands pass from a clean machine.
- [ ] `SECURITY.md` reporting channel is enabled on GitHub.
- [ ] `CONTRIBUTING.md` commands match the current repository gate.
- [ ] CI passes on `main` and on the release commit.
- [ ] Release workflow verifies tag, build, foreign-CWD smoke, and release.
- [ ] Dependency URLs fetch anonymously without SSH credentials.
- [ ] Release-readiness report reviewed; skips explained.
- [ ] aarch64 boundary stated in the release notes.
- [ ] Announcement contains the non-claim.
- [ ] Feedback issue opened and monitored.

## Non-claims

This plan does not establish product-market fit, security certification,
performance, correctness, or release eligibility. It does not authorize a tag,
announcement, upload, or registry publication. Those actions remain explicit
maintainer decisions with their own evidence.
