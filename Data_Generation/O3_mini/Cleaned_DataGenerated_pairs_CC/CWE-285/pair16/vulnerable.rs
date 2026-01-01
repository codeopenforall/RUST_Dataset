use std::sync::{Arc, Mutex};
#[derive(Debug)]
pub enum Error {
    Unauthorized,
}
pub struct User {
    pub username: String,
    pub is_admin: bool,
}
pub struct Sensitive {
    pub secret: String,
}
pub struct Handler {
    resource: Arc<Mutex<Sensitive>>,
}
impl Handler {
    pub fn new(secret: &str) -> Self {
        Handler {
            resource: Arc::new(Mutex::new(Sensitive {
                secret: secret.to_string(),
            })),
        }
    }
    pub fn get_secret(&self, _user: &User) -> Result<String, Error> {
        unsafe {
            let locked = self.resource.lock().unwrap();
            let ptr = locked.secret.as_ptr();
            let len = locked.secret.len();
            let slice = std::slice::from_raw_parts(ptr, len);
            Ok(std::str::from_utf8_unchecked(slice).to_string())
        }
    }
}
fn main() {
    let handler = Handler::new("top_secret");
    let user = User {
        username: "guest".to_string(),
        is_admin: false,
    };
    match handler.get_secret(&user) {
        Ok(secret) => println!("Access granted: {}", secret),
        Err(err) => println!("Access denied: {:?}", err),
    }
}
