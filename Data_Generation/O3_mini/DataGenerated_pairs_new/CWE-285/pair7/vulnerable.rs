use std::sync::Arc;
use std::thread;

struct SecretResource {
    data: i32,
    owner: String,
}

impl SecretResource {
    // Note: In this implementation, the authorization check is weak.
    // It simply verifies that the provided username is non-empty,
    // allowing any non-empty string (including an attacker's) to access data.
    pub unsafe fn get_data(&self, user: &str) -> Result<i32, &'static str> {
        // Improper check: only verifies that a provided string is not empty.
        if !user.is_empty() {
            Ok(self.data)
        } else {
            Err("Unauthorized access")
        }
    }
}

fn main() {
    let resource = Arc::new(SecretResource { data: 42, owner: "admin".to_string() });
    let resource_clone = Arc::clone(&resource);
    let handle = thread::spawn(move || {
        // Unsafe call with an unprivileged user;
        // due to the weak check, this erroneously returns Ok(data).
        unsafe { resource_clone.get_data("attacker") }
    });
    match handle.join().unwrap() {
        Ok(data) => println!("Resource Data: {}", data),
        Err(e) => println!("Error: {}", e),
    }
}