use crate::utils::database::Database;
use anyhow::Result;
use std::io::{self, Write};

#[derive(Debug)]
pub struct CompleteCommand {
    id: i64,
}

impl CompleteCommand {
    pub fn new(id: i64) -> Self {
        Self { id }
    }

    pub fn execute(&self) -> Result<CommandResult> {
        let mut db =
            Database::new().map_err(|e| anyhow::anyhow!("Failed to open database: {}", e))?;

        // First, find the todo to check if it exists
        match db.find_todo(self.id) {
            Ok(Some(todo)) => {
                if todo.completed == 1 {
                    println!("Todo #{} is already completed", self.id);
                    return Ok(CommandResult::new(false, "Already completed".to_string()));
                }

                // Try to complete the todo
                match db.complete_todo(self.id) {
                    Ok(true) => {
                        print!("✓ Completed todo #{}: {}", self.id, todo.text);
                        io::stdout().flush().unwrap_or(());
                        println!();
                        Ok(CommandResult::new(true, "Todo completed".to_string()))
                    }
                    Ok(false) => Ok(CommandResult::new(
                        false,
                        "Failed to complete todo".to_string(),
                    )),
                    Err(e) => Ok(CommandResult::new(false, e.to_string())),
                }
            }
            Ok(None) => {
                println!("Todo #{} not found", self.id);
                Ok(CommandResult::new(false, "Todo not found".to_string()))
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
    fn test_complete_command_creation() {
        let cmd = CompleteCommand::new(42);
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
