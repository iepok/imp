use anyhow::{Result, bail};
use crate::auth::token_manager;
use serde::Deserialize;

#[derive(Deserialize)]
struct SummaryResponse {
    summary: String,
}

#[derive(Deserialize)]
struct OccurrenceItem {
    timestamp: String,
    text: String,
}

#[derive(Deserialize)]
struct OccurrencesResponse {
    occurrences: Vec<OccurrenceItem>,
}

pub async fn view_command(what: &str) -> Result<()> {
    let token = token_manager::get_valid_token().await?;
    let client = reqwest::Client::new();

    match what {
        "occurrences" => {
            let response = client
                .get("https://api.iepok.com/view/occurrences")
                .bearer_auth(token)
                .send()
                .await?;

            if !response.status().is_success() {
                bail!("Failed to get occurrences: {}", response.status());
            }

            let data: OccurrencesResponse = response.json().await?;

            if data.occurrences.is_empty() {
                println!("No occurrences.");
            } else {
                for occ in data.occurrences {
                    // Parse and format timestamp as HH:MM
                    let time = if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(&occ.timestamp) {
                        dt.format("%H:%M").to_string()
                    } else {
                        occ.timestamp
                    };
                    println!("{} - {}", time, occ.text);
                }
            }
        }
        _ => {
            let response = client
                .get("https://api.iepok.com/view")
                .bearer_auth(token)
                .send()
                .await?;

            if !response.status().is_success() {
                bail!("Failed to get view: {}", response.status());
            }

            let data: SummaryResponse = response.json().await?;
            println!("{}", data.summary);
        }
    }

    Ok(())
}
