use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use crate::errors::FilyError;

/// Represents the type of filesystem entry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EntryType {
    File,
    Directory,
    Symlink,
    Other,
}

/// A lightweight representation of a filesystem entry.
///
/// This is the abstraction that the rest of Fily will use instead
/// of directly depending on `std::fs::DirEntry`.
#[derive(Debug, Clone)]
pub struct FileEntry {
    path: PathBuf,
    name: String,
    entry_type: EntryType,
    size: u64,
    modified: Option<SystemTime>,
    hidden: bool,
}

impl FileEntry {
    /// Creates a FileEntry from a filesystem path.
    pub fn from_path(path: PathBuf) -> Result<Self, FilyError> {
        let metadata = fs::symlink_metadata(&path)?;

        let entry_type = if metadata.file_type().is_symlink() {
            EntryType::Symlink
        } else if metadata.is_file() {
            EntryType::File
        } else if metadata.is_dir() {
            EntryType::Directory
        } else {
            EntryType::Other
        };

        let name = path
            .file_name()
            .map(|name| name.to_string_lossy().into_owned())
            .unwrap_or_default();

        let hidden = Self::detect_hidden(&name);

        let size = if metadata.is_file() {
            metadata.len()
        } else {
            0
        };

        let modified = metadata.modified().ok();

        Ok(Self {
            path,
            name,
            entry_type,
            size,
            modified,
            hidden,
        })
    }

    fn detect_hidden(name: &str) -> bool {
    name.starts_with('.')
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn entry_type(&self) -> EntryType {
        self.entry_type
    }

    pub fn size(&self) -> u64 {
        self.size
    }

    pub fn modified(&self) -> Option<SystemTime> {
        self.modified
    }

    pub fn is_hidden(&self) -> bool {
        self.hidden
    }

    pub fn is_file(&self) -> bool {
        self.entry_type == EntryType::File
    }

    pub fn is_directory(&self) -> bool {
        self.entry_type == EntryType::Directory
    }

    pub fn is_symlink(&self) -> bool {
        self.entry_type == EntryType::Symlink
    }
}

/// Lists the entries contained directly inside a directory.
pub fn list_directory(
    path: &std::path::Path,
) -> Result<Vec<FileEntry>, FilyError> {
    let mut entries = Vec::new();

    for entry in std::fs::read_dir(path)? {
        let entry = entry?;
        let file_entry = FileEntry::from_path(entry.path())?;
        entries.push(file_entry);
    }

    entries.sort_by(|a, b| {
        a.name()
            .to_lowercase()
            .cmp(&b.name().to_lowercase())
            .then_with(|| a.name().cmp(b.name()))
    });

    Ok(entries)
}