use super::file::FileEntry;
use super::path::FilyPath;

/// Represents a directory in the filesystem.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryEntry {
    /// Name of the directory.
    pub name: String,

    /// Full path to the directory.
    pub path: FilyPath,
}

impl DirectoryEntry {
    /// Creates a new directory entry.
    pub fn new(name: String, path: FilyPath) -> Self {
        Self { name, path }
    }

    /// Returns the directory name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the full directory path.
    pub fn path(&self) -> &FilyPath {
        &self.path
    }
}

/// Represents the contents of a directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectoryListing {
    /// Directory being listed.
    pub path: FilyPath,

    /// Files contained in the directory.
    pub files: Vec<FileEntry>,

    /// Directories contained in the directory.
    pub directories: Vec<DirectoryEntry>,
}

impl DirectoryListing {
    /// Creates an empty directory listing.
    pub fn new(path: FilyPath) -> Self {
        Self {
            path,
            files: Vec::new(),
            directories: Vec::new(),
        }
    }

    /// Returns the total number of entries.
    pub fn entry_count(&self) -> usize {
        self.files.len() + self.directories.len()
    }

    /// Returns whether the directory contains no entries.
    pub fn is_empty(&self) -> bool {
        self.files.is_empty() && self.directories.is_empty()
    }
}