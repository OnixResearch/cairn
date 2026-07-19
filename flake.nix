{
  description = "Rust/Nickel Cairn lifecycle engine";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.artifact-auth = {
    url = "git+ssh://git@github.com/OnixResearch/artifact-auth.git?rev=799459346d5416fbd7b9f55840a7371441b55afa";
    flake = false;
  };
  inputs.nickel-export = {
    url = "github:OnixResearch/nickel-export/257fafc1c746f1faf156207043a4c826bfb16d49";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  inputs.slotcar.url = "github:OnixResearch/slotcar";

  outputs =
    {
      self,
      nixpkgs,
      artifact-auth,
      nickel-export,
      slotcar,
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      eachSystem = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      rustCargoLock = {
        lockFile = ./Cargo.lock;
        allowBuiltinFetchGit = true;
        outputHashes = {
          "nickel-export-core-0.1.0" = "sha256-dV4+/jghpJ89F5DHprdjxyru8kllMAurS2DsBGn/ibA=";
          "rat-canvas-0.1.0" = "sha256-WHMtm38pQirmjZ/5Ua0unGhj4pIDaEixXG1pBUYWmTQ=";
          "rat-nodegraph-0.1.0" = "sha256-WHMtm38pQirmjZ/5Ua0unGhj4pIDaEixXG1pBUYWmTQ=";
        };
      };
      artifactAuthSourcePin = import ./nix/artifact-auth-source-pin.nix {
        lib = nixpkgs.lib;
      };
      artifactAuthSourcePinInput = {
        cargoWorkspace = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        coreManifest = builtins.fromTOML (builtins.readFile ./crates/cairn-core/Cargo.toml);
        cliManifest = builtins.fromTOML (builtins.readFile ./crates/cairn-cli/Cargo.toml);
        cargoLock = builtins.fromTOML (builtins.readFile ./Cargo.lock);
        flakeLock = builtins.fromJSON (builtins.readFile ./flake.lock);
        releaseProfile = builtins.fromJSON (builtins.readFile "${artifact-auth}/release/profile.json");
        standaloneWorkspaceManifest = builtins.fromTOML (builtins.readFile "${artifact-auth}/Cargo.toml");
        standaloneCoreManifest = builtins.fromTOML (
          builtins.readFile "${artifact-auth}/crates/artifact-auth-core/Cargo.toml"
        );
        standaloneEd25519Manifest = builtins.fromTOML (
          builtins.readFile "${artifact-auth}/crates/artifact-auth-ed25519/Cargo.toml"
        );
        resolvedNixRevision = artifact-auth.rev or null;
        resolvedNixNarHash = artifact-auth.narHash or null;
      };
    in
    {
      packages = eachSystem (pkgs: {
        cairn = pkgs.rustPlatform.buildRustPackage {
          pname = "cairn";
          version = "0.1.0";
          src = self;
          cargoLock = rustCargoLock;
          cargoBuildFlags = [
            "-p"
            "cairn-cli"
            "--features"
            "redb-registry"
          ];
          nativeBuildInputs = [ pkgs.makeWrapper ];
          postInstall = ''
            # r[impl cairn.supersede.policy_export_packaged_runtime.wrapper]
            wrapProgram $out/bin/cairn \
              --prefix PATH : ${
                pkgs.lib.makeBinPath [
                  pkgs.nickel
                  nickel-export.packages.${pkgs.stdenv.hostPlatform.system}.nickel-export
                ]
              }
            wrapProgram $out/bin/ironspec \
              --prefix PATH : ${
                pkgs.lib.makeBinPath [
                  pkgs.nickel
                  nickel-export.packages.${pkgs.stdenv.hostPlatform.system}.nickel-export
                ]
              }
          '';
          doCheck = false;
        };
        ironspec = self.packages.${pkgs.stdenv.hostPlatform.system}.cairn;
        default = self.packages.${pkgs.stdenv.hostPlatform.system}.cairn;
      });

      apps = eachSystem (pkgs: {
        cairn = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn";
          meta.description = "Run the cairn CLI";
        };
        ironspec = {
          type = "app";
          program = "${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/ironspec";
          meta.description = "Run the legacy ironspec compatibility alias for Cairn";
        };
        default = self.apps.${pkgs.stdenv.hostPlatform.system}.cairn;
      });

      checks = eachSystem (pkgs: {
        artifact-auth-source-pin =
          let
            validation = artifactAuthSourcePin.validate artifactAuthSourcePinInput;
            selfTestIssues = artifactAuthSourcePin.selfTest artifactAuthSourcePinInput;
            issues = validation.issues ++ selfTestIssues;
          in
          if issues != [ ] then
            throw "artifact-auth source pin invalid:\n${nixpkgs.lib.concatStringsSep "\n" issues}"
          else
            pkgs.runCommand "cairn-artifact-auth-source-pin" { } ''
              # r[verify cairn.artifact_auth.repository_extraction.cutover]
              printf '%s\n' '${validation.revision}' > "$out"
            '';

        # r[verify cairn.supersede.rust_nickel_only_gates.no_python_scripts]
        policy-fresh =
          pkgs.runCommand "cairn-policy-fresh"
            {
              nativeBuildInputs = [ pkgs.nickel ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn policy export --check
              touch $out
            '';

        authenticated-policy-profiles-fresh =
          pkgs.runCommand "cairn-authenticated-policy-profiles-fresh"
            {
              nativeBuildInputs = [ pkgs.nickel ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              # r[verify cairn.stack_provenance_gates.authenticated_inputs.policy_identity]
              nickel export --format json cairn-policy/stack-provenance-authentication-profile.ncl > stack-profile.json
              nickel export --format json cairn-policy/function-address-authentication-profile.ncl > function-profile.json
              cmp stack-profile.json cairn-policy/generated/stack-provenance-authentication-profile.json
              cmp function-profile.json cairn-policy/generated/function-address-authentication-profile.json
              touch $out
            '';

        policy-stale-detects-drift =
          pkgs.runCommand "cairn-policy-stale-detects-drift"
            {
              nativeBuildInputs = [ pkgs.nickel ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              # r[verify cairn.supersede.deterministic_gates.policy_freshness]
              substituteInPlace cairn-policy/default.ncl \
                --replace-fail 'name = "cairn-default"' 'name = "cairn-default-stale-probe"'
              if ${
                self.packages.${pkgs.stdenv.hostPlatform.system}.cairn
              }/bin/cairn policy export --check >policy-check.out 2>policy-check.err; then
                echo "expected stale generated policy check to fail" >&2
                exit 1
              fi
              grep -q "generated policy is stale" policy-check.err
              touch $out
            '';

        packaged-policy-export-runtime =
          pkgs.runCommand "cairn-packaged-policy-export-runtime"
            {
              nativeBuildInputs = [ ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              # r[verify cairn.supersede.policy_export_packaged_runtime.check]
              ${pkgs.coreutils}/bin/env -i \
                HOME="$TMPDIR" \
                PATH=${pkgs.coreutils}/bin \
                ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn \
                policy export --check >packaged-policy-export.out
              grep -q "policy fresh: cairn-policy/generated/cairn-policy.json" packaged-policy-export.out
              touch $out
            '';

        nickel-export-cutover =
          pkgs.runCommand "cairn-nickel-export-cutover"
            {
              nativeBuildInputs = [
                pkgs.nickel
                nickel-export.packages.${pkgs.stdenv.hostPlatform.system}.nickel-export
              ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              # r[verify cairn.nickel_export_cutover.source]
              grep -q '257fafc1c746f1faf156207043a4c826bfb16d49' Cargo.lock
              grep -q '257fafc1c746f1faf156207043a4c826bfb16d49' flake.lock
              command -v nickel-export >/dev/null
              # r[verify cairn.nickel_export_cutover.dual_run]
              ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn policy export --check
              ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn lifecycle check \
                --source fixtures/structured-lifecycle/positive-bundle.ncl > lifecycle-check.json
              grep -q '"verdict": "PASS"' lifecycle-check.json
              touch $out
            '';

        preserves-corpus-fresh =
          pkgs.runCommand "cairn-preserves-corpus-fresh"
            {
              nativeBuildInputs = [ ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              # r[impl cairn.determinism.preserves_corpus_ci_guard.clean_check]
              # r[verify cairn.determinism.preserves_corpus_ci_guard.clean_check]
              # r[verify cairn.determinism.preserves_corpus_ci_guard.validation]
              ${
                self.packages.${pkgs.stdenv.hostPlatform.system}.cairn
              }/bin/cairn preserves-corpus update --path fixtures/preserves-canonicalization/corpus.json >preserves-corpus-fresh.json
              grep -q '"valid": true' preserves-corpus-fresh.json
              grep -q '"stale_count": 0' preserves-corpus-fresh.json
              touch $out
            '';

        preserves-corpus-stale-detects-drift =
          pkgs.runCommand "cairn-preserves-corpus-stale-detects-drift"
            {
              nativeBuildInputs = [ ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              # r[impl cairn.determinism.preserves_corpus_ci_guard.stale_detection]
              # r[verify cairn.determinism.preserves_corpus_ci_guard.stale_detection]
              substituteInPlace fixtures/preserves-canonicalization/corpus.json \
                --replace-fail '4111d2d01cd6f342ffbf86955fa72cd2af2e4cbb38d4db96bb3c28f77eb9661a' \
                               '0000000000000000000000000000000000000000000000000000000000000000'
              if ${
                self.packages.${pkgs.stdenv.hostPlatform.system}.cairn
              }/bin/cairn preserves-corpus update --path fixtures/preserves-canonicalization/corpus.json >preserves-corpus-stale.json 2>preserves-corpus-stale.err; then
                echo "expected stale Preserves corpus check to fail" >&2
                exit 1
              fi
              grep -q '"valid": false' preserves-corpus-stale.json
              grep -q '"stale_count": 1' preserves-corpus-stale.json
              grep -q '"mutated": false' preserves-corpus-stale.json
              touch $out
            '';

        rustfmt =
          pkgs.runCommand "cairn-rustfmt-check"
            {
              nativeBuildInputs = [
                pkgs.cargo
                pkgs.rustfmt
              ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              cargo fmt --check
              touch $out
            '';

        rust-tests = pkgs.rustPlatform.buildRustPackage {
          pname = "cairn-tests";
          version = "0.1.0";
          src = self;
          cargoLock = rustCargoLock;
          cargoBuildFlags = [
            "--workspace"
            "--all-targets"
            "--features"
            "redb-registry"
          ];
          cargoTestFlags = [
            "--workspace"
            "--features"
            "redb-registry"
          ];
          nativeCheckInputs = [
            pkgs.git
            pkgs.nickel
          ];
          preCheck = ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn policy export --check
          '';
        };

        cairn-validate =
          pkgs.runCommand "cairn-validate"
            {
              nativeBuildInputs = [ ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              # r[verify cairn.supersede.repo_native_labels.check]
              ${
                self.packages.${pkgs.stdenv.hostPlatform.system}.cairn
              }/bin/cairn validate --root . --strict > /tmp/cairn-validate.json
              touch $out
            '';

        tracey-coverage =
          pkgs.runCommand "cairn-traceability-coverage"
            {
              nativeBuildInputs = [ ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn traceability coverage --root .
              touch $out
            '';

        rename-classification =
          pkgs.runCommand "cairn-rename-classification"
            {
              nativeBuildInputs = [ ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              ${
                self.packages.${pkgs.stdenv.hostPlatform.system}.cairn
              }/bin/cairn rename-classification >/tmp/cairn-rename-classification.json
              touch $out
            '';

        self-dogfood = pkgs.rustPlatform.buildRustPackage {
          pname = "cairn-self-dogfood";
          version = "0.1.0";
          src = self;
          cargoLock = rustCargoLock;
          cargoBuildFlags = [
            "-p"
            "cairn-cli"
            "--features"
            "redb-registry"
          ];
          nativeCheckInputs = [ pkgs.nickel ];
          doCheck = true;
          checkPhase = ''
            runHook preCheck
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn policy export --check
            cairn=target/${pkgs.stdenv.hostPlatform.config}/release/cairn
            $cairn policy >/tmp/cairn-policy.json
            scratch=$(mktemp -d)
            $cairn init --root "$scratch" >/tmp/cairn-init.json
            $cairn change create native-lifecycle --root "$scratch" >/tmp/cairn-change-create.json
            $cairn change list --root "$scratch" >/tmp/cairn-change-list.json
            $cairn change show native-lifecycle --root "$scratch" >/tmp/cairn-change-show.json
            $cairn status --root "$scratch" >/tmp/cairn-scratch-status.json
            # r[verify cairn.supersede.repo_native_only.doctor] r[verify cairn.supersede.repo_native_verification.import]
            $cairn doctor --root . >/tmp/cairn-doctor.json
            $cairn status --root . >/tmp/cairn-status.json
            $cairn validate --root . --strict >/tmp/cairn-validate.json
            $cairn status --root fixtures/basic >/tmp/cairn-fixture-status.json
            registry_store=$(mktemp --suffix=.redb)
            rm -f "$registry_store"
            $cairn registry import --scope workspace:self-dogfood --adapter redb.v1 --store "$registry_store" --ingest cairn.canonical-records.v1 --input fixtures/global-registry/canonical-records.json --execute >/tmp/cairn-registry-import.json
            $cairn registry query --scope workspace:self-dogfood --adapter redb.v1 --registry "$registry_store" --tag operator:channel=dev >/tmp/cairn-registry-query.json
            $cairn registry graph --scope workspace:self-dogfood --adapter redb.v1 --registry "$registry_store" --profile change-dependencies >/tmp/cairn-registry-graph.json
            $cairn registry export --scope workspace:self-dogfood --adapter redb.v1 --registry "$registry_store" --output /tmp/cairn-registry-export.json --execute >/tmp/cairn-registry-export-receipt.json
            $cairn registry verify --registry /tmp/cairn-registry-export.json >/tmp/cairn-registry-verify.json
            legacy=$(mktemp -d)
            cp -R fixtures/basic/. "$legacy"/
            rm -rf "$legacy/cairn"
            $cairn status --root "$legacy" >/tmp/cairn-legacy-status.json
            legacy_ironspec=$(mktemp -d)
            cp -R fixtures/basic/. "$legacy_ironspec"/
            mv "$legacy_ironspec/cairn" "$legacy_ironspec/ironspec"
            $cairn status --root "$legacy_ironspec" >/tmp/cairn-legacy-ironspec-status.json
            $cairn migrate ironspec --root "$legacy_ironspec" >/tmp/cairn-migrate-ironspec-plan.json
            $cairn migrate ironspec --root "$legacy_ironspec" --execute >/tmp/cairn-migrate-ironspec-execute.json
            $cairn migrate openspec --root "$legacy" >/tmp/cairn-migrate-plan.json
            $cairn migrate openspec --root "$legacy" --execute >/tmp/cairn-migrate-execute.json
            $cairn status --root "$legacy" >/tmp/cairn-native-status.json
            $cairn change list --root fixtures/basic >/tmp/cairn-fixture-change-list.json
            $cairn change show demo --root fixtures/basic >/tmp/cairn-fixture-change-show.json
            $cairn spec list --root fixtures/basic >/tmp/cairn-fixture-spec-list.json
            $cairn spec show demo --root fixtures/basic >/tmp/cairn-fixture-spec-show.json
            $cairn sync demo --root fixtures/basic >/tmp/cairn-fixture-sync.json
            $cairn archive demo --root fixtures/basic >/tmp/cairn-fixture-archive.json
            $cairn archive ready --root fixtures/archive-ready >/tmp/cairn-archive-ready.json
            $cairn sync no-delta --root fixtures/no-delta >/tmp/cairn-no-delta-sync.json
            $cairn archive no-delta --root fixtures/no-delta >/tmp/cairn-no-delta-archive.json
            $cairn sync missing --root fixtures/missing-artifacts >/tmp/cairn-missing-artifacts-sync.json
            runHook postCheck
          '';
        };

        clippy = pkgs.rustPlatform.buildRustPackage {
          pname = "cairn-clippy";
          version = "0.1.0";
          src = self;
          cargoLock = rustCargoLock;
          nativeBuildInputs = [ pkgs.clippy ];
          buildPhase = ''
            runHook preBuild
            cargo clippy --workspace --all-targets --features redb-registry -- -D warnings
            runHook postBuild
          '';
          installPhase = ''
            runHook preInstall
            touch $out
            runHook postInstall
          '';
          doCheck = false;
        };

        slotcar = slotcar.lib.mkConsumerCheck {
          system = pkgs.stdenv.hostPlatform.system;
          src = self;
          cargoLock = ./Cargo.lock;
          cargoExtraArgs = "--all-targets --features redb-registry";
          nativeBuildInputs = [ pkgs.stdenv.cc ];
        };
      });

      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.rustc
            pkgs.rustfmt
            pkgs.clippy
            pkgs.nickel
            nickel-export.packages.${pkgs.stdenv.hostPlatform.system}.nickel-export
            slotcar.packages.${pkgs.stdenv.hostPlatform.system}.cargo-tigerstyle
          ];
        };
      });
    };
}
