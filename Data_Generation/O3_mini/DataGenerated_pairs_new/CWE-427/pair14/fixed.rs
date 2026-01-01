//////////////////////////////
// Corrected Version Code   //
// CWE-427 Mitigation: Validating and Setting a Secure PATH
//////////////////////////////

use std::env;
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

    // Safe method that does NOT rely on untrusted input for setting the system PATH.
    fn execute(&self) -> Result<String, String> {
        // Fixed: instead of using an untrusted variable, we set the PATH to a hardcoded safe value.
        let secure_path = "/usr/local/sbin:/usr/local/bin:/usr/sbin:/usr/bin:/sbin:/bin";
        env::set_var("PATH", secure_path);

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
            match guard.execute() {
                Ok(result) => println!("Result: {}", result),
                Err(err) => eprintln!("Error: {}", err),
            }
        });
        threads.push(handle);
    }

    for handle in threads {
        handle.join().unwrap();
    }
}