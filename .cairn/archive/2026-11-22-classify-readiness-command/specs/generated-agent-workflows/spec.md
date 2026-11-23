# Generated workflow command routes

## ADDED Requirements

### Requirement: Shipped workflow commands stay inside the allowed classes

r[cairn.generated_agent_workflows.command_routes] Every Cairn command that a workflow descriptor in Cairn's shipped default policy tells the agent to run MUST resolve to a command catalog entry whose command class the descriptor's `command_classes` allows. A test over the shipped default policy MUST verify this for every descriptor and every catalog entry. Policy admission MUST NOT read workflow descriptor text, so a consumer policy loads whatever commands its workflows name.

A named command is a lowercase `cairn` word followed by one or more lowercase command words in the descriptor's goal, success evidence, required evidence, progress rule, immediate action, output shape, constraints or non-claims, read as Markdown: paired emphasis markers (`*`, `**`, `_`, `__`) inside or around the command are ignored; code span content and link labels are read, and their edges end a command; link destinations, code fence lines and fence info strings are not read; an unpaired or intraword marker stays part of its word; parentheses, brackets, quotes and sentence punctuation end a command. A named command resolves to the catalog entries whose invocation command words form the longest prefix of its command words, found by one lookup per word.

#### Scenario: The shipped workflows name only permitted commands

r[cairn.generated_agent_workflows.command_routes.permitted]
- GIVEN Cairn's shipped default policy
- WHEN the command-route test reads every workflow descriptor's text
- THEN every named command MUST resolve to a catalog entry in a class that workflow allows
- AND a named command MUST resolve to the catalog entry with the longest matching command-word prefix, so `cairn workflow status` resolves to the `workflow status` entry rather than the `status` entry
- AND formatting such as `**cairn** gate tasks`, `cairn **gate** tasks`, a code span or a link label such as `[cairn readiness](#guide)` MUST NOT hide a command
- AND a code fence info string such as `cairn`, an unpaired marker such as `_cairn status`, and capitalized prose such as "Cairn command classes" MUST NOT be read as a command.

#### Scenario: A policy naming commands outside the allowed classes fails the test but loads

r[cairn.generated_agent_workflows.command_routes.refused]
- GIVEN a policy whose workflow text names a catalog command whose class the workflow does not allow, or a Cairn command the catalog does not list, such as the `9082e1ae` default policy
- WHEN the command-route test reads it
- THEN the test MUST report the field that names the command, the command words and, for a catalog command, its catalog ID and class
- AND policy admission of that policy MUST still succeed.
