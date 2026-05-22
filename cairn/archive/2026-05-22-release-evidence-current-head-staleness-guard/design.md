# Design: Release evidence current-head staleness guard

## Command shape

Add:

```text
cairn release-evidence check-index --root PATH --index FILE [--mode current-head|audited-payload]
```

The command emits a deterministic JSON receipt and exits nonzero when `valid=false`.

- `--mode audited-payload` verifies the parsed payload commit exists and is reachable from live HEAD.
- `--mode current-head` additionally requires the parsed payload commit to equal live short HEAD.
- Default mode is inferred from the index path/name/title:
  - paths or titles containing `current-head` use `current-head` mode;
  - otherwise use `audited-payload` mode.

## Parsing

The checker parses the generated Markdown page's stable payload block:

```text
- Payload commit: `<short-sha>`
- Branch: `...`
- Worktree clean: `true|false`
- Upstream: `...`
- Ahead/behind: `...`
```

The parser is deliberately narrow and fail-closed: missing or malformed payload fields produce deterministic issues.

## Git checks

The imperative shell runs git locally under `--root` to collect:

- live short HEAD;
- whether the payload commit resolves;
- whether the payload commit is an ancestor of HEAD;
- whether current-head mode matches live HEAD.

No network access or mutation is required.

## Receipt

The receipt includes stable fields:

- `command`
- `schema`
- `root`
- `index`
- `mode`
- `valid`
- `verdict`
- `payload_commit`
- `live_head`
- `payload_exists`
- `payload_ancestor_of_head`
- `current_head_match`
- `issues`
- `receipt_hash`

## Integration

`release-readiness` should include the checked-in release evidence page check when the conventional docs page exists. This makes stale current-head claims visible during the existing readiness rail without requiring another ad-hoc command.

Docs should stop recommending a committed file as perpetually current. Fresh current-head evidence belongs in `target/` or CI artifacts; committed docs are audited payload snapshots unless refreshed in a dedicated evidence-doc commit.
