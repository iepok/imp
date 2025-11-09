use clap::Subcommand;
use std::process::Command;

#[derive(Subcommand, Debug)]
pub enum MetaCommands {
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

pub fn handle_meta(cmd: MetaCommands) {
    match cmd {
        MetaCommands::Edit => handle_edit(),
        MetaCommands::Install => handle_install(),
        MetaCommands::Commit { message } => handle_push(&message),
    }
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
        Ok(s) if s.success() => {}
        Ok(s) => eprintln!("nvim exited with status: {}", s),
        Err(e) => eprintln!("Failed to run nvim: {}", e),
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
        Ok(s) if s.success() => println!("✓ Installation complete!"),
        Ok(s) => eprintln!("Installation failed with status: {}", s),
        Err(e) => eprintln!("Failed to run cargo: {}", e),
    }
}

fn handle_push(message: &str) {
    if let Err(e) = try_push(message) {
        eprintln!("Error: {}", e);
    }
}

fn try_push(message: &str) -> Result<(), String> {
    let target_dir = get_project_dir();

    let run = |args: &[&str]| -> Result<(), String> {
        Command::new("git")
            .args(args)
            .current_dir(&target_dir)
            .status()
            .map_err(|e| e.to_string())?
            .success()
            .then_some(())
            .ok_or_else(|| "Command failed".to_string())
    };

    println!("Adding files...");
    run(&["add", "."])?;

    println!("Committing...");
    run(&["commit", "-m", message])?;

    println!("Pushing...");
    run(&["push"]).or_else(|_| {
        println!("Setting upstream...");
        run(&["push", "--set-upstream", "origin", "HEAD"])
    })?;

    println!("✓ Success!");
    Ok(())
}

