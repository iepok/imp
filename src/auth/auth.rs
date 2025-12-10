use aws_config::{BehaviorVersion, defaults};
use aws_sdk_cognitoidentityprovider::{Client, types::{AuthFlowType, ChallengeNameType}};
use crate::auth::types::Tokens;
use anyhow::{Context, Result};

pub const REGION: &str = "us-east-1";
pub const USER_POOL_ID: &str = "us-east-1_DAvkrVxUh";
pub const CLIENT_ID: &str = "6tlohqsfgoqiehi7q6027a3rl3";

pub async fn send_otp(email: &str) -> Result<String> {
    let config = defaults(BehaviorVersion::latest())
        .region(REGION)
        .load()
        .await;
    
    let client = Client::new(&config);

    let response = client
        .initiate_auth()
        .client_id(CLIENT_ID)
        .auth_flow(AuthFlowType::UserAuth)
        .auth_parameters("USERNAME", email)
        .auth_parameters("PREFERRED_CHALLENGE", "EMAIL_OTP")
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to send OTP: {:?}", e))?;

    let session = response
        .session()
        .context("No session returned")?
        .to_string();
    
    Ok(session)
}

pub async fn verify_otp(
    email: &str,
    code: &str,
    session: &str,
) -> Result<Tokens> {
    let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
        .region(aws_config::Region::new(REGION))
        .load()
        .await;
    let client = Client::new(&config);
    
    let response = client
        .respond_to_auth_challenge()
        .client_id(CLIENT_ID)
        .challenge_name(ChallengeNameType::EmailOtp)
        .session(session)
        .challenge_responses("EMAIL_OTP_CODE", code)
        .challenge_responses("USERNAME", email)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to verify OTP: {:?}", e))?;

    let auth_result = response
        .authentication_result()
        .context("No authentication result")?;

    let tokens = Tokens {
        access_token: auth_result.access_token().context("No access token")?.to_string(),
        id_token: auth_result.id_token().context("No ID token")?.to_string(),
        refresh_token: auth_result.refresh_token().context("No refresh token")?.to_string(),
    };
    
    Ok(tokens)
}

pub async fn refresh_tokens(refresh_token: &str) -> Result<Tokens> {
    let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let client = Client::new(&config);

    let response = client
        .initiate_auth()
        .client_id(CLIENT_ID)
        .auth_flow(AuthFlowType::RefreshTokenAuth)
        .auth_parameters("REFRESH_TOKEN", refresh_token)
        .send()
        .await
        .context("Failed to refresh tokens")
        .map_err(|e| {
            println!("Cognito error: {:?}", e);
            e
        })?;

    let auth_result = response
        .authentication_result()
        .context("No authentication result")?;

    let tokens = Tokens {
        access_token: auth_result.access_token().context("No access token")?.to_string(),
        id_token: auth_result.id_token().context("No ID token")?.to_string(),
        refresh_token: refresh_token.to_string(), // Keep original
    };

    Ok(tokens)
}

pub async fn global_sign_out(access_token: &str) -> Result<()> {
    let config = defaults(BehaviorVersion::latest())
        .region(REGION)
        .load()
        .await;
    let client = Client::new(&config);

    client
        .global_sign_out()
        .access_token(access_token)
        .send()
        .await
        .context("Failed to sign out globally")?;

    Ok(())
}
