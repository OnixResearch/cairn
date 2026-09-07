{ lib }:
let
  expected = {
    bounded-exec = "29dac88ecded94457572db3fdfaaaab95fa91525";
    durable-file-publication = "951c27f59003cea9bfdb40ed4d89653d50fada1f";
    mantle-build-contract = "0bbab77e23ab95caa4e4184b9945cfaa5c8040e2";
    kiln-core = "5d99f4e9bb954743443e340baece31256b4c53f1";
  };
  validate =
    {
      workspace,
      lock,
      inputs,
    }:
    lib.all (
      name:
      let
        revision = expected.${name};
        dependency = workspace.workspace.dependencies.${name};
        packages = builtins.filter (package: package.name == name) lock.package;
      in
      dependency.rev == revision
      && inputs.${name}.rev == revision
      && builtins.length packages == 1
      && (builtins.head packages).source == "git+${dependency.git}?rev=${revision}#${revision}"
    ) (builtins.attrNames expected);
  validateMantlePackaging =
    { workspace, manifest }:
    manifest.package.name == "mantle-build-contract"
    && manifest.package.version.workspace
    && manifest.package.edition.workspace
    && manifest.package.license.workspace
    &&
      workspace.workspace.package == {
        version = "0.1.0";
        edition = "2024";
        license = "AGPL-3.0-or-later";
      }
    && lib.all (dependency: !(dependency ? path)) (builtins.attrValues manifest.dependencies);
  kilnSource = "git+https://seed.radicle.garden/z2wsvXm5S2sJGvuV1k5JHiwi1PbKE.git?rev=${expected.kiln-core}#${expected.kiln-core}";
  kilnNarHash = "sha256-ULVaEH8tOiR2SoHVkpB5dB7ZKrvu99VxQ8KBLytP+RM=";
  # SHA-256 is the Nix source-admission boundary, not a Cairn receipt identity.
  kilnNotices = {
    LICENSE-APACHE = "cfc7749b96f63bd31c3c42b5c471bf756814053e847c10f3eb003417bc523d30";
    LICENSE-MIT = "873b1334ca4c0643b7b432ea5b3437ddd4176e1fa255deedf963fc43ebe7d11d";
  };
  validateKilnSource =
    {
      revision,
      narHash,
      packages,
    }:
    revision == expected.kiln-core
    && narHash == kilnNarHash
    && builtins.length packages == 1
    && (builtins.head packages).name == "kiln-core"
    && (builtins.head packages).version == "0.1.0"
    && (builtins.head packages).source == kilnSource;
  validateKilnPackaging =
    {
      workspace,
      manifest,
      notices,
    }:
    manifest.package.name == "kiln-core"
    && manifest.package.version.workspace
    && manifest.package.edition.workspace
    && manifest.package.license.workspace
    && manifest.package.rust-version.workspace
    && manifest.lints.workspace
    && workspace.workspace.resolver == "3"
    &&
      workspace.workspace.package == {
        version = "0.1.0";
        edition = "2024";
        license = "MIT OR Apache-2.0";
        rust-version = "1.88";
      }
    &&
      builtins.attrNames manifest.dependencies == [
        "blake3"
        "serde"
      ]
    && manifest.dependencies.serde.workspace
    && !(workspace.workspace.dependencies.serde ? path)
    && lib.all (dependency: !(dependency ? path)) (builtins.attrValues manifest.dependencies)
    && notices == kilnNotices;
  kilnSelfTest =
    sourceFacts: packaging:
    validateKilnSource sourceFacts
    && validateKilnPackaging packaging
    && lib.all (mutation: !(validateKilnSource (sourceFacts // mutation))) [
      { revision = "substituted"; }
      { narHash = "substituted"; }
      { packages = [ ]; }
      { packages = sourceFacts.packages ++ sourceFacts.packages; }
      { packages = [ ((builtins.head sourceFacts.packages) // { source = "substituted"; }) ]; }
      { packages = [ ((builtins.head sourceFacts.packages) // { name = "substituted"; }) ]; }
      { packages = [ ((builtins.head sourceFacts.packages) // { version = "substituted"; }) ]; }
    ]
    && lib.all (
      name:
      !(validateKilnPackaging (
        packaging
        // {
          notices = packaging.notices // {
            ${name} = "substituted";
          };
        }
      ))
    ) (builtins.attrNames kilnNotices)
    && !(validateKilnPackaging (packaging // { notices = { }; }))
    && !(validateKilnPackaging (
      packaging
      // {
        manifest = packaging.manifest // {
          package = packaging.manifest.package // {
            name = "substituted";
          };
        };
      }
    ));
in
{
  inherit
    expected
    validate
    validateMantlePackaging
    kilnSource
    kilnNotices
    validateKilnSource
    validateKilnPackaging
    kilnSelfTest
    ;
}
