use crate::errors::{FilyError, FilyResult};
use crate::filesystem::path::FilyPath;

/// Manages the current location and navigation history.
#[derive(Debug, Clone)]
pub struct Navigator {
    current: FilyPath,
    back_history: Vec<FilyPath>,
    forward_history: Vec<FilyPath>,
}

impl Navigator {
    /// Creates a navigator at the given path.
    pub fn new(path: FilyPath) -> FilyResult<Self> {
        if !path.exists() {
            return Err(FilyError::NotFound(path.to_path_buf()));
        }

        if !path.is_directory() {
            return Err(FilyError::InvalidPath(path.to_path_buf()));
        }

        Ok(Self {
            current: path,
            back_history: Vec::new(),
            forward_history: Vec::new(),
        })
    }

    /// Returns the current location.
    pub fn current(&self) -> &FilyPath {
        &self.current
    }

    /// Navigates into a directory.
    pub fn enter(&mut self, path: FilyPath) -> FilyResult<()> {
        if !path.exists() {
            return Err(FilyError::NotFound(path.to_path_buf()));
        }

        if !path.is_directory() {
            return Err(FilyError::InvalidPath(path.to_path_buf()));
        }

        if path == self.current {
            return Ok(());
        }

        self.back_history.push(self.current.clone());
        self.current = path;

        self.forward_history.clear();

        Ok(())
    }

    /// Navigates to the parent directory.
    pub fn parent(&mut self) -> FilyResult<()> {
        let Some(parent) = self.current.parent() else {
            return Ok(());
        };

        self.enter(parent)
    }

    /// Returns whether backward navigation is available.
    pub fn can_go_back(&self) -> bool {
        !self.back_history.is_empty()
    }

    /// Returns whether forward navigation is available.
    pub fn can_go_forward(&self) -> bool {
        !self.forward_history.is_empty()
    }

    /// Navigates backward in history.
    pub fn back(&mut self) -> FilyResult<()> {
        let Some(previous) = self.back_history.pop() else {
            return Ok(());
        };

        self.forward_history.push(self.current.clone());
        self.current = previous;

        Ok(())
    }

    /// Navigates forward in history.
    pub fn forward(&mut self) -> FilyResult<()> {
        let Some(next) = self.forward_history.pop() else {
            return Ok(());
        };

        self.back_history.push(self.current.clone());
        self.current = next;

        Ok(())
    }

    /// Returns the number of entries in the back history.
    pub fn back_history_len(&self) -> usize {
        self.back_history.len()
    }

    /// Returns the number of entries in the forward history.
    pub fn forward_history_len(&self) -> usize {
        self.forward_history.len()
    }
}