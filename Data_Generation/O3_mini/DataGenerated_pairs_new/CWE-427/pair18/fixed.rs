//////////////////////////////////////////////
// Fixed Code Sample (secured search path)
//////////////////////////////////////////////
use std::process::Command;
use std::thread;

// Use a constant, hardcoded safe search path that is known to contain only trusted directories.
const SECURE_PATH: &str = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";

// Instead of reading from the external environment, return a safe path.
fn establish_safe_env() -> String {
    SECURE_PATH.to_string()
}

// Launches a subprocess that now relies on a trusted, predetermined PATH for command resolution.
// This eliminates the risk from an attacker-controlled environment variable.
fn spawn_process() -> bool {
    let safe_path = establish_safe_env();
    let output = Command::new("sh")
        .arg("-c")
        .arg("which ls")
        .env("PATH", safe_path)
        .output();
    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

// Public function that executes the operation concurrently.
pub fn do_action() -> bool {
    let handle = thread::spawn(|| spawn_process());
    handle.join().unwrap_or(false)
}

fn main() {
    if do_action() {
        println!("Operation completed successfully.");
    } else {
        println!("Operation failed.");
    }
}