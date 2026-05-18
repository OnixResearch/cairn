# Design: Native Gate Policy Alignment

## Context

AgentKit's OpenGate policy defines stage vocabulary, task marker enforcement, finding defaults, archive ignoring, and bounded exemptions in Nickel. Cairn already has a broader Nickel policy but lacks a native gate command surface.

## Goals / Non-Goals

**Goals:**
- Reuse AgentKit semantics as Cairn-owned Nickel data.
- Evaluate gate readiness in pure core logic from a shell-supplied change snapshot.
- Render deterministic JSON receipts in the CLI.

**Non-Goals:**
- Vendor AgentKit files wholesale.
- Add external AgentKit/Node runtime dependency.
- Implement LLM review prompts in this slice.

## Decisions

### 1. Policy remains Nickel-authored Cairn data

**Choice:** Extend `cairn-policy/{contracts,default}.ncl` with gate policy fields matching AgentKit semantics.
**Rationale:** This preserves one policy source and generated JSON freshness checks.
**Alternative:** Import AgentKit policy directly; rejected because Cairn should be self-owned.

### 2. Gate evaluation is functional core

**Choice:** Add pure gate evaluation over `ChangeStatus`, `Policy`, and a stage enum.
**Rationale:** It keeps filesystem reads and JSON rendering in the CLI while keeping decisions testable.

### 3. CLI emits JSON gate receipts

**Choice:** Add `cairn gate proposal|design|tasks <change>`.
**Rationale:** Receipts are easy to consume by agents/CI and mirror existing command style.

## Risks / Trade-offs

**Initial gate depth is shallow** → Mitigation: enforce deterministic artifact/stage/task-marker gates first; later slices can add richer proposal/design quality checks.
