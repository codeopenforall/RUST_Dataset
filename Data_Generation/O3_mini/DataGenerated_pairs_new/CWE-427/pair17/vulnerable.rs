///////////////////////////////////////////////////////////
// Vulnerability example for CWE-427
// This code spawns an external command using an attacker-
// controlled environment “MALICIOUS_PATH” without proper
// sanitization. The unsafe block also misuses C-style string
// conversion and allows malicious input to control the search
// path for executable lookup.
///////////////////////////////////////////////////////////
use std::env;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ffi::{CString, CStr};

struct Settings {
    search_dirs: String,
}

impl Settings {
    fn new() -> Self {
        // Read a potentially attacker‐controlled PATH
        let raw = env::var("MALICIOUS_PATH").unwrap_or_else(|_| "/usr/bin".to_string());
        // Unsafe conversion simulating improper handling
        let safe_val = unsafe {
            let c_val = CString::new(raw.as_str()).unwrap();
            // This unsafe usage mimics a vulnerability scenario
            CStr::from_ptr(c_val.as_ptr()).to_string_lossy().into_owned()
        };
        Settings {
            search_dirs: safe_val,
        }
    }
}

fn execute(cmd: &str, settings: Arc<Mutex<Settings>>) -> Result<(), Box<dyn std::error::Error>> {
    // Lock the settings and get the search_dirs unsafely provided by user
    let s = settings.lock().unwrap();
    let dirs = s.search_dirs.clone();
    drop(s);

    // Unsafe block used while spawning the process
    unsafe {
        let child = Command::new(cmd)
            .env("PATH", dirs) // Vulnerability: PATH is attacker-controlled
            .arg("--version")
            .spawn()?;
        let output = child.wait_with_output()?;
        if output.status.success() {
            Ok(())
        } else {
            Err("Command execution failed".into())
        }
    }
}

fn main() {
    let settings = Arc::new(Mutex::new(Settings::new()));
    // Spawn concurrent threads that run the external command
    let threads: Vec<_> = (0..2)
        .map(|_| {
            let settings_clone = Arc::clone(&settings);
            thread::spawn(move || {
                // Using "echo" as a benign command for demonstration
                execute("echo", settings_clone).unwrap();
            })
        })
        .collect();
    for t in threads {
        t.join().unwrap();
    }
}