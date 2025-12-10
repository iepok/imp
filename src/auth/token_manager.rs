use crate::auth::{auth, jwk, keyring};
use anyhow::{bail, Result};

pub async fn validate_and_refresh() -> Result<String> {
    let mut tokens = keyring::load_tokens()?;

    if jwk::validate_token(&tokens.access_token).await.is_ok() {
        return Ok(tokens.access_token);
    }

    tokens = auth::refresh_tokens(&tokens.refresh_token).await?;

    keyring::save_tokens(&tokens)?;

    if jwk::validate_token(&tokens.access_token).await.is_ok() {
        return Ok(tokens.access_token);
    }

    if jwk::fetch_jwks().await.is_ok() {
        if jwk::validate_token(&tokens.access_token).await.is_ok() {
            return Ok(tokens.access_token);
        }
    }

    bail!("Token validation failed. Please login again with: imp login")
}

pub async fn get_valid_token() -> Result<String> {
    validate_and_refresh().await
        .map_err(|_| anyhow::anyhow!("Not logged in. Run: imp login"))
}
