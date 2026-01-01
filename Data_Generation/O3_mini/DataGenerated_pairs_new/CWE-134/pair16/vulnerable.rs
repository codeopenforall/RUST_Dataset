////////////////////////////////////////////////////////////
// Vulnerable Code Sample
////////////////////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct LogHolder {
    secret: &'static str,
}

impl LogHolder {
    fn new() -> Self {
        LogHolder { secret: "VerySecretInfo" }
    }

    // This method uses a user-supplied string directly as the format string.
    // It unsafely retrieves the secret value and then invokes the formatting macro.
    // If an attacker supplies a malicious format string, the "secret" can be exposed.
    fn process(&self, user_fmt: &str) -> String {
        // Unsafe block to mimic use of raw pointer operations in real-world code.
        let secret_ptr = self.secret.as_ptr();
        let secret_str: &str = unsafe {
            // Reconstruct a &str without verifying UTF-8 (for demonstration only)
            let slice = std::slice::from_raw_parts(secret_ptr, self.secret.len());
            std::str::from_utf8_unchecked(slice)
        };

        // Vulnerability: the user-supplied string is directly used as a format string.
        // An attacker can inject format specifiers to reveal internal secrets.
        format!(user_fmt, secret_str)
    }
}

// Simulate concurrent access via a spawned thread.
fn simulate(user_input: &str) -> String {
    let logger = Arc::new(LogHolder::new());
    let output = Arc::new(Mutex::new(String::new()));

    let logger_clone = Arc::clone(&logger);
    let output_clone = Arc::clone(&output);

    let handle = thread::spawn(move || {
        let result = logger_clone.process(user_input);
        let mut out_lock = output_clone.lock().unwrap();
        *out_lock = result;
    });

    handle.join().unwrap();
    let res = output.lock().unwrap();
    res.clone()
}

fn main() {
    // In a realistic scenario this string would come from an external source.
    // Using a malicious format string to expose the secret.
    let user_controlled_input = "Alert: {}";
    let result = simulate(user_controlled_input);
    println!("Result: {}", result);
}