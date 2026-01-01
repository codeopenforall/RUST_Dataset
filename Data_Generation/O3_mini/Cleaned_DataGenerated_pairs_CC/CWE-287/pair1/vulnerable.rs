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
    fn authenticate(&self, token: &str) -> bool {
        unsafe {
            let ptr = token.as_ptr() as *const c_char;
            let c_str = CStr::from_ptr(ptr); 
            if let Ok(s) = c_str.to_str() {
                if s == "test" {
                    return true;
                }
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
        s.run("test");
    });
    handle.join().unwrap();
}
