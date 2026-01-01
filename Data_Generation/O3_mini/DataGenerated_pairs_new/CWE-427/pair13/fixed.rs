////////////////////////////////////////////////////////////////////////////////
// Fixed Example: Controlled Search Path Element (CWE-427 Mitigation)
//
// This version sanitizes the input by filtering the supplied search path against
// a fixed whitelist of allowed directories. The safe search path is then used when
// setting the system PATH prior to spawning a command. The unsafe code remains for
// demonstration purposes but now operates on a validated string.
////////////////////////////////////////////////////////////////////////////////
use std::env;
use std::process::Command;
use std::thread;
use std::sync::{Arc, Mutex};

struct Runner {
    // Contains the externally provided search path string.
    input_search: String,
}

impl Runner {
    // Validate and filter the search path removing any directories not in the whitelist.
    fn sanitize(&self) -> String {
        let safe_whitelist = vec!["/usr/bin", "/bin", "/usr/local/bin"];
        let parts: Vec<&str> = self.input_search.split(':').collect();
        let validated: Vec<&str> = parts.into_iter()
            .filter(|d| safe_whitelist.contains(d))
            .collect();
        if validated.is_empty() {
            // Fallback to a default safe search path if filtering removes all entries.
            safe_whitelist.join(":")
        } else {
            validated.join(":")
        }
    }

    // Execute the command using the sanitized search path.
    fn execute(&self) {
        let validated_search = self.sanitize();
        // Set the system PATH to the validated search path.
        env::set_var("PATH", &validated_search);

        // Demonstrate sharing an immutable pointer to the validated search path.
        let raw_ptr = validated_search.as_ptr();
        let len = validated_search.len();
        let shared_ptr = Arc::new(Mutex::new(raw_ptr));
        let handle = {
            let sp = Arc::clone(&shared_ptr);
            thread::spawn(move || {
                unsafe {
                    let stored_ptr = *sp.lock().unwrap();
                    let slice = std::slice::from_raw_parts(stored_ptr, len);
                    // Safe conversion since the validated string is proper UTF-8.
                    let path_str = std::str::from_utf8(slice).unwrap();
                    println!("Validated search path: {}", path_str);
                }
            })
        };
        handle.join().unwrap();

        // Execute a process that now uses the sanitized search path.
        let output = Command::new("echo")
            .arg("Executing secure command")
            .output()
            .expect("command execution failed");

        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}

fn main() {
    // Read the externally supplied "BAD_PATH", defaulting if not provided.
    let input_search = env::var("BAD_PATH")
        .unwrap_or_else(|_| "/usr/bin:/bin".to_string());
    let runner = Runner { input_search };
    runner.execute();
}