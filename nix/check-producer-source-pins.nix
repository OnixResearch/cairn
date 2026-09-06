{ lib }:
let
  expected = {
    bounded-exec = "29dac88ecded94457572db3fdfaaaab95fa91525";
    durable-file-publication = "951c27f59003cea9bfdb40ed4d89653d50fada1f";
    mantle-build-contract = "0bbab77e23ab95caa4e4184b9945cfaa5c8040e2";
  };
  validate = { workspace, lock, inputs }:
    lib.all (name:
      let
        revision = expected.${name};
        dependency = workspace.workspace.dependencies.${name};
        packages = builtins.filter (package: package.name == name) lock.package;
      in dependency.rev == revision
        && inputs.${name}.rev == revision
        && builtins.length packages == 1
        && (builtins.head packages).source == "git+${dependency.git}?rev=${revision}#${revision}"
    ) (builtins.attrNames expected);
in { inherit expected validate; }
