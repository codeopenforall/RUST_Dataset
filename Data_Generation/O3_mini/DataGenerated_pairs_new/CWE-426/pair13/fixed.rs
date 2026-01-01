//////////////////// Fixed Code ////////////////////
use std::process::Command;
use std::path::Path;
use std::sync::Arc;
use std::thread;
use std::ffi::CString;
use std::os::raw::c_char;


/// This unsafe function now validates that the provided command name represents an absolute path.
/// If not, it will not execute the command, mitigating the CWE-426 risk.
unsafe fn external_exec(cmd: *const c_char) -> i32 {
    if cmd.is_null() {
        return -1;
    }
    // Convert the raw pointer back to a CString.
    let cstr = CString::from_raw(cmd as *mut c_char);
    let cmd_str = cstr.to_str().unwrap_or_default();
    // Validate that the command is given as an absolute path.
    if Path::new(cmd_str).is_absolute() {
        // In a real-world scenario, we would execute the command.
        // Here we simulate a successful execution.
        0
    } else {
        // Reject execution when the path is not absolute.
        -1
    }
}

/// A loader structure that holds the executable command name.
struct Loader {
    pub cmd: String,
}

impl Loader {
    /// Runs the external command using an unsafe FFI-like call with proper path validation.
    fn run(&self) -> i32 {
        unsafe {
            // Create a CString and leak its raw pointer to pass into the unsafe function.
            let ptr = CString::new(self.cmd.clone()).unwrap().into_raw();
            let ret = external_exec(ptr);
            ret
        }
    }
}

fn main() {
    // Fixed: using an absolute path to the trusted executable.
    let loader = Arc::new(Loader {
        cmd: "/usr/bin/safe_app".to_string(), // absolute path, mitigating untrusted search path risk
    });

    let mut handles = Vec::new();
    // Spawn several threads to simulate concurrent usage.
    for _ in 0..4 {
        let loader_clone = Arc::clone(&loader);
        let handle = thread::spawn(move || {
            let code = loader_clone.run();
            // In this fixed version, we check that the execution simulation succeeded.
            if code != 0 {
                std::process::exit(1);
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join();
    }
}