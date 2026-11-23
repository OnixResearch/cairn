# Generated workflow command routes

## ADDED Requirements

### Requirement: Named workflow commands stay inside the allowed classes

r[cairn.generated_agent_workflows.command_routes] Admission of the generated agent-workflow policy MUST reject a workflow descriptor whose rendered text names a Cairn command that resolves to no command catalog entry, or to a catalog entry whose command class the descriptor's `command_classes` does not allow. The rule MUST apply to every descriptor and every catalog entry.

A named command is a lowercase `cairn` token followed by one or more lowercase command words in the descriptor's goal, success evidence, required evidence, progress rule, immediate action, output shape, constraints or non-claims. It resolves to the catalog entries whose invocation command words form the longest prefix of its command words.

#### Scenario: Commands in allowed classes are admitted

r[cairn.generated_agent_workflows.command_routes.permitted]
- GIVEN a policy whose descriptors name only catalog commands in their allowed classes
- WHEN the generated agent-workflow policy is admitted
- THEN admission MUST succeed
- AND a named command MUST resolve to the catalog entry with the longest matching command-word prefix, so `cairn workflow status` resolves to the `workflow status` entry rather than the `status` entry
- AND capitalized prose such as "Cairn command classes" MUST NOT be read as a command.

#### Scenario: Commands outside the allowed classes are refused

r[cairn.generated_agent_workflows.command_routes.refused]
- GIVEN a descriptor whose text names a catalog command whose class the descriptor does not allow, or a Cairn command the catalog does not list
- WHEN the generated agent-workflow policy is admitted
- THEN admission MUST fail with a descriptor issue at the field that names the command
- AND the issue MUST name the command words and, for a catalog command, its catalog ID and class.
