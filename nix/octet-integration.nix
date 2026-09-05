# r[verify cairn.octet_integration.canonical] r[verify cairn.octet_integration.evidence]
{
  inputs,
  system,
  root,
  checks,
  devShell,
}:
let
  expectedSource = {
    owner = "OnixResearch";
    repo = "octet";
    rev = "2f8213a88781d9d38bb81319d06a50805d48ccd3";
    narHash = "sha256-OvqB00mDIsGDOcuK5jvPJUModVz9WbIBuM/l06TABHc=";
  };
  expectedScope = [ "--workspace" ];
  expectedCargoArgs = [ "--all-targets" ];
  expectedCheckTail = " -- --all-targets --features redb-registry";
  legacyName = "slotcar";
  legacyRunner = "cargo-tigerstyle";
  driftRevision = "0000000000000000000000000000000000000000";
  flakeLock = builtins.fromJSON (builtins.readFile "${root}/flake.lock");
  sourceNode = flakeLock.nodes.${flakeLock.nodes.root.inputs.octet};
  workspace = (builtins.fromTOML (builtins.readFile "${root}/Cargo.toml")).workspace;
  lintConfig = builtins.fromTOML (builtins.readFile "${root}/dylint.toml");
  observed = {
    inputNames = builtins.attrNames inputs;
    checkNames = builtins.attrNames checks;
    source = {
      inherit (sourceNode.locked)
        owner
        repo
        rev
        narHash
        ;
    };
    originalRevision = sourceNode.original.rev or null;
    resolvedRevision = inputs.octet.rev;
    resolvedNarHash = inputs.octet.narHash;
    metadataNames = builtins.attrNames workspace.metadata;
    lintConfigNames = builtins.attrNames lintConfig;
    scope = workspace.metadata.octet.default_scope or [ ];
    cargoArgs = workspace.metadata.octet.cargo_check_args or [ ];
    hasRunner = builtins.elem (toString inputs.octet.packages.${system}.cargo-octet) (
      map toString devShell.nativeBuildInputs
    );
    flakeSource = builtins.readFile "${root}/flake.nix";
    checkCommand = checks.octet.buildCommand;
  };
  contains = text: value: builtins.match ".*${text}.*" value != null;
  valid =
    value:
    builtins.elem "octet" value.inputNames
    && !(builtins.elem legacyName value.inputNames)
    && builtins.elem "octet" value.checkNames
    && !(builtins.elem legacyName value.checkNames)
    && value.source == expectedSource
    && value.originalRevision == expectedSource.rev
    && value.resolvedRevision == expectedSource.rev
    && value.resolvedNarHash == expectedSource.narHash
    && builtins.elem "octet" value.metadataNames
    && !(builtins.elem legacyName value.metadataNames)
    && builtins.elem "octet" value.lintConfigNames
    && !(builtins.elem legacyName value.lintConfigNames)
    && value.scope == expectedScope
    && value.cargoArgs == expectedCargoArgs
    && value.hasRunner
    && !(contains legacyName value.flakeSource)
    && !(contains legacyRunner value.flakeSource)
    && contains "/bin/cargo-octet check" value.checkCommand
    && contains expectedCheckTail value.checkCommand;
  negativeCases = {
    legacy-input = observed // {
      inputNames = [ legacyName ];
    };
    legacy-check = observed // {
      checkNames = [ legacyName ];
    };
    legacy-repository = observed // {
      source = observed.source // {
        repo = legacyName;
      };
    };
    source-drift = observed // {
      source = observed.source // {
        rev = driftRevision;
      };
    };
    nar-drift = observed // {
      source = observed.source // {
        narHash = "changed-source";
      };
    };
    floating-input = observed // {
      originalRevision = null;
    };
    resolved-source-drift = observed // {
      resolvedRevision = driftRevision;
    };
    resolved-nar-drift = observed // {
      resolvedNarHash = "changed-source";
    };
    legacy-metadata = observed // {
      metadataNames = [ legacyName ];
    };
    legacy-lint-config = observed // {
      lintConfigNames = [ legacyName ];
    };
    missing-scope = observed // {
      scope = [ ];
    };
    missing-target-scope = observed // {
      cargoArgs = [ ];
    };
    missing-runner = observed // {
      hasRunner = false;
    };
    legacy-selector = observed // {
      flakeSource = "${legacyName}.packages";
    };
    legacy-runner-selector = observed // {
      flakeSource = "octet.packages.${legacyRunner}";
    };
    missing-check-command = observed // {
      checkCommand = "";
    };
    narrowed-check = observed // {
      checkCommand = "/bin/cargo-octet check -- --lib";
    };
  };
  negativeResults = builtins.mapAttrs (_: value: !(valid value)) negativeCases;
  positivePassed = valid observed;
  negativePassed = builtins.all (result: result) (builtins.attrValues negativeResults);
in
{
  valid = positivePassed && negativePassed;
  positive = positivePassed;
  negative = negativeResults;
  source = expectedSource;
  non_claim = "These controls verify the consumer integration, not Octet execution or Rust correctness.";
}
