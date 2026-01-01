//////////////////////
// Vulnerable Code
//////////////////////
use std::process::Command;
use std::thread;
use std::env;
use std::sync::Arc;

pub trait Runner {
    fn execute(&self) -> Result<(), String>;
}

/// In this implementation the target executable is chosen based on an environment variable.
/// If the variable "USE_MALICIOUS" is set to "1", a relative path to a malicious binary is used.
/// This exposes an untrusted search path vulnerability (CWE-426) when using Command without an absolute path.
pub struct Dispatch;

impl Dispatch {
    // Returns the executable name/path based on the environment.
    // Insecure because a relative path will be subject to untrusted search path resolution.
    pub fn get_executable(&self) -> String {
        if env::var("USE_MALICIOUS").unwrap_or_default() == "1" {
            "malicious_helper".to_string()  // relative path chosen under attacker control
        } else {
            "helper_bin".to_string()         // default relative path
        }
    }
}

impl Runner for Dispatch {
    fn execute(&self) -> Result<(), String> {
        let exe = self.get_executable();
        // Wrap the command string in an Arc to simulate shared state in concurrent context.
        let exe_arc = Arc::new(exe);
        let exec_clone = Arc::clone(&exe_arc);
        let handle = thread::spawn(move || {
            unsafe {
                // UNSAFE: invoking Command::new with a relative path. If the search path is manipulated,
                // a malicious executable located in an untrusted directory could be run.
                Command::new(exec_clone.as_str())
                    .arg("--version")
                    .spawn()
                    .map_err(|e| format!("spawn error: {}", e))
                    .and_then(|mut child| {
                        // Wait for the process to complete.
                        child.wait()
                            .map_err(|e| format!("wait error: {}", e))
                            .map(|_| ())
                    })
            }
        });
        // Propagate result or errors.
        handle.join().unwrap()
    }
}

/// Public function so external tests may call it.
pub fn run_app() -> Result<(), String> {
    let runner = Dispatch;
    runner.execute()
}

/// Exposes the selected executable for external verification (e.g. oracle).
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