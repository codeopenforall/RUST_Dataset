//////////////////////////////////////////////
// Vulnerable Code Example - Improper Authentication
//////////////////////////////////////////////
use std::thread;

static mut GLOBAL_TOKEN: Option<String> = None;

struct Service {
    token: String,
}

impl Service {
    // Initializes the service and sets a global token unsafely.
    pub fn new() -> Self {
        unsafe {
            GLOBAL_TOKEN = Some("supersecrettoken".to_string());
        }
        Self {
            token: "supersecrettoken".to_string(),
        }
    }

    // Flawed authentication: It simply checks that a provided token is non-empty,
    // ignoring whether it actually matches the expected secure token.
    // Additionally, it unsafely accesses a global variable in a spawned thread.
    pub fn authenticate(&self, input: Option<String>) -> bool {
        if let Some(user_token) = input {
            // Spawn a thread that unsafely accesses the global token
            thread::spawn(|| {
                unsafe {
                    if let Some(ref expected) = GLOBAL_TOKEN {
                        // Debug print to simulate concurrent usage of the insecure global variable.
                        println!("DEBUG: Global token is {}", expected);
                    }
                }
            });
            // Vulnerability: any non-empty token is accepted.
            return !user_token.is_empty();
        }
        false
    }
}

fn main() {
    let svc = Service::new();
    // Test input that is intentionally incorrect.
    let input = Some("wrongtoken".to_string());
    if svc.authenticate(input) {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}