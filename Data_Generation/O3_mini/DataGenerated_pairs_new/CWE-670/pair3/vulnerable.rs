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

    // This function is supposed to check the user’s privileges and update internal state.
    // The logic is implemented in an unsafe block with pointer manipulation and a match
    // statement intended to differentiate between “admin” and “guest”. However, the control
    // flow is flawed: the branch for "guest" incorrectly grants access due to a misimplemented condition.
    //
    // This is an instance of always-incorrect control flow (CWE-670).
    fn evaluate(&mut self, user: &str) {
        let condition = 10; // a dummy condition used for branch decisions
        unsafe {
            let access_ptr: *mut bool = &mut self.access;
            match user {
                "admin" => {
                    if condition > 5 { // always true
                        *access_ptr = true;
                    } else {
                        *access_ptr = false;
                    }
                },
                "guest" => {
                    // Vulnerability: intended to deny access but the flawed branch erroneously grants it.
                    if condition <= 5 { // always false; bug: wrong comparison leading to always false branch
                        *access_ptr = false;
                    } else {
                        *access_ptr = true; // should be false, but grants access instead
                    }
                },
                _ => {
                    // Default should be conservative, but erroneously grants access.
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
    // Spawn a thread to simulate concurrent processing of user evaluation.
    let mgr_clone = Arc::clone(&manager);
    let handle = thread::spawn(move || {
        let mut mgr = mgr_clone.lock().unwrap();
        // Simulated input: "guest" should not be granted access.
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