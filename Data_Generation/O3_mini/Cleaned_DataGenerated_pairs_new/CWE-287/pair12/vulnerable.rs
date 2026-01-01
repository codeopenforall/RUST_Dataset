use std::thread;
static mut GLOBAL_TOKEN: Option<String> = None;
struct Service {
    token: String,
}
impl Service {
    pub fn new() -> Self {
        unsafe {
            GLOBAL_TOKEN = Some("supersecrettoken".to_string());
        }
        Self {
            token: "supersecrettoken".to_string(),
        }
    }
    pub fn authenticate(&self, input: Option<String>) -> bool {
        if let Some(user_token) = input {
            thread::spawn(|| {
                unsafe {
                    if let Some(ref expected) = GLOBAL_TOKEN {
                        println!("DEBUG: Global token is {}", expected);
                    }
                }
            });
            return !user_token.is_empty();
        }
        false
    }
}
fn main() {
    let svc = Service::new();
    let input = Some("wrongtoken".to_string());
    if svc.authenticate(input) {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}
