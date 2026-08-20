use crate::filesystem::path::FilyPath;
use crate::filesystem::policy::CollisionPolicy;

/// Describes a filesystem operation that Fily can execute.
///
/// An operation is only a description of the requested action.
/// The actual execution will be handled by the operation engine.
#[derive(Debug, Clone)]
pub enum Operation {
    /// Copy one or more files/directories into a destination directory.
    Copy {
        sources: Vec<FilyPath>,
        destination: FilyPath,
        policy: CollisionPolicy,
    },

    /// Move one or more files/directories into a destination directory.
    Move {
        sources: Vec<FilyPath>,
        destination: FilyPath,
        policy: CollisionPolicy,
    },

    /// Delete one or more files/directories.
    Delete {
        sources: Vec<FilyPath>,
        recursive: bool,
    },

    /// Rename a single file or directory.
    Rename {
        source: FilyPath,
        destination: FilyPath,
        policy: CollisionPolicy,
    },
}