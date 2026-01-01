#![allow(unused)]
use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

struct Runner {
    prog: String,
}

impl Runner {
    // This function uses an unsafe block to emulate risky low‐level access (a no‑op in this context)
    // and, critically, uses the PATH environment variable directly obtained from the process environment.
    // An attacker may manipulate PATH (or LD_LIBRARY_PATH on other systems) to force the loading
    // of a malicious executable version of the command.
    fn execute(&self) -> Result<String, String> {
        unsafe {
            // Retrieve the current PATH without any validation.
            let path_value = env::var("PATH")
                .map_err(|e| format!("Failed to retrieve PATH: {:?}", e))?;
            // Simulate unsafe use by obtaining a raw pointer to the PATH string.
            let _raw_ptr = path_value.as_ptr();
            // Spawn the process using the attacker-controlled PATH.
            let output = Command::new(&self.prog)
                .env("PATH", path_value) // vulnerability: unsanitized environment variable
                .arg("--version")
                .stdout(Stdio::piped())
                .output()
                .map_err(|e| format!("Execution failure: {:?}", e))?;
            if output.status.success() {
                Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
            } else {
                Err("Command exited with a non-zero status".into())
            }
        }
    }
}

fn main() {
    // Create a shared Runner instance guarded by a Mutex.
    let runner = Arc::new(Mutex::new(Runner { prog: "ls".to_string() }));
    let mut threads = vec![];
    
    // Spawn several threads that concurrently execute the command.
    for _ in 0..4 {
        let runner_clone = Arc::clone(&runner);
        let handle = thread::spawn(move || {
            let guard = runner_clone.lock().unwrap();
            match guard.execute() {
                Ok(output) => println!("Output: {}", output),
                Err(err) => eprintln!("Error: {}", err),
            }
        });
        threads.push(handle);
    }
    
    for t in threads {
        t.join().unwrap();
    }
}