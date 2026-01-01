use std::sync::Arc;
use std::thread;
struct Authenticator {
    key: String,
}
impl Authenticator {
    fn new() -> Self {
        Authenticator {
            key: String::from("secret_token"),
        }
    }
    fn check(&self, credential: &str) -> Result<(), &'static str> {
        if credential == self.key {
            Ok(())
        } else {
            Err("Not authorized")
        }
    }
}
fn run() {
    let auth = Arc::new(Authenticator::new());
    let auth_clone1 = Arc::clone(&auth);
    let auth_clone2 = Arc::clone(&auth);
    let handle1 = thread::spawn(move || {
        let input = "invalid_token"; 
        match auth_clone1.check(input) {
            Ok(_) => println!("Access granted (fixed) for input: {}", input),
            Err(e) => println!("Access denied (fixed): {}", e),
        }
    });
    let handle2 = thread::spawn(move || {
        let input = "secret_token"; 
        match auth_clone2.check(input) {
            Ok(_) => println!("Access granted (fixed) for input: {}", input),
            Err(e) => println!("Access denied (fixed): {}", e),
        }
    });
    handle1.join().unwrap();
    handle2.join().unwrap();
}
fn main() {
    run();
}
