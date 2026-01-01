use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

struct Executor;

impl Executor {
    // Executes the provided input as a shell command.
    fn run_command(&self, input: &str) -> Result<String, String> {
        // Unsafe conversion from Rust &str to a C-style string for demonstration.
        let command = unsafe {
            // NOTE: This unsafe block is misused: it creates a CString from input without
            // any sanitization, then converts it back. This pattern is unnecessary and error‐prone.
            let c_str = std::ffi::CString::new(input).unwrap();
            std::ffi::CStr::from_ptr(c_str.as_ptr()).to_string_lossy().into_owned()
        };
        // Vulnerability: The unsanitized command is passed directly to the shell via "sh -c".
        let output = Command::new("sh")
            .arg("-c")
            .arg(&command)
            .output()
            .map_err(|e| e.to_string())?;
        if output.status.success() {
            Ok(String::from_utf8_lossy(&output.stdout).to_string())
        } else {
            Err(String::from_utf8_lossy(&output.stderr).to_string())
        }
    }
}

// Function to simulate concurrent command execution.
fn concurrent_exec(input: &str) -> Vec<Result<String, String>> {
    let executor = Arc::new(Mutex::new(Executor));
    let mut handles = vec![];
    for _ in 0..4 {
        let exec = Arc::clone(&executor);
        let input_data = input.to_string();
        let handle = thread::spawn(move || {
            let exec_locked = exec.lock().unwrap();
            exec_locked.run_command(&input_data)
        });
        handles.push(handle);
    }
    handles.into_iter().map(|h| h.join().unwrap()).collect()
}

fn main() {
    // Unsafe payload: an attacker could inject additional commands.
    let injection_input = "echo Hello; echo hacked";
    let results = concurrent_exec(injection_input);
    for res in results {
        match res {
            Ok(out) => println!("{}", out),
            Err(e) => eprintln!("{}", e),
        }
    }
}