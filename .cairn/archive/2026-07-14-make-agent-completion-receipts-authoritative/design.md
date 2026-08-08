## Context

`crates/cairn-core/src/verified/agent_receipt/verification.rs` currently checks only whether a successful receipt contains at least one `verification_manifest.checks` element. The element's shape, result status, evidence hash, command identity, subject freshness, and relationship to repository-required checks are not evaluated. The public completion helper takes only the receipt, so it has no independent authority against which to compare a producer's verification claims.

A truthful completion decision needs three independent inputs: a repository-owned verification contract, producer-owned execution receipts, and the final subject identity. The repository must choose the checks; the producer may execute and summarize them; Cairn must decide whether the supplied evidence satisfies the contract.

## Decisions

### 1. Separate completion requests from completion authority

**Choice:** Treat wrapper verdicts, model completion markers, task checkbox state, process exit status, and archive presence as requests or observations only. Authoritative completion requires a passing Cairn validation receipt over the current verification contract, execution evidence, and final subject identity.

**Rationale:** The actor requesting completion must not also be the sole authority deciding whether its evidence is sufficient.

### 2. Add a versioned authoritative contract

**Choice:** Introduce a new agent-run receipt schema/profile containing a `verification_contract` identity with repository ID, registry ID and BLAKE3 hash, selection hash, policy hash, final subject hash, required check IDs, and prerequisite edges. Each check result binds its check ID, exact command identity, status, subject hash, evidence hash, and execution-receipt hash.

**Rationale:** A non-empty producer-authored list cannot prove required coverage. Independent contract identity makes omissions and substitutions detectable.

### 3. Keep check selection repository-owned

**Choice:** Cairn policy defines the generic contract and status schema, but each consuming repository owns its check registry and change-aware selection. CLI/MCP shells accept an explicit bounded registry/selection artifact or a current repository policy export; Cairn does not infer checks from languages, broad workspace heuristics, sibling repositories, or model prose.

**Rationale:** The repository knows its behavioral boundaries. Cairn should validate selection evidence, not become a top-level monorepo check authority.

### 4. Make only one status passing

**Choice:** `passed` is the sole completion-eligible check status. `failed`, `blocked`, `skipped`, `deferred`, `timed_out`, `cancelled`, `dry_run`, `cleanup_failed`, `infrastructure_error`, `not_selected`, and `unknown` remain non-passing. Status normalization is explicit and unknown values fail closed.

**Rationale:** Inability or refusal to run a required check is useful diagnostic evidence, but it is not evidence that the check passed.

### 5. Bind evidence to the final subject

**Choice:** Every required result must cite the same final subject hash used by the completion decision. Registry, selection, policy, command, prerequisite, artifact, and execution receipt identities enter canonical BLAKE3 hash material. Any mutation after validation changes the subject identity and invalidates the prior completion receipt.

**Rationale:** Passing checks against an earlier tree cannot justify completion of a later tree.

### 6. Evaluate exact coverage in a pure core

**Choice:** Add a pure core that receives normalized contract rows, normalized result rows, current subject identity, and policy metadata. It validates row shape, unique IDs, exact required coverage, prerequisite closure, status eligibility, hash consistency, freshness, and coherent aggregate verdict without filesystem, process, environment, clock, network, stdout, or stderr access.

**Rationale:** The critical truthfulness decision must be easy to exercise with plain positive and negative fixtures.

### 7. Return actionable, bounded feedback

**Choice:** Verification diagnostics carry a stable diagnostic ID and code, check ID, result status, subject/registry/selection/evidence/receipt hashes as applicable, a bounded repository-relative source span when known, remediation label and bounded summary, exact reproduction argv plus a safe working-root label, safe artifact references with BLAKE3 hashes, severity, and explicit non-claims. Raw logs, prompts, credentials, environment values, absolute host paths, raw PTY output, and unbounded command output are prohibited.

**Rationale:** Backpressure is useful only when the next agent turn can identify and reproduce the failure without receiving an unsafe transcript dump.

### 8. Keep execution in the shell

**Choice:** Cairn validation does not launch repository checks. The shell performs bounded reads and hashing and may validate already-produced execution receipts. Check runners remain repository/host-owned and must produce safe receipts before Cairn evaluates completion.

**Rationale:** Receipt authority does not require granting the validation core ambient process or filesystem authority.

### 9. Version compatibility honestly

**Choice:** Legacy v1 receipts remain parseable through a clearly labeled inspection path, but a successful v1 receipt is classified `legacy_unscoped` and cannot yield authoritative completion. CLI and MCP expose the same classification. The new schema, generated policy hash, fixtures, replay cases, public docs, JSON hashes, and Preserves goldens change together.

**Rationale:** Silent reinterpretation would either break historical evidence or continue accepting unsound completion claims.

### 10. Finalize one coherent outcome

**Choice:** The authoritative completion report derives `valid`, `verdict`, required/passed/non-passing sets, diagnostics, and receipt hash exactly once from the final normalized issue set.

**Rationale:** No adapter may append a blocker after a passing verdict has already been derived.

## Risks / Trade-offs

- Authoritative validation requires an independent repository contract input, which adds an operator/wrapper integration step.
- Existing wrappers that rely on v1 `PASS` behavior will need migration and should fail closed until they supply the new contract.
- Registry and policy changes intentionally invalidate older completion receipts; diagnostics must distinguish policy drift, selection drift, and stale subject evidence.
- Cairn proves contract coverage, identity linkage, and receipt consistency. It does not prove that a check oracle is sound, that command execution was honest beyond supplied evidence, that the model behaved correctly, or that the whole product is correct.
