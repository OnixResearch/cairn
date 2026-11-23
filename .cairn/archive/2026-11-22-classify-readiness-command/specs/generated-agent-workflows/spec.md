# Generated workflow command routes

## ADDED Requirements

### Requirement: Named workflow commands stay inside the allowed classes

r[cairn.generated_agent_workflows.command_routes] Admission of the generated agent-workflow policy MUST reject a workflow descriptor whose rendered text names a Cairn command that resolves to no command catalog entry, or to a catalog entry whose command class the descriptor's `command_classes` does not allow. The rule MUST apply to every descriptor and every catalog entry.

A named command is a lowercase `cairn` token followed by one or more lowercase command words in the descriptor's goal, success evidence, required evidence, progress rule, immediate action, output shape, constraints or non-claims. Before matching, Markdown emphasis, code spans, quotes, brackets and parentheses around a token, and trailing `.`, `,`, `:` or `;`, are stripped; trailing wrapping or punctuation ends the command. The info string of a Markdown code fence line is never read as a command; the lines inside the fence are. A named command resolves to the catalog entries whose invocation command words form the longest prefix of its command words. Admission MUST read each token once and at most the catalog's largest command-word count of tokens after each `cairn`, so its work is linear in the descriptor text.

#### Scenario: Commands in allowed classes are admitted

r[cairn.generated_agent_workflows.command_routes.permitted]
- GIVEN a policy whose descriptors name only catalog commands in their allowed classes
- WHEN the generated agent-workflow policy is admitted
- THEN admission MUST succeed
- AND a named command MUST resolve to the catalog entry with the longest matching command-word prefix, so `cairn workflow status` resolves to the `workflow status` entry rather than the `status` entry
- AND capitalized prose such as "Cairn command classes" MUST NOT be read as a command
- AND a command wrapped in Markdown, such as `**cairn status**`, MUST resolve as the bare command, and the info string of a code fence opened with three backticks followed by `cairn` MUST NOT be read as a command.

#### Scenario: Commands outside the allowed classes are refused

r[cairn.generated_agent_workflows.command_routes.refused]
- GIVEN a descriptor whose text names a catalog command whose class the descriptor does not allow, or a Cairn command the catalog does not list
- WHEN the generated agent-workflow policy is admitted
- THEN admission MUST fail with a descriptor issue at the field that names the command
- AND the issue MUST name the command words and, for a catalog command, its catalog ID and class
- AND the issue MUST copy at most 64 bytes of any policy-supplied identifier or command text, marking a truncated value with its full length.
