{
  description = "Rust/Nickel OpenSpec engine";

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
        policy-fresh = pkgs.runCommand "cairn-policy-fresh" {
          nativeBuildInputs = [ pkgs.nickel pkgs.python3 ];
        } ''
          cp -r ${self} source
          chmod -R u+w source
          cd source
          python3 tools/export-policy.py --check
          touch $out
        '';

        policy-stale-detects-drift = pkgs.runCommand "cairn-policy-stale-detects-drift" {
          nativeBuildInputs = [ pkgs.nickel pkgs.python3 ];
        } ''
          cp -r ${self} source
          chmod -R u+w source
          cd source
          # r[verify cairn.supersede.deterministic_gates.policy_freshness]
          python3 - <<'PY'
from pathlib import Path
path = Path("cairn-policy/default.ncl")
text = path.read_text()
path.write_text(text.replace('name = "cairn-default"', 'name = "cairn-default-stale-probe"'))
PY
          if python3 tools/export-policy.py --check >policy-check.out 2>policy-check.err; then
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
          nativeCheckInputs = [ pkgs.nickel pkgs.python3 ];
          preCheck = ''
            python3 tools/export-policy.py --check
          '';
        };

        openspec-validate = pkgs.runCommand "cairn-openspec-validate" {
          nativeBuildInputs = [ pkgs.python3 ];
        } ''
          cp -r ${self} source
          chmod -R u+w source
          cd source
          ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn validate --root . > /tmp/cairn-openspec-validate.json
          python3 -c 'import json; data=json.load(open("/tmp/cairn-openspec-validate.json")); assert data["valid"] is True, data; assert data["issues"] == [], data; assert data["spec_issues"] == [], data'
          touch $out
        '';

        tracey-coverage = pkgs.runCommand "cairn-tracey-coverage" {
          nativeBuildInputs = [ pkgs.python3 ];
        } ''
          cp -r ${self} source
          chmod -R u+w source
          cd source
          python3 tools/check-tracey-coverage.py --root .
          test -f .config/tracey/config.styx
          touch $out
        '';

        rename-classification = pkgs.runCommand "cairn-rename-classification" {
          nativeBuildInputs = [ pkgs.python3 ];
        } ''
          cp -r ${self} source
          chmod -R u+w source
          cd source
          python3 tools/check-rename-classification.py >/tmp/cairn-rename-classification.json
          python3 -c 'import json; data=json.load(open("/tmp/cairn-rename-classification.json")); assert data["valid"] is True, data; assert data["buckets"].get("needs-fix", 0) == 0, data'
          touch $out
        '';

        self-dogfood = pkgs.rustPlatform.buildRustPackage {
          pname = "cairn-self-dogfood";
          version = "0.1.0";
          src = self;
          cargoLock.lockFile = ./Cargo.lock;
          cargoBuildFlags = [ "-p" "cairn-cli" ];
          nativeCheckInputs = [ pkgs.nickel pkgs.python3 ];
          doCheck = true;
          checkPhase = ''
            runHook preCheck
            python3 tools/export-policy.py --check
            cairn=target/${pkgs.stdenv.hostPlatform.config}/release/cairn
            $cairn policy >/tmp/cairn-policy.json
            scratch=$(mktemp -d)
            $cairn init --root "$scratch" >/tmp/cairn-init.json
            $cairn change create replace-openspec --root "$scratch" >/tmp/cairn-change-create.json
            $cairn change list --root "$scratch" >/tmp/cairn-change-list.json
            $cairn change show replace-openspec --root "$scratch" >/tmp/cairn-change-show.json
            $cairn status --root "$scratch" >/tmp/cairn-scratch-status.json
            $cairn status --root . >/tmp/cairn-status.json
            $cairn validate --root . >/tmp/cairn-validate.json
            python3 -c 'import json; data=json.load(open("/tmp/cairn-validate.json")); assert data["valid"] is True, data; assert data["spec_issues"] == [], data'
            $cairn status --root fixtures/basic >/tmp/cairn-fixture-status.json
            python3 -c 'import json; data=json.load(open("/tmp/cairn-fixture-status.json")); assert data["layout"] == "cairn", data'
            legacy=$(mktemp -d)
            cp -R fixtures/basic/. "$legacy"/
            rm -rf "$legacy/cairn"
            $cairn status --root "$legacy" >/tmp/cairn-legacy-status.json
            python3 -c 'import json; data=json.load(open("/tmp/cairn-legacy-status.json")); assert data["layout"] == "cairn", data; assert data["specs"] == [], data; assert data["changes"] == [], data'
            legacy_ironspec=$(mktemp -d)
            cp -R fixtures/basic/. "$legacy_ironspec"/
            mv "$legacy_ironspec/cairn" "$legacy_ironspec/ironspec"
            $cairn status --root "$legacy_ironspec" >/tmp/cairn-legacy-ironspec-status.json
            python3 -c 'import json; data=json.load(open("/tmp/cairn-legacy-ironspec-status.json")); assert data["layout"] == "cairn", data; assert data["specs"] == [], data; assert data["changes"] == [], data'
            $cairn migrate ironspec --root "$legacy_ironspec" >/tmp/cairn-migrate-ironspec-plan.json
            python3 -c 'import json; data=json.load(open("/tmp/cairn-migrate-ironspec-plan.json")); assert data["source_layout"] == "ironspec", data; assert data["destination_layout"] == "cairn", data; assert data["dry_run"] is True, data; assert data["blocked"] is False, data; assert data["mutated"] is False, data; assert len(data["actions"]) > 0, data'
            $cairn migrate ironspec --root "$legacy_ironspec" --execute >/tmp/cairn-migrate-ironspec-execute.json
            python3 -c 'import json; data=json.load(open("/tmp/cairn-migrate-ironspec-execute.json")); assert data["dry_run"] is False, data; assert data["blocked"] is False, data; assert data["mutated"] is True, data'
            $cairn migrate openspec --root "$legacy" >/tmp/cairn-migrate-plan.json
            python3 -c 'import json; data=json.load(open("/tmp/cairn-migrate-plan.json")); assert data["dry_run"] is True, data; assert data["blocked"] is False, data; assert data["mutated"] is False, data; assert len(data["actions"]) > 0, data'
            $cairn migrate openspec --root "$legacy" --execute >/tmp/cairn-migrate-execute.json
            python3 -c 'import json; data=json.load(open("/tmp/cairn-migrate-execute.json")); assert data["dry_run"] is False, data; assert data["blocked"] is False, data; assert data["mutated"] is True, data'
            $cairn status --root "$legacy" >/tmp/cairn-native-status.json
            python3 -c 'import json; data=json.load(open("/tmp/cairn-native-status.json")); assert data["layout"] == "cairn", data'
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
            pkgs.python3
            slotcar.packages.${pkgs.stdenv.hostPlatform.system}.cargo-tigerstyle
          ];
        };
      });
    };
}
