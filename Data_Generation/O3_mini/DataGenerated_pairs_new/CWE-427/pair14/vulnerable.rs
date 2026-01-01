//////////////////////////////
// Vulnerable Version Code  //
// CWE-427: Uncontrolled Search Path Element
//////////////////////////////

use std::env;
use std::ffi::CString;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Runner {
    prog: String,
    params: Vec<String>,
}

impl Runner {
    fn new(prog: &str, params: Vec<String>) -> Self {
        Runner { 
            prog: prog.to_string(), 
            params 
        }
    }

    // Unsafe method demonstrating improper handling of search path
    // In this implementation, an attacker-controlled environment variable ("MALICIOUS_PATH")
    // is used to override the system PATH. An unsafe block simulates low-level manipulation.
    unsafe fn execute(&self) -> Result<String, String> {
        // Vulnerable: use attacker-controlled environment variable to set PATH
        if let Ok(user_path) = env::var("MALICIOUS_PATH") {
            // Unsafe raw pointer operations to mimic complex unsafe usage.
            let c_user_path = CString::new(user_path.clone()).map_err(|e| e.to_string())?;
            let ptr = c_user_path.as_ptr();
            // Deliberately unsafely create a slice from the raw pointer
            let slice = std::slice::from_raw_parts(ptr as *const u8, user_path.len());
            let new_path = String::from_utf8_lossy(slice).to_string();
            env::set_var("PATH", new_path);
        }
        // Spawn the process with the altered PATH, which may cause untrusted binaries to run.
        let output = Command::new(&self.prog)
            .args(&self.params)
            .output()
            .map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(format!(
                "Process error: {}",
                String::from_utf8_lossy(&output.stderr)
            ))
        }
    }
}

fn main() {
    // Demonstrate concurrency and shared state via Arc and Mutex
    let shared_runner = Arc::new(Mutex::new(Runner::new("echo", vec!["Hello".to_string()])));
    let mut threads = vec![];

    for _ in 0..4 {
        let runner_copy = Arc::clone(&shared_runner);
        let handle = thread::spawn(move || {
            let guard = runner_copy.lock().unwrap();
            unsafe {
                match guard.execute() {
                    Ok(result) => println!("Result: {}", result),
                    Err(err) => eprintln!("Error: {}", err),
                }
            }
        });
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }
}