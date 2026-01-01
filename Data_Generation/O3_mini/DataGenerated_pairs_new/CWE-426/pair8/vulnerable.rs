////////////////////////////////////////////////////////////
// Vulnerability demonstration: Untrusted Search Path
// CWE-426: Untrusted Search Path - Loading executable by relative path.
// This simulation uses an unsafe block and concurrency to mimic a real-world issue.
////////////////////////////////////////////////////////////

use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Loader {
    // Insecure: The command is stored as a relative path.
    command: String,
}

impl Loader {
    fn new() -> Self {
        // Vulnerable: using a relative executable name.
        Loader { command: String::from("vuln_cmd_exe") }
    }

    fn run(&self) -> Result<String, String> {
        // When running in test mode, simulate the insecure behavior.
        if std::env::var("TEST_MODE").is_ok() {
            return Err(String::from("Insecure execution: relative path used"));
        }
        // Clone the command string.
        let cmd_clone = self.command.clone();
        // Unsafe block to simulate raw pointer manipulation.
        let ptr: *const u8 = unsafe { cmd_clone.as_ptr().offset(0) };
        unsafe {
            let bytes = std::slice::from_raw_parts(ptr, cmd_clone.len());
            let loaded_cmd = String::from_utf8_lossy(bytes).into_owned();
            // Vulnerable usage: launching a process using a relative path,
            // which can be hijacked via an untrusted search path.
            let output = Command::new(loaded_cmd)
                .arg("--version")
                .output();
            match output {
                Ok(o) => Ok(String::from_utf8_lossy(&o.stdout).into_owned()),
                Err(e) => Err(format!("Process launch failed: {}", e)),
            }
        }
    }
}

fn main() {
    let instance = Arc::new(Loader::new());
    let mut threads = vec![];

    // Spawn several threads to simulate concurrent execution.
    for _ in 0..4 {
        let inst = Arc::clone(&instance);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10)); // simulate asynchronous timing
            match inst.run() {
                Ok(out) => println!("Result: {}", out),
                Err(err) => println!("Error: {}", err),
            }
        });
        threads.push(handle);
    }

    for handle in threads {
        let _ = handle.join();
    }
}