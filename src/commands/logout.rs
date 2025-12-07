use crate::auth::{auth, keyring};
use anyhow::{bail, Result};

pub async fn logout_command(all: bool) -> Result<()> {
    if all {
        let tokens = match keyring::load_tokens() {
            Ok(t) => t,
            Err(_) => bail!("Not logged in. Nothing to logout from."),
        };
        auth::global_sign_out(&tokens.access_token).await?;
        keyring::delete_tokens()?;
        println!("✅ Logged out from all devices!");
    } else {
        match keyring::delete_tokens() {
            Ok(_) => println!("✅ Logged out successfully!"),
            Err(_) => bail!("Not logged in. Nothing to logout from."),
        }
    }
    Ok(())
}

