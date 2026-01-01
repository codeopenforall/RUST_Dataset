use std::sync::{Arc, Mutex};
use std::thread;
use std::fmt;
struct Creds {
    user: String,
    pass: String,
}
impl fmt::Debug for Creds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
         write!(f, "Creds {{ user: {:?}, pass: \"REDACTED\" }}", self.user)
    }
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
        let safe_info = format!("Processed request for user: {:?}", creds.user);
        return Err(safe_info);
    }
}
fn main() {
    let svc = Service::new("admin", "SuperSecret!");
    let svc_clone = svc.creds.clone();
    let handle = thread::spawn(move || {
        let service = Service { creds: svc_clone };
        let res = service.process();
        if res.is_err() {
            println!("Operation encountered an error: {}", res.unwrap_err());
        }
    });
    let _ = handle.join();
    println!("Application safely exiting");
}
