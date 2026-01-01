//////////////////////////////////////////////////////////////
// Vulnerable Version - Unsafe executable loader with untrusted search path
//////////////////////////////////////////////////////////////

use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;

struct Loader {
    // The path to the executable; using a relative path may cause untrusted search path issues.
    path: String,
}

impl Loader {
    // Executes the external command. The unsafe block simulates a low‐level conversion.
    // In this vulnerable variant, if a relative path is provided, it “simulates” loading a hijacked binary.
    fn invoke(&self) -> Result<String, &'static str> {
        unsafe {
            // Unnecessary unsafe conversion (for demonstration) of the path pointer.
            let _raw = self.path.as_ptr() as *const u8;

            // Vulnerability: using a relative executable path without validation.
            // If the path does not start with '/', we simulate that a malicious binary is loaded.
            if !self.path.starts_with("/") {
                // Instead of executing a trusted binary, a malicious one (hijacked) is run.
                // This simulates the risk of DLL or executable preloading from an untrusted location.
                return Ok("hijacked".to_string());
            }

            // For absolute paths, proceed to call the external command.
            let output = Command::new(self.path.clone())
                .output()
                .map_err(|_| "Command execution failed")?;
            String::from_utf8(output.stdout).map_err(|_| "Output was not valid UTF-8")
        }
    }
}

fn main() {
    // Here we deliberately use a relative path to simulate the vulnerability (e.g. "insecure").
    let loader = Arc::new(Mutex::new(Loader { path: "insecure".to_string() }));
    let loader_clone = loader.clone();
    let handle = thread::spawn(move || {
        let instance = loader_clone.lock().unwrap();
        match instance.invoke() {
            Ok(res) => println!("Result: {}", res),
            Err(err) => eprintln!("Error: {}", err),
        }
    });
    handle.join().unwrap();
}