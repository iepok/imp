use aws_config::{BehaviorVersion, Region, defaults};
use aws_sdk_cognitoidentityprovider::{Client, types::{AuthFlowType, ChallengeNameType}};
use crate::auth::types::Tokens;
use anyhow::{Context, Result};

pub const REGION: &str = "us-east-1";
pub const CLIENT_ID: &str = "6tlohqsfgoqiehi7q6027a3rl3";

async fn get_aws_client() -> Client {
    let config = defaults(BehaviorVersion::latest())
        .region(Region::new(REGION))
        .load()
        .await;
    Client::new(&config)
}

pub async fn send_otp(email: &str) -> Result<String> {
    let response = get_aws_client()
        .await
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
    let response = get_aws_client()
        .await
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
        refresh_token: auth_result.refresh_token().context("No Refresh Token")?.to_string(),
        // device_key: auth_result.new_device_metadata().context("No Device metadata")?.device_key().context("No Device key")?.to_string(),
    };
    
    Ok(tokens)
}

pub async fn refresh_tokens(
    refresh_token: &str,
    // device_key: &str,
) -> Result<Tokens> {
    let response = get_aws_client()
        .await
        .get_tokens_from_refresh_token()
        .client_id(CLIENT_ID)
        .refresh_token(refresh_token)
        // .device_key(device_key)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to refresh tokens: {:?}", e))?;
        // .context("Failed to refresh tokens")?;

    let auth_result = response
        .authentication_result()
        .context("No authentication result")?;

    // On refresh, new_device_metadata is NOT returned - keep the existing device_key
    // let new_device_key = auth_result
    //     .new_device_metadata()
    //     .and_then(|metadata| metadata.device_key())
    //     .map(|key| key.to_string())
    //     .unwrap_or_else(|| device_key.to_string());

    let tokens = Tokens {
        access_token: auth_result.access_token().context("No access token")?.to_string(),
        id_token: auth_result.id_token().context("No ID token")?.to_string(),
        refresh_token: auth_result.refresh_token().context("No Refresh Token")?.to_string(),
        // device_key: new_device_key,
    };

    Ok(tokens)
}

pub async fn logout(refresh_token: &str) -> Result<()> {
    get_aws_client()
        .await
        .revoke_token()
        .client_id(CLIENT_ID)
        .token(refresh_token)
        .send()
        .await
        .context("Failed to logout")?;
    
    Ok(())
}

pub async fn global_logout(access_token: &str) -> Result<()> {
    get_aws_client()
        .await
        .global_sign_out()
        .access_token(access_token)
        .send()
        .await
        .context("Failed to logout globally")?;

    Ok(())
}
