------------------ vulnerable.rs ------------------
#![allow(unused_unsafe)]
use std::env;
use std::ffi::{CStr, CString};
use std::process::Command;
use std::thread;

struct Executor;

impl Executor {
    // This function launches an external shell command.
    fn launch(&self, path: &str) -> i32 {
        // Simulate a vulnerability: if the provided search PATH is malicious,
        // we simulate that it causes the command lookup to fail (return non-zero exit code).
        if path.contains("/tmp/malicious") {
            // Attacker-controlled search path detected.
            return 1;
        }
        // Otherwise, launch a shell command using the provided environment PATH.
        let mut child = Command::new("sh")
            .arg("-c")
            .arg("echo Execution")
            .env("PATH", path)
            .spawn()
            .expect("failed to spawn process");
        let status = child.wait().expect("process wasn't running");
        status.code().unwrap_or(-1)
    }
}

// Unsafe routine that simulates conversion from a raw C string pointer to a Rust String.
// In real-world code this might be used for FFI interactions.
unsafe fn convert_raw(ptr: *mut i8) -> String {
    let c_str = CStr::from_ptr(ptr);
    c_str.to_string_lossy().into_owned()
}

// Simulate obtaining an untrusted search path from the environment.
// An attacker can set MAL_INPUT to a malicious value.
fn acquire_untrusted() -> String {
    env::var("MAL_INPUT").unwrap_or_else(|_| String::from("/malicious/path"))
}

fn main() {
    let exec = Executor{};
    let untrusted_input = acquire_untrusted();

    // Use unsafe block to simulate a risky conversion of a mutable C string pointer.
    let mut raw_bytes = untrusted_input.clone().into_bytes();
    raw_bytes.push(0); // add nul terminator
    let boxed = raw_bytes.into_boxed_slice();
    let ptr = Box::into_raw(boxed) as *mut i8;
    let converted = unsafe { convert_raw(ptr) };
    // We intentionally do not free 'ptr' to mimic a real-world unsafe FFI conversion issue.

    // Launch the command concurrently.
    let handle = thread::spawn(move || {
        exec.launch(&converted)
    });
    let result = handle.join().expect("Thread panicked");
    println!("Process exited with code: {}", result);
}
----------------------------------------------------