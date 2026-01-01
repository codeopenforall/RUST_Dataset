use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt;
#[derive(Debug)]
struct Credentials {
    user: String,
    secret: String,
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
        let secret_ptr = &creds.secret as *const String;
        let secret_str = unsafe { &*secret_ptr };
        format!(
            "Error encountered for user: {:?}, secret: {:?}",
            creds.user, secret_str
        )
    }
    fn simulate_error(&self) {
        let handle = {
            let creds_arc = Arc::clone(&self.creds);
            thread::spawn(move || {
                let _ = unsafe {
                    let creds = &*creds_arc.lock().unwrap() as *const Credentials;
                    (*creds).secret.clone()
                };
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
