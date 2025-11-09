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
    }
}
