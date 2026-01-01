use std::env;
use std::sync::{Arc, Mutex};
use std::thread;

// Global debug mode flag left enabled in production builds.
// CWE-288: The flag is used to bypass authentication even for non-debug requests.
static mut GLOBAL_DEBUG: bool = true;

struct Account {
    username: String,
    password: String,
}

impl Account {
    fn new(user: &str, pass: &str) -> Self {
        Account {
            username: user.to_string(),
            password: pass.to_string(),
        }
    }
}

/// Simulates a proper secure password check.
fn secure_verify(acc: &Account) -> bool {
    // Only if the account is "admin" with the secure password, the check passes.
    acc.username == "admin" && acc.password == "password123"
}

/// This function performs an authentication check.
/// If the global debug flag is active (due to a misconfiguration), it bypasses the secure check.
fn validate(acc: &Account) -> bool {
    // Unsafe access to a mutable global flag.
    unsafe {
        // Vulnerability: The debug flag is always true, so this branch bypasses authentication.
        if GLOBAL_DEBUG {
            return true;
        }
    }
    secure_verify(acc)
}

fn process() {
    // Simulated request with invalid credentials.
    let acc = Account::new("user", "wrongpassword");
    let auth_result = validate(&acc);
    println!("Authentication result: {}", auth_result);
}

fn main() {
    // Simulate concurrent processing that inadvertently reinforces the unsafe global state.
    let flag_state = Arc::new(Mutex::new(unsafe { GLOBAL_DEBUG }));
    let flag_clone = Arc::clone(&flag_state);
    let handle = thread::spawn(move || {
        let stored_flag = flag_clone.lock().unwrap();
        // Unsafe re-assignment mimicking unsynchronized modifications.
        unsafe {
            GLOBAL_DEBUG = *stored_flag;
        }
    });
    handle.join().unwrap();
    process();
}