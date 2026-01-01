use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
struct Settings {
    search_dirs: String,
}
impl Settings {
    fn new() -> Self {
        let raw = env::var("MALICIOUS_PATH").unwrap_or_else(|_| "".to_string());
        let valid = raw.chars().all(|c| c.is_alphanumeric() || c == '/' || c == ':' || c == '-' || c == '.');
        let safe_path = if valid && !raw.is_empty() {
            raw
        } else {
            "/usr/bin".to_string()
        };
        Settings {
            search_dirs: safe_path,
        }
    }
}
fn execute(cmd: &str, settings: Arc<Mutex<Settings>>) -> Result<(), Box<dyn std::error::Error>> {
    let s = settings.lock().unwrap();
    let dirs = s.search_dirs.clone();
    drop(s);
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
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let settings_clone = Arc::clone(&settings);
            thread::spawn(move || {
                execute("echo", settings_clone).unwrap();
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}
