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

// Enhanced CLI struct with global options
#[derive(Parser)]
#[command(name = "task-tracker")]
#[command(about = "A comprehensive task tracker CLI")]
#[command(version = "1.0")]
#[command(author = "Your Name <your.email@example.com>")]
#[command(
    long_about = "A simple yet powerful task tracking CLI tool with status management and filtering capabilities"
)]
pub struct Cli {
    /// Enable verbose output for debugging
    #[arg(short, long, global = true)]
    pub verbose: bool,

    /// Custom data file path (default: tasks.json)
    #[arg(short, long, global = true, value_name = "FILE")]
    pub file: Option<String>,

    /// Show additional timestamps and metadata
    #[arg(long, global = true)]
    pub show_metadata: bool,

    #[command(subcommand)]
    pub command: Command,
}

// Enhanced Command enum with comprehensive validation
#[derive(Subcommand)]
pub enum Command {
    /// Add a new task with description
    #[command(alias = "a")]
    #[command(
        long_about = "Create a new task with the given description. The description cannot be empty and will be trimmed of whitespace."
    )]
    Add {
        /// Task description (2-200 characters, cannot be empty)
        #[arg(
            value_name = "DESCRIPTION",
            help = "Description of the task to add",
            long_help = "The description of the task. Must be between 2-200 characters and cannot be empty or just whitespace.",
            value_parser = validate_description
        )]
        description: String,

        /// Set initial priority (1-5, where 5 is highest)
        #[arg(short, long, value_parser = validate_priority)]
        priority: Option<u8>,

        /// Add tags to the task (comma-separated)
        #[arg(short, long, value_delimiter = ',')]
        tags: Vec<String>,
    },

    /// Update an existing task's description and properties
    #[command(alias = "u")]
    #[command(
        long_about = "Update an existing task's description. Both the task ID must exist and the new description must be valid."
    )]
    Update {
        /// Task ID to update (must be a positive integer)
        #[arg(
            value_name = "ID",
            help = "ID of the task to update",
            long_help = "The unique identifier of the task to update. Must be a positive integer.",
            value_parser = validate_task_id
        )]
        id: TaskId,

        /// New task description (2-200 characters)
        #[arg(
            value_name = "DESCRIPTION",
            help = "New description for the task",
            long_help = "The new description for the task. Must be between 2-200 characters.",
            value_parser = validate_description
        )]
        description: String,

        /// Update priority (1-5, where 5 is highest)
        #[arg(short, long, value_parser = validate_priority)]
        priority: Option<u8>,
    },

    /// Delete a task permanently
    #[command(alias = "d")]
    #[command(
        long_about = "Permanently delete a task. This action cannot be undone. Use with caution."
    )]
    Delete {
        /// Task ID to delete (must be a positive integer)
        #[arg(
            value_name = "ID",
            help = "ID of the task to delete",
            long_help = "The unique identifier of the task to permanently delete. This action cannot be undone.",
            value_parser = validate_task_id
        )]
        id: TaskId,

        /// Force deletion without confirmation
        #[arg(short, long)]
        force: bool,
    },

    /// Mark task as in progress
    #[command(name = "mark-in-progress", alias = "wip")]
    #[command(
        long_about = "Mark a task as currently in progress. This indicates active work on the task."
    )]
    MarkInProgress {
        /// Task ID to mark (must be a positive integer)
        #[arg(
            value_name = "ID",
            help = "ID of the task to mark as in progress",
            value_parser = validate_task_id
        )]
        id: TaskId,

        /// Add a note about the progress
        #[arg(short, long)]
        note: Option<String>,
    },

    /// Mark task as completed
    #[command(name = "mark-done", alias = "done")]
    #[command(
        long_about = "Mark a task as completed. This indicates the task has been finished successfully."
    )]
    MarkDone {
        /// Task ID to mark (must be a positive integer)
        #[arg(
            value_name = "ID",
            help = "ID of the task to mark as done",
            value_parser = validate_task_id
        )]
        id: TaskId,

        /// Add completion notes
        #[arg(short, long)]
        note: Option<String>,
    },

    /// List tasks with filtering and sorting options
    #[command(alias = "ls")]
    #[command(
        long_about = "List tasks with various filtering and sorting options. By default shows all tasks."
    )]
    List {
        /// Filter by status (todo, in-progress, done)
        #[arg(
            value_enum,
            value_name = "STATUS",
            help = "Filter tasks by status",
            long_help = "Filter tasks by their current status. Available options: todo, in-progress, done"
        )]
        status: Option<Status>,

        /// Sort by field (id, description, status, priority)
        #[arg(long, value_enum)]
        sort_by: Option<SortField>,

        /// Reverse sort order
        #[arg(long)]
        reverse: bool,

        /// Limit number of results
        #[arg(short, long, value_parser = validate_limit)]
        limit: Option<usize>,

        /// Show only tasks with specific tags
        #[arg(long, value_delimiter = ',')]
        tags: Vec<String>,
    },

    /// Search tasks by description content
    #[command(alias = "find")]
    Search {
        /// Search query (case-insensitive)
        #[arg(
            value_name = "QUERY",
            help = "Search term to find in task descriptions",
            value_parser = validate_search_query
        )]
        query: String,

        /// Use regex pattern matching
        #[arg(long)]
        regex: bool,

        /// Case-sensitive search
        #[arg(long)]
        case_sensitive: bool,
    },
}

