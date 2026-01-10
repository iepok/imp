use anyhow::Result;
use colored::Colorize;
use crate::auth::token_manager;

pub async fn plan_command(goal: Vec<String>) -> Result<()> {
    let token = token_manager::get_valid_token().await?;
    let text = format!("{} #plan", goal.join(" "));

    println!("{} {}", "Creating plan:".bright_green().bold(), goal.join(" ").cyan());

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.iepok.com/log")
        .bearer_auth(token)
        .json(&serde_json::json!({ "raw_input": text }))
        .send()
        .await?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await?;
        anyhow::bail!("Failed to create plan: {} - {}", status, body);
    }

    println!("{}", "✓ Plan created".bright_green());

    Ok(())
}
