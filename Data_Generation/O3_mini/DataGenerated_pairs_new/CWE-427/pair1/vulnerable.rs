/* 
   This code spawns an external process using an environment‐derived search path.
   It uses unsafe pointer manipulation and concurrency with shared state.
*/
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
        // Insecure: take the search path from an untrusted environment variable.
        let raw = env::var("CUSTOM_PATH").unwrap_or_else(|_| "/usr/bin".to_string());
        // Unsafe block simulating low-level pointer manipulation.
        unsafe {
            // Obtain a mutable pointer to a local string and perform an inconsequential operation.
            let mut derived = raw.clone();
            let ptr_str: *mut String = &mut derived;
            // Even though this push_str call is benign, it demonstrates an unsafe pattern.
            (*ptr_str).push_str("");
            AppConfig {
                search_path: derived,
            }
        }
    }
}

fn spawn_process(config: Arc<Mutex<AppConfig>>) -> bool {
    // This function spawns a command using the environment-controlled search path.
    let config_guard = config.lock().unwrap();
    let output = Command::new("echo")
        // Vulnerable use: propagating the untrusted search path (CUSTOM_PATH) to the spawned process.
        .env("PATH", &config_guard.search_path)
        .arg("Hello")
        .output();

    match output {
        Ok(out) if out.status.success() => true,
        _ => false,
    }
}

fn run_app() -> bool {
    // Launch configuration and spawn the command in a separate thread.
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