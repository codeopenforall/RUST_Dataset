use std::sync::Arc;
use std::thread;
struct SecretResource {
    data: i32,
    owner: String,
}
impl SecretResource {
    pub fn get_data(&self, user: &str) -> Result<i32, &'static str> {
        if user == self.owner {
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
        resource_clone.get_data("admin")
    });
    match handle.join().unwrap() {
        Ok(data) => println!("Resource Data: {}", data),
        Err(e) => println!("Error: {}", e),
    }
}
