//////////////////////////////////////////////////////////////////
// This Rust code sample demonstrates a flaw in how an application
// handles uncontrolled search path elements (CWE-427). It unsafely
// appends a library path from an environment variable directly
// into the system PATH. This might allow an attacker to inject a
// malicious directory into the search path when the program spawns
// a subprocess. The code also uses unsafe pointer manipulation,
// concurrency, lifetimes, and Arc to mimic a realistic, complex
// structure seen in real-world issues.
//////////////////////////////////////////////////////////////////

use std::env;
use std::process::Command;
use std::sync::Arc;
use std::thread;

struct Executor<'a> {
    info: &'a str,
}

impl<'a> Executor<'a> {
    fn new(text: &'a str) -> Executor<'a> {
        Executor { info: text }
    }
    
    // The unsafe block below dereferences a raw pointer from a user-supplied string.
    // It directly appends the content of the library path provided by the environment variable
    // to the system PATH variable without proper validation.
    unsafe fn run_command(&self, lib: &str) -> Result<String, String> {
        // Unsafe processing: obtain a raw pointer from the incoming string.
        let ptr_lib = lib.as_ptr();
        // Retrieve current PATH (possibly empty)
        let mut current = env::var("PATH").unwrap_or_default();
        // Vulnerable: appending a colon without sanitizing input.
        current.push_str(":");
        // Convert back to string from raw parts unsafely.
        let slice = std::slice::from_raw_parts(ptr_lib, lib.len());
        let untrusted = std::str::from_utf8_unchecked(slice);
        current.push_str(untrusted);
        // Setting the system PATH with inserted untrusted data.
        env::set_var("PATH", &current);
        
        // Spawn a command that uses PATH to look up executables.
        let output = Command::new("echo").arg(self.info).output();
        match output {
            Ok(out) => Ok(String::from_utf8_lossy(&out.stdout).to_string()),
            Err(e) => Err(format!("Command execution error: {}", e)),
        }
    }
}

fn main() {
    let executor = Executor::new("vulnerable run");
    // Read a library path from an environment variable (attacker-controlled)
    let lib_input = env::var("CUSTOM_PATH").unwrap_or_else(|_| "/tmp/insecure".to_string());
    
    let shared = Arc::new(executor);
    let cloned = Arc::clone(&shared);
    
    // Simulate concurrency by spawning a thread.
    let handler = thread::spawn(move || {
        unsafe { cloned.run_command(&lib_input) }
    });
    
    match handler.join() {
        Ok(Ok(result)) => println!("Output: {}", result),
        _ => println!("Execution error occurred"),
    }
}