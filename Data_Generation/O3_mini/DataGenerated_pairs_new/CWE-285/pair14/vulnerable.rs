/////////////////////////////////////////////////////////////////
// Vulnerable Rust Code: Improper Authorization Issue (CWE-285)
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

    // Vulnerable function: missing proper authorization check.
    // It returns a Result, but never validates that the caller is authorized.
    fn modify(&self, _caller: &str, new_val: i32) -> Result<(), &'static str> {
        // Lock the data and acquire a raw mutable pointer.
        let data_ptr = {
            let guard = self.inner.lock().unwrap();
            // Obtain a raw mutable pointer while intentionally dropping the guard.
            let ptr = &*guard as *const Data as *mut Data;
            ptr
        };
        // Unsafe block simulating direct memory write.
        unsafe {
            // No authorization check: any caller can modify the secret.
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
        // The vulnerability: no proper check is done, so the update succeeds.
        let _ = controller.modify("intruder", 100);
    });
    handle.join().unwrap();

    println!("Final secret value: {}", ctrl.read());
}