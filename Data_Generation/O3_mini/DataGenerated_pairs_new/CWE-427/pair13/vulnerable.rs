////////////////////////////////////////////////////////////////////////////////
// Vulnerability Example: Uncontrolled Search Path Element (CWE-427)
//
// This code accepts a user-controlled PATH string from the environment variable
// "BAD_PATH" without validation, uses unsafe pointer operations in a threaded
// context, and then sets the system PATH variable to this unsanitized value before
// spawning a command. An attacker can inject a malicious directory into the search path.
////////////////////////////////////////////////////////////////////////////////
use std::env;
use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};

struct Executor {
    // Contains the raw search path supplied by an external source.
    user_search: String,
}

impl Executor {
    // This method demonstrates unsafe pointer usage in a concurrent thread and
    // then uses the unvalidated search path to set the process PATH for command resolution.
    fn run(&self) {
        // Create a raw pointer to the user-supplied search path.
        let raw_ptr = self.user_search.as_ptr();
        let len = self.user_search.len();

        // Simulate concurrent access by sharing the raw pointer inside a Mutex wrapped in an Arc.
        let shared_ptr = Arc::new(Mutex::new(raw_ptr));
        let handle = {
            let sp = Arc::clone(&shared_ptr);
            thread::spawn(move || {
                unsafe {
                    // Unsafely obtain and convert the raw pointer to a slice.
                    let stored_ptr = *sp.lock().unwrap();
                    let slice = std::slice::from_raw_parts(stored_ptr, len);
                    // The conversion here skips UTF-8 validation.
                    let path_str = std::str::from_utf8_unchecked(slice);
                    println!("User supplied search path: {}", path_str);
                }
            })
        };
        handle.join().unwrap();

        // Vulnerability: Untrusted search path is used to set the system PATH.
        env::set_var("PATH", &self.user_search);

        // Spawn a process that relies on PATH resolution.
        let output = Command::new("echo")
            .arg("Executing command")
            .output()
            .expect("failed to execute process");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn main() {
    // Read the user-controlled "BAD_PATH" without any validation.
    let user_search = env::var("BAD_PATH")
        .unwrap_or_else(|_| "/usr/bin:/bin".to_string());
    let exec = Executor { user_search };
    exec.run();
}