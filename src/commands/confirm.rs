use anyhow::{Result, bail};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};
use std::io::{self, Write};
use crate::auth::token_manager;

#[derive(Deserialize)]
struct ConfirmationResponse {
    id: Uuid,
    #[allow(dead_code)]
    user_id: Uuid,
    created_at: DateTime<Utc>,
    #[allow(dead_code)]
    log_id: Uuid,
    prompt_shown: String,
    choices: Vec<String>,
}

#[derive(Serialize)]
struct ConfirmRequest {
    confirmation_id: Uuid,
    raw_input: String,
}

pub async fn confirm_command() -> Result<()> {
    let token = token_manager::get_valid_token().await?;

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.iepok.com/confirm")
        .bearer_auth(&token)
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        bail!("Failed to get confirmations: {} - {}", status, body);
    }

    let confirmations: Vec<ConfirmationResponse> = response.json().await?;

    if confirmations.is_empty() {
        println!("{}", "No pending confirmations".dimmed());
        return Ok(());
    }

    println!("{} pending confirmation(s)\n", confirmations.len().to_string().bright_yellow());

    for confirmation in confirmations {
        println!("{}", "─".repeat(50).dimmed());
        println!("{} {}", "Created:".dimmed(), confirmation.created_at.format("%Y-%m-%d %H:%M"));
        println!("\n{}\n", confirmation.prompt_shown.bright_white());

        for (i, choice) in confirmation.choices.iter().enumerate() {
            println!("  {}. {}", (i + 1).to_string().bright_cyan(), choice);
        }
        println!("  {}. {}", "0".bright_cyan(), "Enter custom response");

        print!("\n{} ", "Your choice:".bright_green());
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        let raw_input = if let Ok(num) = input.parse::<usize>() {
            if num == 0 {
                print!("{} ", "Enter response:".bright_green());
                io::stdout().flush()?;
                let mut custom = String::new();
                io::stdin().read_line(&mut custom)?;
                custom.trim().to_string()
            } else if num > 0 && num <= confirmation.choices.len() {
                confirmation.choices[num - 1].clone()
            } else {
                println!("{}", "Invalid choice, skipping...".red());
                continue;
            }
        } else {
            input.to_string()
        };

        if raw_input.is_empty() {
            println!("{}", "Empty response, skipping...".red());
            continue;
        }

        let confirm_response = client
            .post("https://api.iepok.com/confirm")
            .bearer_auth(&token)
            .json(&ConfirmRequest {
                confirmation_id: confirmation.id,
                raw_input: raw_input.clone(),
            })
            .send()
            .await?;

        if confirm_response.status().is_success() {
            println!("{} {}\n", "✓".bright_green(), "Confirmed".bright_green());
        } else {
            let status = confirm_response.status();
            let body = confirm_response.text().await?;
            println!("{} {} - {}\n", "✗".red(), status, body);
        }
    }

    Ok(())
}
