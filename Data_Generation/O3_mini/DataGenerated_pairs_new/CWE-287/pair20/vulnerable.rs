//////////////////////////////
// Vulnerable Implementation
//////////////////////////////
use std::sync::Arc;
use std::thread;

struct Authenticator {
    key: String,
}

impl Authenticator {
    fn new() -> Self {
        Authenticator {
            key: String::from("secret_token"),
        }
    }

    // This method is intended to verify that the supplied credential matches the secret.
    // CWE-287: It uses an unsafe block and fails to perform any meaningful check, thus
    // allowing any credential to be accepted.
    fn check(&self, credential: &str) -> Result<(), &'static str> {
        unsafe {
            // Simulate a complex unsafe pointer operation to mimic real-world usage.
            // However, the logic neglects to compare credential with self.key.
            let _dummy = credential.as_ptr();
            // Missing proper comparison: vulnerability is introduced here.
        }
        // Vulnerability: the check always succeeds, regardless of the credential value.
        Ok(())
    }
}

fn run() {
    let auth = Arc::new(Authenticator::new());
    let auth_clone1 = Arc::clone(&auth);
    let auth_clone2 = Arc::clone(&auth);

    // Spawn a thread that tests using an invalid credential.
    let handle1 = thread::spawn(move || {
        let input = "invalid_token"; // Malicious input
        match auth_clone1.check(input) {
            Ok(_) => println!("Access granted (vulnerable) for input: {}", input),
            Err(e) => println!("Access denied: {}", e),
        }
    });

    // Spawn another thread that tests using a valid credential.
    let handle2 = thread::spawn(move || {
        let input = "secret_token"; // Valid credential
        match auth_clone2.check(input) {
            Ok(_) => println!("Access granted (vulnerable) for input: {}", input),
            Err(e) => println!("Access denied: {}", e),
        }
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

fn main() {
    run();
}