use std::fs;
use std::path::Path;

use super::directory::{DirectoryEntry, DirectoryListing};
use super::file::FileEntry;
use super::metadata::{FileMetadata, FileType};
use super::path::FilyPath;
use super::policy::CollisionPolicy;
use crate::errors::{FilyError, FilyResult};


/// Lists the contents of a directory.
pub fn list_directory(path: &FilyPath) -> FilyResult<DirectoryListing> {
    if !path.exists() {
        return Err(FilyError::NotFound(path.to_path_buf()));
    }

    if !path.is_directory() {
        return Err(FilyError::InvalidPath(path.to_path_buf()));
    }

    let mut listing = DirectoryListing::new(path.clone());

    for entry in fs::read_dir(path.as_path())? {
        let entry = entry?;
        let entry_path = entry.path();

        let name = entry
            .file_name()
            .to_string_lossy()
            .into_owned();

        if entry_path.is_dir() {
            listing
                .directories
                .push(DirectoryEntry::new(
                    name,
                    FilyPath::new(entry_path),
                ));
        } else if entry_path.is_file() {
            listing
                .files
                .push(FileEntry::new(
                    name,
                    FilyPath::new(entry_path),
                ));
        }
    }

    Ok(listing)
}

/// Creates an empty file.
pub fn create_file(path: &FilyPath) -> FilyResult<()> {
    if path.exists() {
        return Err(FilyError::AlreadyExists(path.to_path_buf()));
    }

    fs::File::create(path.as_path())?;

    Ok(())
}

/// Creates a directory.
pub fn create_directory(path: &FilyPath) -> FilyResult<()> {
    if path.exists() {
        return Err(FilyError::AlreadyExists(path.to_path_buf()));
    }

    fs::create_dir(path.as_path())?;

    Ok(())
}

/// Creates a directory and all missing parent directories.
pub fn create_directory_all(path: &FilyPath) -> FilyResult<()> {
    if path.exists() {
        return Err(FilyError::AlreadyExists(path.to_path_buf()));
    }

    fs::create_dir_all(path.as_path())?;

    Ok(())
}

/// Renames a file or directory using the specified collision policy.
pub fn rename(
    source: &FilyPath,
    destination: &FilyPath,
    policy: CollisionPolicy,
) -> FilyResult<()> {
    ensure_source_exists(source)?;

    if destination.exists() {
        match policy {
            CollisionPolicy::Fail => {
                return Err(FilyError::AlreadyExists(
                    destination.to_path_buf(),
                ));
            }

            CollisionPolicy::Skip => {
                return Ok(());
            }

            CollisionPolicy::Overwrite => {
                remove_existing_destination(destination)?;
            }
        }
    }

    fs::rename(source.as_path(), destination.as_path())?;

    Ok(())
}

/// Copies a file to another location using the specified collision policy.
pub fn copy_file(
    source: &FilyPath,
    destination: &FilyPath,
    policy: CollisionPolicy,
) -> FilyResult<()> {
    ensure_source_exists(source)?;

    if !source.is_file() {
        return Err(FilyError::InvalidPath(source.to_path_buf()));
    }

    if destination.exists() {
        match policy {
            CollisionPolicy::Fail => {
                return Err(FilyError::AlreadyExists(
                    destination.to_path_buf(),
                ));
            }

            CollisionPolicy::Skip => {
                return Ok(());
            }

            CollisionPolicy::Overwrite => {
                fs::remove_file(destination.as_path())?;
            }
        }
    }

    fs::copy(source.as_path(), destination.as_path())?;

    Ok(())
}

/// Copies a directory recursively.
pub fn copy_directory(source: &FilyPath, destination: &FilyPath) -> FilyResult<()> {
    ensure_source_exists(source)?;

    if !source.is_directory() {
        return Err(FilyError::InvalidPath(source.to_path_buf()));
    }

    if destination.exists() {
        return Err(FilyError::AlreadyExists(destination.to_path_buf()));
    }

    copy_directory_recursive(source.as_path(), destination.as_path())?;

    Ok(())
}

