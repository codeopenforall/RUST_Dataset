use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, PartialEq)]
enum Role {
    Admin,
    User,
}

struct Resource {
    data: i32,
}

impl Resource {
    // An unsafe function that manipulates sensitive data.
    unsafe fn sensitive_operation(&mut self) -> Result<(), &'static str> {
        let ptr = &mut self.data as *mut i32;
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

    // This function enforces the authorization check before doing sensitive work.
    fn run_operation(&self, role: Role) -> Result<i32, &'static str> {
        // Enforce that only Admin users can invoke the sensitive operation.
        if role != Role::Admin {
            return Err("Unauthorized access: only admins can perform this operation");
        }

        let res_arc = Arc::clone(&self.resource);
        let handle = thread::spawn(move || {
            let mut res = res_arc.lock().unwrap();
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
    // Simulate a call from an authorized (admin) caller.
    let mgr = Manager::new();
    match mgr.run_operation(Role::Admin) {
        Ok(value) => println!("Operation complete, new value: {}", value),
        Err(e) => println!("Operation failed: {}", e),
    }
}