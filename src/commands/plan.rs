use anyhow::Result;
use crate::auth::token_manager;

pub async fn plan_command(goal: Vec<String>) -> Result<()> {
    let token = token_manager::get_valid_token().await?;

    println!("Creating plan: {}", goal.join(" "));

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.iepok.com/plan")
        .bearer_auth(token)
        .json(&serde_json::json!({ "text": goal.join(" ") }))
        .send()
        .await?;

    let _result = response.text().await?;

    Ok(())
}
