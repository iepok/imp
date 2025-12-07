use std::{env, fs, io::{BufReader, BufRead, Write}, process::Command};
use anyhow::Result;

pub fn uninstall_command() -> Result<()> {
    if cfg!(windows) {
        handle_uninstall_windows();
    } else {
        handle_uninstall_unix();
    }
    Ok(())
}

fn handle_uninstall_windows() {
    let local_app_data = env::var("LOCALAPPDATA").unwrap_or_else(|_| ".".to_string());
    let imp_dir = format!(r"{}\imp", local_app_data);
    let imp_path = format!(r"{}\imp.exe", imp_dir);

    // Remove binary
    if let Err(e) = fs::remove_file(&imp_path) {
        eprintln!("Failed to remove imp binary: {}", e);
    } else {
        println!("Removed {}", imp_path);
    }

    // Remove directory
    let _ = fs::remove_dir(&imp_dir);

    // Remove from PATH using PowerShell
    let ps_script = format!(
        r#"
        $p = [Environment]::GetEnvironmentVariable('Path','User')
        $new = ($p -split ';' | Where-Object {{ $_ -ne '{}' }}) -join ';'
        [Environment]::SetEnvironmentVariable('Path',$new,'User')
        "#,
        imp_dir
    );

    let status = Command::new("powershell")
        .args(&["-NoProfile", "-Command", &ps_script])
        .status();

    match status {
        Ok(s) if s.success() => println!("Removed {} from PATH", imp_dir),
        Ok(_) => eprintln!("PowerShell command failed to update PATH"),
        Err(e) => eprintln!("Failed to run PowerShell: {}", e),
    }

    println!("✅ imp uninstalled");
}

fn handle_uninstall_unix() {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let imp_path = format!("{}/.local/bin/imp", home);

    if let Err(e) = fs::remove_file(&imp_path) {
        eprintln!("Failed to remove imp binary: {}", e);
    } else {
        println!("Removed {}", imp_path);
    }

    remove_path_from_file(&format!("{}/.bashrc", home));
    remove_path_from_file(&format!("{}/.zshrc", home));

    println!("✅ imp uninstalled");
}

fn remove_path_from_file(file_path: &str) {
    let Ok(file) = fs::File::open(file_path) else {
        return;
    };

    let reader = BufReader::new(file);
    let lines: Vec<String> = reader
        .lines()
        .filter_map(|line| line.ok())
        .filter(|line| !line.contains(".local/bin") || !line.contains("imp"))
        .collect();

    if let Ok(mut file) = fs::File::create(file_path) {
        for line in lines {
            let _ = writeln!(file, "{}", line);
        }
    }
}

