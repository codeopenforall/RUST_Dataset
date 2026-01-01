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

// An unsafe helper that converts a raw pointer and length into a String without bounds validation.
unsafe fn unsafe_process_path(input: *const u8, len: usize) -> String {
    let slice = std::slice::from_raw_parts(input, len);
    String::from_utf8_lossy(slice).into_owned()
}

// Spawns an external process using an environment variable for the search path.
// This function is declared public for testing purposes.
pub fn spawn_process(cfg: Config) -> Result<String, String> {
    // Read attacker-controlled PATH environment variable.
    let user_path = env::var("PATH").unwrap_or_default();
    let mut processed_path = String::new();

    // Unsafe operation to reprocess the uncontrolled PATH value.
    unsafe {
        let ptr = user_path.as_ptr();
        let len = user_path.len();
        // Vulnerability: directly processing attacker-controlled data unsafely.
        processed_path = unsafe_process_path(ptr, len);
    }

    // Launching the command in a multi-threaded context.
    let output_data = Arc::new(Mutex::new(String::new()));
    let thread_data = Arc::clone(&output_data);
    let child = thread::spawn(move || {
        // Spawn external command relying on the unsanitized and attacker-controlled PATH.
        // The command "printenv" is used with argument "PATH" to display the effective PATH.
        let res = Command::new(&cfg.command)
            .arg("PATH")
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