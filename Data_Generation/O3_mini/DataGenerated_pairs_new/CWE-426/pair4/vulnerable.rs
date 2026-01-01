use std::process::Command;
use std::sync::Arc;
use std::thread;

struct DataHolder {
    info: &'static str,
}

impl DataHolder {
    // Unsafe initialization simulating complex, low-level setup.
    unsafe fn new(info: &'static str) -> Self {
        DataHolder { info }
    }
}

// Returns the executable name using a relative path.
// This can lead to untrusted search path attacks (CWE-426),
// as an attacker might control the PATH environment variable
// to inject a malicious executable.
fn entry_path() -> &'static str {
    "my_plugin"
}

// Executes a command by spawning a process based on a given path.
// Uses an unsafe block to simulate low-level or FFI interactions.
fn execute_command(path: &str) {
    unsafe {
        let result = Command::new(path)
            .args(&["--run", "test"])
            .output()
            .expect("Failed to execute process");
        println!("Command output: {:?}", result);
    }
}

fn main() {
    let shared = Arc::new(unsafe { DataHolder::new("unsafe_data") });
    let _clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        // Vulnerability trigger:
        // Using a relative path here can be exploited via an untrusted search path.
        execute_command(entry_path());
    });
    handle.join().unwrap();
}