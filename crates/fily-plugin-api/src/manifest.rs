use crate::{PluginCapability, PluginId, PluginVersion};

/// Describes the identity and requirements of a Fily plugin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginManifest {
    id: PluginId,
    name: String,
    version: PluginVersion,
    author: String,
    entry_point: String,
    capabilities: Vec<PluginCapability>,
}

impl PluginManifest {
    /// Creates a new plugin manifest.
    pub fn new(
        id: PluginId,
        name: String,
        version: PluginVersion,
        author: String,
        entry_point: String,
        capabilities: Vec<PluginCapability>,
    ) -> Result<Self, PluginManifestError> {
        if name.trim().is_empty() {
            return Err(PluginManifestError::EmptyName);
        }

        if author.trim().is_empty() {
            return Err(PluginManifestError::EmptyAuthor);
        }

        if entry_point.trim().is_empty() {
            return Err(PluginManifestError::EmptyEntryPoint);
        }

        Ok(Self {
            id,
            name,
            version,
            author,
            entry_point,
            capabilities,
        })
    }

    /// Returns the plugin identifier.
    pub fn id(&self) -> &PluginId {
        &self.id
    }

    /// Returns the plugin display name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the plugin version.
    pub fn version(&self) -> &PluginVersion {
        &self.version
    }

    /// Returns the plugin author.
    pub fn author(&self) -> &str {
        &self.author
    }

    /// Returns the plugin entry point.
    pub fn entry_point(&self) -> &str {
        &self.entry_point
    }

    /// Returns the capabilities requested by the plugin.
    pub fn capabilities(&self) -> &[PluginCapability] {
        &self.capabilities
    }

    /// Returns whether the plugin requests a specific capability.
    pub fn has_capability(&self, capability: PluginCapability) -> bool {
        self.capabilities.contains(&capability)
    }
}

/// Errors that can occur while creating a plugin manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginManifestError {
    EmptyName,
    EmptyAuthor,
    EmptyEntryPoint,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PluginCapability, PluginId, PluginVersion};

    fn test_manifest() -> PluginManifest {
        PluginManifest::new(
            PluginId::new("example-plugin").unwrap(),
            "Example Plugin".to_string(),
            PluginVersion::new(1, 0, 0),
            "Paras".to_string(),
            "plugin.dll".to_string(),
            vec![
                PluginCapability::ReadFiles,
                PluginCapability::Navigation,
            ],
        )
        .unwrap()
    }

    #[test]
    fn manifest_is_created_correctly() {
        let manifest = test_manifest();

        assert_eq!(manifest.id().as_str(), "example-plugin");
        assert_eq!(manifest.name(), "Example Plugin");
        assert_eq!(manifest.version().to_string(), "1.0.0");
        assert_eq!(manifest.author(), "Paras");
        assert_eq!(manifest.entry_point(), "plugin.dll");

        assert_eq!(manifest.capabilities().len(), 2);
        assert!(manifest.has_capability(PluginCapability::ReadFiles));
        assert!(manifest.has_capability(PluginCapability::Navigation));
        assert!(!manifest.has_capability(PluginCapability::DeleteFiles));
    }

    #[test]
    fn empty_name_is_rejected() {
        let result = PluginManifest::new(
            PluginId::new("example-plugin").unwrap(),
            String::new(),
            PluginVersion::new(1, 0, 0),
            "Paras".to_string(),
            "plugin.dll".to_string(),
            Vec::new(),
        );

        assert_eq!(result, Err(PluginManifestError::EmptyName));
    }

    #[test]
    fn whitespace_only_name_is_rejected() {
        let result = PluginManifest::new(
            PluginId::new("example-plugin").unwrap(),
            "   ".to_string(),
            PluginVersion::new(1, 0, 0),
            "Paras".to_string(),
            "plugin.dll".to_string(),
            Vec::new(),
        );

        assert_eq!(result, Err(PluginManifestError::EmptyName));
    }

    #[test]
    fn empty_author_is_rejected() {
        let result = PluginManifest::new(
            PluginId::new("example-plugin").unwrap(),
            "Example Plugin".to_string(),
            PluginVersion::new(1, 0, 0),
            String::new(),
            "plugin.dll".to_string(),
            Vec::new(),
        );

        assert_eq!(result, Err(PluginManifestError::EmptyAuthor));
    }

    #[test]
    fn empty_entry_point_is_rejected() {
        let result = PluginManifest::new(
            PluginId::new("example-plugin").unwrap(),
            "Example Plugin".to_string(),
            PluginVersion::new(1, 0, 0),
            "Paras".to_string(),
            String::new(),
            Vec::new(),
        );

        assert_eq!(result, Err(PluginManifestError::EmptyEntryPoint));
    }

    #[test]
    fn manifest_can_have_no_capabilities() {
        let manifest = PluginManifest::new(
            PluginId::new("basic-plugin").unwrap(),
            "Basic Plugin".to_string(),
            PluginVersion::new(1, 0, 0),
            "Paras".to_string(),
            "plugin.dll".to_string(),
            Vec::new(),
        )
        .unwrap();

        assert!(manifest.capabilities().is_empty());
    }
}