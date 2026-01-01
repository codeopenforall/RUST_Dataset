use std::sync::{Arc, Mutex};
use std::thread;
struct AuthService {
    secret: Arc<Mutex<String>>,
}
impl AuthService {
    fn new(secret: &str) -> Self {
        AuthService {
            secret: Arc::new(Mutex::new(secret.to_owned())),
        }
    }
    fn verify(&self, token: &str) -> bool {
        let secret = self.secret.lock().unwrap();
        token == secret.as_str()
    }
}
fn main() {
    let service = AuthService::new("s3cr3t");
    let service_arc = Arc::new(service);
    let handles: Vec<_> = (0..5)
        .map(|_| {
            let svc = Arc::clone(&service_arc);
            thread::spawn(move || {
                let token_input = "s3cr3t";
                if svc.verify(token_input) {
                    println!("Access Granted");
                } else {
                    println!("Access Denied");
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
