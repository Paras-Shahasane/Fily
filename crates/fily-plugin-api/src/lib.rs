pub mod command;
pub use command::{CommandContext,CommandResult,PluginCommand,};

pub mod plugin;
pub use plugin::{Plugin, PluginContext, PluginError};

pub mod capability;
pub use capability::PluginCapability;

mod manifest;
pub use manifest::{PluginManifest, PluginManifestError};

use std::fmt;

/// A validated Fily plugin identifier.
///
/// Plugin IDs:
/// - must not be empty
/// - may contain only lowercase ASCII letters, digits, and hyphens
/// - must start with a lowercase ASCII letter
/// - must end with a lowercase ASCII letter or digit
/// - must not contain consecutive hyphens
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginId(String);

/// Errors that can occur when creating a [`PluginId`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginIdError {
    Empty,
    InvalidCharacters,
    InvalidStart,
    InvalidEnd,
    ConsecutiveHyphens,
}

impl fmt::Display for PluginIdError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let message = match self {
            Self::Empty => "plugin ID cannot be empty",
            Self::InvalidCharacters => {
                "plugin ID may only contain lowercase ASCII letters, digits, and hyphens"
            }
            Self::InvalidStart => {
                "plugin ID must start with a lowercase ASCII letter"
            }
            Self::InvalidEnd => {
                "plugin ID must end with a lowercase ASCII letter or digit"
            }
            Self::ConsecutiveHyphens => {
                "plugin ID cannot contain consecutive hyphens"
            }
        };

        write!(formatter, "{message}")
    }
}

impl std::error::Error for PluginIdError {}

impl PluginId {
    /// Creates a validated plugin identifier.
    pub fn new(value: impl Into<String>) -> Result<Self, PluginIdError> {
        let value = value.into();

        if value.is_empty() {
            return Err(PluginIdError::Empty);
        }

        // Validate the complete character set first.
        // This ensures values such as "MyPlugin" correctly
        // return InvalidCharacters rather than InvalidStart.
        if !value
            .chars()
            .all(|character| {
                character.is_ascii_lowercase()
                    || character.is_ascii_digit()
                    || character == '-'
            })
        {
            return Err(PluginIdError::InvalidCharacters);
        }

        let first = value.as_bytes()[0];

        if !first.is_ascii_lowercase() {
            return Err(PluginIdError::InvalidStart);
        }

        let last = value.as_bytes()[value.len() - 1];

        if !(last.is_ascii_lowercase() || last.is_ascii_digit()) {
            return Err(PluginIdError::InvalidEnd);
        }

        if value.contains("--") {
            return Err(PluginIdError::ConsecutiveHyphens);
        }

        Ok(Self(value))
    }

    /// Returns the plugin identifier as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PluginId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

/// A semantic plugin version.
///
/// Represents:
///
/// `major.minor.patch`
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginVersion {
    major: u64,
    minor: u64,
    patch: u64,
}

impl PluginVersion {
    /// Creates a semantic version.
    pub const fn new(major: u64, minor: u64, patch: u64) -> Self {
        Self {
            major,
            minor,
            patch,
        }
    }

    /// Returns the major version.
    pub const fn major(&self) -> u64 {
        self.major
    }

    /// Returns the minor version.
    pub const fn minor(&self) -> u64 {
        self.minor
    }

    /// Returns the patch version.
    pub const fn patch(&self) -> u64 {
        self.patch
    }
}

impl fmt::Display for PluginVersion {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}.{}.{}",
            self.major,
            self.minor,
            self.patch
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_plugin_id_is_created() {
        let id = PluginId::new("image-preview").unwrap();

        assert_eq!(id.as_str(), "image-preview");
        assert_eq!(id.to_string(), "image-preview");
    }

    #[test]
    fn empty_plugin_id_is_rejected() {
        assert_eq!(
            PluginId::new(""),
            Err(PluginIdError::Empty)
        );
    }

    #[test]
    fn plugin_id_with_invalid_start_is_rejected() {
        assert_eq!(
            PluginId::new("-plugin"),
            Err(PluginIdError::InvalidStart)
        );
    }

    #[test]
    fn plugin_id_with_invalid_end_is_rejected() {
        assert_eq!(
            PluginId::new("plugin-"),
            Err(PluginIdError::InvalidEnd)
        );
    }

    #[test]
    fn plugin_id_with_consecutive_hyphens_is_rejected() {
        assert_eq!(
            PluginId::new("my--plugin"),
            Err(PluginIdError::ConsecutiveHyphens)
        );
    }

    #[test]
    fn uppercase_plugin_id_is_rejected() {
        assert_eq!(
            PluginId::new("MyPlugin"),
            Err(PluginIdError::InvalidCharacters)
        );
    }

    #[test]
    fn plugin_id_accepts_digits_after_start() {
        let id = PluginId::new("plugin123").unwrap();

        assert_eq!(id.as_str(), "plugin123");
    }

    #[test]
    fn semantic_version_is_created_correctly() {
        let version = PluginVersion::new(1, 2, 3);

        assert_eq!(version.major(), 1);
        assert_eq!(version.minor(), 2);
        assert_eq!(version.patch(), 3);
    }

    #[test]
    fn semantic_version_is_displayed_correctly() {
        let version = PluginVersion::new(1, 2, 3);

        assert_eq!(version.to_string(), "1.2.3");
    }
}