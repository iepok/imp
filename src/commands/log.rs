use colored::Colorize;
use anyhow::{Result, bail};
use crate::auth::token_manager;

pub async fn log_command(args: &[String]) -> Result<()> {
    if !args[0].contains(' ') {
        bail!("Invalid log command");
    }

    let token = token_manager::get_valid_token().await?;

    println!("{} {}", "Logging:".bright_green().bold(), args[0].cyan());

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.iepok.com/log")
        .bearer_auth(token)
        .json(&serde_json::json!({
            "raw_input": args[0]
        }))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        bail!("Failed to log: {} - {}", status, body);
    }

    println!("{}", "✓ Logged".bright_green());

    Ok(())
}

