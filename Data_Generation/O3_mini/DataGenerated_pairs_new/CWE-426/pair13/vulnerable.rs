/////////////////// Vulnerable Code ///////////////////
use std::process::Command;
use std::sync::Arc;
use std::thread;
use std::ffi::CString;
use std::os::raw::c_char;

/// This unsafe function simulates an external command execution using an untrusted search path.
/// It intentionally “executes” the command given by a raw C string without verifying its absolute path.
unsafe fn external_exec(cmd: *const c_char) -> i32 {
    if cmd.is_null() {
        return -1;
    }
    // Convert the raw pointer back to a CString.
    let cstr = CString::from_raw(cmd as *mut c_char);
    let cmd_str = cstr.to_str().unwrap_or_default();
    // Simulated execution: if the command name matches a known insecure name,
    // we return 0 to simulate that it was executed (even though it was loaded from a relative path).
    // This mimics the CWE-426 Untrusted Search Path vulnerability.
    if cmd_str == "insecure_app" {
        // Vulnerability: executing a binary from an untrusted, relative path.
        0
    } else {
        -1
    }
}

/// A loader structure that holds the executable command name.
struct Loader {
    pub cmd: String,
}

impl Loader {
    /// Runs the external command using an unsafe FFI-like call.
    fn run(&self) -> i32 {
        unsafe {
            // Create a CString and leak its raw pointer to pass into the unsafe function.
            let ptr = CString::new(self.cmd.clone()).unwrap().into_raw();
            let ret = external_exec(ptr);
            // Note: The raw pointer is not reclaimed, intentionally simulating an unsafe misuse.
            ret
        }
    }
}

fn main() {
    // Vulnerable: using a relative (untrusted search path) command name.
    let loader = Arc::new(Loader {
        cmd: "insecure_app".to_string(), // relative path exploitation risk
    });

    let mut handles = Vec::new();
    // Spawn several threads simulating concurrent usage.
    for _ in 0..4 {
        let loader_clone = Arc::clone(&loader);
        let handle = thread::spawn(move || {
            let _ = loader_clone.run();
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}