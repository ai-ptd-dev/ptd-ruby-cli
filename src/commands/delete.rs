use crate::utils::database::Database;
use anyhow::Result;

#[derive(Debug)]
pub struct DeleteCommand {
    id: i64,
}

impl DeleteCommand {
    pub fn new(id: i64) -> Self {
        Self { id }
    }

    pub fn execute(&self) -> Result<CommandResult> {
        let mut db =
            Database::new().map_err(|e| anyhow::anyhow!("Failed to open database: {}", e))?;

        // First, find the todo to check if it exists and get its text
        match db.find_todo(self.id) {
            Ok(Some(todo)) => {
                // Try to delete the todo
                match db.delete_todo(self.id) {
                    Ok(true) => {
                        println!("✗ Deleted todo #{}: {}", self.id, todo.text);
                        Ok(CommandResult::new(true, "Todo deleted".to_string()))
                    }
                    Ok(false) => {
                        println!("Failed to delete todo #{}", self.id);
                        Ok(CommandResult::new(
                            false,
                            "Failed to delete todo".to_string(),
                        ))
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                        Ok(CommandResult::new(false, e.to_string()))
                    }
                }
            }
            Ok(None) => {
                println!("Todo #{} not found", self.id);
                Ok(CommandResult::new(false, "Todo not found".to_string()))
            }
            Err(e) => {
                println!("Error: {}", e);
                Ok(CommandResult::new(false, e.to_string()))
            }
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
    fn test_delete_command_creation() {
        let cmd = DeleteCommand::new(42);
        assert_eq!(cmd.id, 42);
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
