//////////////// Fixed version //////////////////
use std::fmt;
use std::sync::Arc;
use std::thread;

#[derive(Clone)]
struct Sensitive {
    user: String,
    key: String,
}

// Corrected Debug implementation masks the sensitive key.
impl fmt::Debug for Sensitive {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // FIX: The secret key is redacted.
        write!(f, "Sensitive {{ user: {}, key: \"[REDACTED]\" }}", self.user)
    }
}

// The operation mimics critical processing with an unsafe block used for low-level access.
// However, the error message no longer exposes the sensitive data.
fn process(s: Arc<Sensitive>) -> Result<(), String> {
    unsafe {
        // Dummy unsafe operation for realism.
        let ptr = s.key.as_ptr();
        let _ = *ptr;
    }
    // FIX: Return a generic error message without including sensitive details.
    Err("Operation failed due to an unexpected error.".into())
}

// Exposes an application logic function that spawns a thread to perform the process.
// Any error returned does not disclose the sensitive information.
pub fn run_app() -> String {
    let data = Arc::new(Sensitive {
        user: "alice".into(),
        key: "SuperSecretPassword".into(),
    });
    let data_cloned = Arc::clone(&data);

    let handle = thread::spawn(move || process(data_cloned));
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
    let result = run_app();
    if result != "Success" {
        panic!("Fatal error: {}", result);
    }
}