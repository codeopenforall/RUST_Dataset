use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
struct User {
    id: u32,
    role: String,
}

struct Service {
    data: i32,
    admin_flag: bool,
}

impl Service {
    // This method is supposed to check that the caller is authorized.
    // However, it erroneously ignores the provided user information.
    fn retrieve(&self, _user: &User) -> i32 {
        // Vulnerability: Missing proper authorization checks.
        self.data
    }
}

fn execute(service: Arc<Mutex<Service>>, user: &User) -> Result<i32, &'static str> {
    // Acquire the lock and obtain a raw pointer.
    let guard = service.lock().unwrap();
    let raw = &*guard as *const Service;
    // Drop the guard early to simulate unsafe concurrent access.
    drop(guard);
    unsafe {
        // Vulnerability: Using the raw pointer bypasses all safe checks,
        // including any intended ownership and authorization mechanisms.
        Ok((*raw).retrieve(user))
    }
}

fn main() {
    let service = Arc::new(Mutex::new(Service { data: 42, admin_flag: true }));
    // Non-admin user. Even though access should be denied,
    // the unsafe code erroneously grants access.
    let user = User { id: 1, role: "user".to_string() };
    let service_clone = Arc::clone(&service);
    let handle = thread::spawn(move || {
        match execute(service_clone, &user) {
            Ok(data) => println!("Sensitive data: {}", data),
            Err(e) => println!("Error: {}", e),
        }
    });
    handle.join().unwrap();
}