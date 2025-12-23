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
    // id: Uuid,
    text: String,
    created_at: DateTime<Utc>,
}

#[derive(Serialize, FromRow)]
pub struct Pok {
    pub id: Uuid,
    pub log_id: Uuid,
    pub topic: String,
    pub operator: String,
    pub value: String,
    pub dimension: Option<String>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
struct ViewResponse {
    logs: Vec<LogEntry>,
    plans: Vec<PlanEntry>,
    poks: Vec<MeasurementEntry>,
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

    println!("\n{}", "=== Logs ===".bright_green().bold());
    if view_data.logs.is_empty() {
        println!("  {}", "No logs yet".dimmed());
    } else {
        for log in view_data.logs {
            println!(
                "  {} - {}",
                log.created_at.format("%Y-%m-%d %H:%M").to_string().dimmed(),
                log.text.cyan()
            );

            let log_poks: Vec<_> = view_data.poks
                .iter()
                .filter(|p| p.log_id == log.id)
                .collect();

            for pok in log_poks {
                let dimension_part = pok.dimension
                    .as_ref()
                    .map(|d| format!(" ({})", d))
                    .unwrap_or_default();

                let text = format!(
                    "{}{}: {}",
                    pok.topic,
                    dimension_part,
                    pok.value
                );

                println!("    {}", text);
            }
        }

    }

    println!();
    Ok(())
}
