///////////////////////////////////////////
// Vulnerable Code Snippet (CWE-200)
// This code contains a flaw where sensitive information (the expected secret)
// is inadvertently exposed via detailed error messages during authentication failures.
// An unsafe block is used in simulating concurrent access to internal state, which is then
// exploited by including the sensitive data in the error message.
///////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};

struct Credentials {
    username: String,
    secret: String,
}

impl Credentials {
    fn new(user: &str, secret: &str) -> Self {
        Credentials {
            username: String::from(user),
            secret: String::from(secret),
        }
    }
}

struct AuthManager {
    creds: Arc<Mutex<Credentials>>,
}

impl AuthManager {
    fn new(creds: Credentials) -> Self {
        AuthManager {
            creds: Arc::new(Mutex::new(creds)),
        }
    }

    fn verify(&self, input: &str) -> Result<(), String> {
        // Lock the credentials and simulate unsafe concurrent access.
        let guard = self.creds.lock().unwrap();
        // Obtain an unsafe pointer to the secret.
        let ptr: *const String = &guard.secret;
        unsafe {
            // Mimic a delay that might allow unexpected concurrent interactions.
            thread::sleep(Duration::from_millis(10));
            // Check the input against the secret via unsafe dereference.
            if *ptr != String::from(input) {
                // Vulnerability: The error message reveals the expected secret.
                return Err(format!(
                    "Access denied for '{}'. Expected secret: {:?}",
                    guard.username,
                    *ptr
                ));
            }
        }
        Ok(())
    }
}

fn main() {
    let creds = Credentials::new("admin", "s3cr3t!");
    let manager = AuthManager::new(creds);
    // Provide an incorrect secret to trigger the error that leaks sensitive data.
    match manager.verify("wrong_password") {
        Ok(_) => println!("Access granted."),
        Err(err) => panic!("{}", err), // Panics with an error message that includes the secret.
    }
}