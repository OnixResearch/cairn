# Consumer-owned packaging only. The upstream checkout, crate sources and
# workspace inheritance remain unchanged; unrelated Mantle examples are not
# packages admitted by Cairn's lockfile. No lint scope or command is changed.
{
  pkgs,
  crane,
  mantleSource,
  cargoLock,
}:
let
  revision = "0bbab77e23ab95caa4e4184b9945cfaa5c8040e2";
  source = "git+ssh://git@github.com/OnixResearch/mantle.git?rev=${revision}#${revision}";
  craneLib = crane.mkLib pkgs;
  mantleContractSource = pkgs.runCommand "mantle-build-contract-source-${revision}" { } ''
        mkdir -p "$out/crates"
        cat > "$out/Cargo.toml" <<'EOF'
    [workspace]
    resolver = "2"
    members = ["crates/mantle-build-contract"]

    [workspace.package]
    version = "0.1.0"
    edition = "2024"
    license = "AGPL-3.0-or-later"
    EOF
        cp -R ${mantleSource}/crates/mantle-build-contract "$out/crates/mantle-build-contract"
        cp ${mantleSource}/LICENSE* "$out/"
  '';
in
assert mantleSource.rev == revision;
craneLib.vendorCargoDeps {
  inherit cargoLock;
  overrideVendorGitCheckout =
    packages: drv:
    if builtins.any (package: package.source == source) packages then
      assert builtins.length packages == 1;
      assert (builtins.head packages).name == "mantle-build-contract";
      drv.overrideAttrs (old: {
        src = mantleContractSource;
        # Keep Cargo's own package file admission and Crane's inheritance
        # resolver. Restrict discovery to the admitted contract crate and its
        # workspace inheritance file, not unrelated Mantle example packages.
        installPhase =
          assert pkgs.lib.hasInfix ''find "$(pwd)" -name Cargo.toml'' old.installPhase;
          pkgs.lib.replaceStrings
            [ ''find "$(pwd)" -name Cargo.toml'' ]
            [ ''find "$(pwd)/crates/mantle-build-contract" -name Cargo.toml'' ]
            old.installPhase;
        postInstall = (old.postInstall or "") + ''
          cp ${mantleSource}/LICENSE* "$out/mantle-build-contract-0.1.0/"
          diff -r ${mantleSource}/crates/mantle-build-contract/src "$out/mantle-build-contract-0.1.0/src"
          cmp ${mantleSource}/LICENSE "$out/mantle-build-contract-0.1.0/LICENSE"
          test "$(find "$out" -mindepth 1 -maxdepth 1 -type d | wc -l)" -eq 1
        '';
      })
    else
      drv;
}
