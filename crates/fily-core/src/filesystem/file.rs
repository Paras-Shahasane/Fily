use super::path::FilyPath;

/// Represents a file discovered by Fily Core.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileEntry {
    /// Name of the file without its parent path.
    pub name: String,

    /// Full path to the file.
    pub path: FilyPath,
}

impl FileEntry {
    /// Creates a new file entry.
    pub fn new(name: String, path: FilyPath) -> Self {
        Self { name, path }
    }

    /// Returns the file name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the full file path.
    pub fn path(&self) -> &FilyPath {
        &self.path
    }
}