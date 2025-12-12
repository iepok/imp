use anyhow::{Result, bail};
use colored::Colorize;
use crate::auth::token_manager;
use serde::Deserialize;
use chrono::{self, DateTime, Utc};
use uuid::Uuid;

#[derive(Deserialize)]
struct LogEntry {
    id: Uuid,
    text: String,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct PlanEntry {
    id: Uuid,
    text: String,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize)]
struct ViewResponse {
    logs: Vec<LogEntry>,
    plans: Vec<PlanEntry>,
}

pub async fn view_command() -> Result<()> {
    let token = token_manager::get_valid_token().await?;

    let client = reqwest::Client::new();
    let response = client
        .get("https://api.iepok.com/view")
        .bearer_auth(token)
        .send()
        .await?;

    if !response.status().is_success() {
        bail!("Unauthorized");
    }

    let view_data: ViewResponse = response.json().await?;

    println!("\n{}", "=== Logs ===".bright_green().bold());
    if view_data.logs.is_empty() {
        println!("  {}", "No logs yet".dimmed());
    } else {
        for log in view_data.logs {
            println!("  {} - {}",
                log.created_at.format("%Y-%m-%d %H:%M").to_string().dimmed(),
                log.text.cyan()
            );
        }
    }

    println!("\n{}", "=== Plans ===".bright_yellow().bold());
    if view_data.plans.is_empty() {
        println!("  {}", "No plans yet".dimmed());
    } else {
        for plan in view_data.plans {
            println!("  {} - {}",
                plan.created_at.format("%Y-%m-%d %H:%M").to_string().dimmed(),
                plan.text.yellow()
            );
        }
    }

    println!();
    Ok(())
}
