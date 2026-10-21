let O = import "mechanism-observations.json" in
{
  schema = "cairn.downstream-mcp-mechanism-receipt.v1",
  owner = "cairn-cli",
  current_consumer = "ChaosControl's full portable completion contract",
  immediate_outcome = "Integrated producer and selected-policy mechanisms pass focused verification; actual failed formatting and malformed selections remain nonpassing",
  durable_capability = "Consumer-owned completion paths, exact selected-policy propagation, bounded dependency observations and maintained command profiles",
  maintenance_owner = "Cairn CLI/core maintainers; ChaosControl owns its consumer registry and source/dependency review",
  focused_checks_passed | std.contract.Equal true = std.array.all (fun row => row.exit_code == 0 && !row.operational_timeout) (std.record.values O.focused_commands),
  failed_command_rejected | std.contract.Equal true = !O.actual_failed_command.report.valid && !O.actual_failed_command.report.complete,
  original_consumers_reject_failed_rows | std.contract.Equal true = std.array.all (fun row => !row.report.valid) (std.record.values O.failed_command_consumers),
  invalid_present_selection_rejected | std.contract.Equal true = std.array.all (fun row => row.exit_code != 0) O.invalid_present_selection_controls,
  fresh_corpus_accepted | std.contract.Equal true = (std.array.at 0 O.consumer_corpus_controls).report.valid,
  stale_corpus_rejected | std.contract.Equal true = !(std.array.at 1 O.consumer_corpus_controls).report.valid && (std.array.at 1 O.consumer_corpus_controls).report.stale_count == 1,
  full_consumer_contract_exercised | std.contract.Equal false = O.full_consumer_contract_exercised,
  production_ready | std.contract.Equal false = O.production_ready,
  evidence_boundary = O.source_boundary,
}
