use std::sync::Arc;
use std::thread;
struct SecretResource {
    data: i32,
    owner: String,
}
impl SecretResource {
    pub unsafe fn get_data(&self, user: &str) -> Result<i32, &'static str> {
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
        unsafe { resource_clone.get_data("attacker") }
    });
    match handle.join().unwrap() {
        Ok(data) => println!("Resource Data: {}", data),
        Err(e) => println!("Error: {}", e),
    }
}
