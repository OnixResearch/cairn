# Consumer-owned packaging only. The upstream checkout, crate sources and
# workspace inheritance remain unchanged; unrelated producer examples are not
# packages admitted by Cairn's lockfile. No lint scope or command is changed.
{
  pkgs,
  crane,
  mantleSource,
  kilnSource,
  cargoLock,
}:
let
  revision = "0bbab77e23ab95caa4e4184b9945cfaa5c8040e2";
  source = "git+ssh://git@github.com/OnixResearch/mantle.git?rev=${revision}#${revision}";
  craneLib = crane.mkLib pkgs;
  checker = import ./check-producer-source-pins.nix { lib = pkgs.lib; };
  kilnWorkspace = builtins.fromTOML (builtins.readFile "${kilnSource}/Cargo.toml");
  kilnPackaging = {
    workspace = kilnWorkspace;
    manifest = builtins.fromTOML (builtins.readFile "${kilnSource}/crates/kiln-core/Cargo.toml");
    notices = builtins.mapAttrs (
      name: _: builtins.hashFile "sha256" "${kilnSource}/${name}"
    ) checker.kilnNotices;
  };
  # Project only Cargo membership. Inherited values come from the exact
  # producer manifest, not a consumer reconstruction of producer semantics.
  kilnManifest = (pkgs.formats.toml { }).generate "kiln-core-workspace.toml" {
    workspace = {
      members = [ "crates/kiln-core" ];
      inherit (kilnWorkspace.workspace) resolver package lints;
      dependencies = { inherit (kilnWorkspace.workspace.dependencies) serde; };
    };
  };
  kilnCoreSource = pkgs.runCommand "kiln-core-source-${checker.expected.kiln-core}" { } ''
    mkdir -p "$out/crates"
    cp ${kilnManifest} "$out/Cargo.toml"
    cp -R ${kilnSource}/crates/kiln-core "$out/crates/kiln-core"
    cp ${kilnSource}/LICENSE-APACHE ${kilnSource}/LICENSE-MIT "$out/"
    diff -r ${kilnSource}/crates/kiln-core "$out/crates/kiln-core"
  '';
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
    else if
      builtins.any (package: package.source == checker.kilnSource || package.name == "kiln-core") packages
    then
      assert checker.validateKilnSource {
        revision = kilnSource.rev;
        narHash = kilnSource.narHash;
        inherit packages;
      };
      assert checker.validateKilnPackaging kilnPackaging;
      drv.overrideAttrs (old: {
        src = kilnCoreSource;
        installPhase =
          assert pkgs.lib.hasInfix ''find "$(pwd)" -name Cargo.toml'' old.installPhase;
          pkgs.lib.replaceStrings
            [ ''find "$(pwd)" -name Cargo.toml'' ]
            [ ''find "$(pwd)/crates/kiln-core" -name Cargo.toml'' ]
            old.installPhase;
        postInstall = (old.postInstall or "") + ''
          cp ${kilnSource}/LICENSE-APACHE ${kilnSource}/LICENSE-MIT "$out/kiln-core-0.1.0/"
          diff -r ${kilnSource}/crates/kiln-core/src "$out/kiln-core-0.1.0/src"
          cmp ${kilnSource}/LICENSE-APACHE "$out/kiln-core-0.1.0/LICENSE-APACHE"
          cmp ${kilnSource}/LICENSE-MIT "$out/kiln-core-0.1.0/LICENSE-MIT"
          test "$(find "$out" -mindepth 1 -maxdepth 1 -type d | wc -l)" -eq 1
        '';
      })
    else
      drv;
}
