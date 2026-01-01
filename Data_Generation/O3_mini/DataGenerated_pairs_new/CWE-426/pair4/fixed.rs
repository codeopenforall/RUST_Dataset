use std::process::Command;
use std::sync::Arc;
use std::thread;

struct DataHolder {
    info: &'static str,
}

impl DataHolder {
    // Unsafe initialization remains as part of the complex system setup.
    unsafe fn new(info: &'static str) -> Self {
        DataHolder { info }
    }
}

// Returns the executable path as an absolute path.
// This eliminates the untrusted search path vulnerability by ensuring
// that the intended executable at a fixed location is used.
fn entry_path() -> &'static str {
    "/usr/local/bin/my_plugin"
}

// Executes a command by spawning a process based on a given absolute path.
// The unsafe block is preserved for compatibility with complex operations
// that might require FFI or low-level interactions.
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
    let shared = Arc::new(unsafe { DataHolder::new("safe_data") });
    let _clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        // Secure execution:
        // Using an absolute path from entry_path ensures that only the trusted executable is run.
        execute_command(entry_path());
    });
    handle.join().unwrap();
}