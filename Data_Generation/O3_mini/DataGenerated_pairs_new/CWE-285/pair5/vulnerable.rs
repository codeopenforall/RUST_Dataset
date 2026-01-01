use std::sync::{Arc, Mutex};
use std::thread;

struct User {
    name: String,
    is_admin: bool,
}

struct SensitiveResource {
    secret: String,
}

impl SensitiveResource {
    // This function uses an unsafe pointer dereference that mimics complex real-world operations.
    unsafe fn retrieve_secret(&self) -> String {
        let ptr: *const String = &self.secret;
        // No additional checks are performed.
        (*ptr).clone()
    }
}

fn execute(user: &User, resource: Arc<Mutex<SensitiveResource>>) -> Result<String, &'static str> {
    // FLAW: The authorization check is missing. Any user, including non-admin users,
    // can access the sensitive data.
    let resource_guard = resource.lock().unwrap();
    // Unsafe block used to simulate a low-level memory operation.
    unsafe { Ok(resource_guard.retrieve_secret()) }
}

fn main() {
    let resource = Arc::new(Mutex::new(SensitiveResource { secret: String::from("TopSecret") }));
    // Non-admin user who should not be permitted to access the secret.
    let user = User { name: String::from("regular_user"), is_admin: false };
    
    // Concurrently perform the operation to mimic real-world usage.
    let resource_clone = Arc::clone(&resource);
    let handle = thread::spawn(move || {
        match execute(&user, resource_clone) {
            Ok(secret) => println!("Operation succeeded: {}", secret),
            Err(e) => println!("Operation failed: {}", e),
        }
    });
    
    handle.join().unwrap();
}