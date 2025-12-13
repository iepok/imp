use crate::auth::{auth, tokens, token_manager};
use anyhow::Result;
use std::io::{self, Write};

pub async fn login_command() -> Result<()> {
    match token_manager::validate_and_refresh().await {
        Ok(_) => {
            println!("✅ Already logged in!");
            return Ok(());
        },
        Err(e) => {
            // println!("{}", e);
        }
    }
    // if token_manager::validate_and_refresh().await.is_ok() {
    //     println!("✅ Already logged in!");
    //     return Ok(());
    // }

    print!("Your email: ");
    io::stdout().flush()?;
    let mut email = String::new();
    io::stdin().read_line(&mut email)?;
    let email = email.trim();

    println!("Sending OTP to {}...", email);
    let session = auth::send_otp(email).await?;

    print!("Enter OTP code from your email: ");
    io::stdout().flush()?;
    let mut code = String::new();
    io::stdin().read_line(&mut code)?;
    let code = code.trim();

    println!("Verifying...");
    let tokens = auth::verify_otp(email, code, &session).await?;

    tokens::save_tokens(&tokens)?;

    println!("✅ Successfully logged in!");

    Ok(())
}

