use std::env;
use std::io::{self, Write};
use std::process::Command;

fn get_battery_percentage() -> String {
    if cfg!(target_os = "windows") {
        let output = Command::new("powershell")
            .arg("(Get-WmiObject -Class Win32_Battery).EstimatedChargeRemaining")
            .output()
            .expect("Failed to fetch battery percentage");

        if output.status.success() {
            if let Ok(battery_percent) = String::from_utf8(output.stdout) {
                if let Ok(percent) = battery_percent.trim().parse::<i32>() {
                    return format!("{}%", percent);
                }
            }
        }
    }

    String::new()
}

fn get_username() -> String {
    if cfg!(target_os = "windows") {
        if let Ok(username) = env::var("USERNAME") {
            return username;
        }
    }

    String::from("Unknown")
}

fn list_all_commands() -> Vec<String> {
    let mut commands = Vec::new();

    if let Some(paths) = env::var_os("PATH") {
        for path in env::split_paths(&paths) {
            if let Ok(entries) = std::fs::read_dir(&path) {
                for entry in entries {
                    if let Ok(entry) = entry {
                        if let Some(file_name) = entry.file_name().to_str() {
                            commands.push(file_name.to_string());
                        }
                    }
                }
            }
        }
    }

    commands
}

fn execute_command(command: &str) {
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", command])
            .output()
            .expect("Failed to execute command")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("Failed to execute command")
    };

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        if !stdout.is_empty() {
            println!("Output:\n{}", stdout);
        }

        if !stderr.is_empty() {
            eprintln!("Error:\n{}", stderr);
        }
    } else {
        eprintln!("Command '{}' failed to execute", command);
    }
}

fn main() {
    let username = get_username();
    let mut current_dir = env::current_dir().unwrap_or_else(|_| "/".into());

    loop {
        let battery_percent = get_battery_percentage();
        let prompt = format!(
            "╭─[{}]─[Battery: {}]─[Current Directory: {}]\n╰─> $ ",
            username,
            battery_percent,
            current_dir.display()
        );

        print!("{}", prompt);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let command = input.trim();

        if command.is_empty() {
            continue;
        }

        if command.starts_with("cd ") {
            let new_dir = command.trim_start_matches("cd ").trim();
            if new_dir.is_empty() {
                eprintln!("Error: Missing directory path");
            } else {
                match env::set_current_dir(new_dir) {
                    Ok(_) => {
                        current_dir = env::current_dir().unwrap_or_else(|_| "/".into());
                    }
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            continue;
        }

        match command.to_lowercase().as_str() {
            "exit" => break,
            "help" | "list" => {
                let commands = list_all_commands();
                println!("Available commands: {:?}", commands);
            }
            _ => execute_command(command),
        }
    }
}
