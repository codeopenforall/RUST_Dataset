/* 
   Secure version: The search path is set to a predefined safe value,
   eliminating the risk of an uncontrolled search path element.
*/
use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct AppConfig {
    search_path: String,
}

impl AppConfig {
    fn new() -> Self {
        // Secure: use a hard-coded, trusted search path.
        let safe_path = "/usr/bin".to_string();
        AppConfig {
            search_path: safe_path,
        }
    }
}

fn spawn_process(config: Arc<Mutex<AppConfig>>) -> bool {
    let config_guard = config.lock().unwrap();
    let output = Command::new("echo")
        // The trusted search path is used regardless of any external environment variable.
        .env("PATH", &config_guard.search_path)
        .arg("Hello")
        .output();

    match output {
        Ok(out) if out.status.success() => true,
        _ => false,
    }
}

fn run_app() -> bool {
    let config = Arc::new(Mutex::new(AppConfig::new()));
    let config_clone = Arc::clone(&config);
    let handle = thread::spawn(move || {
        spawn_process(config_clone)
    });
    handle.join().unwrap_or(false)
}

fn main() {
    let result = run_app();
    if result {
        println!("Process spawned successfully");
    } else {
        println!("Failed to spawn process");
    }
}