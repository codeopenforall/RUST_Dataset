/////////////////////// FIXED CODE ///////////////////////
use std::fmt;
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::thread;

#[derive(Clone)]
struct Credential {
    username: String,
    password: String, // sensitive field: secret information
}

impl Credential {
    fn new(u: &str, p: &str) -> Self {
        Credential {
            username: u.to_string(),
            password: p.to_string(),
        }
    }
}

// Revised Debug implementation that redacts the sensitive field.
impl fmt::Debug for Credential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Only log non-sensitive info.
        write!(f, "Credential {{ username: {}, password: [REDACTED] }}", self.username)
    }
}

// Custom error type without embedding sensitive credential data.
#[derive(Debug)]
struct SecError {
    details: String,
}

impl fmt::Display for SecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Do not print any sensitive data in the error message.
        write!(f, "SecError: {}", self.details)
    }
}

impl Error for SecError {}

fn process(cred: &Credential) -> Result<(), SecError> {
    unsafe {
        let ptr = cred as *const Credential;
        let data = &*ptr;
        if data.username.len() < 5 {
            // Return generic error details without the credential info.
            return Err(SecError {
                details: "Username too short".to_string(),
            });
        }
    }
    Ok(())
}

fn concurrent_process(cred: Arc<Credential>) -> Result<(), SecError> {
    let shared_err = Arc::new(Mutex::new(None));
    let mut handles = vec![];

    for _ in 0..4 {
        let cred_clone = cred.clone();
        let err_clone = shared_err.clone();
        let handle = thread::spawn(move || {
            let res = process(&cred_clone);
            if res.is_err() {
                let mut guard = err_clone.lock().unwrap();
                *guard = res.err();
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_err = shared_err.lock().unwrap();
    if let Some(ref err) = *final_err {
        return Err(SecError {
            details: format!("Concurrent error: {}", err.details),
        });
    }
    Ok(())
}

fn main() {
    // The same triggering condition is used, but error messages will not leak the secret.
    let cred = Arc::new(Credential::new("usr", "SuperSecretPassword123"));
    match concurrent_process(cred.clone()) {
        Ok(()) => println!("Operation succeeded"),
        Err(e) => {
            // Log error without sensitive data.
            eprintln!("Operation failed with error: {}", e);
            // Panic with a generic error message.
            panic!("Critical failure encountered");
        }
    }
}
///////////////////////////////////////////////////////////