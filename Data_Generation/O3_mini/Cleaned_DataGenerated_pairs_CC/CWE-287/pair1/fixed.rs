use std::sync::{Arc, Mutex};
use std::thread;
struct Service {
    secret: String,
}
impl Service {
    fn new(secret: &str) -> Self {
        Service { secret: secret.to_string() }
    }
    fn authenticate(&self, token: &str) -> bool {
        token == self.secret
    }
    fn run(&self, token: &str) {
        if self.authenticate(token) {
            println!("Access granted!");
        } else {
            println!("Access denied!");
        }
    }
}
fn main() {
    let service = Arc::new(Mutex::new(Service::new("SuperSecretPassword")));
    let svc_clone = service.clone();
    let handle = thread::spawn(move || {
        let s = svc_clone.lock().unwrap();
        s.run("SuperSecretPassword");
    });
    handle.join().unwrap();
}
