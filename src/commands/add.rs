use crate::utils::database::Database;
use anyhow::Result;

#[derive(Debug)]
pub struct AddCommand {
    text: String,
    priority: String,
}

impl AddCommand {
    pub fn new(text: String, priority: Option<String>) -> Self {
        Self {
            text,
            priority: priority.unwrap_or_else(|| "medium".to_string()),
        }
    }

    pub fn execute(&self) -> Result<CommandResult> {
        let mut db =
            Database::new().map_err(|e| anyhow::anyhow!("Failed to open database: {}", e))?;

        match db.add_todo(&self.text, &self.priority) {
            Ok(id) => {
                println!("✓ Added todo #{}: {} [{}]", id, self.text, self.priority);
                Ok(CommandResult::new(
                    true,
                    format!("Todo added with ID {}", id),
                ))
            }
            Err(e) => Ok(CommandResult::new(false, e.to_string())),
        }
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct CommandResult {
    pub success: bool,
    pub message: String,
}

#[allow(dead_code)]
impl CommandResult {
    pub fn new(success: bool, message: String) -> Self {
        Self { success, message }
    }

    pub fn is_success(&self) -> bool {
        self.success
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_command_creation() {
        let cmd = AddCommand::new("Test todo".to_string(), Some("high".to_string()));
        assert_eq!(cmd.text, "Test todo");
        assert_eq!(cmd.priority, "high");
    }

    #[test]
    fn test_add_command_default_priority() {
        let cmd = AddCommand::new("Test todo".to_string(), None);
        assert_eq!(cmd.text, "Test todo");
        assert_eq!(cmd.priority, "medium");
    }

    #[test]
    fn test_command_result() {
        let result = CommandResult::new(true, "Success".to_string());
        assert!(result.is_success());
        assert_eq!(result.message, "Success");

        let result = CommandResult::new(false, "Error".to_string());
        assert!(!result.is_success());
        assert_eq!(result.message, "Error");
    }
}
