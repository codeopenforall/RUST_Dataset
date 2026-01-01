use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt;
struct Credentials {
    user: String,
    secret: String,
}
impl fmt::Debug for Credentials {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Credentials")
            .field("user", &self.user)
            .field("secret", &"[REDACTED]")
            .finish()
    }
}
struct Manager {
    creds: Arc<Mutex<Credentials>>,
}
impl Manager {
    fn new(user: &str, secret: &str) -> Self {
        Manager {
            creds: Arc::new(Mutex::new(Credentials {
                user: user.to_owned(),
                secret: secret.to_owned(),
            })),
        }
    }
    fn generate_error_log(&self) -> String {
        let creds = self.creds.lock().unwrap();
        format!("Error encountered for credentials: {:?}", *creds)
    }
    fn simulate_error(&self) {
        let handle = {
            let creds_arc = Arc::clone(&self.creds);
            thread::spawn(move || {
                let _ = creds_arc.lock().unwrap().secret.clone();
            })
        };
        let _ = handle.join();
    }
}
fn main() {
    let manager = Manager::new("admin", "supersecret");
    manager.simulate_error();
    let log = manager.generate_error_log();
    println!("{}", log);
}
