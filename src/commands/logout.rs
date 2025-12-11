use crate::auth::{auth, tokens};
use anyhow::{bail, Result};

pub async fn logout_command(all: bool) -> Result<()> {
    let tokens = match tokens::load_tokens() {
        Ok(t) => t,
        Err(_) => bail!("Not logged in. Nothing to logout from."),
    };
    if all {
        auth::global_logout(&tokens.access_token).await?;
        println!("✅ Logged out from all devices!");
    } else {
        auth::logout(&tokens.refresh_token).await?;
        println!("✅ Logged out successfully!");
    }
    
    tokens::delete_tokens().ok();
    
    Ok(())
}

