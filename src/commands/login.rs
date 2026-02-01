use crate::auth::{auth, auth::OtpResult, tokens, token_manager};
use anyhow::Result;
use std::io::{self, Write};

pub async fn login_command() -> Result<()> {
    if token_manager::validate_and_refresh().await.is_ok() {
        println!("✅ Already logged in!");
        return Ok(());
    }

    print!("Your email: ");
    io::stdout().flush()?;
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;
    let email = email.trim();

    println!("Sending code to {}...", email);
    let result = auth::send_otp(email).await?;

    print!("Enter code from your email: ");
    io::stdout().flush()?;
    let mut code = String::new();
    io::stdin().read_line(&mut code)?;
    let code = code.trim();

    println!("Verifying...");
    let tokens = match result {
        OtpResult::Session(session) => auth::verify_otp(email, code, &session).await?,
        OtpResult::NeedsConfirmation { session } => {
            auth::confirm_signup_and_auth(email, code, &session).await?
        }
    };

    tokens::save_tokens(&tokens)?;

    println!("✅ Successfully logged in!");

    Ok(())
}

