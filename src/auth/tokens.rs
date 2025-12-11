use anyhow::Result;
use crate::auth::types::Tokens;
use std::fs;
use std::path::PathBuf;

fn tokens_path() -> PathBuf {
    dirs::config_dir()
        .unwrap()
        .join("imp")
        .join("tokens.json")
}

pub fn save_tokens(tokens: &Tokens) -> Result<()> {
    let path = tokens_path();
    fs::create_dir_all(path.parent().unwrap())?;
    let json = serde_json::to_string(tokens)?;
    fs::write(&path, json)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600))?;
    }

    Ok(())
}

pub fn load_tokens() -> Result<Tokens> {
    let path = tokens_path();
    let json = fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&json)?)
}

pub fn delete_tokens() -> Result<()> {
    let path = tokens_path();
    fs::remove_file(&path)?;
    Ok(())
}
