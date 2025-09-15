use crate::utils::database::{Database, Todo};
use anyhow::Result;
use colored::*;

#[derive(Debug)]
pub struct ListCommand {
    show_completed: bool,
    format: String,
}

impl ListCommand {
    pub fn new(show_completed: bool, format: Option<String>) -> Self {
        Self {
            show_completed,
            format: format.unwrap_or_else(|| "table".to_string()),
        }
    }

    pub fn execute(&self) -> Result<CommandResult> {
        let db = Database::new().map_err(|e| anyhow::anyhow!("Failed to open database: {}", e))?;

        match db.list_todos(self.show_completed) {
            Ok(todos) => {
                if todos.is_empty() {
                    println!("No todos found.");
                    return Ok(CommandResult::new(true, "No todos".to_string()));
                }

                match self.format.as_str() {
                    "json" => self.output_json(&todos)?,
                    _ => self.output_table(&todos),
                }

                Ok(CommandResult::new(
                    true,
                    format!("Listed {} todos", todos.len()),
                ))
            }
            Err(e) => Ok(CommandResult::new(false, e.to_string())),
        }
    }

    fn output_table(&self, todos: &[Todo]) {
        println!("\nTodos:");
        println!("{}", "─".repeat(80));

        for todo in todos {
            let status = if todo.completed == 1 { "✓" } else { "○" };
            let priority_text = format!("[{}]", todo.priority.to_uppercase());

            let colored_priority = match todo.priority.as_str() {
                "high" => priority_text.red(),
                "medium" => priority_text.yellow(),
                "low" => priority_text.green(),
                _ => priority_text.white(),
            };

            if todo.completed == 1 {
                println!(
                    "{} {:>3} {} {}",
                    status,
                    todo.id,
                    todo.text.bright_black(),
                    colored_priority
                );
            } else {
                println!(
                    "{} {:>3} {} {}",
                    status, todo.id, todo.text, colored_priority
                );
            }
        }

        println!("{}", "─".repeat(80));
        let completed_count = todos.iter().filter(|t| t.completed == 1).count();
        let pending_count = todos.iter().filter(|t| t.completed == 0).count();
        println!(
            "Total: {} ({} pending, {} completed)",
            todos.len(),
            pending_count,
            completed_count
        );
    }

    fn output_json(&self, todos: &[Todo]) -> Result<()> {
        let json_todos: Vec<_> = todos.iter().map(|todo| todo.to_hash_map()).collect();
        let json_output = serde_json::to_string_pretty(&json_todos)
            .map_err(|e| anyhow::anyhow!("Failed to serialize todos to JSON: {}", e))?;
        println!("{}", json_output);
        Ok(())
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
    fn test_list_command_creation() {
        let cmd = ListCommand::new(true, Some("json".to_string()));
        assert!(cmd.show_completed);
        assert_eq!(cmd.format, "json");
    }

    #[test]
    fn test_list_command_defaults() {
        let cmd = ListCommand::new(false, None);
        assert!(!cmd.show_completed);
        assert_eq!(cmd.format, "table");
    }

    #[test]
    fn test_command_result() {
        let result = CommandResult::new(true, "Success".to_string());
        assert!(result.is_success());
        assert_eq!(result.message, "Success");
    }
}
