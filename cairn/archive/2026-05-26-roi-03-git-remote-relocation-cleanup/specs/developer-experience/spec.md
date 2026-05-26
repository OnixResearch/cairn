# Developer Experience Specification Delta

## ADDED Requirements

### Requirement: Canonical Git remote is documented for operators
r[developer_experience.git_remote_relocation_cleanup.configure] The system SHOULD document the canonical Cairn Git remote so operators can remove relocation-warning noise from future pushes.

#### Scenario: Remote configuration is verified
r[developer_experience.git_remote_relocation_cleanup.verify]
- GIVEN an operator updates the local `origin` URL
- WHEN they inspect `git remote -v` and branch status
- THEN the remote SHALL point at `git@github.com:OnixResearch/cairn.git` and the worktree SHALL remain clean and synced.

#### Scenario: Remote note avoids credential overclaims
r[developer_experience.git_remote_relocation_cleanup.docs]
r[developer_experience.git_remote_relocation_cleanup.validation]
- GIVEN the operator note is committed
- WHEN future agents inspect repository docs
- THEN it SHALL describe the canonical URL and SHALL NOT expose credentials, SSH agent sockets, or imply credential validation.
