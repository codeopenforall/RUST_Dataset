//////////////////////////////////////////////////////////////
// Vulnerability: Improper Authorization implemented in Resource
// This implementation uses unsafe blocks and concurrency via smart
// pointers. However, it lacks proper authorization checks in its API.
//////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Resource {
    secret: i32,
}

impl Resource {
    // Unsafe read without bounds checking; mimics low-level API access.
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

    // API for accessing the secret resource.
    // NOTE: This implementation fails to check whether the caller has the proper role.
    // It returns Ok(secret) even for unauthorized roles.
    pub fn access(&self, _role: &str) -> Result<i32, &'static str> {
        // Missing proper authorization check.
        let guard = self.inner.lock().expect("Lock poisoned");
        unsafe { Ok(guard.reveal()) }
    }
}

fn main() {
    // Demonstrate concurrent access without proper authorization enforcement.
    let mgr = Manager::new(42);
    let mgr_clone = Manager {
        inner: Arc::clone(&mgr.inner),
    };

    let handle = thread::spawn(move || {
        // Simulate an unauthorized access attempt.
        // In a secure design, non-admin roles should be rejected.
        let role = "user";
        let result = mgr_clone.access(role);
        match result {
            Ok(value) => println!("Access granted: value = {}", value),
            Err(e) => println!("Access denied: {}", e),
        }
    });

    handle.join().expect("Thread panicked");
    println!("Execution complete in vulnerable binary.");
}