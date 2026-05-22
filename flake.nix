{
  description = "Rust/Nickel Cairn lifecycle engine";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.slotcar.url = "github:OnixResearch/slotcar";

  outputs = { self, nixpkgs, slotcar }:
    let
      systems = [ "x86_64-linux" "aarch64-linux" ];
      eachSystem = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
    in {
      packages = eachSystem (pkgs: {
        cairn = pkgs.rustPlatform.buildRustPackage {
          pname = "cairn";
          version = "0.1.0";
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [ "-p" "cairn-cli" ];
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
        # r[verify cairn.supersede.rust_nickel_only_gates.no_python_scripts]
        policy-fresh = pkgs.runCommand "cairn-policy-fresh" {
          nativeBuildInputs = [ pkgs.nickel ];
        } ''
          cp -r ${self} source
          chmod -R u+w source
          cd source
          ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn policy export --check
          touch $out
        '';

        policy-stale-detects-drift = pkgs.runCommand "cairn-policy-stale-detects-drift" {
          nativeBuildInputs = [ pkgs.nickel ];
        } ''
          cp -r ${self} source
          chmod -R u+w source
          cd source
          # r[verify cairn.supersede.deterministic_gates.policy_freshness]
          substituteInPlace cairn-policy/default.ncl \
            --replace-fail 'name = "cairn-default"' 'name = "cairn-default-stale-probe"'
          if ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn policy export --check >policy-check.out 2>policy-check.err; then
            echo "expected stale generated policy check to fail" >&2
            exit 1
          fi
          grep -q "generated policy is stale" policy-check.err
          touch $out
        '';

        rustfmt = pkgs.runCommand "cairn-rustfmt-check" {
          nativeBuildInputs = [ pkgs.cargo pkgs.rustfmt ];
        } ''
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
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [ "--workspace" "--all-targets" ];
          cargoTestFlags = [ "--workspace" ];
          nativeCheckInputs = [ pkgs.git pkgs.nickel ];
          preCheck = ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn policy export --check
          '';
        };

        cairn-validate = pkgs.runCommand "cairn-validate" {
          nativeBuildInputs = [ ];
        } ''
          cp -r ${self} source
          chmod -R u+w source
          cd source
          # r[verify cairn.supersede.repo_native_labels.check]
          ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn validate --root . --strict > /tmp/cairn-validate.json
          touch $out
        '';

        tracey-coverage = pkgs.runCommand "cairn-tracey-coverage" {
          nativeBuildInputs = [ ];
        } ''
          cp -r ${self} source
          chmod -R u+w source
          cd source
          ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn tracey coverage --root .
          test -f .config/tracey/config.styx
          touch $out
        '';

        rename-classification = pkgs.runCommand "cairn-rename-classification" {
          nativeBuildInputs = [ ];
        } ''
          cp -r ${self} source
          chmod -R u+w source
          cd source
          ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn rename-classification >/tmp/cairn-rename-classification.json
          touch $out
        '';

        self-dogfood = pkgs.rustPlatform.buildRustPackage {
          pname = "cairn-self-dogfood";
          version = "0.1.0";
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [ "-p" "cairn-cli" ];
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
          cargoLock.lockFile = ./Cargo.lock;
          nativeBuildInputs = [ pkgs.clippy ];
          buildPhase = ''
            runHook preBuild
            cargo clippy --workspace --all-targets -- -D warnings
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
          cargoExtraArgs = "--all-targets";
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
            slotcar.packages.${pkgs.stdenv.hostPlatform.system}.cargo-tigerstyle
          ];
        };
      });
    };
}
