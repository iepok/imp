use anyhow::{Result, bail};
use colored::Colorize;
use crate::auth::token_manager;
use serde::Deserialize;

#[derive(Deserialize)]
struct ViewResponse {
    summary: String,
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
        let status = response.status();
        let body = response.text().await?;
        bail!("Failed to get view: {} - {}", status, body);
    }

    let view_data: ViewResponse = response.json().await?;

    println!("\n{}\n", view_data.summary);

    Ok(())
}
