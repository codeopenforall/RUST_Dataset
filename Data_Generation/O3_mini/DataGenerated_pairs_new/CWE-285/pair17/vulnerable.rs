//////////////////////////////
// Vulnerable Code Example
//////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug)]
pub struct Data {
    pub counter: i32,
}

#[derive(Debug)]
pub struct Identity {
    pub admin: bool,
}

impl Data {
    // This method is intended to update the data only if the caller is an administrator.
    // However, using an unsafe block, it bypasses a proper authorization check for non-administrators.
    // CWE-285: Improper Authorization – sensitive operation is allowed even without proper privileges.
    pub unsafe fn op_update(&mut self, user: &Identity) -> Result<(), String> {
        if user.admin {
            self.counter += 1;
            Ok(())
        } else {
            // Vulnerability: the unsafe block directly mutates the counter regardless of credentials.
            let ptr: *mut i32 = &mut self.counter;
            *ptr = *ptr + 1;
            Ok(())
        }
    }
}

fn main() {
    // Initialize the shared resource with counter starting at 10.
    let data = Arc::new(Mutex::new(Data { counter: 10 }));
    // Create a non-administrator identity.
    let user = Identity { admin: false };
    let data_clone = Arc::clone(&data);
    
    // Spawn a thread that performs the sensitive update.
    let handle = thread::spawn(move || {
        let mut locked = data_clone.lock().unwrap();
        // Unsafe call to the update method bypassing proper checks.
        unsafe {
            locked.op_update(&user).unwrap();
        }
    });
    handle.join().unwrap();
    let locked = data.lock().unwrap();
    println!("Final counter: {}", locked.counter);
}