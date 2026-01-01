//////////////////////
// Fixed Code
//////////////////////
use std::process::Command;
use std::thread;
use std::env;
use std::path::Path;
use std::sync::Arc;

pub trait Runner {
    fn execute(&self) -> Result<(), String>;
}

/// In this corrected version the executable path is hardcoded as an absolute path,
/// eliminating the untrusted search path vulnerability (CWE-426).
pub struct Dispatch;

impl Dispatch {
    // Always returns a fixed absolute path to the helper binary.
    pub fn get_executable(&self) -> String {
        // On Unix-like systems, adjust the path accordingly.
        "/usr/local/bin/helper_bin".to_string()
    }
}

impl Runner for Dispatch {
    fn execute(&self) -> Result<(), String> {
        let exe = self.get_executable();
        // Validate that the path is absolute.
        if !Path::new(&exe).is_absolute() {
            return Err("Executable path must be absolute".to_string());
        }
        let exe_arc = Arc::new(exe);
        let exec_clone = Arc::clone(&exe_arc);
        let handle = thread::spawn(move || {
            unsafe {
                // Even though we use an unsafe block, the risk is mitigated
                // by using a fixed, absolute path.
                Command::new(exec_clone.as_str())
                    .arg("--version")
                    .spawn()
                    .map_err(|e| format!("spawn error: {}", e))
                    .and_then(|mut child| {
                        child.wait()
                            .map_err(|e| format!("wait error: {}", e))
                            .map(|_| ())
                    })
            }
        });
        handle.join().unwrap()
    }
}

/// Public function so external tests may call it.
pub fn run_app() -> Result<(), String> {
    let runner = Dispatch;
    runner.execute()
}

/// Exposes the executable path for external verification.
pub fn get_target() -> String {
    let d = Dispatch;
    d.get_executable()
}

fn main() {
    match run_app() {
        Ok(_) => println!("Task completed successfully"),
        Err(e) => {
            eprintln!("Execution error: {}", e);
            std::process::exit(1);
        }
    }
}