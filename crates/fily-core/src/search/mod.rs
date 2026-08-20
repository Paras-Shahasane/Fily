use std::path::Path;

use crate::errors::FilyError;
use crate::filesystem::FileEntry;

mod matcher;

pub use matcher::matches_name;

/// Controls how Fily searches the filesystem.
#[derive(Debug, Clone)]
pub struct SearchOptions {
    /// Whether filename matching should ignore letter case.
    pub case_insensitive: bool,

    /// Whether hidden files and directories should be included.
    pub include_hidden: bool,

    /// Whether directories should be searched recursively.
    pub recursive: bool,
}

impl Default for SearchOptions {
    fn default() -> Self {
        Self {
            case_insensitive: true,
            include_hidden: false,
            recursive: true,
        }
    }
}

/// Searches for entries whose names match the given query.
pub fn search(
    root: &Path,
    query: &str,
    options: &SearchOptions,
) -> Result<Vec<FileEntry>, FilyError> {
    let mut results = Vec::new();

    search_directory(root, query, options, &mut results)?;

    Ok(results)
}

fn search_directory(
    directory: &Path,
    query: &str,
    options: &SearchOptions,
    results: &mut Vec<FileEntry>,
) -> Result<(), FilyError> {
    let entries = crate::filesystem::list_directory(directory)?;

    for entry in entries {
        if !options.include_hidden && entry.is_hidden() {
            continue;
        }

        if matches_name(entry.name(), query, options.case_insensitive) {
            results.push(entry.clone());
        }

        if options.recursive && entry.is_directory() {
            search_directory(
                &entry.path(),
                query,
                options,
                results,
            )?;
        }
    }

    Ok(())
}