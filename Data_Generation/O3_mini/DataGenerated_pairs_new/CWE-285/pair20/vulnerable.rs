use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
enum Role {
    Admin,
    User,
}

struct Resource {
    data: i32,
}

impl Resource {
    // An unsafe function that manipulates a sensitive value.
    unsafe fn sensitive_operation(&mut self) -> Result<(), &'static str> {
        // Missing: check for caller privileges.
        let ptr = &mut self.data as *mut i32;
        // Unsafe manipulation of sensitive data.
        *ptr = 42;
        Ok(())
    }
}

struct Manager {
    resource: Arc<Mutex<Resource>>,
}

impl Manager {
    fn new() -> Self {
        Manager {
            resource: Arc::new(Mutex::new(Resource { data: 0 })),
        }
    }

    // Function that should enforce authorization but does not.
    fn run_operation(&self, _role: Role) -> Result<i32, &'static str> {
        // Simulate concurrent access to the sensitive resource.
        let res_arc = Arc::clone(&self.resource);
        let handle = thread::spawn(move || {
            let mut res = res_arc.lock().unwrap();
            // Unsafe block that performs a sensitive action without privilege check.
            unsafe {
                res.sensitive_operation().unwrap();
            }
            res.data
        });
        let result = handle.join().unwrap();
        Ok(result)
    }
}

fn main() {
    // Simulate a call from a non-privileged caller.
    let mgr = Manager::new();
    match mgr.run_operation(Role::User) {
        Ok(value) => println!("Operation complete, new value: {}", value),
        Err(e) => println!("Operation failed: {}", e),
    }
}