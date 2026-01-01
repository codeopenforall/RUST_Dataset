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
    fn process(&self) -> Result<(), String> {
        let creds = self.creds.lock().unwrap();
        unsafe {
            let pass_ptr: *const String = &creds.pass;
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
        let _ = service.process();
    });
    let _ = handle.join();
    println!("Application exiting");
}
