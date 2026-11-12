{ lib }:
let
  sources = {
    cap-root = {
      package = "cap-root-core";
      path = "vendor/cap-root-core";
      license = "AGPL-3.0-or-later";
    };
    durable-file-publication = {
      package = "durable-file-publication";
      path = "vendor/durable-file-publication";
      license = "MIT OR Apache-2.0";
    };
  };
  require = condition: code: lib.optional (!condition) code;
  manifestAt =
    input: expected:
    builtins.fromTOML (
      builtins.readFile "${input.root}/${expected.path}/Cargo.toml"
    );
  validate =
    input:
    lib.concatLists (
      lib.mapAttrsToList (
        name: expected:
        let
          dependency = input.workspace.workspace.dependencies.${expected.package} or { };
          packages = builtins.filter (p: p.name == expected.package) input.cargoLock.package;
          manifest = (input.manifests or { }).${name} or (manifestAt input expected);
          license = manifest.package.license or null;
        in
        require ((dependency.path or null) == expected.path) "${name}.cargo-path"
        ++ require (
          !(dependency ? git || dependency ? rev || dependency ? branch || dependency ? tag)
        ) "${name}.cargo-git-source"
        ++ require (input.cli.dependencies.${expected.package}.workspace or false) "${name}.consumer"
        ++ require (builtins.length packages == 1) "${name}.cargo-lock-unique"
        ++ require (
          builtins.length packages == 1
          && (builtins.head packages).version == "0.1.0"
          && !((builtins.head packages) ? source)
        ) "${name}.cargo-lock-path"
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
        in
        reject "${name}.cargo-path" (
          lib.recursiveUpdate input {
            workspace.workspace.dependencies.${expected.package}.path = "vendor/substituted";
          }
        )
        ++ reject "${name}.cargo-git-source" (
          lib.recursiveUpdate input {
            workspace.workspace.dependencies.${expected.package}.git = "https://example.invalid/source.git";
          }
        )
        ++ reject "${name}.consumer" (
          lib.recursiveUpdate input {
            cli.dependencies.${expected.package}.workspace = false;
          }
        )
        ++ reject "${name}.cargo-lock-path" (
          input
          // {
            cargoLock = input.cargoLock // {
              package = map (
                package:
                if package.name == expected.package then
                  package // { source = "git+https://example.invalid/source.git"; }
                else
                  package
              ) input.cargoLock.package;
            };
          }
        )
        ++ reject "${name}.notices" (
          input
          // {
            manifests = (input.manifests or { }) // {
              ${name} = (manifestAt input expected) // {
                package = (manifestAt input expected).package // {
                  license = "substituted";
                };
              };
            };
          }
        )
      ) sources
    );
in
{
  inherit validate selfTest;
}
