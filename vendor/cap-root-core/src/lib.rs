//! Pure lexical admission for paths interpreted beneath a directory capability.
//!
//! This crate performs no filesystem access. It normalizes platform-native
//! [`Path`] components, rejects lexical escapes, and resolves relative symlink
//! target text without following a link. Consumers retain directory handles,
//! no-follow I/O, authorization, diagnostics, and evidence policy.

#![forbid(unsafe_code)]

use std::ffi::OsStr;
use std::fmt;
use std::path::Component;
use std::path::Path;
use std::path::PathBuf;

/// Admission mode for a path interpreted beneath a capability root.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdmissionMode {
    /// Require at least one normalized relative component.
    NonEmptyRelative,
    /// Permit `.` to designate the capability root while rejecting empty input.
    RelativeOrRoot,
    /// Permit a leading logical root separator, then require a non-root target.
    ///
    /// This supports protocol paths such as `/run/service/state` without
    /// treating the leading separator as ambient host authority. Platform
    /// prefixes remain forbidden.
    LogicalTarget,
}

/// A normalized lexical path confined to a caller-owned capability root.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct RootRelativePath {
    path: PathBuf,
}

impl RootRelativePath {
    /// Validate and normalize a platform-native path under an explicit mode.
    ///
    /// # Errors
    ///
    /// Returns a typed error for empty input, disallowed absolute or prefixed
    /// paths, parent traversal, or a mode that requires a non-root target.
    pub fn new(path: impl AsRef<Path>, mode: AdmissionMode) -> Result<Self, PathAdmissionError> {
        let source = path.as_ref();
        if source.as_os_str().is_empty() {
            return Err(PathAdmissionError::new(
                PathAdmissionErrorKind::Empty,
                source,
            ));
        }

        let mut normalized = PathBuf::new();
        for component in source.components() {
            match component {
                Component::Normal(segment) => normalized.push(segment),
                Component::CurDir => {}
                Component::ParentDir => {
                    return Err(PathAdmissionError::new(
                        PathAdmissionErrorKind::ParentTraversal,
                        source,
                    ));
                }
                Component::RootDir if mode == AdmissionMode::LogicalTarget => {}
                Component::RootDir | Component::Prefix(_) => {
                    return Err(PathAdmissionError::new(
                        PathAdmissionErrorKind::AbsoluteOrPrefixed,
                        source,
                    ));
                }
            }
        }

        if normalized.as_os_str().is_empty() {
            if mode == AdmissionMode::RelativeOrRoot {
                normalized.push(".");
            } else {
                return Err(PathAdmissionError::new(
                    PathAdmissionErrorKind::Empty,
                    source,
                ));
            }
        }

        Ok(Self { path: normalized })
    }

    /// Validate a single normalized path component.
    ///
    /// # Errors
    ///
    /// Returns the same errors as [`Self::new`] and rejects nested paths.
    pub fn new_leaf(path: impl AsRef<Path>) -> Result<Self, PathAdmissionError> {
        let source = path.as_ref();
        let admitted = Self::new(source, AdmissionMode::NonEmptyRelative)?;
        if admitted.parent().is_some() {
            return Err(PathAdmissionError::new(
                PathAdmissionErrorKind::NestedLeaf,
                source,
            ));
        }
        Ok(admitted)
    }

    /// Return the normalized path (`.` represents the capability root).
    #[must_use]
    pub fn as_path(&self) -> &Path {
        &self.path
    }

    /// Return whether this value designates the capability root itself.
    #[must_use]
    pub fn is_root(&self) -> bool {
        self.path == Path::new(".")
    }

    /// Return the non-empty normalized parent, if this is a nested path.
    #[must_use]
    pub fn parent(&self) -> Option<&Path> {
        if self.is_root() {
            return None;
        }
        self.path
            .parent()
            .filter(|parent| !parent.as_os_str().is_empty())
    }

    /// Return the final component, or `None` when this designates the root.
    #[must_use]
    pub fn file_name(&self) -> Option<&OsStr> {
        if self.is_root() {
            None
        } else {
            self.path.file_name()
        }
    }

    /// Consume the typed path and return its normalized representation.
    #[must_use]
    pub fn into_path_buf(self) -> PathBuf {
        self.path
    }

