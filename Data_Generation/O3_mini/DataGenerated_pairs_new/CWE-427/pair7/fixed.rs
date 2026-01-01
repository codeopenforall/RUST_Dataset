use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

// Application configuration structure.
struct Config {
    command: String,
    timeout: u64,
}

impl Config {
    fn new(cmd: &str, timeout: u64) -> Self {
        Config {
            command: cmd.to_string(),
            timeout,
        }
    }
}

// An unsafe helper that converts a raw pointer and length into a String.
// Here, it is used on a known good string.
unsafe fn process_static_path(input: *const u8, len: usize) -> String {
    let slice = std::slice::from_raw_parts(input, len);
    String::from_utf8_lossy(slice).into_owned()
}

// Spawns an external process with a secure, fixed PATH value.
// This function is declared public for testing purposes.
pub fn spawn_process(cfg: Config) -> Result<String, String> {
    // Instead of using the uncontrolled environment variable,
    // a secure default PATH is set.
    let secure_path = "/usr/bin:/bin".to_string();
    let mut processed_path = String::new();
    unsafe {
        let ptr = secure_path.as_ptr();
        let len = secure_path.len();
        processed_path = process_static_path(ptr, len);
    }

    // Launch the external command with cleared environment variables
    // to eliminate unintended influences.
    let output_data = Arc::new(std::sync::Mutex::new(String::new()));
    let thread_data = std::sync::Arc::clone(&output_data);
    let child = thread::spawn(move || {
        // Spawn external command using the secure PATH setting.
        let res = Command::new(&cfg.command)
            .arg("PATH")
            .env_clear()
            .env("PATH", processed_path)
            .stdout(Stdio::piped())
            .spawn();
        match res {
            Ok(mut child_proc) => {
                if let Ok(out) = child_proc.wait_with_output() {
                    let text = String::from_utf8_lossy(&out.stdout).into_owned();
                    let mut guard = thread_data.lock().unwrap();
                    *guard = text;
                }
            }
            Err(_) => {}
        }
    });
    let _ = child.join();
    let final_output = output_data.lock().unwrap();
    Ok(final_output.clone())
}

fn main() {
    // Using "printenv" so that the spawned process prints the effective PATH.
    let cfg = Config::new("printenv", 10);
    match spawn_process(cfg) {
        Ok(out) => println!("Process output: {}", out),
        Err(e) => eprintln!("Error: {}", e),
    }
}