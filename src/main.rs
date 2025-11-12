mod meta;
mod app;
use std::env;

use clap::{Parser, Subcommand, command};
use colored::Colorize;
use meta::MetaCommands;
use app::AppCommands;
use rustyline::{DefaultEditor, error::ReadlineError};

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
    let args: Vec<_> = env::args().collect();
    if args.len() == 1 {
        interactive_mode();
    } else {
        process_command(args);
    }
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
                rl.add_history_entry(trimmed).ok();
                
                // Build args as if typed on command line
                let mut full_args = vec!["imp".into()];
                full_args.extend(trimmed.split_whitespace().map(String::from));
                process_command(full_args);
            }
            Err(ReadlineError::Interrupted | ReadlineError::Eof) => {
                break println!("Exiting...");
            }
            Err(err) => {
                break eprintln!("Error: {:?}", err);
            }
        }
    }
}

fn process_command(args: Vec<String>) {
    match Args::try_parse_from(&args) {
        Ok(parsed_args) => match parsed_args.command {
            Commands::Meta(cmd) => meta::handle_meta(cmd),
            Commands::App(cmd) => app::handle_app(cmd),
        }
        Err(err) => {
            // Check if it's an unknown subcommand error
            if err.kind() == clap::error::ErrorKind::InvalidSubcommand && args.len() > 1 {
                if handle_log_command(&args[1..]).is_ok() {
                    return;
                }
            }
            err.exit();
        }
    }
}

fn handle_log_command(args: &[String]) -> Result<(), ()> {
    if args.is_empty() {
        return Err(());
    }
    
    let content = args.join(" ");
    if !content.contains(' ') {
        return Err(());
    }
    
    println!("{} {}", "Logging:".bright_green().bold(), content.cyan());
    Ok(())
}
