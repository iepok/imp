mod commands;
mod auth;

use std::env;
use clap::error::ErrorKind;
use clap::{Parser, Subcommand};

use crate::commands::analyze::analyze_command;
use crate::commands::devices::devices_command;
use crate::commands::log::log_command;
use crate::commands::login::login_command;
use crate::commands::logout::logout_command;
use crate::commands::passkey::remove_passkey_command;
use crate::commands::plan::plan_command;
use crate::commands::uninstall::uninstall_command;
use crate::commands::update::update_command;
use crate::commands::view::view_command;

#[derive(Parser, Debug)]
#[command(name = "imp", about = "Simple CLI tool", version)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Login via browser (magic link or passkey)
    Login,

    /// Sign out from this device
    Logout {
        /// Sign out from ALL devices (if device lost/stolen)
        #[arg(long, short)]
        all: bool,
    },

    /// List devices where you're signed in
    Devices,

    /// Remove selected device
    Remove {
        device_id: String,
    },

    /// Check login status
    Status,
    
    /// Make a new plan
    Plan {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        goal: Vec<String>,
    },
    
    /// Analyze your implementations
    Analyze,
    
    /// View your history  
    View,
    
    /// Update imp
    Update,
    
    /// Uninstall imp and remove it from path
    Uninstall,
}

#[tokio::main]
async fn main() {
    let args: Vec<_> = env::args().collect();
    match Args::try_parse_from(&args) {
        Ok(parsed_args) => {
            let result = match parsed_args.command {
                Commands::Login => login_command().await,
                Commands::Logout { all } => logout_command(all).await,
                Commands::Remove { device_id } => remove_passkey_command(&device_id),
                Commands::Devices => devices_command(),
                Commands::Status => Ok(()),
                Commands::Plan { goal } => plan_command(goal).await,
                Commands::Analyze => analyze_command(),
                Commands::View => view_command().await,
                Commands::Update => update_command(),
                Commands::Uninstall => uninstall_command(),
            };
            if let Err(e) = result {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        Err(err) => {
            if err.kind() == ErrorKind::InvalidSubcommand
              && args.len() > 1
              && log_command(&args[1..]).await.is_ok() {
                return;
            }
            err.exit();
        }
    }
}

