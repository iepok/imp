use std::process::Command;
use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "imp", about = "Simple CLI tool")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Open the imp source in nvim
    Edit,
    /// Install imp using cargo
    Install,
    /// Git add, commit, and push with a message
    Commit {
        /// Commit message
        message: String,
    },
}

fn get_project_dir() -> String {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/projects/imp", home)
}

fn handle_edit() {
    let target_dir = get_project_dir();
    
    let status = Command::new("nvim")
        .arg("src/main.rs")
        .current_dir(&target_dir)
        .status();
    
    match status {
        Ok(exit_status) => {
            if !exit_status.success() {
                eprintln!("nvim exited with status: {}", exit_status);
            }
        }
        Err(e) => {
            eprintln!("Failed to run nvim: {}", e);
        }
    }
}

fn handle_install() {
    let target_dir = get_project_dir();
    
    println!("Installing imp from {}...", target_dir);
    
    let status = Command::new("cargo")
        .args(["install", "--path", "."])
        .current_dir(&target_dir)
        .status();
    
    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                println!("✓ Installation complete!");
            } else {
                eprintln!("Installation failed with status: {}", exit_status);
            }
        }
        Err(e) => {
            eprintln!("Failed to run cargo: {}", e);
        }
    }
}

fn handle_push(message: &str) {
    let target_dir = get_project_dir();
    
    println!("Adding files...");
    let add_status = Command::new("git")
        .args(["add", "."])
        .current_dir(&target_dir)
        .status();
    
    match add_status {
        Ok(status) if status.success() => {
            println!("✓ Files added");
        }
        Ok(status) => {
            eprintln!("git add failed with status: {}", status);
            return;
        }
        Err(e) => {
            eprintln!("Failed to run git add: {}", e);
            return;
        }
    }
    
    println!("Committing with message: \"{}\"", message);
    let commit_status = Command::new("git")
        .args(["commit", "-m", message])
        .current_dir(&target_dir)
        .status();
    
    match commit_status {
        Ok(status) if status.success() => {
            println!("✓ Committed");
        }
        Ok(status) => {
            eprintln!("git commit failed with status: {}", status);
            return;
        }
        Err(e) => {
            eprintln!("Failed to run git commit: {}", e);
            return;
        }
    }
    
    println!("Pushing to remote...");
    let push_status = Command::new("git")
        .args(["push"])
        .current_dir(&target_dir)
        .status();
    
    match push_status {
        Ok(status) if status.success() => {
            println!("✓ Pushed successfully!");
        }
        Ok(status) => {
            eprintln!("git push failed with status: {}", status);
        }
        Err(e) => {
            eprintln!("Failed to run git push: {}", e);
        }
    }
}

fn main() {
    let args = Args::parse();
    
    match args.command {
        Commands::Edit => handle_edit(),
        Commands::Install => handle_install(),
        Commands::Commit { message } => handle_push(&message),
    }
}
