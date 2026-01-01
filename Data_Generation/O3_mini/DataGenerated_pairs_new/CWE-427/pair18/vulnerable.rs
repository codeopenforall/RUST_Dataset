//////////////////////////////////////////////
// Vulnerable Code Sample (unsafe search path)
//////////////////////////////////////////////
use std::env;
use std::ffi::CString;
use std::process::Command;
use std::thread;

// Global static mutable storage holding an unsanitized copy of PATH.
static mut GLOBAL_PATH: Option<CString> = None;

// This function reads the system PATH environment variable without validation
// and stores it unsafely into a global variable.
fn initialize_env() {
    let path_str = env::var("PATH").unwrap_or_else(|_| String::from(""));
    unsafe {
        // Unsafe conversion without sanitization; an attacker can control PATH.
        GLOBAL_PATH = Some(CString::new(path_str).unwrap());
    }
}

// Retrieves the unsanitized PATH from global memory.
fn fetch_env() -> String {
    unsafe {
        GLOBAL_PATH
            .as_ref()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    }
}

// Launches a subprocess that relies on PATH for command resolution.
// Here, the "sh -c 'which ls'" command depends on the search path to resolve "ls".
// Using an attacker-controlled PATH (stored unsafely) creates an uncontrolled search path vulnerability (CWE-427).
fn spawn_process() -> bool {
    let current_path = fetch_env();
    let output = Command::new("sh")
        .arg("-c")
        .arg("which ls")
        .env("PATH", current_path)
        .output();
    match output {
        Ok(out) => out.status.success(),
        Err(_) => false,
    }
}

// Public function that executes the sensitive operation concurrently.
pub fn do_action() -> bool {
    let handle = thread::spawn(|| spawn_process());
    handle.join().unwrap_or(false)
}

fn main() {
    // Directly initialize the search path from the external environment.
    initialize_env();
    if do_action() {
        println!("Operation completed successfully.");
    } else {
        println!("Operation failed.");
    }
}