// Additional enum for sorting
#[derive(Debug, Clone, ValueEnum)]
pub enum SortField {
    #[value(name = "id")]
    Id,
    #[value(name = "description")]
    Description,
    #[value(name = "status")]
    Status,
    #[value(name = "priority")]
    Priority,
}

// Custom validation functions
fn validate_description(s: &str) -> Result<String, String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        Err("Description cannot be empty".to_string())
    } else if trimmed.len() < 2 {
        Err("Description must be at least 2 characters long".to_string())
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

fn validate_priority(s: &str) -> Result<u8, String> {
    match s.parse::<u8>() {
        Ok(p) if (1..=5).contains(&p) => Ok(p),
        Ok(_) => Err("Priority must be between 1 and 5".to_string()),
        Err(_) => Err("Priority must be a valid number".to_string()),
    }
}

fn validate_limit(s: &str) -> Result<usize, String> {
    match s.parse::<usize>() {
        Ok(l) if l > 0 => Ok(l),
        Ok(_) => Err("Limit must be greater than 0".to_string()),
        Err(_) => Err("Limit must be a valid number".to_string()),
    }
}

fn validate_search_query(s: &str) -> Result<String, String> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        Err("Search query cannot be empty".to_string())
    } else if trimmed.len() < 2 {
        Err("Search query must be at least 2 characters".to_string())
    } else {
        Ok(trimmed.to_string())
    }
}

// Enhanced main function with comprehensive handling
fn main() {
    let cli = Cli::parse();

    // Initialize store with custom file if provided
    // let mut store = Store::new(cli.file.as_deref());

    if cli.verbose {
        println!("🔧 Running in verbose mode...");
        if let Some(ref file) = cli.file {
            println!("📁 Using custom data file: {}", file);
        }
        if cli.show_metadata {
            println!("📊 Metadata display enabled");
        }
    }
}

/*
Enhanced usage examples:

Basic commands:
$ ./task-tracker add "Buy groceries"
$ ./task-tracker add "Study Rust" --priority 5 --tags work,learning
$ ./task-tracker update 1 "Buy organic groceries" --priority 3
$ ./task-tracker delete 1 --force
$ ./task-tracker mark-in-progress 2 --note "Started shopping"
$ ./task-tracker mark-done 2 --note "All items purchased"

Advanced listing:
$ ./task-tracker list --status todo --sort-by priority --reverse
$ ./task-tracker list --limit 10 --tags work
$ ./task-tracker ls --status done  # Using alias

Search:
$ ./task-tracker search "groceries" --case-sensitive
$ ./task-tracker find "rust.*learning" --regex

Global options:
$ ./task-tracker --verbose --file custom.json list
$ ./task-tracker --show-metadata list

Aliases:
$ ./task-tracker a "New task"     # add
$ ./task-tracker u 1 "Updated"   # update
$ ./task-tracker d 1 --force     # delete
$ ./task-tracker wip 1           # mark-in-progress
$ ./task-tracker done 1          # mark-done
$ ./task-tracker ls              # list
$ ./task-tracker find "query"   # search

Help:
$ ./task-tracker --help
$ ./task-tracker add --help
$ ./task-tracker list --help
*/
