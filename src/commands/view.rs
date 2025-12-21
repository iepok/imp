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

#[derive(Deserialize)]
struct MeasurementEntry {
    id: Uuid,
    log_id: Uuid,
    subject: String,
    dimension: Option<String>,
    measurement: Option<String>,
    timestamp: DateTime<Utc>,
}

#[derive(Deserialize)]
struct ViewResponse {
    logs: Vec<LogEntry>,
    plans: Vec<PlanEntry>,
    measurements: Vec<MeasurementEntry>,
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
            println!("  {} - {}",
                log.created_at.format("%Y-%m-%d %H:%M").to_string().dimmed(),
                log.text.cyan()
            );

            let log_measurements: Vec<_> = view_data.measurements.iter()
                .filter(|m| m.log_id == log.id)
                .collect();

            if !log_measurements.is_empty() {
                for measurement in log_measurements {
                    let dimension_part = measurement.dimension
                        .as_ref()
                        .map(|d| format!(" ({})", d))
                        .unwrap_or_default();
                    let measurement_text = format!(
                        "{}{}: {}",
                        measurement.subject,
                        dimension_part,
                        measurement.measurement.as_ref().unwrap_or(&"N/A".to_string())
                    );
                    println!("    {}", measurement_text.dimmed());
                }
            }
        }
    }

    println!();
    Ok(())
}
