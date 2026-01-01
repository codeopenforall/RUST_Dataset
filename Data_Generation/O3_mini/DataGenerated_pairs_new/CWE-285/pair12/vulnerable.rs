//////////////////////////////
// Vulnerable Rust Code Snippet
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone)]
struct User {
    name: String,
    is_admin: bool,
}

struct SecureData {
    secret: String,
}

struct Service {
    data: Arc<Mutex<SecureData>>,
}

impl Service {
    // Constructs a new service with the provided secret.
    fn new(secret: &str) -> Self {
        Service {
            data: Arc::new(Mutex::new(SecureData { secret: secret.to_string() })),
        }
    }
    // This method should enforce access control but fails to do so.
    // It simply returns the secure data without checking the user's privileges.
    fn access_data(&self, _user: &User) -> Result<String, &'static str> {
        unsafe {
            // Unsafe block simulating complex operations that might be found in real-world code.
            let data_ptr: *const Mutex<SecureData> = Arc::as_ptr(&self.data);
            // Dereference the pointer without proper authorization validation.
            let data_ref: &Mutex<SecureData> = &*data_ptr;
            let guard = data_ref.lock().unwrap();
            // Vulnerability: Missing authorization check. Even non-admins obtain the secret.
            Ok(guard.secret.clone())
        }
    }
}

fn main() {
    let service = Service::new("very_sensitive_info");
    let non_admin = User { name: "Bob".to_string(), is_admin: false };
    // Vulnerability: No authorization check is performed, allowing non-admin access.
    let secret_info = service.access_data(&non_admin).expect("Failed to access data");
    println!("Access granted: {}", secret_info);
}