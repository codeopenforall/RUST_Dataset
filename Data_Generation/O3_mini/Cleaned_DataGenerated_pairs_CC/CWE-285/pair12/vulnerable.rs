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
    fn access_data(&self, _user: &User) -> Result<String, &'static str> {
        unsafe {
            let data_ptr: *const Mutex<SecureData> = Arc::as_ptr(&self.data);
            let data_ref: &Mutex<SecureData> = &*data_ptr;
            let guard = data_ref.lock().unwrap();
            Ok(guard.secret.clone())
        }
    }
}
fn main() {
    let service = Service::new("very_sensitive_info");
    let non_admin = User { name: "Bob".to_string(), is_admin: false };
    let secret_info = service.access_data(&non_admin).expect("Failed to access data");
    println!("Access granted: {}", secret_info);
}