    fn from_normalized(path: PathBuf) -> Self {
        debug_assert!(!path.as_os_str().is_empty());
        debug_assert!(!path.is_absolute());
        debug_assert!(
            !path
                .components()
                .any(|component| matches!(component, Component::ParentDir))
        );
        Self { path }
    }
}

impl AsRef<Path> for RootRelativePath {
    fn as_ref(&self) -> &Path {
        self.as_path()
    }
}

impl TryFrom<&Path> for RootRelativePath {
    type Error = PathAdmissionError;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        Self::new(path, AdmissionMode::NonEmptyRelative)
    }
}

/// Stable lexical path-admission failure kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PathAdmissionErrorKind {
    /// Input is empty or normalizes to a disallowed root marker.
    Empty,
    /// Input carries an ambient root or platform prefix not admitted by mode.
    AbsoluteOrPrefixed,
    /// Input contains a parent component.
    ParentTraversal,
    /// A leaf-only constructor received more than one component.
    NestedLeaf,
}

impl PathAdmissionErrorKind {
    /// Return a stable diagnostic class without consumer-specific prose.
    #[must_use]
    pub const fn class(self) -> &'static str {
        match self {
            Self::Empty => "empty-relative-path",
            Self::AbsoluteOrPrefixed => "absolute-or-prefixed-path",
            Self::ParentTraversal => "parent-traversal",
            Self::NestedLeaf => "nested-leaf",
        }
    }
}

/// Typed lexical path-admission failure with the rejected platform-native path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PathAdmissionError {
    kind: PathAdmissionErrorKind,
    path: PathBuf,
}

impl PathAdmissionError {
    fn new(kind: PathAdmissionErrorKind, path: &Path) -> Self {
        Self {
            kind,
            path: path.to_path_buf(),
        }
    }

    /// Return the stable failure kind.
    #[must_use]
    pub const fn kind(&self) -> PathAdmissionErrorKind {
        self.kind
    }

    /// Return the stable failure class.
    #[must_use]
    pub const fn class(&self) -> &'static str {
        self.kind.class()
    }

    /// Return the rejected platform-native path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

impl fmt::Display for PathAdmissionError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: path is not lexically admitted beneath the capability root",
            self.class()
        )
    }
}

impl std::error::Error for PathAdmissionError {}

/// A lexically confined relative symlink target and its resolved root-relative path.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InternalSymlinkTarget {
    link_path: RootRelativePath,
    raw_target: PathBuf,
    resolved_target: RootRelativePath,
}

impl InternalSymlinkTarget {
    /// Resolve relative symlink target text from the link's parent without I/O.
    ///
    /// A target may use parent components only while the lexical stack remains
    /// beneath the capability root. Resolving exactly to the capability root is
    /// represented by `.` and left for consumer policy to accept or reject.
    ///
    /// # Errors
    ///
    /// Returns an error for a root link path, empty target, absolute or
    /// platform-prefixed target, or parent traversal beyond the root.
    pub fn resolve(
        link_path: &RootRelativePath,
        raw_target: impl AsRef<Path>,
    ) -> Result<Self, SymlinkTargetError> {
        if link_path.is_root() {
            return Err(SymlinkTargetError::LinkPathIsRoot);
        }
        let raw_target = raw_target.as_ref();
        if raw_target.as_os_str().is_empty() {
            return Err(SymlinkTargetError::Empty);
        }

        let mut resolved = link_path
            .parent()
            .map(Path::to_path_buf)
            .unwrap_or_default();
        for component in raw_target.components() {
            match component {
                Component::Normal(segment) => resolved.push(segment),
                Component::CurDir => {}
                Component::ParentDir => {
                    if !resolved.pop() {
                        return Err(SymlinkTargetError::Escape);
                    }
                }
                Component::RootDir | Component::Prefix(_) => {
                    return Err(SymlinkTargetError::AbsoluteOrPrefixed);
                }
            }
        }

        if resolved.as_os_str().is_empty() {
            resolved.push(".");
        }
        let resolved_target = RootRelativePath::from_normalized(resolved);
        Ok(Self {
            link_path: link_path.clone(),
            raw_target: raw_target.to_path_buf(),
            resolved_target,
        })
    }

    /// Return the normalized root-relative link path.
    #[must_use]
    pub const fn link_path(&self) -> &RootRelativePath {
        &self.link_path
    }

    /// Return the original platform-native relative target text.
    #[must_use]
    pub fn raw_target(&self) -> &Path {
        &self.raw_target
    }

