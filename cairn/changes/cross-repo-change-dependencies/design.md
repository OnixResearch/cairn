## Design

Extend Cairn change metadata with dependency rows: upstream repo label, change ID, required stage or receipt kind, expected receipt hash or accepted status, and non-claims. The pure core validates parsed dependencies against supplied upstream receipt summaries. The shell resolves local or imported evidence bundles.

Dependencies are evidence links only. Cairn does not fetch remote repos by default and does not prove the upstream implementation is correct.
