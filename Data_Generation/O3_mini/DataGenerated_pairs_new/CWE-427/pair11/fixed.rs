#![allow(unused)]
use std::env;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

struct Runner {
    prog: String,
}

impl Runner {
    // This revised implementation eliminates the unsafe block as it is no longer necessary,
    // and it explicitly sets the PATH variable to a trusted, fixed set of directories.
    // This prevents the influence of attacker-controlled search paths.
    fn execute(&self) -> Result<String, String> {
        // Establish a safe search path. On many Unix systems, "/usr/bin:/bin" is a trusted default.
        let trusted_path = "/usr/bin:/bin";
        let output = Command::new(&self.prog)
            .env("PATH", trusted_path) // override any external influence on PATH
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

fn main() {
    // Create a shared Runner instance guarded by a Mutex.
    let runner = Arc::new(Mutex::new(Runner { prog: "ls".to_string() }));
    let mut threads = vec![];
    
    // Spawn multiple threads to execute the command concurrently.
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