# Git remote configuration

Cairn's canonical SSH remote is:

```text
git@github.com:OnixResearch/cairn.git
```

Use this URL for both fetch and push to avoid GitHub relocation warnings from older lowercase remote configuration:

```bash
git remote set-url origin git@github.com:OnixResearch/cairn.git
git remote set-url --push origin git@github.com:OnixResearch/cairn.git
git remote -v
```

This note does not validate credentials, expose SSH agent sockets, or require any force-push/rewrite operation.

r[verify developer_experience.git_remote_relocation_cleanup.configure]
r[verify developer_experience.git_remote_relocation_cleanup.verify]
r[verify developer_experience.git_remote_relocation_cleanup.docs]
r[verify developer_experience.git_remote_relocation_cleanup.validation]
