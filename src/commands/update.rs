use tokio::process::Command;
use anyhow::Result;

pub async fn update_command() -> Result<()> {
    println!("Updating imp...");

    if cfg!(windows) {
        // Windows: use PowerShell with irm + iex
        let status = Command::new("powershell")
            .arg("-Command")
            .arg("irm https://api.iepok.com/imp/install.ps1 | iex")
            .status()
            .await;

        match status {
            Ok(exit) if exit.success() => {
                println!("✅ Updated!");
                std::process::exit(0);
            },
            Ok(exit) => eprintln!("❌ Update failed with exit code: {:?}", exit.code()),
            Err(e) => eprintln!("❌ Failed to start updater: {}", e),
        }
    } else {
        // Linux / macOS / *nix
        let status = Command::new("sh")
            .arg("-c")
            .arg("curl -fsSL https://api.iepok.com/imp/install | sh")
            .status()
            .await;

        match status {
            Ok(exit) if exit.success() => println!("✅ Updated!"),
            Ok(exit) => eprintln!("❌ Update failed with exit code: {:?}", exit.code()),
            Err(e) => eprintln!("❌ Failed to start updater: {}", e),
        }
    };

    Ok(())
}
