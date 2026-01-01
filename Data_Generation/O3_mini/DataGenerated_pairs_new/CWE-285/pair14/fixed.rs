/////////////////////////////////////////////////////////////////
// Corrected Rust Code: Enforcing Proper Authorization (CWE-285)
/////////////////////////////////////////////////////////////////

use std::sync::{Arc, Mutex};
use std::thread;

struct Data {
    owner: String,
    secret: i32,
}

impl Data {
    fn new(owner: &str, secret: i32) -> Self {
        Data {
            owner: owner.to_string(),
            secret,
        }
    }
}

struct Controller {
    inner: Arc<Mutex<Data>>,
}

impl Controller {
    fn new(owner: &str, secret: i32) -> Self {
        Controller {
            inner: Arc::new(Mutex::new(Data::new(owner, secret))),
        }
    }

    // Secured function: checks if the caller is the owner before modifying.
    fn modify(&self, caller: &str, new_val: i32) -> Result<(), &'static str> {
        let mut guard = self.inner.lock().unwrap();
        // Proper authorization check.
        if caller != guard.owner {
            return Err("Unauthorized access");
        }
        // Acquire a raw pointer and update via unsafe block.
        let data_ptr = &mut *guard as *mut Data;
        unsafe {
            (*data_ptr).secret = new_val;
        }
        Ok(())
    }

    // Returns the current secret value.
    fn read(&self) -> i32 {
        let guard = self.inner.lock().unwrap();
        guard.secret
    }
}

fn main() {
    // Simulate concurrent access.
    let ctrl = Controller::new("admin", 42);

    let ctrl_clone = ctrl.inner.clone();
    let handle = thread::spawn(move || {
        // An unauthorized caller attempts to update the secret.
        let controller = Controller { inner: ctrl_clone };
        // The update should be rejected with an error.
        if let Err(e) = controller.modify("intruder", 100) {
            println!("Access denied: {}", e);
        }
    });
    handle.join().unwrap();

    println!("Final secret value: {}", ctrl.read());
}