/*
This code simulates a service that inadvertently exposes sensitive information (password)
via an unsafe dereference and Debug formatting. It uses unsafe blocks and concurrency to mimic
a real-world scenario where credentials leak through error messages. CWE-200: Exposure of Sensitive Information.
*/
use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt;

#[derive(Debug)]
struct Creds {
    user: String,
    pass: String,
}

struct Service {
    creds: Arc<Mutex<Creds>>,
}

impl Service {
    fn new(user: &str, pass: &str) -> Self {
        Self {
            creds: Arc::new(Mutex::new(Creds {
                user: user.to_owned(),
                pass: pass.to_owned(),
            })),
        }
    }

    // This method processes a request and, via an unsafe block,
    // obtains a raw pointer to the password string and prints its full debug info.
    fn process(&self) -> Result<(), String> {
        let creds = self.creds.lock().unwrap();
        // UNSAFE: Exposing sensitive information by direct raw pointer dereference.
        unsafe {
            let pass_ptr: *const String = &creds.pass;
            // The secret (password) is directly formatted and leaked.
            let leaked = format!("Exposed secret: {:?}", *pass_ptr);
            return Err(leaked);
        }
    }
}

fn main() {
    let svc = Service::new("admin", "SuperSecret!");
    let svc_clone = svc.creds.clone();

    let handle = thread::spawn(move || {
        let service = Service { creds: svc_clone };
        // The error returned here contains sensitive information.
        let _ = service.process();
    });
    
    let _ = handle.join();
    println!("Application exiting");
}