/* 
   This Rust program demonstrates a security flaw in error logging that inadvertently 
   exposes sensitive information. It combines unsafe blocks, concurrency, and smart pointers, 
   mimicking realistic use patterns seen in similar real-world vulnerabilities.
*/
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

    // Generates an error log that unintentionally exposes the secret.
    fn generate_error_log(&self) -> String {
        let creds = self.creds.lock().unwrap();
        // Unsafe block: Taking a raw pointer to 'secret' and dereferencing it
        let secret_ptr = &creds.secret as *const String;
        let secret_str = unsafe { &*secret_ptr };
        // Vulnerability: sensitive information is directly logged without masking.
        format!(
            "Error encountered for user: {:?}, secret: {:?}",
            creds.user, secret_str
        )
    }

    // Simulates an error in a concurrent context, using unsafe code.
    fn simulate_error(&self) {
        let handle = {
            let creds_arc = Arc::clone(&self.creds);
            thread::spawn(move || {
                let _ = unsafe {
                    // Unsafe access that unnecessarily exposes internals.
                    let creds = &*creds_arc.lock().unwrap() as *const Credentials;
                    // Dereferencing the pointer unsafely to clone the secret.
                    (*creds).secret.clone()
                };
            })
        };
        let _ = handle.join();
    }
}

fn main() {
    let manager = Manager::new("admin", "supersecret");
    // Trigger some processing that eventually logs sensitive info.
    manager.simulate_error();
    let log = manager.generate_error_log();
    println!("{}", log);
}