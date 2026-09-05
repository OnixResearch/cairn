{
  root,
  packages ? [ ],
}:
let
  flake = builtins.getFlake root;
  system = "x86_64-linux";
  producer = flake.inputs.octet;
  runner = producer.packages.${system}.cargo-octet;
  workspace = flake.checks.${system}.octet;
  selected =
    if packages == [ ] then
      workspace
    else
      producer.lib.mkConsumerCheck {
        inherit system packages;
        src = flake.outPath;
        cargoLock = flake.outPath + "/Cargo.lock";
        cargoExtraArgs = "--all-targets --all-features";
        nativeBuildInputs = workspace.nativeBuildInputs;
      };
  original = "${runner}/bin/cargo-octet check";
  replacement = "${producer.outPath}/hooks/octet-deny-all.sh";
  command = builtins.replaceStrings [ original ] [ replacement ] selected.buildCommand;
in
assert command != selected.buildCommand;
selected.overrideAttrs (old: {
  nativeBuildInputs = old.nativeBuildInputs ++ [ runner ];
  OCTET_PRECOMMIT_USE_INSTALLED = "true";
  DYLINT_RUSTFLAGS = "-D warnings";
  buildCommand = command;
})
