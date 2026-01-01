//////////////// Vulnerable version //////////////////
use std::fmt;
use std::sync::Arc;
use std::thread;

#[derive(Clone)]
struct Secret {
    user: String,
    key: String,
}

// The Debug implementation below mistakenly exposes the secret key.
impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Vulnerability: leaking sensitive information by printing the key
        write!(f, "Secret {{ user: {}, key: {} }}", self.user, self.key)
    }
}

// This function simulates a critical operation and deliberately triggers an error.
// It uses an unsafe block to mimic a low-level operation.
fn operate(s: Arc<Secret>) -> Result<(), String> {
    unsafe {
        // Dummy unsafe conversion: reading a byte from the secret key.
        let ptr = s.key.as_ptr();
        // (Potential UB if key were empty, omitted here for realism)
        let _ = *ptr;
    }
    // ERROR: The error message includes the complete Debug output of the secret,
    // thereby leaking the sensitive key.
    Err(format!("Operation failed due to an unexpected error: {:?}", s))
}

// Exposes an application logic function that spawns a thread to perform the operation.
// If an error occurs, it returns the error message.
pub fn run_app() -> String {
    let secret = Arc::new(Secret {
        user: "alice".into(),
        key: "SuperSecretPassword".into(),
    });
    let secret_cloned = Arc::clone(&secret);

    let handle = thread::spawn(move || operate(secret_cloned));
    match handle.join() {
        Ok(Err(e)) => e,
        Ok(Ok(_)) => "Success".into(),
        Err(err) => {
            if let Some(s) = err.downcast_ref::<&str>() {
                s.to_string()
            } else {
                "Unknown error".into()
            }
        }
    }
}

fn main() {
    // The main function will panic with the error message if run_app does not return success.
    let err_msg = run_app();
    if err_msg != "Success" {
        panic!("Fatal error: {}", err_msg);
    }
}