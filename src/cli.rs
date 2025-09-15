use anyhow::Result;
use clap::{Parser, Subcommand};

mod commands {
    pub mod add;
    pub mod complete;
    pub mod delete;
    pub mod hello;
    pub mod list;
    pub mod version;
}

mod utils {
    pub mod database;
    pub mod file_handler;
    pub mod logger;
}

use commands::{
    add::AddCommand, complete::CompleteCommand, delete::DeleteCommand, list::ListCommand,
    version::VersionCommand,
};

#[derive(Parser)]
#[command(name = "todocli-rust")]
#[command(author, version, about = "TodoCli - High Performance Todo Manager (Rust Version)", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new todo item
    Add {
        /// Text of the todo item
        text: String,

        /// Priority: high, medium, or low
        #[arg(long, default_value = "medium")]
        priority: String,
    },

    /// List all todos
    List {
        /// Show completed todos too
        #[arg(short, long)]
        all: bool,

        /// Output format: table or json
        #[arg(long, default_value = "table")]
        format: String,
    },

    /// Mark a todo as completed
    Complete {
        /// ID of the todo to complete
        id: i64,
    },

    /// Delete a todo
    Delete {
        /// ID of the todo to delete
        id: i64,
    },

    /// Display version information
    Version {
        /// Output version info as JSON
        #[arg(long)]
        json: bool,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Add { text, priority } => {
            let command = AddCommand::new(text, Some(priority));
            command.execute()?;
        }
        Commands::List { all, format } => {
            let command = ListCommand::new(all, Some(format));
            command.execute()?;
        }
        Commands::Complete { id } => {
            let command = CompleteCommand::new(id);
            command.execute()?;
        }
        Commands::Delete { id } => {
            let command = DeleteCommand::new(id);
            command.execute()?;
        }
        Commands::Version { json } => {
            let command = VersionCommand::new(json);
            command.execute()?;
        }
    }

    Ok(())
}
