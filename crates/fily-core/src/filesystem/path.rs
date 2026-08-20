use std::path::{Path, PathBuf};

/// A platform-independent path used throughout Fily Core.
///
/// Internally this wraps Rust's `PathBuf`, allowing Fily to work
/// with native paths on Windows, Linux, and macOS.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilyPath(PathBuf);

impl FilyPath {
    /// Creates a new FilyPath from anything that can be converted
    /// into a PathBuf.
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self(path.into())
    }

    /// Returns the underlying path.
    pub fn as_path(&self) -> &Path {
        &self.0
    }

    /// Returns the path as a PathBuf clone.
    pub fn to_path_buf(&self) -> PathBuf {
        self.0.clone()
    }

    /// Returns the file or directory name.
    pub fn file_name(&self) -> Option<&std::ffi::OsStr> {
        self.0.file_name()
    }

    /// Returns the parent directory.
    pub fn parent(&self) -> Option<Self> {
        self.0.parent().map(Self::new)
    }

    /// Returns whether the path exists.
    pub fn exists(&self) -> bool {
        self.0.exists()
    }

    /// Returns whether the path points to a file.
    pub fn is_file(&self) -> bool {
        self.0.is_file()
    }

    /// Returns whether the path points to a directory.
    pub fn is_directory(&self) -> bool {
        self.0.is_dir()
    }

    /// Creates a new path by appending a component.
    pub fn join<P: AsRef<Path>>(&self, path: P) -> Self {
        Self::new(self.0.join(path))
    }
}

impl From<PathBuf> for FilyPath {
    fn from(path: PathBuf) -> Self {
        Self::new(path)
    }
}

impl From<&Path> for FilyPath {
    fn from(path: &Path) -> Self {
        Self::new(path)
    }
}

impl AsRef<Path> for FilyPath {
    fn as_ref(&self) -> &Path {
        &self.0
    }
}

impl std::fmt::Display for FilyPath {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}", self.0.display())
    }
}