use crate::{PluginId, PluginManifest, PluginVersion};

/// Context provided to a Fily plugin while it is running.
///
/// This will grow as the plugin API develops.
#[derive(Debug, Clone)]
pub struct PluginContext {
    manifest: PluginManifest,
}

impl PluginContext {
    /// Creates a new plugin context from a manifest.
    pub fn new(manifest: PluginManifest) -> Self {
        Self { manifest }
    }

    /// Returns the plugin manifest.
    pub fn manifest(&self) -> &PluginManifest {
        &self.manifest
    }

    /// Returns the plugin identifier.
    pub fn plugin_id(&self) -> &PluginId {
        self.manifest.id()
    }

    /// Returns the plugin version.
    pub fn plugin_version(&self) -> &PluginVersion {
        self.manifest.version()
    }
}

/// Errors that can occur while interacting with a plugin.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PluginError {
    /// Plugin initialization failed.
    InitializationFailed(String),

    /// Plugin shutdown failed.
    ShutdownFailed(String),

    /// Plugin execution failed.
    ExecutionFailed(String),
}

impl std::fmt::Display for PluginError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InitializationFailed(message) => {
                write!(formatter, "plugin initialization failed: {message}")
            }
            Self::ShutdownFailed(message) => {
                write!(formatter, "plugin shutdown failed: {message}")
            }
            Self::ExecutionFailed(message) => {
                write!(formatter, "plugin execution failed: {message}")
            }
        }
    }
}

impl std::error::Error for PluginError {}

/// The fundamental interface implemented by every Fily plugin.
pub trait Plugin {
    /// Returns the plugin identifier.
    fn id(&self) -> &PluginId;

    /// Returns the plugin version.
    fn version(&self) -> &PluginVersion;

    /// Initializes the plugin.
    fn initialize(&mut self, context: &PluginContext) -> Result<(), PluginError>;

    /// Shuts down the plugin.
    fn shutdown(&mut self) -> Result<(), PluginError>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{PluginCapability, PluginManifest};

    fn test_manifest() -> PluginManifest {
    PluginManifest::new(
        PluginId::new("example-plugin").unwrap(),
        "Example Plugin".to_string(),
        PluginVersion::new(1, 0, 0),
        "Fily Community".to_string(),
        "plugin_entry".to_string(),
        vec![PluginCapability::ReadFiles],
    )
    .unwrap()
    }

    struct TestPlugin {
        id: PluginId,
        version: PluginVersion,
        initialized: bool,
    }

    impl TestPlugin {
        fn new() -> Self {
            Self {
                id: PluginId::new("example-plugin").unwrap(),
                version: PluginVersion::new(1, 0, 0),
                initialized: false,
            }
        }
    }

    impl Plugin for TestPlugin {
        fn id(&self) -> &PluginId {
            &self.id
        }

        fn version(&self) -> &PluginVersion {
            &self.version
        }

        fn initialize(&mut self, _context: &PluginContext) -> Result<(), PluginError> {
            self.initialized = true;
            Ok(())
        }

        fn shutdown(&mut self) -> Result<(), PluginError> {
            self.initialized = false;
            Ok(())
        }
    }

    #[test]
    fn plugin_context_exposes_manifest_information() {
        let manifest = test_manifest();
        let context = PluginContext::new(manifest);

        assert_eq!(context.plugin_id().as_str(), "example-plugin");
        assert_eq!(context.plugin_version().to_string(), "1.0.0");
    }

    #[test]
    fn plugin_can_initialize_and_shutdown() {
        let context = PluginContext::new(test_manifest());
        let mut plugin = TestPlugin::new();

        assert!(!plugin.initialized);

        plugin.initialize(&context).unwrap();
        assert!(plugin.initialized);

        plugin.shutdown().unwrap();
        assert!(!plugin.initialized);
    }

    #[test]
    fn plugin_reports_identity() {
        let plugin = TestPlugin::new();

        assert_eq!(plugin.id().as_str(), "example-plugin");
        assert_eq!(plugin.version().to_string(), "1.0.0");
    }

    #[test]
    fn plugin_error_displays_correctly() {
        let error = PluginError::ExecutionFailed("something went wrong".into());

        assert_eq!(
            error.to_string(),
            "plugin execution failed: something went wrong"
        );
    }
}