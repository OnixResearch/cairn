# Authoritative saved-probe gates

Cairn task gates observe saved probe declarations and receipts without executing probe commands. The shell bounds reads, rejects malformed or unsafe evidence, computes BLAKE3 declaration and receipt identities, and passes normalized issues to the pure gate finalizer. The finalizer derives `valid` and `verdict` once from the complete ordered issue set. r[verify cairn.probe_execution_boundary.authoritative_gate_composition]

Only a current receipt with `status_class="success"`, `status_matches=true`, `valid=true`, `verdict="PASS"`, passing assertions, matching declaration/policy/requirement identities, and a valid receipt hash satisfies a declaration. Missing, malformed, stale, tampered, skipped, blocked, deferred, timed-out, dry-run, cancelled, cleanup-failed, infrastructure-failed, and unsupported statuses block. r[verify cairn.probe_execution_boundary.authoritative_status]

CLI and MCP task gates expose bounded `probe_evidence` identities. Gate, sync, and archive input hashes bind those identities. Sync/archive and mutation-capable MCP tools call the same saved-evidence observer before any write; a failed observation emits `mutated=false`. Inline probe execution remains disabled. r[verify cairn.probe_execution_boundary.authoritative_mutation_block]

Probe evidence proves only the declared command/assertion result for the bound declaration, policy, and saved receipt. It does not prove undeclared behavior, external freshness, whole-system correctness, or release eligibility.
