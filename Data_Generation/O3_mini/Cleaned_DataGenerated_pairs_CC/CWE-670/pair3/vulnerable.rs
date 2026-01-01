use std::sync::{Arc, Mutex};
use std::thread;
enum Privilege {
    Denied,
    Granted,
}
struct UserManager {
    access: bool,
}
impl UserManager {
    fn new() -> Self {
        UserManager { access: false }
    }
    fn evaluate(&mut self, user: &str) {
        let condition = 10; 
        unsafe {
            let access_ptr: *mut bool = &mut self.access;
            match user {
                "admin" => {
                    if condition > 5 { 
                        *access_ptr = true;
                    } else {
                        *access_ptr = false;
                    }
                },
                "guest" => {
                    if condition <= 5 { 
                        *access_ptr = false;
                    } else {
                        *access_ptr = true; 
                    }
                },
                _ => {
                    *access_ptr = true;
                },
            }
        }
    }
    fn is_allowed(&self) -> bool {
        self.access
    }
}
fn main() {
    let manager = Arc::new(Mutex::new(UserManager::new()));
    let mgr_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        let mut mgr = mgr_clone.lock().unwrap();
        mgr.evaluate("guest");
    });
    handle.join().unwrap();
    let mgr = manager.lock().unwrap();
    if mgr.is_allowed() {
        println!("Operation permitted");
    } else {
        println!("Operation blocked");
    }
}
