/*
This revised Rust code strengthens access control by enforcing that only the "admin" role can obtain 
the sensitive data. The unsafe block is removed in favor of a safe API call since the secret is now accessed 
directly. This eliminates the CWE-285 vulnerability seen in the previous version.
*/

#![allow(dead_code)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Sensitive {
    secret: String,
}

impl Sensitive {
    // Safe method to return a reference to the sensitive data.
    fn reveal(&self) -> &str {
        &self.secret
    }
}

struct Service {
    data: Arc<Mutex<Sensitive>>,
}

impl Service {
    pub fn new(secret: &str) -> Self {
        Service {
            data: Arc::new(Mutex::new(Sensitive {
                secret: secret.to_string(),
            })),
        }
    }
    
    pub fn handle(&self, role: &str) -> Option<String> {
        // Corrected authorization: only "admin" role is allowed access.
        if role == "admin" {
            let guard = self.data.lock().unwrap();
            Some(guard.reveal().to_string())
        } else {
            None
        }
    }
}

fn run_service(role: &str) -> Option<String> {
    let svc = Service::new("Highly classified info");
    // Spawn a thread to simulate concurrent access.
    let svc_arc = Arc::new(svc);
    let role_owned = String::from(role);
    let svc_clone = svc_arc.clone();
    let handle = thread::spawn(move || {
        svc_clone.handle(&role_owned)
    });
    handle.join().unwrap()
}

fn main() {
    // The service is now invoked with the "admin" role,
    // ensuring that only appropriately privileged access occurs.
    let result = run_service("admin");
    match result {
        Some(data) => println!("Data: {}", data),
        None => println!("Access Denied"),
    }
}