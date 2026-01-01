/////////////////////// VULNERABLE CODE ///////////////////////
use std::fmt;
use std::sync::{Arc, Mutex};
use std::error::Error;
use std::thread;

#[derive(Clone)]
struct Credential {
    username: String,
    password: String, // sensitive field: secret information exposed
}

impl Credential {
    fn new(u: &str, p: &str) -> Self {
        Credential {
            username: u.to_string(),
            password: p.to_string(),
        }
    }
}

// Debug implementation that directly prints the sensitive field.
impl fmt::Debug for Credential {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // CWE-200: Exposing sensitive data via logging/debug output.
        write!(f, "Credential {{ username: {}, password: {} }}", self.username, self.password)
    }
}

// Custom error type that embeds the sensitive credential.
#[derive(Debug)]
struct SecError {
    details: String,
    cred: Credential, // includes sensitive details
}

impl fmt::Display for SecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Directly incorporate the credential's Debug output (sensitive info) in the error.
        write!(f, "SecError: {} with credential {:?}", self.details, self.cred)
    }
}

impl Error for SecError {}

fn process(cred: &Credential) -> Result<(), SecError> {
    unsafe {
        // Simulate a complex unsafe pointer dereference.
        let ptr = cred as *const Credential;
        let data = &*ptr;
        // A condition triggering an error: if the username is too short.
        if data.username.len() < 5 {
            return Err(SecError {
                details: "Username too short".to_string(),
                cred: data.clone(),
            });
        }
    }
    Ok(())
}

fn concurrent_process(cred: Arc<Credential>) -> Result<(), SecError> {
    // Using a mutex-protected shared error container to simulate concurrent error reporting.
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
        // Again, the error message here will expose the sensitive data.
        return Err(SecError {
            details: format!("Concurrent error: {}", err.details),
            cred: err.cred.clone(),
        });
    }
    Ok(())
}

fn main() {
    // Username deliberately short to trigger the error and leak details.
    let cred = Arc::new(Credential::new("usr", "SuperSecretPassword123"));
    match concurrent_process(cred.clone()) {
        Ok(()) => println!("Operation succeeded"),
        Err(e) => {
            // Logging the error with Debug: leaks the secret.
            eprintln!("Operation failed with error: {:?}", e);
            // Panic including sensitive information.
            panic!("Critical failure with detailed state: {:?}", e);
        }
    }
}
//////////////////////////////////////////////////////////////