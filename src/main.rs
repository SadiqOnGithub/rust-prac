use clap::{Parser, Subcommand, ValueEnum};

// Your existing types
pub type TaskId = u32;

#[derive(Debug, Clone, ValueEnum)]
pub enum Status {
    #[value(name = "todo")]
    ToDo,
    #[value(name = "in-progress")]
    InProgress,
    #[value(name = "done")]
    Done,
}

// Main CLI struct with global options
#[derive(Parser)]
#[command(name = "task-tracker")]
#[command(about = "A simple task tracker CLI")]
#[command(version = "1.0")]
#[command(author = "Your Name")]
#[command(long_about = "A comprehensive task tracking CLI tool with status management")]
pub struct Cli {
    /// Enable verbose output
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Custom data file path
    #[arg(short, long, global = true, value_name = "FILE")]
    pub file: Option<String>,

    #[command(subcommand)]
    pub command: Command,
}

// Your EXACT Command enum structure with comprehensive validation
#[derive(Subcommand)]
pub enum Command {
    /// Add a new task with description
    #[command(alias = "a")]
    Add(
        /// Task description (must not be empty)
        #[arg(
            value_name = "DESCRIPTION",
            help = "Description of the task to add",
            long_help = "The description of the task. Cannot be empty and should be meaningful.",
            value_parser = validate_non_empty_string
        )]
        String,
    ),

    /// Update an existing task's description
    #[command(alias = "u")]
    Update(
        /// Task ID to update (must be positive)
        #[arg(
            value_name = "ID",
            help = "ID of the task to update",
            value_parser = validate_task_id
        )]
        TaskId,
        /// New task description (must not be empty)
        #[arg(
            value_name = "DESCRIPTION",
            help = "New description for the task",
            value_parser = validate_non_empty_string
        )]
        String,
    ),

    /// Delete a task permanently
    #[command(alias = "d")]
    Delete(
        /// Task ID to delete (must be positive)
        #[arg(
            value_name = "ID",
            help = "ID of the task to delete",
            long_help = "The ID of the task to permanently delete. This action cannot be undone.",
            value_parser = validate_task_id
        )]
        TaskId,
    ),

    /// Mark task as in progress
    #[command(name = "mark-in-progress", alias = "wip")]
    MarkInProgress(
        /// Task ID to mark (must be positive)
        #[arg(
            value_name = "ID",
            help = "ID of the task to mark as in progress",
            value_parser = validate_task_id
        )]
        TaskId,
    ),

    /// Mark task as completed
    #[command(name = "mark-done", alias = "done")]
    MarkDone(
        /// Task ID to mark (must be positive)
        #[arg(
            value_name = "ID",
            help = "ID of the task to mark as done",
            value_parser = validate_task_id
        )]
        TaskId,
    ),

    /// List tasks with optional filtering
    #[command(alias = "ls")]
    List(
        /// Filter by status (optional)
        #[arg(
            value_enum,
            value_name = "STATUS",
            help = "Filter tasks by status",
            long_help = "Filter tasks by their current status. If not provided, shows all tasks."
        )]
        Option<Status>,
    ),
}

// Custom validation functions
fn validate_non_empty_string(s: &str) -> Result<String, String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        Err("Description cannot be empty".to_string())
    } else if trimmed.len() > 200 {
        Err("Description too long (max 200 characters)".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

fn validate_task_id(s: &str) -> Result<TaskId, String> {
    match s.parse::<TaskId>() {
        Ok(id) if id > 0 => Ok(id),
        Ok(_) => Err("Task ID must be greater than 0".to_string()),
        Err(_) => Err("Task ID must be a valid positive number".to_string()),
    }
}

// Enhanced main function with validation feedback
fn main() {
    let cli = Cli::parse();

    // Initialize store with custom file if provided
    // let mut store = Store::new(cli.file.as_deref());

    if cli.verbose {
        println!("Running in verbose mode...");
        if let Some(ref file) = cli.file {
            println!("Using custom data file: {}", file);
        }
    }

    // Your existing match logic works EXACTLY the same!
    // But now all inputs are pre-validated by clap
    // match cli.command {
    //     Command::Add(description) => {
    //         // description is guaranteed to be non-empty and <= 200 chars
    //         let new_id = store.add_task(description);
    //         println!("Added new task with ID {new_id}");
    //         if cli.verbose {
    //             println!("Task added successfully to store");
    //         }
    //     }
    //     Command::Delete(id) => {
    //         // id is guaranteed to be > 0
    //         store.delete_task(id);
    //         if cli.verbose {
    //             println!("Attempted to delete task with ID {}", id);
    //         }
    //     }
    //     Command::Update(id, description) => {
    //         // Both id and description are pre-validated
    //         store.update_task(id, description);
    //         if cli.verbose {
    //             println!("Updated task {} with new description", id);
    //         }
    //     }
    //     Command::MarkInProgress(id) => {
    //         store.mark_task(id, Status::InProgress);
    //         if cli.verbose {
    //             println!("Marked task {} as in progress", id);
    //         }
    //     }
    //     Command::MarkDone(id) => {
    //         store.mark_task(id, Status::Done);
    //         if cli.verbose {
    //             println!("Marked task {} as done", id);
    //         }
    //     }
    //     Command::List(status) => {
    //         let result = store.list_tasks(status);
    //         if cli.verbose {
    //             let filter_msg = match status {
    //                 Some(s) => format!("Listing tasks with status: {:?}", s),
    //                 None => "Listing all tasks".to_string(),
    //             };
    //             println!("{}", filter_msg);
    //         }

    //         for item in result {
    //             println!(
    //                 "ID {0}: {1}. Status: {2}",
    //                 item.id(),
    //                 item.description(),
    //                 item.status()
    //             );
    //         }
    //     }
    // }
}

// Add this to your Cargo.toml:
// [dependencies]
// clap = { version = "4.0", features = ["derive"] }
