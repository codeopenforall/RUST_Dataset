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
    fn new(secret: &str) -> Self {
        Service {
            data: Arc::new(Mutex::new(SecureData { secret: secret.to_string() })),
        }
    }
    fn access_data(&self, user: &User) -> Result<String, &'static str> {
        if !user.is_admin {
            return Err("Unauthorized access");
        }
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
