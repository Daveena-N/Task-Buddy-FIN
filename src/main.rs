use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{Read, Write};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    id: u32,
    description: String,
    completed: bool,
}

#[derive(Debug)]
struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        let mut manager = TaskManager { tasks: Vec::new() };
        manager.load();
        manager
    }

    fn load(&mut self) {
        let file_path = "tasks.json";
        let mut file = match File::open(file_path) {
            Ok(file) => file,
            Err(_) => {
                File::create(file_path).expect("Failed to create tasks.json");
                return;
            }
        };

        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Failed to read file");
        if !contents.is_empty() {
            self.tasks = serde_json::from_str(&contents).expect("Failed to parse tasks");
        }
    }

    fn save(&self) {
        let file_path = "tasks.json";
        let json = serde_json::to_string_pretty(&self.tasks).expect("Failed to serialize tasks");
        let mut file = File::create(file_path).expect("Failed to create file");
        file.write_all(json.as_bytes()).expect("Failed to write to file");
    }

    fn add_task(&mut self, description: String) {
        let id = if let Some(last) = self.tasks.last() {
            last.id + 1
        } else {
            1
        };
        let task = Task {
            id,
            description,
            completed: false,
        };
        self.tasks.push(task.clone());
        self.save();
        println!("Task added: {}", task.description);
    }

    fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No tasks available.");
            return;
        }

        println!("ID\tDescription\t\tStatus");
        println!("----------------------------------");
        for task in &self.tasks {
            println!(
                "{}\t{}\t\t{}",
                task.id,
                task.description,
                if task.completed { "Completed" } else { "Pending" }
            );
        }
    }

    fn mark_as_done(&mut self, id: u32) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            self.save();
            println!("Task {} marked as done.", id);
        } else {
            println!("Task not found.");
        }
    }

    fn remove_task(&mut self, id: u32) {
        self.tasks.retain(|t| t.id != id);
        self.save();
        println!("Task {} removed.", id);
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>, // Make the command optional
}

#[derive(Subcommand, Debug)]
enum Commands {
    Add {
        description: String,
    },
    List,
    Done {
        id: u32,
    },
    Remove {
        id: u32,
    },
}

fn main() {
    let cli = Cli::parse();
    let mut manager = TaskManager::new();

    // Default to listing tasks if no command is provided
    match &cli.command {
        Some(Commands::Add { description }) => manager.add_task(description.clone()),
        Some(Commands::List) | None => manager.list_tasks(),
        Some(Commands::Done { id }) => manager.mark_as_done(*id),
        Some(Commands::Remove { id }) => manager.remove_task(*id),
    }
}
