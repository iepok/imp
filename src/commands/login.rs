use crate::auth::{auth, tokens, token_manager};
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

// async fn try_passkey_login() -> Result<()> {
//     let rp_id = RP_ID.to_string();
//     let origin = Url::parse(RP_ORIGIN)?;
//
//     let mut auth = WebauthnAuthenticator::new_unsafe_allowing_arbitrary_origin();
//     let request = RequestAuthentication {
//         challenge: vec![0; 32].into(),
//         origin: origin.clone(),
//         rp_id: rp_id.clone(),
//         allow_credentials: vec![],
//         user_verification: UserVerificationPolicy::Required,
//         ..Default::default()
//     };
//
//     let result = auth.do_authentication(origin, request)?;
//
//     // Success → save fake refresh token (replace with real Cognito flow later)
//     Entry::new(SERVICE_NAME, "current_user")?.set_password("passkey-refresh-token")?;
//     
//     Ok(())
// }
//
