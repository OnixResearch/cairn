# Design: aarch64 release support boundary

Record the current release evidence boundary: local x86_64 flake checks passed, Nix omitted incompatible aarch64 checks, and a future operator must run a builder-backed `nix flake check --all-systems` or targeted aarch64 package/check before claiming aarch64 release support. The docs note keeps infrastructure noise separate from product-local release proof.
