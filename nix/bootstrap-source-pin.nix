{ lib }:
let
  sources = {
    cap-root = {
      package = "cap-root-core";
      url = "https://seed.radicle.garden/z3cWBCK7VPbEmbL5MAvKJWKjCE745.git";
      revision = "8991e2572578c16e590c2f1c0915db6c46c5d9cb";
      license = "AGPL-3.0-or-later";
      narHash = "sha256-2eiyxmhW8/qj30ALccgkt5hatRMk2OcazmU3SCI/Opo=";
    };
    durable-file-publication = {
      package = "durable-file-publication";
      url = "https://seed.radicle.garden/z3tAR4For7qw8ZirkJzoDw1VNDDLM.git";
      revision = "951c27f59003cea9bfdb40ed4d89653d50fada1f";
      license = "MIT OR Apache-2.0";
      narHash = "sha256-fKxZ+3rzWzVuawILnmpiYCCe8PT/z6usopFMbr2KFbI=";
    };
  };
  require = condition: code: lib.optional (!condition) code;
  validate =
    input:
    lib.concatLists (
      lib.mapAttrsToList (
        name: expected:
        let
          dependency = input.workspace.workspace.dependencies.${expected.package} or { };
          packages = builtins.filter (p: p.name == expected.package) input.cargoLock.package;
          nodeName = input.flakeLock.nodes.root.inputs.${name} or "missing";
          node = input.flakeLock.nodes.${nodeName} or { };
          locked = node.locked or { };
          original = node.original or { };
          resolved = input.resolved.${name};
          manifest = builtins.fromTOML (builtins.readFile "${resolved}/Cargo.toml");
          license = manifest.workspace.package.license or (manifest.package.license or null);
        in
        require ((dependency.git or null) == expected.url) "${name}.cargo-source"
        ++ require ((dependency.rev or null) == expected.revision) "${name}.cargo-revision"
        ++ require (
          !(dependency ? path || dependency ? branch || dependency ? tag)
        ) "${name}.cargo-floating-or-path"
        ++ require (input.cli.dependencies.${expected.package}.workspace or false) "${name}.consumer"
        ++ require (
          builtins.length packages == 1
          &&
            ((builtins.head packages).source or null)
            == "git+${expected.url}?rev=${expected.revision}#${expected.revision}"
        ) "${name}.cargo-lock"
        ++ require ((node.flake or true) == false) "${name}.flake-enabled"
        ++ require (
          (locked.type or null) == "git" && (locked.url or null) == expected.url
        ) "${name}.nix-source"
        ++ require (
          (locked.rev or null) == expected.revision
          && (original.rev or null) == expected.revision
          && (resolved.rev or null) == expected.revision
        ) "${name}.nix-revision"
        ++ require (
          (original.url or null) == expected.url && !(original ? ref || original ? path)
        ) "${name}.nix-floating-or-path"
        ++ require (
          (locked.narHash or null) == expected.narHash && (resolved.narHash or null) == expected.narHash
        ) "${name}.nix-content"
        ++ require (
          (input.outputHashes."${expected.package}-0.1.0" or null) == expected.narHash
        ) "${name}.cargo-nix-content"
        ++ require (license == expected.license) "${name}.notices"
      ) sources
    );
  selfTest =
    input:
    lib.concatLists (
      lib.mapAttrsToList (
        name: expected:
        let
          reject = code: changed: require (builtins.elem code (validate changed)) "self-test.${code}";
          node = input.flakeLock.nodes.root.inputs.${name};
        in
        reject "${name}.cargo-revision" (
          lib.recursiveUpdate input {
            workspace.workspace.dependencies.${expected.package}.rev =
              "0000000000000000000000000000000000000000";
          }
        )
        ++ reject "${name}.cargo-floating-or-path" (
          lib.recursiveUpdate input {
            workspace.workspace.dependencies.${expected.package}.path = "../ambient";
          }
        )
        ++ reject "${name}.cargo-floating-or-path" (
          lib.recursiveUpdate input {
            workspace.workspace.dependencies.${expected.package}.branch = "main";
          }
        )
        ++ reject "${name}.nix-revision" (
          lib.recursiveUpdate input {
            flakeLock.nodes.${node}.locked.rev = "0000000000000000000000000000000000000000";
          }
        )
        ++ reject "${name}.nix-content" (
          lib.recursiveUpdate input {
            flakeLock.nodes.${node}.locked.narHash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
          }
        )
        ++ reject "${name}.cargo-nix-content" (input // { outputHashes = { }; })
        ++ reject "${name}.cargo-lock" (
          input
          // {
            cargoLock = input.cargoLock // {
              package = [ ];
            };
          }
        )
      ) sources
    );
in
{
  inherit validate selfTest;
}
