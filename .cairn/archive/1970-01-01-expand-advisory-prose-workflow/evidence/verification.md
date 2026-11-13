# Expanded advisory prose integration observations

## Checked source behavior

- The original seven CLI consumer tests passed before the command cutover.
- Final focused checks passed: 19 pure analysis/renderer tests and 14 CLI consumer tests, including Markdown-divider versus AsciiDoc-listing discrimination and closed front-matter handling.
- `cargo clippy -p cairn-cli --all-targets --features redb-registry -- -D warnings` passed with the repository Rust 1.94.1 cohort.
- The pinned Nickel/nickel-export boundary accepted the checked-in prose profile export as fresh, admitted the positive custom profile fixture, and rejected the unknown-rule fixture.
- Native proposal, design and tasks gates passed. Synchronization applied eight added requirements and the modified input-selection requirement. Traceability reported 2367 declared IDs with no missing or dangling references. This is linkage, not satisfaction.
- The complete x86_64 `nix flake check . -L` passed, including workspace tests, workspace Clippy, Octet, generated policy/profile checks, the positive/negative/stale prose-profile export rail, native validation/traceability, and self-dogfood. This run omitted incompatible aarch64 checks; no aarch64 claim is made. Builders were local, automatic GC was disabled for the invocation, and the public Nix cache and isolated evaluator cache were selected explicitly.
- The full gate evaluated the final implementation before evidence/checklist-only updates and archival. No implementation changed after that passing snapshot.
- The reviewed archive action completed. Native validation and traceability passed after archival with all 2367 declared requirement IDs linked.

## Live native CLI

From a temporary working directory outside the checkout, with empty ambient PATH and no CAIRN_POLICY override, the native CLI produced legacy JSON v1, enriched v2, human text, warning annotations and SARIF. Its v2 freshness check succeeded for current inputs. The source bytes `He are late.\n` remained unchanged.

The emitted SARIF 2.1.0 example passed Draft 7 schema validation against the OASIS SARIF 2.1.0 schema mirrored at `https://json.schemastore.org/sarif-2.1.0.json` (schema ID `https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json`). The isolated development validator was jsonschema 4.26.0; it is not a runtime or shipped dependency.

Consumer regressions also rejected stale input bytes, profile drift, changed selected file membership, and a modified suggestion whose report hash had been recomputed. Output limits failed before successful partial output. All suggestions remained review-only.

## Installed package

`nix build .#cairn -o target/prose-package` passed for the final implementation. From a foreign working directory, with initially empty PATH and no CAIRN_POLICY override, that package ran legacy v1, machine v2, machine freshness rechecking, text, GitHub warning annotations and schema-valid SARIF. It selected this active change deterministically and checked profile-export freshness using its bundled tools. Source bytes and the actual repository's native validation and proposal-gate receipts were unchanged.

The emitted example v2 report and SARIF are retained in `packaged-report-v2.json` and `packaged-sarif.json`. `packaged-observation.json` records the observed assertions and package output identity. These examples contain only the explicit `He are late.\n` fixture and are not lifecycle evidence.

## Claim boundary

These are local, bounded integration observations for the pinned englishc revision `336ed696eec73e5b9858c5d7952b54c8e93b9df9`. Grammar reports and saved examples are not requirement, implementation, verification, readiness, or release evidence. Freshness is not authenticity, approval, an atomic snapshot, or safe-application authority. No suggestion was applied, no diagnostic was uploaded, and no release tag was created.
