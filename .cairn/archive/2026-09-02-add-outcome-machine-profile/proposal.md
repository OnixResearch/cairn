# Proposal: Add an outcome-machine workflow profile

## Why

A lifecycle can reward visible completion while repeatability, ownership, and failure behavior become weaker.

AI increases this risk because generated output can appear complete while it depends on hidden session state or untracked operator knowledge.

Cairn can require review artifacts, but no current profile distinguishes isolated outcomes from durable capability. Repository policy needs an enforceable and product-neutral profile for this distinction.

## What Changes

- Add an opt-in `outcome-machine` workflow profile.
- Scaffold one typed `outcome-machine.json` review artifact during change creation.
- Admit delivery, foundation, and bounded-exception routes with distinct required facts.
- Validate the review record in the pure core over shell-loaded JSON.
- Block the design gate when the route record is malformed or incomplete.
- Block sync and archive until completion evidence and fresh-state disposition are recorded.
- Reject hidden session state and non-repository completion authority.
- Add positive and negative policy, core, CLI, gate, sync, archive, and compatibility tests.

## Impact

- **Policy:** Nickel gains one workflow schema and one structural admission class.
- **Core:** Artifact-workflow evaluation gains deterministic outcome-machine review admission.
- **CLI:** Change creation scaffolds the typed review artifact and the shell loads its bounded JSON.
- **Lifecycle:** Repositories can select or require the new profile without changing existing profile behavior.
- **Documentation:** The workflow profile guide defines route, evidence, and claim boundaries.

## Non-goals

- Do not make `outcome-machine` the Cairn default.
- Do not let Cairn decide whether a product outcome is valuable.
- Do not claim that recorded evidence is true or sufficient.
- Do not require every change to create a shared component.
- Do not weaken existing profile, gate, receipt, sync, or archive compatibility.
