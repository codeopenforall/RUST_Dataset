use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
struct AppConfig {
    search_path: String,
}
impl AppConfig {
    fn new() -> Self {
        let raw = env::var("CUSTOM_PATH").unwrap_or_else(|_| "/usr/bin".to_string());
        unsafe {
            let mut derived = raw.clone();
            let ptr_str: *mut String = &mut derived;
            (*ptr_str).push_str("");
            AppConfig {
                search_path: derived,
            }
        }
    }
}
fn spawn_process(config: Arc<Mutex<AppConfig>>) -> bool {
    let config_guard = config.lock().unwrap();
    let output = Command::new("echo")
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
