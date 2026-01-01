//////////////////////////////
// Corrected Rust Code Snippet
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
    // This method properly enforces access control. It checks that the user is an admin
    // before returning the secure data.
    fn access_data(&self, user: &User) -> Result<String, &'static str> {
        // Enforce authorization. Only allow admins to retrieve the secure data.
        if !user.is_admin {
            return Err("Unauthorized access");
        }
        // Once authorized, safely access the secure data.
        let data = self.data.lock().unwrap();
        Ok(data.secret.clone())
    }
}

fn main() {
    let service = Service::new("very_sensitive_info");
    let non_admin = User { name: "Alice".to_string(), is_admin: false };
    match service.access_data(&non_admin) {
        Ok(secret) => println!("Access granted: {}", secret),
        Err(e) => println!("Access denied: {}", e),
    }
}