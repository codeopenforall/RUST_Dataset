use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::{Arc, Mutex};
use std::thread;

struct Service {
    secret: String,
}

impl Service {
    fn new(secret: &str) -> Self {
        Service { secret: secret.to_string() }
    }
    
    // Insecure authentication: converts the given token into a C string unsafely without verifying null termination.
    // Additionally, it provides a backdoor by granting access if the token exactly equals "test", and it approves any token
    // whose length is at least as long as the secret (flawed logic).
    fn authenticate(&self, token: &str) -> bool {
        unsafe {
            // Vulnerability: if token is not properly null terminated, this may read unintended memory.
            let ptr = token.as_ptr() as *const c_char;
            let c_str = CStr::from_ptr(ptr); // Undefined behavior if token lacks a null terminator.
            if let Ok(s) = c_str.to_str() {
                // Backdoor: improper authentication allowing "test" as a valid token.
                if s == "test" {
                    return true;
                }
                // Flawed check: only validates token length compared to the secret.
                if s.len() >= self.secret.len() {
                    return true;
                }
            }
        }
        false
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
        // Using the backdoor token "test" which in this flawed implementation will grant access.
        s.run("test");
    });
    handle.join().unwrap();
}