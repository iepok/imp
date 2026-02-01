use aws_config::{BehaviorVersion, Region, defaults};
use aws_sdk_cognitoidentityprovider::{Client, types::{AuthFlowType, ChallengeNameType, AttributeType}};
use crate::auth::types::Tokens;
use anyhow::{Context, Result};

pub const REGION: &str = "us-east-1";
pub const CLIENT_ID: &str = "6tlohqsfgoqiehi7q6027a3rl3";

pub enum OtpResult {
    Session(String),
    NeedsConfirmation { session: String },
}

async fn get_aws_client() -> Client {
    let config = defaults(BehaviorVersion::latest())
        .region(Region::new(REGION))
        .load()
        .await;
    Client::new(&config)
}

pub async fn send_otp(email: &str) -> Result<OtpResult> {
    let client = get_aws_client().await;

    // Try signup first (like allinloop)
    let signup_result = client
        .sign_up()
        .client_id(CLIENT_ID)
        .username(email)
        .user_attributes(
            AttributeType::builder()
                .name("email")
                .value(email)
                .build()
                .context("Failed to build email attribute")?
        )
        .send()
        .await;

    match signup_result {
        Ok(response) => {
            // New user - confirmation code sent
            let session = response
                .session()
                .context("No session returned from signup")?
                .to_string();
            Ok(OtpResult::NeedsConfirmation { session })
        }
        Err(e) => {
            let error_str = format!("{:?}", e);
            if error_str.contains("UsernameExistsException") {
                // User exists - initiate auth
                let auth_response = client
                    .initiate_auth()
                    .client_id(CLIENT_ID)
                    .auth_flow(AuthFlowType::UserAuth)
                    .auth_parameters("USERNAME", email)
                    .auth_parameters("PREFERRED_CHALLENGE", "EMAIL_OTP")
                    .send()
                    .await
                    .map_err(|e| anyhow::anyhow!("Failed to send OTP: {:?}", e))?;

                let session = auth_response
                    .session()
                    .context("No session returned")?
                    .to_string();
                Ok(OtpResult::Session(session))
            } else {
                Err(anyhow::anyhow!("Failed to sign up: {:?}", e))
            }
        }
    }
}

pub async fn confirm_signup_and_auth(email: &str, code: &str, session: &str) -> Result<Tokens> {
    let client = get_aws_client().await;

    let response = client
        .confirm_sign_up()
        .client_id(CLIENT_ID)
        .username(email)
        .confirmation_code(code)
        .session(session)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to confirm signup: {:?}", e))?;

    let auth_session = response
        .session()
        .context("No session returned from confirmation")?;

    // Continue auth with the session from confirmation
    let auth_response = client
        .initiate_auth()
        .client_id(CLIENT_ID)
        .auth_flow(AuthFlowType::UserAuth)
        .auth_parameters("USERNAME", email)
        .session(auth_session)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to continue auth: {:?}", e))?;

    let auth_result = auth_response
        .authentication_result()
        .context("No authentication result")?;

    Ok(Tokens {
        access_token: auth_result.access_token().context("No access token")?.to_string(),
        id_token: auth_result.id_token().context("No ID token")?.to_string(),
        refresh_token: auth_result.refresh_token().context("No refresh token")?.to_string(),
    })
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

    Ok(Tokens {
        access_token: auth_result.access_token().context("No access token")?.to_string(),
        id_token: auth_result.id_token().context("No ID token")?.to_string(),
        refresh_token: auth_result.refresh_token().context("No Refresh Token")?.to_string(),
    })
}

pub async fn refresh_tokens(refresh_token: &str) -> Result<Tokens> {
    let response = get_aws_client()
        .await
        .get_tokens_from_refresh_token()
        .client_id(CLIENT_ID)
        .refresh_token(refresh_token)
        .send()
        .await
        .map_err(|e| anyhow::anyhow!("Failed to refresh tokens: {:?}", e))?;

    let auth_result = response
        .authentication_result()
        .context("No authentication result")?;

    Ok(Tokens {
        access_token: auth_result.access_token().context("No access token")?.to_string(),
        id_token: auth_result.id_token().context("No ID token")?.to_string(),
        refresh_token: auth_result.refresh_token().context("No Refresh Token")?.to_string(),
    })
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