    /// Return the normalized root-relative lexical destination.
    #[must_use]
    pub const fn resolved_target(&self) -> &RootRelativePath {
        &self.resolved_target
    }
}

/// Stable lexical internal-symlink target failure.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SymlinkTargetError {
    /// A capability root marker cannot itself be a symlink entry.
    LinkPathIsRoot,
    /// Symlink target text is empty.
    Empty,
    /// Target text carries an ambient root or platform prefix.
    AbsoluteOrPrefixed,
    /// Parent traversal would leave the capability root.
    Escape,
}

impl SymlinkTargetError {
    /// Return a stable diagnostic class without consumer-specific prose.
    #[must_use]
    pub const fn class(self) -> &'static str {
        match self {
            Self::LinkPathIsRoot => "root-link-path",
            Self::Empty => "empty-symlink-target",
            Self::AbsoluteOrPrefixed => "absolute-or-prefixed-symlink-target",
            Self::Escape => "escaping-symlink-target",
        }
    }
}

impl fmt::Display for SymlinkTargetError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: symlink target is not lexically confined to the capability root",
            self.class()
        )
    }
}

impl std::error::Error for SymlinkTargetError {}

#[cfg(test)]
mod tests {
    use super::*;

    const NESTED_PATH: &str = "nested/tree/value";
    const NORMALIZED_PATH: &str = "nested/value";
    const ROOTED_LOGICAL_PATH: &str = "/run/service/value";
    const RELATIVE_LOGICAL_PATH: &str = "run/service/value";

    #[test]
    fn non_empty_mode_normalizes_current_components_and_preserves_shape() {
        let nested = RootRelativePath::new(NESTED_PATH, AdmissionMode::NonEmptyRelative)
            .expect("nested path is admitted");
        let normalized = RootRelativePath::new("./nested/./value", AdmissionMode::NonEmptyRelative)
            .expect("current components normalize");

        assert_eq!(nested.as_path(), Path::new(NESTED_PATH));
        assert_eq!(normalized.as_path(), Path::new(NORMALIZED_PATH));
        assert_eq!(nested.parent(), Some(Path::new("nested/tree")));
        assert_eq!(nested.file_name(), Some(OsStr::new("value")));
        assert!(!nested.is_root());
    }

    #[test]
    fn root_mode_distinguishes_explicit_root_marker_from_empty_input() {
        let root = RootRelativePath::new("./.", AdmissionMode::RelativeOrRoot)
            .expect("explicit relative root marker is admitted");
        let empty = RootRelativePath::new("", AdmissionMode::RelativeOrRoot)
            .expect_err("empty input is not a root grant");
        let ambient = RootRelativePath::new("/", AdmissionMode::RelativeOrRoot)
            .expect_err("ambient root is rejected");

        assert!(root.is_root());
        assert_eq!(root.as_path(), Path::new("."));
        assert_eq!(root.parent(), None);
        assert_eq!(root.file_name(), None);
        assert_eq!(empty.kind(), PathAdmissionErrorKind::Empty);
        assert_eq!(ambient.kind(), PathAdmissionErrorKind::AbsoluteOrPrefixed);
    }

    #[test]
    fn logical_target_mode_strips_only_logical_root_syntax() {
        let rooted = RootRelativePath::new(ROOTED_LOGICAL_PATH, AdmissionMode::LogicalTarget)
            .expect("rooted logical target is admitted");
        let relative = RootRelativePath::new(RELATIVE_LOGICAL_PATH, AdmissionMode::LogicalTarget)
            .expect("relative logical target is admitted");
        let root_only = RootRelativePath::new("/", AdmissionMode::LogicalTarget)
            .expect_err("logical root alone does not name a target");

        assert_eq!(rooted, relative);
        assert_eq!(rooted.as_path(), Path::new(RELATIVE_LOGICAL_PATH));
        assert_eq!(root_only.kind(), PathAdmissionErrorKind::Empty);
    }

