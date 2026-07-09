## Design

Create a Nickel-authored profile matrix with rows for repo, profile ID, purpose, toolchain pins, validation commands, required artifacts, and non-claims. A pure checker validates row completeness and command/artifact references. A shell can later materialize profile receipts by running configured commands.

The first slice is declaration and validation only; command execution remains explicit and separate from Cairn gates unless a probe or release readiness policy requires saved receipts.
