/// Capabilities that a Fily plugin can request.
///
/// These capabilities describe what a plugin is intended to access or
/// interact with. The actual permission enforcement will be implemented
/// later by the plugin runtime.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginCapability {
    ReadFiles,
    WriteFiles,
    CreateFiles,
    DeleteFiles,
    ReadDirectories,
    CreateDirectories,
    DeleteDirectories,
    Navigation,
    Commands,
}

impl PluginCapability {
    /// Returns the stable string identifier used in plugin manifests.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::ReadFiles => "read_files",
            Self::WriteFiles => "write_files",
            Self::CreateFiles => "create_files",
            Self::DeleteFiles => "delete_files",
            Self::ReadDirectories => "read_directories",
            Self::CreateDirectories => "create_directories",
            Self::DeleteDirectories => "delete_directories",
            Self::Navigation => "navigation",
            Self::Commands => "commands",
        }
    }

    /// Returns every capability supported by this version of the API.
    pub fn all() -> &'static [PluginCapability] {
        &[
            Self::ReadFiles,
            Self::WriteFiles,
            Self::CreateFiles,
            Self::DeleteFiles,
            Self::ReadDirectories,
            Self::CreateDirectories,
            Self::DeleteDirectories,
            Self::Navigation,
            Self::Commands,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::PluginCapability;

    #[test]
    fn capability_strings_are_correct() {
        assert_eq!(PluginCapability::ReadFiles.as_str(), "read_files");
        assert_eq!(PluginCapability::WriteFiles.as_str(), "write_files");
        assert_eq!(PluginCapability::CreateFiles.as_str(), "create_files");
        assert_eq!(PluginCapability::DeleteFiles.as_str(), "delete_files");
        assert_eq!(
            PluginCapability::ReadDirectories.as_str(),
            "read_directories"
        );
        assert_eq!(
            PluginCapability::CreateDirectories.as_str(),
            "create_directories"
        );
        assert_eq!(
            PluginCapability::DeleteDirectories.as_str(),
            "delete_directories"
        );
        assert_eq!(PluginCapability::Navigation.as_str(), "navigation");
        assert_eq!(PluginCapability::Commands.as_str(), "commands");
    }

    #[test]
    fn all_contains_every_capability() {
        assert_eq!(PluginCapability::all().len(), 9);

        for capability in PluginCapability::all() {
            assert!(!capability.as_str().is_empty());
        }
    }

    #[test]
    fn capability_identifiers_are_unique() {
        let capabilities = PluginCapability::all();

        for (index, capability) in capabilities.iter().enumerate() {
            for other in capabilities.iter().skip(index + 1) {
                assert_ne!(
                    capability.as_str(),
                    other.as_str(),
                    "duplicate capability identifier: {}",
                    capability.as_str()
                );
            }
        }
    }
}