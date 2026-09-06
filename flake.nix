{
  description = "Rust/Nickel Cairn lifecycle engine";

  inputs.nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  inputs.artifact = {
    url = "git+ssh://git@github.com/OnixResearch/onix-artifact.git?rev=e41340bec587b6d049b5cc518ec7db925dde84be";
    flake = false;
  };
  inputs.nickel-export = {
    url = "github:OnixResearch/nickel-export/257fafc1c746f1faf156207043a4c826bfb16d49";
    inputs.nixpkgs.follows = "nixpkgs";
  };
  # Exact stack Nickel 1.17.0 cohort, pinned to the reviewed upstream commit.
  # r[impl cairn.nickel_toolchain.cohort] r[impl cairn.nickel_toolchain.lockfile]
  inputs.nickel-1-17-0.url = "github:nickel-lang/nickel/1320a983e6c3d1e2fb53dd2464b084b4903b1426";
  inputs.slotcar.url = "github:OnixResearch/slotcar";
  inputs.kiln-core = {
    url = "git+https://seed.radicle.garden/z2wsvXm5S2sJGvuV1k5JHiwi1PbKE.git?rev=5d99f4e9bb954743443e340baece31256b4c53f1";
    flake = false;
  };
  inputs.mantle-build-contract = {
    url = "git+ssh://git@github.com/OnixResearch/mantle.git?rev=0bbab77e23ab95caa4e4184b9945cfaa5c8040e2";
    flake = false;
  };
  inputs.bounded-exec = {
    url = "git+https://git.onix.computer/z2CpqLFpdP36fZXYUK5ZNWxMibpCo.git?rev=29dac88ecded94457572db3fdfaaaab95fa91525";
    flake = false;
  };
  inputs.cap-root = {
    url = "git+https://seed.radicle.garden/z3cWBCK7VPbEmbL5MAvKJWKjCE745.git?rev=8991e2572578c16e590c2f1c0915db6c46c5d9cb";
    flake = false;
  };
  inputs.durable-file-publication = {
    url = "git+https://seed.radicle.garden/z3tAR4For7qw8ZirkJzoDw1VNDDLM.git?rev=951c27f59003cea9bfdb40ed4d89653d50fada1f";
    flake = false;
  };

  outputs =
    {
      self,
      nixpkgs,
      artifact,
      nickel-export,
      nickel-1-17-0,
      slotcar,
      cap-root,
      durable-file-publication,
      ...
    }:
    let
      systems = [
        "x86_64-linux"
        "aarch64-linux"
      ];
      eachSystem = f: nixpkgs.lib.genAttrs systems (system: f nixpkgs.legacyPackages.${system});
      nickelCohort = system: nickel-1-17-0.packages.${system}.default;
      rustCargoLock = {
        lockFile = ./Cargo.lock;
        allowBuiltinFetchGit = true;
        outputHashes = {
          "cap-root-core-0.1.0" = "sha256-2eiyxmhW8/qj30ALccgkt5hatRMk2OcazmU3SCI/Opo=";
          "durable-file-publication-0.1.0" = "sha256-fKxZ+3rzWzVuawILnmpiYCCe8PT/z6usopFMbr2KFbI=";
          "nickel-export-core-0.1.0" = "sha256-dV4+/jghpJ89F5DHprdjxyru8kllMAurS2DsBGn/ibA=";
          "rat-canvas-0.1.0" = "sha256-WHMtm38pQirmjZ/5Ua0unGhj4pIDaEixXG1pBUYWmTQ=";
          "rat-nodegraph-0.1.0" = "sha256-WHMtm38pQirmjZ/5Ua0unGhj4pIDaEixXG1pBUYWmTQ=";
        };
      };
      artifactAuthSourcePin = import ./nix/artifact-auth-source-pin.nix {
        lib = nixpkgs.lib;
      };
      artifactAuthSourceHashes = {
        core = "839ea3e35429e2a1252020dd779b46ebd185b6911674036820da118d487d1d21";
        ed25519 = "7fd3987fe3ddc6d30efca37878952135f19905a00b52f349db237054be6b3633";
      };
      artifactAuthSourcePinInput = {
        cargoWorkspace = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        coreManifest = builtins.fromTOML (builtins.readFile ./crates/cairn-core/Cargo.toml);
        cliManifest = builtins.fromTOML (builtins.readFile ./crates/cairn-cli/Cargo.toml);
        cargoLock = builtins.fromTOML (builtins.readFile ./Cargo.lock);
        flakeLock = builtins.fromJSON (builtins.readFile ./flake.lock);
        releaseProfile = builtins.fromJSON (builtins.readFile "${artifact}/release/profile.json");
        standaloneWorkspaceManifest = builtins.fromTOML (builtins.readFile "${artifact}/Cargo.toml");
        standaloneCoreManifest = builtins.fromTOML (
          builtins.readFile "${artifact}/crates/artifact-auth-core/Cargo.toml"
        );
        standaloneEd25519Manifest = builtins.fromTOML (
          builtins.readFile "${artifact}/crates/artifact-auth-ed25519/Cargo.toml"
        );
        resolvedNixRevision = artifact.rev or null;
        resolvedNixNarHash = artifact.narHash or null;
      };
    in
    {
      packages = eachSystem (pkgs: {
        nickel-1-17-0 = nickelCohort pkgs.stdenv.hostPlatform.system;
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
            install -Dm644 ${cap-root}/LICENSE $out/share/licenses/cairn/cap-root-LICENSE
            install -Dm644 ${durable-file-publication}/LICENSE-MIT $out/share/licenses/cairn/durable-file-publication-MIT
            install -Dm644 ${durable-file-publication}/LICENSE-APACHE $out/share/licenses/cairn/durable-file-publication-APACHE
            install -Dm644 ${durable-file-publication}/NOTICE $out/share/licenses/cairn/durable-file-publication/NOTICE
            install -Dm644 ${durable-file-publication}/provenance/SOURCE.md $out/share/licenses/cairn/durable-file-publication/provenance/SOURCE.md
            install -Dm644 ${durable-file-publication}/provenance/source.ncl $out/share/licenses/cairn/durable-file-publication/provenance/source.ncl
            install -Dm644 ${durable-file-publication}/provenance/source.json $out/share/licenses/cairn/durable-file-publication/provenance/source.json
            # r[impl cairn.supersede.policy_export_packaged_runtime.wrapper]
            wrapProgram $out/bin/cairn \
              --prefix PATH : ${
                pkgs.lib.makeBinPath [
                  (nickelCohort pkgs.stdenv.hostPlatform.system)
                  nickel-export.packages.${pkgs.stdenv.hostPlatform.system}.nickel-export
                ]
              }
            wrapProgram $out/bin/ironspec \
              --prefix PATH : ${
                pkgs.lib.makeBinPath [
                  (nickelCohort pkgs.stdenv.hostPlatform.system)
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
        check-producer-source-pins =
          let
            checker = import ./nix/check-producer-source-pins.nix { lib = pkgs.lib; };
            facts = {
              workspace = builtins.fromTOML (builtins.readFile ./Cargo.toml);
              lock = builtins.fromTOML (builtins.readFile ./Cargo.lock);
              inputs = self.inputs;
            };
            changed = facts // {
              inputs = facts.inputs // {
                bounded-exec = {
                  rev = "substituted";
                };
              };
            };
            mantle = self.inputs.mantle-build-contract;
            packaging = {
              workspace = builtins.fromTOML (builtins.readFile "${mantle}/Cargo.toml");
              manifest = builtins.fromTOML (
                builtins.readFile "${mantle}/crates/mantle-build-contract/Cargo.toml"
              );
            };
            vendor = import ./nix/producer-cargo-vendor.nix {
              inherit pkgs;
              crane = slotcar.inputs.crane;
              mantleSource = mantle;
              cargoLock = ./Cargo.lock;
            };
          in
          assert checker.validate facts;
          assert !(checker.validate changed);
          assert
            !(checker.validate (
              facts
              // {
                inputs = facts.inputs // {
                  mantle-build-contract = {
                    rev = "substituted";
                  };
                };
              }
            ));
          assert checker.validateMantlePackaging packaging;
          assert
            !(checker.validateMantlePackaging (
              packaging
              // {
                workspace.workspace.package = packaging.workspace.workspace.package // {
                  license = "substituted";
                };
              }
            ));
          pkgs.runCommand "cairn-check-producer-source-pins" { } ''
            test -f ${vendor}/config.toml
            touch "$out"
          '';
        # r[verify cairn.consumer_bootstrap.boundary]
        bootstrap-source-pin =
          let
            contract = import ./nix/bootstrap-source-pin.nix { lib = nixpkgs.lib; };
            input = {
              workspace = builtins.fromTOML (builtins.readFile ./Cargo.toml);
              cli = builtins.fromTOML (builtins.readFile ./crates/cairn-cli/Cargo.toml);
              cargoLock = builtins.fromTOML (builtins.readFile ./Cargo.lock);
              flakeLock = builtins.fromJSON (builtins.readFile ./flake.lock);
              resolved = { inherit cap-root durable-file-publication; };
              outputHashes = rustCargoLock.outputHashes;
            };
            issues = contract.validate input ++ contract.selfTest input;
          in
          if issues != [ ] then
            throw "bootstrap source admission failed: ${nixpkgs.lib.concatStringsSep ", " issues}"
          else
            pkgs.runCommand "cairn-bootstrap-source-pin" { } ''
              test -s ${cap-root}/LICENSE
              test -s ${durable-file-publication}/LICENSE-MIT
              test -s ${durable-file-publication}/LICENSE-APACHE
              test -s ${durable-file-publication}/NOTICE
              test -s ${durable-file-publication}/provenance/SOURCE.md
              touch $out
            '';

        artifact-auth-source-pin =
          let
            validation = artifactAuthSourcePin.validate artifactAuthSourcePinInput;
            selfTestIssues = artifactAuthSourcePin.selfTest artifactAuthSourcePinInput;
            issues = validation.issues ++ selfTestIssues;
          in
          if issues != [ ] then
            throw "artifact-auth source pin invalid:\n${nixpkgs.lib.concatStringsSep "\n" issues}"
          else
            pkgs.runCommand "cairn-artifact-auth-source-pin"
              {
                nativeBuildInputs = [ pkgs.b3sum ];
              }
              ''
                # r[verify cairn.artifact_auth.repository_extraction.cutover]
                core_hash="$(b3sum ${artifact}/crates/artifact-auth-core/src/lib.rs | cut -d ' ' -f 1)"
                ed25519_hash="$(b3sum ${artifact}/crates/artifact-auth-ed25519/src/lib.rs | cut -d ' ' -f 1)"
                test "$core_hash" = '${artifactAuthSourceHashes.core}'
                test "$ed25519_hash" = '${artifactAuthSourceHashes.ed25519}'

                cp ${artifact}/crates/artifact-auth-core/src/lib.rs "$TMPDIR/core.rs"
                chmod u+w "$TMPDIR/core.rs"
                printf '\n' >> "$TMPDIR/core.rs"
                mutated_core_hash="$(b3sum "$TMPDIR/core.rs" | cut -d ' ' -f 1)"
                test "$mutated_core_hash" != '${artifactAuthSourceHashes.core}'

                printf '%s\n' '${validation.revision}' > "$out"
              '';

        # r[impl cairn.nickel_toolchain.cohort]
        # r[verify cairn.nickel_toolchain.cohort]
        nickel-cohort-exact =
          let
            exactNickel = nickelCohort pkgs.stdenv.hostPlatform.system;
          in
          pkgs.runCommand "cairn-nickel-cohort-exact"
            {
              nativeBuildInputs = [ exactNickel ];
            }
            ''
              test '${nickel-1-17-0.rev}' = '1320a983e6c3d1e2fb53dd2464b084b4903b1426'
              test "$(command -v nickel)" = '${exactNickel}/bin/nickel'
              test "$(nickel --version)" = 'nickel-lang-cli nickel 1.17.0 (rev 1320a98)'
              if grep -q 'pkgs\.nickel' ${self}/flake.nix; then
                echo 'older, floating, ambient, or mixed Nickel evaluator reference found' >&2
                exit 1
              fi
              touch $out
            '';

        # r[impl cairn.nickel_toolchain.compatibility]
        # r[verify cairn.nickel_toolchain.compatibility]
        nickel-toolchain-fixtures =
          pkgs.runCommand "cairn-nickel-toolchain-fixtures"
            {
              nativeBuildInputs = [ (nickelCohort pkgs.stdenv.hostPlatform.system) ];
            }
            ''
              fixtures=${self}/fixtures/nickel-toolchain
              for fixture in "$fixtures"/positive/*.ncl; do
                nickel typecheck "$fixture"
                nickel export --format json "$fixture" > /dev/null
              done

              for fixture in \
                "$fixtures"/negative/malformed.ncl \
                "$fixtures"/negative/unknown-field.ncl \
                "$fixtures"/negative/missing-import.ncl \
                "$fixtures"/negative/contract-violation.ncl \
                "$fixtures"/negative/type-mismatch.ncl
              do
                if nickel eval "$fixture" >negative.out 2>negative.err; then
                  echo "negative Nickel fixture unexpectedly passed: $fixture" >&2
                  exit 1
                fi
                test -s negative.err
              done

              nickel export --format json "$fixtures"/negative/stale.ncl > stale.json
              if cmp -s stale.json "$fixtures"/negative/stale.expected.json; then
                echo 'stale Nickel fixture unexpectedly matched current cohort output' >&2
                exit 1
              fi
              touch $out
            '';

        # r[impl cairn.nickel_toolchain.runtime_boundary]
        # r[verify cairn.nickel_toolchain.runtime_boundary]
        runtime-generated-policy-only =
          pkgs.runCommand "cairn-runtime-generated-policy-only"
            {
              nativeBuildInputs = [ ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              poison="$TMPDIR/poison"
              sentinel="$TMPDIR/nickel-started"
              mkdir -p "$poison"
              cat > "$poison/nickel" <<EOF
              #!${pkgs.runtimeShell}
              touch "$sentinel"
              exit 1
              EOF
              chmod +x "$poison/nickel"
              ${pkgs.coreutils}/bin/env -i \
                HOME="$TMPDIR" \
                PATH="$poison" \
                ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/.cairn-wrapped \
                status --root fixtures/basic > runtime-status.json
              grep -q '"valid": true' runtime-status.json
              test ! -e "$sentinel"
              touch $out
            '';

        # r[verify cairn.supersede.rust_nickel_only_gates.no_python_scripts]
        policy-fresh =
          pkgs.runCommand "cairn-policy-fresh"
            {
              nativeBuildInputs = [ (nickelCohort pkgs.stdenv.hostPlatform.system) ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn policy export --check
              touch $out
            '';

        # r[verify cairn.unified_readiness.conformance]
        readiness-policy-fresh =
          pkgs.runCommand "cairn-readiness-policy-fresh"
            {
              nativeBuildInputs = [ (nickelCohort pkgs.stdenv.hostPlatform.system) ];
            }
            ''
              cp -r ${self}/cairn-policy policy
              chmod -R u+w policy
              nickel export --format json --field policy policy/readiness.ncl > readiness.json
              cmp readiness.json policy/generated/readiness.json
              nickel export --format json policy/fixtures/readiness-positive.ncl > positive.json
              if nickel export --format json policy/fixtures/readiness-negative.ncl > negative.json 2> negative.err; then
                echo "expected readiness authority rejection" >&2
                exit 1
              fi
              grep -q 'advisory' negative.err
              substituteInPlace policy/readiness.ncl \
                --replace-fail 'max_blockers | Positive = 512' 'max_blockers | Positive = 511'
              nickel export --format json --field policy policy/readiness.ncl > drift.json
              if cmp -s drift.json policy/generated/readiness.json; then
                echo "expected readiness policy drift" >&2
                exit 1
              fi
              touch $out
            '';

        # r[verify cairn.adoption_report.inputs]
        # r[verify cairn.adoption_report.stages]
        adoption-report-contracts =
          pkgs.runCommand "cairn-adoption-report-contracts"
            {
              nativeBuildInputs = [ (nickelCohort pkgs.stdenv.hostPlatform.system) ];
            }
            ''
              cp -r ${self}/cairn-policy policy
              chmod -R u+w policy
              nickel export --format json --field policy policy/adoption-report.ncl > adoption-policy.json
              nickel export --format json policy/fixtures/adoption-report-positive.ncl > positive.json
              if nickel export --format json policy/fixtures/adoption-report-negative.ncl > negative.json 2> negative.err; then
                echo "expected adoption report bound rejection" >&2
                exit 1
              fi
              grep -q 'max_relationships' negative.err
              touch $out
            '';

        authenticated-policy-profiles-fresh =
          pkgs.runCommand "cairn-authenticated-policy-profiles-fresh"
            {
              nativeBuildInputs = [ (nickelCohort pkgs.stdenv.hostPlatform.system) ];
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
              nativeBuildInputs = [ (nickelCohort pkgs.stdenv.hostPlatform.system) ];
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
                (nickelCohort pkgs.stdenv.hostPlatform.system)
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

        # r[verify cairn.gate_interchange.conformance]
        gate-contract = pkgs.rustPlatform.buildRustPackage {
          pname = "cairn-gate-contract-tests";
          version = "0.1.0";
          src = self;
          cargoLock = rustCargoLock;
          cargoBuildFlags = [
            "-p"
            "cairn-gate-contract"
            "--all-targets"
          ];
          cargoTestFlags = [
            "-p"
            "cairn-gate-contract"
            "--all-targets"
          ];
          postCheck = ''
            # r[verify cairn.gate_interchange.conformance.scenario.drift]
            cargo run --quiet -p cairn-gate-contract --example emit-fixtures \
              > "$TMPDIR/cairn-gate-fixtures-v1.json"
            cmp "$TMPDIR/cairn-gate-fixtures-v1.json" \
              fixtures/cairn-gate-contract/producer/fixtures-v1.json
          '';
          installPhase = ''
            runHook preInstall
            touch $out
            runHook postInstall
          '';
        };

        # r[verify cairn.gate_interchange.conformance]
        gate-contract-wasm = pkgs.rustPlatform.buildRustPackage {
          pname = "cairn-gate-contract-wasm";
          version = "0.1.0";
          src = self;
          cargoLock = rustCargoLock;
          cargoBuildFlags = [
            "-p"
            "cairn-gate-contract"
            "--lib"
            "--target"
            "wasm32-unknown-unknown"
          ];
          doCheck = false;
          installPhase = ''
            runHook preInstall
            touch $out
            runHook postInstall
          '';
        };

        # r[verify cairn.gate_interchange.conformance]
        gate-contract-nickel =
          pkgs.runCommand "cairn-gate-contract-nickel"
            {
              nativeBuildInputs = [ (nickelCohort pkgs.stdenv.hostPlatform.system) ];
            }
            ''
              for fixture in ${self}/fixtures/cairn-gate-contract/positive/*.ncl; do
                nickel typecheck "$fixture"
                nickel export --format json "$fixture" > /dev/null
              done
              for fixture in ${self}/fixtures/cairn-gate-contract/negative/*.ncl; do
                if nickel export --format json "$fixture" >negative.out 2>negative.err; then
                  echo "negative gate-contract fixture unexpectedly passed: $fixture" >&2
                  exit 1
                fi
                test -s negative.err
              done
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
            pkgs.bash
            pkgs.git
            pkgs.nix
            (nickelCohort pkgs.stdenv.hostPlatform.system)
            nickel-export.packages.${pkgs.stdenv.hostPlatform.system}.nickel-export
          ];
          preCheck = ''
            ${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn policy export --check
          '';
        };

        # r[impl cairn.radicle_cob_only.dependencies]
        # r[verify cairn.radicle_cob_only.dependencies.excludes_runtime]
        # r[impl cairn.radicle_cob_only.validation]
        # r[impl cairn.radicle_two_peer_convergence.retired]
        # r[verify cairn.radicle_two_peer_convergence.retired.local_only]
        # r[verify cairn.radicle_cob_only.compatibility.git_refs]
        # r[verify cairn.radicle_cob_only.compatibility.signatures]
        # r[verify cairn.radicle_cob_only.compatibility.malformed]
        # r[impl cairn.cob_interop.boundary]
        # r[impl cairn.cob_interop.validation]
        # r[verify cairn.cob_interop.boundary.offline]
        # r[verify cairn.cob_interop.validation.release]
        radicle-cob-format-compatibility = pkgs.rustPlatform.buildRustPackage {
          pname = "cairn-radicle-cob-format-compatibility";
          version = "0.1.0";
          src = self;
          cargoLock = rustCargoLock;
          cargoBuildFlags = [
            "-p"
            "onix-cairn-radicle-cob"
            "--test"
            "cob_format_compatibility"
            "--test"
            "cob_interoperability"
          ];
          nativeCheckInputs = [
            pkgs.git
            (nickelCohort pkgs.stdenv.hostPlatform.system)
            pkgs.rustfmt
          ];
          doCheck = true;
          checkPhase = ''
            runHook preCheck
            if cargo tree -p onix-cairn-radicle-cob --prefix none \
              | grep -Eq '^(radicle|radicle-node) v'; then
              echo "umbrella Radicle runtime dependency detected" >&2
              exit 1
            fi
            fixture_root=fixtures/radicle-cob-interoperability
            cargo fmt --manifest-path "$fixture_root/producer/Cargo.toml" -- --check
            nickel export --format json "$fixture_root/provenance.ncl" \
              > "$TMPDIR/cob-interoperability-provenance.json"
            cmp "$TMPDIR/cob-interoperability-provenance.json" \
              "$fixture_root/provenance.json"
            git init --bare "$TMPDIR/cob-bundle-verifier.git"
            git -C "$TMPDIR/cob-bundle-verifier.git" bundle verify \
              "$PWD/$fixture_root/external-cairn-cob.bundle"
            cargo test -p onix-cairn-radicle-cob \
              --test cob_format_compatibility \
              --test cob_interoperability
            runHook postCheck
          '';
          installPhase = ''
            runHook preInstall
            touch $out
            runHook postInstall
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

        trace-index-live-proof =
          pkgs.runCommand "cairn-trace-index-live-proof"
            {
              nativeBuildInputs = [ pkgs.gnugrep ];
            }
            ''
              cp -r ${self} source
              chmod -R u+w source
              cd source
              cairn=${self.packages.${pkgs.stdenv.hostPlatform.system}.cairn}/bin/cairn
              proof=proof
              stale_source=stale-source
              mkdir -p "$proof"

              "$cairn" requirement-registry export \
                --root . \
                --repository-id onixresearch/cairn \
                --revision-kind opaque \
                --revision nix-source-snapshot \
                --out "$proof/requirement-registry.json" \
                > "$proof/requirement-registry-export-receipt.json"
              "$cairn" traceability export \
                --root . \
                --registry "$proof/requirement-registry.json" \
                --out "$proof/trace-index.json" \
                > "$proof/trace-index-export-receipt.json"
              "$cairn" traceability verify \
                --root . \
                --registry "$proof/requirement-registry.json" \
                --index "$proof/trace-index.json" \
                > "$proof/trace-index-verify-receipt.json"
              "$cairn" traceability show cairn.trace_index.query \
                --index "$proof/trace-index.json" \
                > "$proof/trace-index-show-receipt.json"

              for receipt in \
                "$proof/requirement-registry-export-receipt.json" \
                "$proof/trace-index-export-receipt.json" \
                "$proof/trace-index-verify-receipt.json" \
                "$proof/trace-index-show-receipt.json"
              do
                grep -q '"valid": true' "$receipt"
                grep -q '"verdict": "pass"' "$receipt"
              done
              grep -Eq '"link_count": [1-9][0-9]*' \
                "$proof/trace-index-export-receipt.json"
              grep -Eq '"links": \[' \
                "$proof/trace-index-show-receipt.json"

              cp -r ${self} "$stale_source"
              chmod -R u+w "$stale_source"
              printf '\n// r[impl cairn.trace_index.query]\n' \
                >> "$stale_source/crates/cairn-cli/src/trace_index.rs"
              if "$cairn" traceability verify \
                --root "$stale_source" \
                --registry "$proof/requirement-registry.json" \
                --index "$proof/trace-index.json" \
                > "$proof/trace-index-stale-verify-receipt.json"
              then
                echo "expected stale trace-index verification to fail" >&2
                exit 1
              fi
              grep -q '"valid": false' \
                "$proof/trace-index-stale-verify-receipt.json"
              grep -q '"verdict": "fail"' \
                "$proof/trace-index-stale-verify-receipt.json"
              grep -Eq 'trace_index\.(link_stale|source_stale|source_unexpected)' \
                "$proof/trace-index-stale-verify-receipt.json"

              mkdir -p "$out"
              cp "$proof"/*.json "$out"/
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
          nativeCheckInputs = [
            pkgs.jq
            (nickelCohort pkgs.stdenv.hostPlatform.system)
          ];
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
            rm -rf "$legacy/.cairn"
            $cairn status --root "$legacy" >/tmp/cairn-legacy-status.json
            # r[verify cairn.lifecycle_directory.legacy_migration]
            legacy_cairn=$(mktemp -d)
            cp -R fixtures/basic/. "$legacy_cairn"/
            mv "$legacy_cairn/.cairn" "$legacy_cairn/cairn"
            if $cairn status --root "$legacy_cairn" >/tmp/cairn-old-directory-status.json 2>/tmp/cairn-old-directory-status.stderr; then
              echo "legacy-only Cairn status unexpectedly succeeded" >&2
              exit 1
            fi
            grep -F 'cairn.lifecycle_directory.migration_required' /tmp/cairn-old-directory-status.stderr
            $cairn migrate cairn --root "$legacy_cairn" >/tmp/cairn-migrate-old-directory-plan.json
            old_directory_plan_hash=$(jq -r .plan_hash /tmp/cairn-migrate-old-directory-plan.json)
            $cairn migrate cairn --root "$legacy_cairn" --execute --plan-hash "$old_directory_plan_hash" >/tmp/cairn-migrate-old-directory-execute.json
            $cairn status --root "$legacy_cairn" >/tmp/cairn-migrated-old-directory-status.json
            legacy_ironspec=$(mktemp -d)
            cp -R fixtures/basic/. "$legacy_ironspec"/
            mv "$legacy_ironspec/.cairn" "$legacy_ironspec/ironspec"
            $cairn status --root "$legacy_ironspec" >/tmp/cairn-legacy-ironspec-status.json
            $cairn migrate ironspec --root "$legacy_ironspec" >/tmp/cairn-migrate-ironspec-plan.json
            ironspec_plan_hash=$(jq -r .plan_hash /tmp/cairn-migrate-ironspec-plan.json)
            $cairn migrate ironspec --root "$legacy_ironspec" --execute --plan-hash "$ironspec_plan_hash" >/tmp/cairn-migrate-ironspec-execute.json
            $cairn migrate openspec --root "$legacy" >/tmp/cairn-migrate-plan.json
            openspec_plan_hash=$(jq -r .plan_hash /tmp/cairn-migrate-plan.json)
            $cairn migrate openspec --root "$legacy" --execute --plan-hash "$openspec_plan_hash" >/tmp/cairn-migrate-execute.json
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

        slotcar =
          let
            vendor = import ./nix/producer-cargo-vendor.nix {
              inherit pkgs;
              crane = slotcar.inputs.crane;
              mantleSource = self.inputs.mantle-build-contract;
              cargoLock = ./Cargo.lock;
            };
            consumer = slotcar.lib.mkConsumerCheck {
              system = pkgs.stdenv.hostPlatform.system;
              src = self;
              # Supply the same locked dependencies through our packaging adapter.
              # The upstream consumer command and full check scope stay unchanged.
              cargoExtraArgs = "--all-targets --features redb-registry";
              nativeBuildInputs = [ pkgs.stdenv.cc ];
            };
          in
          consumer.overrideAttrs (old: {
            buildCommand = ''
              export CARGO_HOME="$TMPDIR/cairn-cargo-home"
              mkdir -p "$CARGO_HOME"
              cp ${vendor}/config.toml "$CARGO_HOME/config.toml"
            ''
            + old.buildCommand;
          });
      });

      devShells = eachSystem (pkgs: {
        default = pkgs.mkShell {
          packages = [
            pkgs.cargo
            pkgs.rustc
            pkgs.rustfmt
            pkgs.clippy
            (nickelCohort pkgs.stdenv.hostPlatform.system)
            nickel-export.packages.${pkgs.stdenv.hostPlatform.system}.nickel-export
            slotcar.packages.${pkgs.stdenv.hostPlatform.system}.cargo-tigerstyle
          ];
        };
      });
    };
}
