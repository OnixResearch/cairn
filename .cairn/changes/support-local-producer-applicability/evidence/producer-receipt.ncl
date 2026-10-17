# Contract over actual producer commands; mechanism conformance is not release authority.
let O = import "producer-observations.json" in
let B3 = std.contract.from_predicate (fun value =>
  std.is_string value && std.string.is_match "^blake3:[0-9a-f]{64}$" value
) in
let Artifact = { path | String, digest | B3, bytes | Number } in
let D = O.direct in
let M = O.mcp in
let Checks = O.required_checks.checks in
let Required = ["workspace-tests", "clippy", "formatting", "policy-freshness",
  "native-validation", "native-traceability", "octet", "nix-checks"] in
let CompleteInventory = std.array.length Checks == std.array.length Required
  && std.array.all (fun name =>
    std.array.length (std.array.filter (fun check => check.name == name) Checks) == 1
  ) Required in
let MechanismsPassed = D.valid
  && D.positive.native_build && D.positive.nonempty_dependencies
  && D.positive.original_completion && D.positive.manifest_verification
  && D.negative.missing_dependencies && D.negative.dependency_drift
  && D.negative.stale_manifest && D.negative.failed_native_build
  && D.negative.exit_zero_nonpassing_report
  && M.valid && std.array.length M.commands == 2
  && std.array.any (fun row => row.name == "positive" && row.exit_code == 0
    && row.valid && !row.checks_executed) M.commands
  && std.array.any (fun row => row.name == "failed-build" && row.exit_code == 0
    && !row.valid && !row.checks_executed) M.commands in
let AllChecksPassed = CompleteInventory && O.required_checks.all_passed
  && std.array.all (fun check => check.passed && check.execution.exit_code == 0
    && !check.execution.operational_timeout) Checks in
{
  schema = "cairn.authorized-producer-mechanism-receipt.v1",
  owner = "cairn-cli",
  immediate_outcome = "Bounded dependency observation and real native Nix payloads reach the original completion, manifest, and read-only MCP consumers; independent repository gates remain nonpassing",
  durable_capability = "Product-neutral local producer admission, exact dependency rechecks, and build-only completion evidence without process-success substitution",
  maintenance_owner = "cairn-cli and cairn-core maintainers",
  current_consumer = "ChaosControl required completion contract",
  source = O.source,
  executable | Artifact = O.executable,
  mechanism_conformance | std.contract.Equal true = MechanismsPassed,
  required_gate_inventory_complete | std.contract.Equal true = CompleteInventory,
  all_required_checks_passed | std.contract.Equal AllChecksPassed = AllChecksPassed,
  consumer_full_contract_exercised | std.contract.Equal false = O.consumer_full_contract_exercised,
  production_ready | std.contract.Equal false = false,
  archive_authorized_by_evidence | std.contract.Equal false = O.archive_authorized_by_evidence,
  outcome = if AllChecksPassed then "mechanism-verified" else "blocked",
  direct_controls = D,
  mcp_controls = M,
  required_checks = O.required_checks,
  command_observations = O.commands,
  artifacts | Array Artifact = O.artifacts,
  diagnosis = O.diagnosis,
  repeatability = "Replay the exact recorded producer/MCP commands with the retained executable and byte-identified source, dependency, contract, profile, result and manifest inputs. Required repository commands remain unchanged.",
  non_claims = [
    "No full ChaosControl MCP completion contract or its mcp_agent_smoke readiness input was supplied by the mechanism controls",
    "MCP controls read retained Rust-produced rows; read-only validation does not execute checks",
    "No global repository, clean-source release, main integration, archive, dependency attestation, sandboxing, or fresh-builder claim",
    "Operational timeouts are not guest bugs and exit zero is not a passing report",
  ],
}
