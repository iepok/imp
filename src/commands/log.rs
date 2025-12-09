use colored::Colorize;
use anyhow::Result;
use crate::auth::token_manager;

pub async fn log_command(args: &[String]) -> Result<()> {
    if !args[0].contains(' ') {
        anyhow::bail!("Invalid log command");
    }

    let token = token_manager::get_valid_token().await?;

    println!("{} {}", "Logging:".bright_green().bold(), args[0].cyan());

    let client = reqwest::Client::new();
    let response = client
        .post("https://api.iepok.com/log")
        .bearer_auth(token)
        .json(&serde_json::json!({
            "text": args[0]
        }))
        .send()
        .await?;

    let _result = response.text().await?;

    Ok(())
}

// pub async fn call_command() -> Result<()> {
//     let username = std::fs::read_to_string(CURRENT_USER_FILE)?.trim().to_owned();
//     let refresh_token = Entry::new(SERVICE_NAME, &username)?.get_password()?;
//
//     let region = RegionProviderChain::default_provider().or_else("eu-central-1");
//     let config = aws_config::from_env().region(region).load().await;
//     let client = Client::new(&config);
//
//     let mut params = HashMap::new();
//     params.insert("REFRESH_TOKEN", refresh_token);
//     if !secret_hash(&username).is_empty() {
//         params.insert("SECRET_HASH", &secret_hash(&username));
//     }
//
//     let resp = client
//         .initiate_auth()
//         .client_id(CLIENT_ID)
//         .auth_flow(AuthFlowType::RefreshTokenAuth)
//         .set_auth_parameters(Some(params))
//         .send()
//         .await?;
//
//     let access_token = resp
//         .authentication_result()
//         .and_then(|r| r.access_token())
//         .ok_or(anyhow!("Failed to refresh token"))?;
//
//     let text = HttpClient::new()
//         .get("http://127.0.0.1:3000/")
//         .bearer_auth(access_token)
//         .send()
//         .await?
//         .text()
//         .await?;
//
//     println!("{text}");
//     Ok(())
// }
//
