use std::env;
use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;
use clap::error::ErrorKind;
use clap::{Parser, Subcommand};
use colored::Colorize;
use rustyline::{DefaultEditor, error::ReadlineError};

#[derive(Parser, Debug)]
#[command(name = "imp", about = "Simple CLI tool", version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Make a new plan
    Plan {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        goal: Vec<String>,
    },
    
    /// Analyze your implementations
    Analyze,
    
    /// View your history  
    View,
    
    /// Uninstall imp and remove it from path
    Uninstall,

    /// Update imp
    Update,
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
        Ok(parsed_args) => handle_command(parsed_args.command),
        Err(err) => {
            if err.kind() == ErrorKind::InvalidSubcommand && args.len() > 1 {
                if handle_log_command(&args[1..]).is_ok() {
                    return;
                }
            }
            err.exit();
        }
    }
}

fn handle_command(cmd: Commands) {
    match cmd {
        Commands::Plan { goal } => {
            println!("Creating plan: {}", goal.join(" "));
        }
        Commands::Analyze => {
            println!("Analyzing...");
        }
        Commands::View => {
            println!("Viewing history...");
        }
        Commands::Uninstall => handle_uninstall(),
        Commands::Update => {
            println!("Updating imp...");
            let status = Command::new("sh")
                .arg("-c")
                .arg("curl -fsSL https://api.iepok.com/imp/install | sh")
                .status();
            
            match status {
                Ok(s) if s.success() => println!("✅ Updated!"),
                _ => eprintln!("Update failed"),
            }
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

fn handle_uninstall() {
    if cfg!(windows) {
        handle_uninstall_windows();
    } else {
        handle_uninstall_unix();
    }
}

fn handle_uninstall_windows() {
    let local_app_data = env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
    let imp_dir = format!(r"{}\imp", local_app_data);
    let imp_path = format!(r"{}\imp.exe", imp_dir);

    // Remove binary
    if let Err(e) = fs::remove_file(&imp_path) {
        eprintln!("Failed to remove imp binary: {}", e);
    } else {
        println!("Removed {}", imp_path);
    }

    // Remove directory
    let _ = fs::remove_dir(&imp_dir);

    // Remove from PATH using PowerShell
    let ps_script = format!(
        r#"
        $p = [Environment]::GetEnvironmentVariable('Path','User')
        $new = ($p -split ';' | Where-Object {{ $_ -ne '{}' }}) -join ';'
        [Environment]::SetEnvironmentVariable('Path',$new,'User')
        "#,
        imp_dir
    );

    let status = Command::new("powershell")
        .args(&["-NoProfile", "-Command", &ps_script])
        .status();

    match status {
        Ok(s) if s.success() => println!("Removed {} from PATH", imp_dir),
        Ok(_) => eprintln!("PowerShell command failed to update PATH"),
        Err(e) => eprintln!("Failed to run PowerShell: {}", e),
    }

    println!("✅ imp uninstalled");
}

fn handle_uninstall_unix() {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let imp_path = format!("{}/.local/bin/imp", home);
    
    if let Err(e) = fs::remove_file(&imp_path) {
        eprintln!("Failed to remove imp binary: {}", e);
    } else {
        println!("Removed {}", imp_path);
    }
    
    remove_path_from_file(&format!("{}/.bashrc", home));
    remove_path_from_file(&format!("{}/.zshrc", home));
    
    println!("✅ imp uninstalled");
}

fn remove_path_from_file(file_path: &str) {
    let Ok(file) = fs::File::open(file_path) else {
        return;
    };
    
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.contains(".local/bin") || !line.contains("imp"))
        .collect();
    
    if let Ok(mut file) = fs::File::create(file_path) {
        for line in lines {
            let _ = writeln!(file, "{}", line);
        }
    }
}

