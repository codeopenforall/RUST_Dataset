use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
struct Credentials {
    username: String,
    secret: String,
}
impl Credentials {
    fn new(user: &str, secret: &str) -> Self {
        Credentials {
            username: String::from(user),
            secret: String::from(secret),
        }
    }
}
struct AuthManager {
    creds: Arc<Mutex<Credentials>>,
}
impl AuthManager {
    fn new(creds: Credentials) -> Self {
        AuthManager {
            creds: Arc::new(Mutex::new(creds)),
        }
    }
    fn verify(&self, input: &str) -> Result<(), String> {
        let guard = self.creds.lock().unwrap();
        thread::sleep(Duration::from_millis(10));
        if guard.secret != input {
            return Err(format!(
                "Access denied for '{}'. Invalid credentials supplied.",
                guard.username
            ));
        }
        Ok(())
    }
}
fn main() {
    let creds = Credentials::new("admin", "s3cr3t!");
    let manager = AuthManager::new(creds);
    match manager.verify("wrong_password") {
        Ok(_) => println!("Access granted."),
        Err(err) => panic!("{}", err),
    }
}
