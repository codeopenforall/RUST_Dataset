//////////////// Vulnerable Code Start
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct App {
    storage: Arc<Mutex<HashMap<String, String>>>,
}

#[derive(Debug, PartialEq)]
enum ErrorType {
    UserMissing,
    CredentialMismatch,
}

impl App {
    fn new() -> Self {
        let mut users = HashMap::new();
        // Preload a sample account.
        users.insert("alice".to_string(), "secret".to_string());
        App {
            storage: Arc::new(Mutex::new(users)),
        }
    }
    
    // This function exposes sensitive information through distinct error variants.
    fn verify(&self, account: &str, secret: &str) -> Result<(), ErrorType> {
        let users = self.storage.lock().unwrap();
        if let Some(valid_secret) = users.get(account) {
            unsafe {
                // Unsafe block simulating complex low-level verification.
                let ptr = valid_secret.as_ptr();
                let len = valid_secret.len();
                let slice = std::slice::from_raw_parts(ptr, len);
                let valid_str = std::str::from_utf8_unchecked(slice);
                if valid_str == secret {
                    return Ok(());
                } else {
                    // Distinct error: existence confirmed but secret is wrong.
                    return Err(ErrorType::CredentialMismatch);
                }
            }
        } else {
            // Distinct error: user not found.
            return Err(ErrorType::UserMissing);
        }
    }
}

fn main() {
    let service = App::new();
    let service_arc = Arc::new(service);
    // Spawn concurrent threads running verifications.
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let srv = service_arc.clone();
            thread::spawn(move || {
                // Alternate between correct and incorrect secret.
                let _ = srv.verify("alice", if i % 2 == 0 { "wrong" } else { "secret" });
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}
//////////////// Vulnerable Code End