/// Moves a file or directory using the specified collision policy.
pub fn move_path(
    source: &FilyPath,
    destination: &FilyPath,
    policy: CollisionPolicy,
) -> FilyResult<()> {
    ensure_source_exists(source)?;

    if destination.exists() {
        match policy {
            CollisionPolicy::Fail => {
                return Err(FilyError::AlreadyExists(
                    destination.to_path_buf(),
                ));
            }

            CollisionPolicy::Skip => {
                return Ok(());
            }

            CollisionPolicy::Overwrite => {
                remove_existing_destination(destination)?;
            }
        }
    }

    fs::rename(source.as_path(), destination.as_path())?;

    Ok(())
}

/// Deletes a file.
pub fn delete_file(path: &FilyPath) -> FilyResult<()> {
    ensure_source_exists(path)?;

    if !path.is_file() {
        return Err(FilyError::InvalidPath(path.to_path_buf()));
    }

    fs::remove_file(path.as_path())?;

    Ok(())
}

/// Deletes an empty directory.
pub fn delete_directory(path: &FilyPath) -> FilyResult<()> {
    ensure_source_exists(path)?;

    if !path.is_directory() {
        return Err(FilyError::InvalidPath(path.to_path_buf()));
    }

    fs::remove_dir(path.as_path())?;

    Ok(())
}

/// Deletes a directory and everything inside it.
pub fn delete_directory_all(path: &FilyPath) -> FilyResult<()> {
    ensure_source_exists(path)?;

    if !path.is_directory() {
        return Err(FilyError::InvalidPath(path.to_path_buf()));
    }

    fs::remove_dir_all(path.as_path())?;

    Ok(())
}

/// Retrieves metadata for a filesystem entry.
pub fn metadata(path: &FilyPath) -> FilyResult<FileMetadata> {
    ensure_source_exists(path)?;

    let metadata = fs::symlink_metadata(path.as_path())?;

    let file_type = if metadata.file_type().is_symlink() {
        FileType::Symlink
    } else if metadata.is_file() {
        FileType::File
    } else if metadata.is_dir() {
        FileType::Directory
    } else {
        FileType::Other
    };

    let hidden = is_hidden(path.as_path());

    let read_only = metadata.permissions().readonly();

    Ok(FileMetadata::new(
        path.clone(),
        file_type,
        metadata.len(),
        metadata.modified().ok(),
        metadata.created().ok(),
        hidden,
        read_only,
    ))
}

/// Ensures that a source path exists.
fn ensure_source_exists(path: &FilyPath) -> FilyResult<()> {
    if !path.exists() {
        return Err(FilyError::NotFound(path.to_path_buf()));
    }

    Ok(())
}

/// Removes an existing destination regardless of whether it is
/// a file or directory.
fn remove_existing_destination(path: &FilyPath) -> FilyResult<()> {
    if path.is_file() {
        fs::remove_file(path.as_path())?;
    } else if path.is_directory() {
        fs::remove_dir_all(path.as_path())?;
    } else {
        fs::remove_file(path.as_path())?;
    }

    Ok(())
}

/// Determines whether a path should be considered hidden.
fn is_hidden(path: &Path) -> bool {
    let Some(file_name) = path.file_name() else {
        return false;
    };

    let name = file_name.to_string_lossy();

    if name == "." || name == ".." {
        return false;
    }

    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;

        if let Ok(metadata) = fs::symlink_metadata(path) {
            const FILE_ATTRIBUTE_HIDDEN: u32 = 0x2;

            return metadata.file_attributes() & FILE_ATTRIBUTE_HIDDEN != 0;
        }
    }

    #[cfg(not(windows))]
    {
        return name.starts_with('.');
    }

    #[allow(unreachable_code)]
    false
}

/// Recursively copies a directory.
fn copy_directory_recursive(
    source: &Path,
    destination: &Path,
) -> FilyResult<()> {
    fs::create_dir(destination)?;

    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());

        if source_path.is_dir() {
            copy_directory_recursive(
                &source_path,
                &destination_path,
            )?;
        } else {
            fs::copy(
                &source_path,
                &destination_path,
            )?;
        }
    }

    Ok(())
}