/// Context supplied to a plugin when one of its commands is executed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommandContext {
    current_path: String,
    selected_paths: Vec<String>,
}

impl CommandContext {
    /// Creates a command context.
    pub fn new(
        current_path: impl Into<String>,
        selected_paths: Vec<String>,
    ) -> Self {
        Self {
            current_path: current_path.into(),
            selected_paths,
        }
    }

    /// Returns the directory currently being viewed.
    pub fn current_path(&self) -> &str {
        &self.current_path
    }

    /// Returns the paths currently selected by the user.
    pub fn selected_paths(&self) -> &[String] {
        &self.selected_paths
    }

    /// Returns whether the user has selected anything.
    pub fn has_selection(&self) -> bool {
        !self.selected_paths.is_empty()
    }
}

/// Result returned after a plugin command executes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommandResult {
    /// The command completed successfully.
    Success,

    /// The command completed and provides a message for Fily to display.
    Message(String),

    /// The command failed.
    Failed(String),
}

impl CommandResult {
    /// Creates a successful result.
    pub fn success() -> Self {
        Self::Success
    }

    /// Creates a successful result with a message.
    pub fn message(message: impl Into<String>) -> Self {
        Self::Message(message.into())
    }

    /// Creates a failed result.
    pub fn failed(message: impl Into<String>) -> Self {
        Self::Failed(message.into())
    }

    /// Returns whether the command succeeded.
    pub fn is_success(&self) -> bool {
        matches!(self, Self::Success | Self::Message(_))
    }

    /// Returns whether the command failed.
    pub fn is_failed(&self) -> bool {
        matches!(self, Self::Failed(_))
    }
}

/// A command exposed by a Fily plugin.
///
/// A `PluginCommand` contains the metadata Fily needs to identify
/// and display a command. Execution is handled separately through
/// [`CommandContext`] and [`CommandResult`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PluginCommand {
    id: String,
    name: String,
    description: String,
}

impl PluginCommand {
    /// Creates a new plugin command.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        description: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: description.into(),
        }
    }

    /// Returns the unique command identifier.
    pub fn id(&self) -> &str {
        &self.id
    }

    /// Returns the human-readable command name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the command description.
    pub fn description(&self) -> &str {
        &self.description
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_is_created_correctly() {
        let command = PluginCommand::new(
            "compress-files",
            "Compress Files",
            "Compresses the selected files into an archive.",
        );

        assert_eq!(command.id(), "compress-files");
        assert_eq!(command.name(), "Compress Files");
        assert_eq!(
            command.description(),
            "Compresses the selected files into an archive."
        );
    }

    #[test]
    fn command_supports_owned_strings() {
        let id = String::from("test-command");
        let name = String::from("Test Command");
        let description = String::from("A test command.");

        let command = PluginCommand::new(id, name, description);

        assert_eq!(command.id(), "test-command");
        assert_eq!(command.name(), "Test Command");
        assert_eq!(command.description(), "A test command.");
    }

    #[test]
    fn commands_can_be_compared() {
        let first = PluginCommand::new(
            "test",
            "Test",
            "Test command",
        );

        let second = PluginCommand::new(
            "test",
            "Test",
            "Test command",
        );

        assert_eq!(first, second);
    }

    #[test]
    fn command_context_reports_selection() {
        let context = CommandContext::new(
            "C:\\Users\\Paras\\Documents",
            vec![
                "C:\\Users\\Paras\\Documents\\file1.txt".to_string(),
                "C:\\Users\\Paras\\Documents\\file2.txt".to_string(),
            ],
        );

        assert_eq!(
            context.current_path(),
            "C:\\Users\\Paras\\Documents"
        );

        assert_eq!(context.selected_paths().len(), 2);
        assert!(context.has_selection());
    }

    #[test]
    fn command_context_handles_empty_selection() {
        let context = CommandContext::new(
            "C:\\Users\\Paras\\Documents",
            Vec::new(),
        );

        assert!(!context.has_selection());
        assert!(context.selected_paths().is_empty());
    }

    #[test]
    fn command_result_success_is_successful() {
        let result = CommandResult::success();

        assert!(result.is_success());
        assert!(!result.is_failed());
    }

    #[test]
    fn command_result_message_is_successful() {
        let result = CommandResult::message(
            "Files compressed successfully."
        );

        assert!(result.is_success());
        assert_eq!(
            result,
            CommandResult::Message(
                "Files compressed successfully.".to_string()
            )
        );
    }

    #[test]
    fn command_result_failure_is_failed() {
        let result = CommandResult::failed(
            "Unable to compress files."
        );

        assert!(!result.is_success());
        assert!(result.is_failed());
    }
}