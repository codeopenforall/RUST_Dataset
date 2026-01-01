///////////////////////////////////////////////////////////
// Fixed example for CWE-427
// This edited version sanitizes the user‐supplied PATH by
// validating that it conforms to an allowed character set,
// falling back to a safe default if necessary. The unsafe
// block has been removed from the command spawner.
///////////////////////////////////////////////////////////
use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Settings {
    search_dirs: String,
}

impl Settings {
    fn new() -> Self {
        // Read the (potentially unsafe) environment variable.
        let raw = env::var("MALICIOUS_PATH").unwrap_or_else(|_| "".to_string());
        // Sanitize the input: allow only alphanumerics, '/', ':', '-', and '.'.
        let valid = raw.chars().all(|c| c.is_alphanumeric() || c == '/' || c == ':' || c == '-' || c == '.');
        let safe_path = if valid && !raw.is_empty() {
            raw
        } else {
            // Fallback to a safe, hard-coded path.
            "/usr/bin".to_string()
        };
        Settings {
            search_dirs: safe_path,
        }
    }
}

fn execute(cmd: &str, settings: Arc<Mutex<Settings>>) -> Result<(), Box<dyn std::error::Error>> {
    // Lock the settings and get the sanitized search_dirs.
    let s = settings.lock().unwrap();
    let dirs = s.search_dirs.clone();
    drop(s);

    // Spawn the command safely without using an unsafe block.
    let child = Command::new(cmd)
        .env("PATH", dirs)
        .arg("--version")
        .spawn()?;
    let output = child.wait_with_output()?;
    if output.status.success() {
        Ok(())
    } else {
        Err("Command execution failed".into())
    }
}

fn main() {
    let settings = Arc::new(Mutex::new(Settings::new()));
    // Spawn concurrent threads that run the external command.
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let settings_clone = Arc::clone(&settings);
            thread::spawn(move || {
                // Using "echo" as a benign command for demonstration.
                execute("echo", settings_clone).unwrap();
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}