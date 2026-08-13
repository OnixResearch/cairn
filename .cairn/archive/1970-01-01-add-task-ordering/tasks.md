## Phase 1: Functional core

- [x] [serial] [task:dag-core] Extract the deterministic DAG order component and preserve change-graph behavior. r[cairn.task_ordering.graph]
- [x] [serial] [task:task-graph] [after:dag-core] Add task marker parsing, graph admission, readiness, and progress checks. r[cairn.task_ordering.graph] r[cairn.task_ordering.progress]
- [x] [serial] [task:core-tests] [after:task-graph] Add positive and negative core tests for graph and progress behavior. r[cairn.task_ordering.tests]

## Phase 2: Policy and receipts

- [x] [serial] [task:policy] [after:core-tests] Add typed Nickel marker policy and refresh generated policy data. r[cairn.task_ordering.markers]
- [x] [serial] [task:gate] [after:policy] Add task-ordering admission to task gates and gate receipts. r[cairn.task_ordering.gate]
- [x] [serial] [task:drain] [after:gate] Add stable order and ready-task guidance to drain-plan receipts. r[cairn.task_ordering.drain]
- [x] [serial] [task:shell-tests] [after:drain] Add positive and negative task-gate and drain-plan tests. r[cairn.task_ordering.tests]

## Phase 3: Completion

- [x] [serial] [task:docs] [after:shell-tests] Document marker syntax, compatibility, and claim boundaries. r[cairn.task_ordering.markers]
- [x] [serial] [task:verify] [after:docs] Run focused and full verification, then sync and archive the change. r[cairn.task_ordering.tests]
