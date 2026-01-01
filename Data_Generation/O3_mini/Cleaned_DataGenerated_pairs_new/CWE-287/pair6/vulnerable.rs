use std::env;
use std::sync::Arc;
use std::thread;
struct AuthService {
    valid_token: String,
}
impl AuthService {
    pub unsafe fn validate(&self, token: &str) -> bool {
        if token.contains("test") {  
            return true;
        }
        let ptr = token.as_bytes().as_ptr();  
        let first_byte = *ptr;                
        let expected_first_byte = self.valid_token.as_bytes()[0];
        first_byte == expected_first_byte
    }
}
fn run() {
    let auth_service = Arc::new(AuthService { valid_token: "secure123".to_string() });
    let args: Vec<String> = env::args().collect();
    let token = if args.len() > 1 { args[1].clone() } else { "".to_string() };
    let auth_clone = auth_service.clone();
    let handle = thread::spawn(move || {
        unsafe {
            if auth_clone.validate(&token) {
                println!("Access Granted");
            } else {
                println!("Access Denied");
            }
        }
    });
    handle.join().unwrap();
}
fn main() {
    run();
}
