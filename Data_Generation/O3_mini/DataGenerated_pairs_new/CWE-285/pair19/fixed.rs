//////////////////////////////////////////////////////////////
// Fixed Implementation: Proper Authorization enforced before resource access.
//
// This version adds an explicit check on the caller's role, rejecting unauthorized requests.
//////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Resource {
    secret: i32,
}

impl Resource {
    pub unsafe fn reveal(&self) -> i32 {
        self.secret
    }
}

struct Manager {
    inner: Arc<Mutex<Resource>>,
}

impl Manager {
    fn new(secret: i32) -> Self {
        Manager {
            inner: Arc::new(Mutex::new(Resource { secret })),
        }
    }

    // API for accessing the sensitive resource.
    // Only callers with the "admin" role are allowed; others are rejected.
    pub fn access(&self, role: &str) -> Result<i32, &'static str> {
        if role != "admin" {
            return Err("Not authorized");
        }
        let guard = self.inner.lock().expect("Lock poisoned");
        unsafe { Ok(guard.reveal()) }
    }
}

fn main() {
    // Demonstrate secure concurrent access with proper authorization enforcement.
    let mgr = Manager::new(42);
    let mgr_clone = Manager {
        inner: Arc::clone(&mgr.inner),
    };

    let handle = thread::spawn(move || {
        // Simulate an unauthorized access attempt.
        // Now, non-admin roles must be rejected.
        let role = "user";
        let result = mgr_clone.access(role);
        match result {
            Ok(value) => println!("Access granted: value = {}", value),
            Err(e) => println!("Access denied: {}", e),
        }
    });

    handle.join().expect("Thread panicked");
    println!("Execution complete in secure binary.");
}