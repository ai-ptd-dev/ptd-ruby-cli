use anyhow::{Context, Result};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: i64,
    pub text: String,
    pub priority: String,
    pub completed: i64,
    pub created_at: String,
    pub completed_at: Option<String>,
}

impl Todo {
    fn from_row(row: &Row) -> rusqlite::Result<Self> {
        Ok(Todo {
            id: row.get("id")?,
            text: row.get("text")?,
            priority: row.get("priority")?,
            completed: row.get("completed")?,
            created_at: row.get("created_at")?,
            completed_at: row.get("completed_at")?,
        })
    }

    pub fn to_hash_map(&self) -> HashMap<String, serde_json::Value> {
        let mut map = HashMap::new();
        map.insert("id".to_string(), serde_json::Value::Number(self.id.into()));
        map.insert(
            "text".to_string(),
            serde_json::Value::String(self.text.clone()),
        );
        map.insert(
            "priority".to_string(),
            serde_json::Value::String(self.priority.clone()),
        );
        map.insert(
            "completed".to_string(),
            serde_json::Value::Number(self.completed.into()),
        );
        map.insert(
            "created_at".to_string(),
            serde_json::Value::String(self.created_at.clone()),
        );

        if let Some(completed_at) = &self.completed_at {
            map.insert(
                "completed_at".to_string(),
                serde_json::Value::String(completed_at.clone()),
            );
        } else {
            map.insert("completed_at".to_string(), serde_json::Value::Null);
        }

        map
    }
}

pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = Self::get_db_path()?;
        Self::ensure_database_directory_exists(&db_path)?;

        let conn = Connection::open(&db_path)
            .with_context(|| format!("Failed to open database at {:?}", db_path))?;

        let mut db = Database { conn };
        db.setup_tables()?;

        Ok(db)
    }

    fn get_db_path() -> Result<PathBuf> {
        let mut path = std::env::current_dir().context("Failed to get current directory")?;
        path.push("tmp");
        path.push("todocli.db");
        Ok(path)
    }

    fn ensure_database_directory_exists(db_path: &Path) -> Result<()> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create directory {:?}", parent))?;
        }
        Ok(())
    }

    fn setup_tables(&mut self) -> Result<()> {
        self.conn
            .execute(
                r#"
            CREATE TABLE IF NOT EXISTS todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                text TEXT NOT NULL,
                priority TEXT DEFAULT 'medium',
                completed INTEGER DEFAULT 0,
                created_at TEXT NOT NULL,
                completed_at TEXT
            )
            "#,
                [],
            )
            .context("Failed to create todos table")?;

        Ok(())
    }

    pub fn add_todo(&mut self, text: &str, priority: &str) -> Result<i64> {
        self.conn.execute(
            "INSERT INTO todos (text, priority, completed, created_at) VALUES (?, ?, ?, datetime('now'))",
            params![text, priority, 0],
        ).context("Failed to insert todo")?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn list_todos(&self, show_completed: bool) -> Result<Vec<Todo>> {
        let sql = if show_completed {
            "SELECT * FROM todos ORDER BY completed ASC, created_at DESC"
        } else {
            r#"SELECT * FROM todos WHERE completed = 0 ORDER BY
               CASE priority
                 WHEN 'high' THEN 1
                 WHEN 'medium' THEN 2
                 WHEN 'low' THEN 3
               END, created_at DESC"#
        };

        let mut stmt = self
            .conn
            .prepare(sql)
            .context("Failed to prepare list todos query")?;

        let todo_iter = stmt
            .query_map([], Todo::from_row)
            .context("Failed to execute list todos query")?;

        let mut todos = Vec::new();
        for todo in todo_iter {
            todos.push(todo.context("Failed to parse todo from row")?);
        }

        Ok(todos)
    }

    pub fn complete_todo(&mut self, id: i64) -> Result<bool> {
        let changes = self
            .conn
            .execute(
                "UPDATE todos SET completed = 1, completed_at = datetime('now') WHERE id = ?",
                params![id],
            )
            .context("Failed to complete todo")?;

        Ok(changes > 0)
    }

    pub fn delete_todo(&mut self, id: i64) -> Result<bool> {
        let changes = self
            .conn
            .execute("DELETE FROM todos WHERE id = ?", params![id])
            .context("Failed to delete todo")?;

        Ok(changes > 0)
    }

    pub fn find_todo(&self, id: i64) -> Result<Option<Todo>> {
        let mut stmt = self
            .conn
            .prepare("SELECT * FROM todos WHERE id = ?")
            .context("Failed to prepare find todo query")?;

        let mut todo_iter = stmt
            .query_map(params![id], Todo::from_row)
            .context("Failed to execute find todo query")?;

        match todo_iter.next() {
            Some(todo) => Ok(Some(todo.context("Failed to parse todo from row")?)),
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn create_test_db() -> (Database, TempDir) {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");

        let conn = Connection::open(&db_path).unwrap();
        let mut db = Database { conn };
        db.setup_tables().unwrap();

        (db, temp_dir)
    }

    #[test]
    fn test_add_todo() {
        let (mut db, _temp_dir) = create_test_db();

        let id = db.add_todo("Test todo", "high").unwrap();
        assert!(id > 0);
    }

    #[test]
    fn test_list_todos() {
        let (mut db, _temp_dir) = create_test_db();

        db.add_todo("Todo 1", "high").unwrap();
        db.add_todo("Todo 2", "medium").unwrap();

        let todos = db.list_todos(false).unwrap();
        assert_eq!(todos.len(), 2);
        assert_eq!(todos[0].text, "Todo 1"); // High priority first
        assert_eq!(todos[1].text, "Todo 2");
    }

    #[test]
    fn test_complete_todo() {
        let (mut db, _temp_dir) = create_test_db();

        let id = db.add_todo("Test todo", "medium").unwrap();
        let result = db.complete_todo(id).unwrap();
        assert!(result);

        let todo = db.find_todo(id).unwrap().unwrap();
        assert_eq!(todo.completed, 1);
    }

    #[test]
    fn test_delete_todo() {
        let (mut db, _temp_dir) = create_test_db();

        let id = db.add_todo("Test todo", "medium").unwrap();
        let result = db.delete_todo(id).unwrap();
        assert!(result);

        let todo = db.find_todo(id).unwrap();
        assert!(todo.is_none());
    }

    #[test]
    fn test_find_todo() {
        let (mut db, _temp_dir) = create_test_db();

        let id = db.add_todo("Test todo", "low").unwrap();
        let todo = db.find_todo(id).unwrap().unwrap();

        assert_eq!(todo.id, id);
        assert_eq!(todo.text, "Test todo");
        assert_eq!(todo.priority, "low");
        assert_eq!(todo.completed, 0);
    }
}