    #[test]
    fn unsafe_paths_and_nested_leaves_are_rejected() {
        let absolute = RootRelativePath::new("/outside", AdmissionMode::NonEmptyRelative)
            .expect_err("absolute path is rejected");
        let parent = RootRelativePath::new("nested/../inside", AdmissionMode::NonEmptyRelative)
            .expect_err("parent traversal is rejected even when it could normalize inward");
        let current = RootRelativePath::new(".", AdmissionMode::NonEmptyRelative)
            .expect_err("root marker is rejected in non-empty mode");
        let nested_leaf =
            RootRelativePath::new_leaf("nested/value").expect_err("nested path is not a leaf");

        assert_eq!(absolute.kind(), PathAdmissionErrorKind::AbsoluteOrPrefixed);
        assert_eq!(parent.kind(), PathAdmissionErrorKind::ParentTraversal);
        assert_eq!(current.kind(), PathAdmissionErrorKind::Empty);
        assert_eq!(nested_leaf.kind(), PathAdmissionErrorKind::NestedLeaf);
    }

    #[test]
    fn internal_symlink_resolution_allows_parent_steps_that_stay_inside() {
        let link = RootRelativePath::new("users/web1/secret.age", AdmissionMode::NonEmptyRelative)
            .expect("link path is admitted");
        let target =
            InternalSymlinkTarget::resolve(&link, "../../machines/web1/generators/tls/secret.age")
                .expect("target stays inside the capability root");

        assert_eq!(target.link_path(), &link);
        assert_eq!(
            target.raw_target(),
            Path::new("../../machines/web1/generators/tls/secret.age")
        );
        assert_eq!(
            target.resolved_target().as_path(),
            Path::new("machines/web1/generators/tls/secret.age")
        );
        assert!(!target.resolved_target().is_root());
    }

    #[test]
    fn internal_symlink_resolution_exposes_safe_root_target_for_consumer_policy() {
        let link = RootRelativePath::new("directory/link", AdmissionMode::NonEmptyRelative)
            .expect("link path is admitted");
        let target = InternalSymlinkTarget::resolve(&link, "..")
            .expect("target resolves to but not beyond the capability root");

        assert!(target.resolved_target().is_root());
        assert_eq!(target.resolved_target().as_path(), Path::new("."));
    }

    #[test]
    fn internal_symlink_resolution_rejects_unsafe_shapes() {
        let link = RootRelativePath::new("users/web1/secret.age", AdmissionMode::NonEmptyRelative)
            .expect("link path is admitted");
        let root = RootRelativePath::new(".", AdmissionMode::RelativeOrRoot)
            .expect("root marker is admitted for the negative fixture");

        assert_eq!(
            InternalSymlinkTarget::resolve(&link, ""),
            Err(SymlinkTargetError::Empty)
        );
        assert_eq!(
            InternalSymlinkTarget::resolve(&link, "/outside"),
            Err(SymlinkTargetError::AbsoluteOrPrefixed)
        );
        assert_eq!(
            InternalSymlinkTarget::resolve(&link, "../../../outside"),
            Err(SymlinkTargetError::Escape)
        );
        assert_eq!(
            InternalSymlinkTarget::resolve(&root, "inside"),
            Err(SymlinkTargetError::LinkPathIsRoot)
        );
    }

    #[cfg(unix)]
    #[test]
    fn platform_native_non_utf8_components_are_preserved() {
        use std::os::unix::ffi::OsStringExt;

        const NON_UTF8_BYTE: u8 = 0x80;
        let segment = std::ffi::OsString::from_vec(vec![b'x', NON_UTF8_BYTE]);
        let input = PathBuf::from(&segment).join("leaf");
        let admitted = RootRelativePath::new(&input, AdmissionMode::NonEmptyRelative)
            .expect("non-UTF-8 native path is admitted without transcoding");
        let first = admitted
            .as_path()
            .components()
            .next()
            .expect("admitted path has its first component");

        assert_eq!(first.as_os_str(), segment);
        assert_eq!(admitted.file_name(), Some(OsStr::new("leaf")));
    }

    #[cfg(windows)]
    #[test]
    fn windows_platform_prefixes_are_rejected() {
        let drive = RootRelativePath::new(r"C:\outside", AdmissionMode::NonEmptyRelative)
            .expect_err("drive-prefixed path is rejected");
        let unc = RootRelativePath::new(r"\\server\share\outside", AdmissionMode::LogicalTarget)
            .expect_err("UNC-prefixed path is rejected");

        assert_eq!(drive.kind(), PathAdmissionErrorKind::AbsoluteOrPrefixed);
        assert_eq!(unc.kind(), PathAdmissionErrorKind::AbsoluteOrPrefixed);
    }
}
