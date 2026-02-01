use anyhow::{Result, bail};
use crate::auth::token_manager;
use serde::Deserialize;

#[derive(Deserialize)]
struct SummaryResponse {
    summary: String,
}

#[derive(Deserialize)]
struct OccurrencesResponse {
    occurrences: String,
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
            println!("{}", data.occurrences);
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
