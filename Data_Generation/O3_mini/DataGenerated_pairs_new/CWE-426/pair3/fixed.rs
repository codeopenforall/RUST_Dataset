////////////////////////////////////////////////////////////////////////////////
// The following code fixes the untrusted search path vulnerability by enforcing 
// that any external executable must be specified using an absolute path. Before 
// launching the process, the path is validated. An attacker cannot substitute a 
// malicious executable unless it is located at the expected absolute location.
// A helper function (check_command) is provided for external test purposes.
////////////////////////////////////////////////////////////////////////////////

use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};
use std::path::Path;

struct Runner {
    binary: String,
}

impl Runner {
    fn new(input: &str) -> Self {
        Self { binary: input.to_string() }
    }

    // Executes the external command with a safety check ensuring that the 
    // binary is provided as an absolute path.
    fn run(&self) -> Result<String, String> {
        let mut output = String::new();
        let bin_path = Path::new(&self.binary);
        if !bin_path.is_absolute() {
            return Err("Command must be absolute path".into());
        }
        let result = unsafe {
            Command::new(&self.binary)
                .arg("--version")
                .output()
        };
        match result {
            Ok(o) => {
                output = String::from_utf8_lossy(&o.stdout).to_string();
                Ok(output)
            },
            Err(e) => Err(format!("Error launching command: {}", e))
        }
    }
}

// Public helper for external tests to trigger the safe behavior.
pub fn check_command(input: &str) -> Result<String, String> {
    let instance = Runner::new(input);
    instance.run()
}

fn execute_parallel() -> i32 {
    let shared = Arc::new(Mutex::new(Runner::new("/usr/bin/untrusted_binary")));
    let mut handles = vec![];

    // Spawn two threads concurrently invoking the command.
    for _ in 0..2 {
        let thread_runner = shared.clone();
        let handle = thread::spawn(move || {
            let guard = thread_runner.lock().unwrap();
            let _ = guard.run();
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    0
}

fn main() {
    // The fixed version uses an absolute path for the external binary.
    std::process::exit(execute_parallel());
}