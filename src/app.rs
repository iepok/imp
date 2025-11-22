use clap::Subcommand;

#[derive(Subcommand, Debug)]
pub enum AppCommands {
    /// Make a new plan
    Plan {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        goal: Vec<String>,
    },
    
    /// Analyze your implementations
    Analyze,
    
    /// View your history  
    View,

    /// Uninstall imp and removing it from path
    Uninstall,
}

pub fn handle_app(cmd: AppCommands) {
    match cmd {
        AppCommands::Plan { goal } => {
            println!("Creating plan: {}", goal.join(" "));
        }
        AppCommands::Analyze => {
            println!("Analyzing...");
        }
        AppCommands::View => {
            println!("Viewing history...");
        }
        
        AppCommands::Uninstall => handle_uninstall(),
    }
}

fn handle_uninstall() {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let imp_path = format!("{}/.local/bin/imp", home);
    
    if let Err(e) = std::fs::remove_file(&imp_path) {
        eprintln!("Failed to remove imp binary: {}", e);
    } else {
        println!("Removed {}", imp_path);
    }
    
    remove_path_from_file(&format!("{}/.bashrc", home));
    remove_path_from_file(&format!("{}/.zshrc", home));
    
    println!("✅ imp uninstalled");
}

fn remove_path_from_file(file_path: &str) {
    use std::fs;
    use std::io::{BufRead, BufReader, Write};
    
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
