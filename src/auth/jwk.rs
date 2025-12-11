use jsonwebtoken::{decode, decode_header, DecodingKey, Validation, Algorithm};
use serde::{Deserialize};
use std::fs;
use std::path::PathBuf;
use crate::auth::auth::{CLIENT_ID};
use anyhow::{Context, Result};

#[derive(Deserialize)]
struct Jwk {
    kid: String,
    #[serde(rename = "n")]
    modulus: String,
    #[serde(rename = "e")]
    exponent: String,
}

#[derive(Deserialize)]
struct JwkSet {
    keys: Vec<Jwk>,
}

const COGNITO_REGION: &str = "us-east-1";
const COGNITO_USER_POOL_ID: &str = "us-east-1_DAvkrVxUh";

fn get_jwks_url() -> String {
    format!(
        "https://cognito-idp.{}.amazonaws.com/{}/.well-known/jwks.json",
        COGNITO_REGION, COGNITO_USER_POOL_ID
    )
}

fn jwk_cache_path() -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join("imp")
        .join("jwks.json")
}

pub async fn fetch_jwks() -> Result<String> {
    let response = reqwest::get(&get_jwks_url()).await.context("Failed to fetch JWKs")?;
    let jwks = response.text().await.context("Failed to read JWKs response")?;

    let cache_path = jwk_cache_path();
    fs::create_dir_all(cache_path.parent().unwrap()).context("Failed to create cache directory")?;
    fs::write(cache_path, &jwks).context("Failed to write JWKs to cache")?;

    Ok(jwks)
}

fn load_cached_jwks() -> Result<String> {
    fs::read_to_string(jwk_cache_path()).context("Failed to read cached JWKs")
}

pub async fn validate_token(token: &str) -> Result<()> {
    let jwks_json = match load_cached_jwks() {
        Ok(jwks) => jwks,
        Err(_) => fetch_jwks().await?,
    };

    let jwk_set: JwkSet = serde_json::from_str(&jwks_json).context("Failed to parse JWKs")?;

    let header = decode_header(token).context("Failed to decode token header")?;
    let kid = header.kid.context("No kid in token")?;

    let jwk = jwk_set
        .keys
        .iter()
        .find(|k| k.kid == kid)
        .context("JWK not found")?;

    let decoding_key = DecodingKey::from_rsa_components(&jwk.modulus, &jwk.exponent)
        .context("Failed to create decoding key")?;
    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_audience(&[CLIENT_ID]);

    decode::<serde_json::Value>(token, &decoding_key, &validation)
        .context("Failed to validate token")?;

    Ok(())
}
