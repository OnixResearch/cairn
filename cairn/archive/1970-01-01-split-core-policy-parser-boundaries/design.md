# Design: Core policy parser boundary split

Keep `verified::policy` as the public module and move cohesive responsibilities behind it:

- `mod.rs`: public facade, exports, and Tracey implementation refs.
- `types.rs`: public policy DTOs and supported version constant.
- `error.rs`: `PolicyError` display/source behavior.
- `fields.rs`: generic field extraction plus task marker, gate, mutation, receipt, exemption, and change metadata decoding helpers.
- `steel.rs`: Steel orchestration policy/profile decoding.
- `parser.rs`: parse/validate assembly that preserves returned `Policy` shape and raw JSON retention.
- `tests.rs`: existing positive/negative policy parser regressions plus split-boundary verification refs.

The split must not rename public structs/functions/constants, alter error variants/messages, or change generated policy expectations.
