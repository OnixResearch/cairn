{ lib }:

let
  expectedRevision = "e41340bec587b6d049b5cc518ec7db925dde84be";
  rollbackRevision = "799459346d5416fbd7b9f55840a7371441b55afa";
  mismatchedRevision = "0000000000000000000000000000000000000000";
  expectedProfileRepository = "https://github.com/OnixResearch/onix-artifact";
  expectedSshRepository = "ssh://git@github.com/OnixResearch/onix-artifact.git";
  expectedNixNarHash = "sha256-2cM912L2YXVnVX9LquwhAPKyjPP/z/oFQRe7Qq9bHHE=";
  expectedLicense = "MIT OR Apache-2.0";
  incompatibleLicense = "AGPL-3.0-or-later";
  expectedVersion = "0.1.0";
  corePackage = "artifact-auth-core";
  ed25519Package = "artifact-auth-ed25519";
  transferPackage = "artifact-transfer-core";
  consumerPackageNames = [
    corePackage
    ed25519Package
  ];
  sourcePackageNames = consumerPackageNames ++ [ transferPackage ];
  expectedWorkspaceMembers = map (packageName: "crates/${packageName}") sourcePackageNames;
  expectedConsumerPackageCount = builtins.length consumerPackageNames;
  expectedCargoSource = "git+${expectedSshRepository}?rev=${expectedRevision}#${expectedRevision}";
  nixInputNode = "artifact";

  valueAt =
    path: fallback: value:
    lib.attrByPath path fallback value;
  issueUnless = condition: code: lib.optional (!condition) code;
  sorted = values: builtins.sort builtins.lessThan values;

  dependencyIssues =
    cargoWorkspace: packageName:
    let
      dependency = valueAt [ "workspace" "dependencies" packageName ] { } cargoWorkspace;
    in
    issueUnless (dependency ? git && dependency.git == expectedSshRepository) "cargo.${packageName}.git"
    ++ issueUnless (
      dependency ? rev && dependency.rev == expectedRevision
    ) "cargo.${packageName}.revision"
    ++ issueUnless (!(dependency ? path)) "cargo.${packageName}.workspace-relative"
    ++ issueUnless (!(dependency ? branch) && !(dependency ? tag)) "cargo.${packageName}.floating";

  lockPackageIssues =
    cargoLock: packageName:
    let
      matching = builtins.filter (package: (package.name or null) == packageName) (
        cargoLock.package or [ ]
      );
      package = if builtins.length matching == 1 then builtins.head matching else { };
    in
    issueUnless (builtins.length matching == 1) "cargo-lock.${packageName}.unique"
    ++ issueUnless ((package.version or null) == expectedVersion) "cargo-lock.${packageName}.version"
    ++ issueUnless ((package.source or null) == expectedCargoSource) "cargo-lock.${packageName}.source";

  consumerLockSetIssues =
    cargoLock:
    let
      sourcePackages = builtins.filter (package: (package.source or null) == expectedCargoSource) (
        cargoLock.package or [ ]
      );
      sourcePackageNamesObserved = map (package: package.name or "") sourcePackages;
    in
    issueUnless (
      sorted sourcePackageNamesObserved == sorted consumerPackageNames
    ) "cargo-lock.consumer-package-set";

  workspaceUseIssues =
    input:
    issueUnless (valueAt [
      "dependencies"
      corePackage
      "workspace"
    ] false input.coreManifest) "cairn-core.workspace-dependency"
    ++ issueUnless (valueAt [
      "dependencies"
      ed25519Package
      "workspace"
    ] false input.cliManifest) "cairn-cli.workspace-dependency";

  nixLockIssues =
    input:
    let
      rootInput = valueAt [ "nodes" "root" "inputs" nixInputNode ] null input.flakeLock;
      node = valueAt [ "nodes" nixInputNode ] { } input.flakeLock;
      locked = node.locked or { };
      original = node.original or { };
    in
    issueUnless (rootInput == nixInputNode) "nix.root-input"
    ++ issueUnless ((node.flake or null) == false) "nix.input.flake-enabled"
    ++ issueUnless ((locked.type or null) == "git") "nix.locked.type"
    ++ issueUnless ((locked.url or null) == expectedSshRepository) "nix.locked.repository"
    ++ issueUnless ((locked.rev or null) == expectedRevision) "nix.locked.revision"
    ++ issueUnless ((locked.narHash or null) == expectedNixNarHash) "nix.locked.nar-hash"
    ++ issueUnless ((original.type or null) == "git") "nix.original.type"
    ++ issueUnless ((original.url or null) == expectedSshRepository) "nix.original.repository"
    ++ issueUnless ((original.rev or null) == expectedRevision) "nix.original.revision"
    ++ issueUnless (!(original ? path) && !(original ? ref)) "nix.original.floating-or-path"
    ++ issueUnless (input.resolvedNixRevision == expectedRevision) "nix.resolved.revision"
    ++ issueUnless (input.resolvedNixNarHash == expectedNixNarHash) "nix.resolved.nar-hash";

  standalonePackageIssues =
    manifest: packageName:
    let
      package = manifest.package or { };
    in
    issueUnless ((package.name or null) == packageName) "standalone.${packageName}.name"
    ++ issueUnless ((package.publish or null) == false) "standalone.${packageName}.publish"
    ++ issueUnless (valueAt [
      "license"
      "workspace"
    ] false package) "standalone.${packageName}.license-inheritance";

  standaloneReleaseIssues =
    input:
    let
      workspacePackage = valueAt [ "workspace" "package" ] { } input.standaloneWorkspaceManifest;
      workspaceMembers = valueAt [ "workspace" "members" ] [ ] input.standaloneWorkspaceManifest;
      profilePackages = input.releaseProfile.packages or [ ];
      distribution = input.releaseProfile.distribution or { };
    in
    issueUnless ((workspacePackage.version or null) == expectedVersion) "standalone.workspace.version"
    ++ issueUnless (
      (workspacePackage.license or null) == expectedLicense
    ) "standalone.workspace.license"
    ++ issueUnless (
      (input.releaseProfile.license or null) == expectedLicense
    ) "standalone.profile.license"
    ++ issueUnless (
      (input.releaseProfile.repository or null) == expectedProfileRepository
    ) "standalone.profile.repository"
    ++ issueUnless (
      sorted workspaceMembers == sorted expectedWorkspaceMembers
    ) "standalone.workspace.members"
    ++ issueUnless (sorted profilePackages == sorted sourcePackageNames) "standalone.profile.packages"
    ++ issueUnless (
      (distribution.channel or null) == "immutable-git-revision"
    ) "standalone.profile.channel"
    ++ issueUnless ((distribution.crates_io or null) == false) "standalone.profile.crates-io"
    ++ issueUnless ((distribution.floating_revisions or null) == false) "standalone.profile.floating"
    ++ issueUnless ((distribution.sibling_paths or null) == false) "standalone.profile.sibling-paths"
    ++ standalonePackageIssues input.standaloneCoreManifest corePackage
    ++ standalonePackageIssues input.standaloneEd25519Manifest ed25519Package;

  validate =
    input:
    let
      issues =
        dependencyIssues input.cargoWorkspace corePackage
        ++ dependencyIssues input.cargoWorkspace ed25519Package
        ++ workspaceUseIssues input
        ++ lockPackageIssues input.cargoLock corePackage
        ++ lockPackageIssues input.cargoLock ed25519Package
        ++ consumerLockSetIssues input.cargoLock
        ++ nixLockIssues input
        ++ standaloneReleaseIssues input;
    in
    {
      valid = issues == [ ];
      inherit issues;
      package_count = expectedConsumerPackageCount;
      revision = expectedRevision;
    };

  replaceWorkspaceDependency =
    input: packageName: dependency:
    input
    // {
      cargoWorkspace = input.cargoWorkspace // {
        workspace = input.cargoWorkspace.workspace // {
          dependencies = input.cargoWorkspace.workspace.dependencies // {
            "${packageName}" = dependency;
          };
        };
      };
    };

  expectRejected =
    expectedIssue: input:
    let
      result = validate input;
    in
    issueUnless (
      !result.valid && builtins.elem expectedIssue result.issues
    ) "self-test.expected-${expectedIssue}";

  selfTest =
    input:
    let
      coreLockPackage = builtins.head (
        builtins.filter (package: (package.name or null) == corePackage) (input.cargoLock.package or [ ])
      );
      transferLockPackage = coreLockPackage // {
        name = transferPackage;
      };
      floatingInput = replaceWorkspaceDependency input corePackage {
        git = expectedSshRepository;
        branch = "main";
      };
      mismatchedInput = input // {
        flakeLock = lib.recursiveUpdate input.flakeLock {
          nodes."${nixInputNode}".locked.rev = mismatchedRevision;
        };
      };
      duplicateInput = input // {
        cargoLock = input.cargoLock // {
          package = (input.cargoLock.package or [ ]) ++ [ coreLockPackage ];
        };
      };
      widenedConsumerGraphInput = input // {
        cargoLock = input.cargoLock // {
          package = (input.cargoLock.package or [ ]) ++ [ transferLockPackage ];
        };
      };
      incompleteSourceProfileInput = input // {
        releaseProfile = input.releaseProfile // {
          packages = consumerPackageNames;
        };
      };
      incompleteSourceWorkspaceInput = input // {
        standaloneWorkspaceManifest = lib.recursiveUpdate input.standaloneWorkspaceManifest {
          workspace.members = map (packageName: "crates/${packageName}") consumerPackageNames;
        };
      };
      incompatibleLicenseInput = input // {
        releaseProfile = input.releaseProfile // {
          license = incompatibleLicense;
        };
        standaloneWorkspaceManifest = lib.recursiveUpdate input.standaloneWorkspaceManifest {
          workspace.package.license = incompatibleLicense;
        };
      };
      workspaceRelativeInput = replaceWorkspaceDependency input corePackage {
        path = "../artifact/crates/artifact-auth-core";
      };
    in
    expectRejected "cargo.${corePackage}.revision" floatingInput
    ++ expectRejected "nix.locked.revision" mismatchedInput
    ++ expectRejected "cargo-lock.${corePackage}.unique" duplicateInput
    ++ expectRejected "cargo-lock.consumer-package-set" widenedConsumerGraphInput
    ++ expectRejected "standalone.profile.packages" incompleteSourceProfileInput
    ++ expectRejected "standalone.workspace.members" incompleteSourceWorkspaceInput
    ++ expectRejected "standalone.workspace.license" incompatibleLicenseInput
    ++ expectRejected "cargo.${corePackage}.workspace-relative" workspaceRelativeInput;
in
{
  inherit selfTest validate;
  expected = {
    inherit expectedCargoSource expectedLicense expectedNixNarHash;
    profile_repository = expectedProfileRepository;
    ssh_repository = expectedSshRepository;
    packages = consumerPackageNames;
    source_packages = sourcePackageNames;
    revision = expectedRevision;
    rollback_revision = rollbackRevision;
    version = expectedVersion;
  };
}
