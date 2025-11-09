use clap::{Parser, Subcommand};
use std::env;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use colored::Colorize;

mod meta;
mod app;
use meta::MetaCommands;
use app::AppCommands;

#[derive(Parser, Debug)]
#[command(name = "imp", about = "Simple CLI tool")]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(flatten)]
    Meta(MetaCommands),
    
    #[command(flatten)]
    App(AppCommands),
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    // If no args, enter interactive mode
    if args.len() == 1 {
        interactive_mode();
        return;
    }
    
    // Normal command mode
    process_command(args);
}

fn interactive_mode() {
    let mut rl = DefaultEditor::new().unwrap();
    
    println!("{}", "Interactive mode - type your commands (Ctrl+C or Ctrl+D to exit)".bright_blue());
    
    loop {
        let prompt = format!("{} ", "imp>".bright_green().bold());
        let readline = rl.readline(&prompt);
        
        match readline {
            Ok(line) => {
                let trimmed = line.trim();
                
                if trimmed.is_empty() {
                    continue;
                }
                
                // Add to history
                let _ = rl.add_history_entry(trimmed);
                
                // Build args as if typed on command line
                let mut full_args = vec!["imp".to_string()];
                full_args.extend(trimmed.split_whitespace().map(String::from));
                
                // Process the command
                process_command(full_args);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Exiting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Exiting...");
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }
}

fn process_command(args: Vec<String>) {
    // Try to parse with clap first
    match Args::try_parse_from(&args) {
        Ok(parsed_args) => {
            // Known command matched
            match parsed_args.command {
                Commands::Meta(cmd) => meta::handle_meta(cmd),
                Commands::App(cmd) => app::handle_app(cmd),
            }
        }
        Err(err) => {
            // Check if it's an unknown subcommand error
            if err.kind() == clap::error::ErrorKind::InvalidSubcommand {
                // Handle as custom log command
                if args.len() > 1 {
                    match handle_log_command(&args[1..]) {
                        Ok(_) => {},
                        Err(_) => {
                            // Show the original clap error
                            eprintln!("{}", err);
                        }
                    }
                } else {
                    eprintln!("{}", err);
                }
            } else {
                // Other errors (like --help, invalid args) - let clap handle them
                eprintln!("{}", err);
            }
        }
    }
}

fn handle_log_command(args: &[String]) -> Result<(), ()> {
    if args.is_empty() {
        return Err(());
    }
    
    let content = args.join(" ");
    
    // Apply your space-check logic
    if !content.contains(' ') {
        return Err(());
    }
    
    println!("{} {}", "Logging:".bright_green().bold(), content.cyan());
    
    Ok(())
}
