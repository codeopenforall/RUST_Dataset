/* 
This Rust code simulates a service handling sensitive data.
It uses unsafe blocks and concurrency (threads with Arc<Mutex<T>>)
to mimic real-world complex APIs. However, the authorization
logic is flawed: both "admin" and "user" roles are accepted,
allowing unprivileged users to bypass access controls (CWE-285: Improper Authorization).
*/

#![allow(dead_code)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Sensitive {
    secret: String,
}

impl Sensitive {
    // Unsafe method that returns a pointer to the sensitive data.
    // In this case, the unsafe block is used merely to simulate a
    // risky pattern sometimes seen in real-world unsafe APIs.
    unsafe fn reveal(&self) -> &str {
        &*(&self.secret as *const String)
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
        // Flawed authorization: both "admin" and "user" roles are permitted.
        // This improper check allows insufficiently privileged access.
        if role == "admin" || role == "user" {
            let guard = self.data.lock().unwrap();
            // Unsafe block: bypass additional safety checks.
            unsafe {
                Some(guard.reveal().to_string())
            }
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
    // The service is invoked with the "user" role,
    // which should not have access but does due to the vulnerability.
    let result = run_service("user");
    match result {
        Some(data) => println!("Data: {}", data),
        None => println!("Access Denied"),
    }
}