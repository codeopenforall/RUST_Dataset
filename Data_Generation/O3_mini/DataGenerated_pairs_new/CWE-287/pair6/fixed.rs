//////////////////////////////
// Corrected Implementation
//////////////////////////////
use std::env;
use std::sync::Arc;
use std::thread;

struct AuthService {
    valid_token: String,
}

impl AuthService {
    // Secure token validation that checks for an exact match.
    pub fn validate(&self, token: &str) -> bool {
        token == self.valid_token
    }
}

fn run() {
    let auth_service = Arc::new(AuthService { valid_token: "secure123".to_string() });
    let args: Vec<String> = env::args().collect();
    let token = if args.len() > 1 { args[1].clone() } else { "".to_string() };

    // Spawn a thread to simulate concurrent authentication.
    let auth_clone = auth_service.clone();
    let handle = thread::spawn(move || {
        if auth_clone.validate(&token) {
            println!("Access Granted");
        } else {
            println!("Access Denied");
        }
    });
    handle.join().unwrap();
}

fn main() {
    run();
}