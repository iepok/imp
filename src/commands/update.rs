use std::process::Command;
use anyhow::Result;

pub fn update_command() -> Result<()> {
    println!("Updating imp...");

    if cfg!(windows) {
        // Windows: use PowerShell with irm + iex
        let _ = Command::new("powershell")
            .arg("-Command")
            .arg("irm https://api.iepok.com/imp/install.ps1 | iex")
            .spawn();
        std::process::exit(0);
    } else {
        // Linux / macOS / *nix
        let status = Command::new("sh")
            .arg("-c")
            .arg("curl -fsSL https://api.iepok.com/imp/install | sh")
            .status();

        match status {
            Ok(exit) if exit.success() => println!("✅ Updated!"),
            Ok(exit) => eprintln!("❌ Update failed with exit code: {:?}", exit.code()),
            Err(e) => eprintln!("❌ Failed to start updater: {}", e),
        }
    };

    Ok(())
}
