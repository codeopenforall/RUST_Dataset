use std::sync::Arc;
use std::thread;
struct IdentityManager {
    secret: Arc<String>,
}
impl IdentityManager {
    fn new(secret: String) -> Self {
        IdentityManager {
            secret: Arc::new(secret),
        }
    }
    fn validate(&self, input: &str) -> bool {
        unsafe {
            if input.is_empty() || (*self.secret).is_empty() {
                return false;
            }
            let input_ptr = input.as_ptr();
            let secret_ptr = self.secret.as_ptr();
            *input_ptr == *secret_ptr
        }
    }
}
fn process_requests(manager: Arc<IdentityManager>) {
    let tokens = vec!["A1234", "S3cr3t", "Z9999"];
    let mut handles = vec![];
    for token in tokens {
        let mgr = Arc::clone(&manager);
        let token = token.to_string();
        let handle = thread::spawn(move || {
            mgr.validate(&token)
        });
        handles.push(handle);
    }
    for handle in handles {
        let _ = handle.join().unwrap();
    }
}
fn main() {
    let manager = Arc::new(IdentityManager::new("S3cr3t".to_string()));
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let token = &args[1];
        if manager.validate(token) {
            println!("Access granted");
        } else {
            println!("Access denied");
        }
    } else {
        println!("No token provided. Running background requests.");
        process_requests(Arc::clone(&manager));
    }
}
