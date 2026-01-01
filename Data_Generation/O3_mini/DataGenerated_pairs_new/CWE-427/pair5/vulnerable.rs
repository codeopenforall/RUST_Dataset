//////////////////////////
// CWE-427: Uncontrolled Search Path Element
// This Rust program erroneously uses an attacker-controlled environment variable to set the PATH for a spawned command.
// It also employs an unsafe block interacting with raw pointers and multi-threading to mimic complex real-world scenarios.
//////////////////////////

use std::env;
use std::process::Command;
use std::thread;
use std::sync::Arc;
use libc;

fn main() {
    // Read the potentially attacker-controlled environment variable.
    let user_input = env::var("INJECTED_PATH").unwrap_or_else(|_| String::from(""));
    let shared_data = Arc::new(user_input);

    // Unsafe block: directly interprets the Arc's data as a null-terminated C string.
    // This is dangerous if the data is not properly formatted.
    unsafe {
        let raw_ptr = Arc::as_ptr(&shared_data) as *const i8;
        // Vulnerability: relying on unchecked external input which may not be null terminated.
        libc::strlen(raw_ptr);
    }

    // Spawn a new thread which uses the uncontrolled search path element while invoking an external command.
    let data_clone = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        // Vulnerable use: Pass the attacker-controlled string directly as the PATH environment variable.
        let status = Command::new("echo")
            .env("PATH", data_clone.as_str())
            .status();

        // Exit based on the external command's result.
        if let Ok(st) = status {
            std::process::exit(if st.success() { 0 } else { 1 });
        }
        std::process::exit(1);
    });

    let _ = handle.join();
}