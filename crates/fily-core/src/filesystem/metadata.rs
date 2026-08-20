use std::time::SystemTime;

use super::path::FilyPath;

/// The type of filesystem entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Other,
}

/// Metadata associated with a filesystem entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FileMetadata {
    /// Path of the filesystem entry.
    pub path: FilyPath,

    /// Type of filesystem entry.
    pub file_type: FileType,

    /// Size in bytes.
    ///
    /// For directories, this value may vary between platforms and
    /// should not be interpreted as the total size of its contents.
    pub size: u64,

    /// Last modification time.
    pub modified: Option<SystemTime>,

    /// Creation time, when supported by the operating system.
    pub created: Option<SystemTime>,

    /// Whether the entry is hidden.
    pub hidden: bool,

    /// Whether the entry is read-only.
    pub read_only: bool,
}

impl FileMetadata {
    /// Creates a new metadata object.
    pub fn new(
        path: FilyPath,
        file_type: FileType,
        size: u64,
        modified: Option<SystemTime>,
        created: Option<SystemTime>,
        hidden: bool,
        read_only: bool,
    ) -> Self {
        Self {
            path,
            file_type,
            size,
            modified,
            created,
            hidden,
            read_only,
        }
    }

    /// Returns whether this entry is a regular file.
    pub fn is_file(&self) -> bool {
        self.file_type == FileType::File
    }

    /// Returns whether this entry is a directory.
    pub fn is_directory(&self) -> bool {
        self.file_type == FileType::Directory
    }

    /// Returns whether this entry is a symbolic link.
    pub fn is_symlink(&self) -> bool {
        self.file_type == FileType::Symlink
    }
}