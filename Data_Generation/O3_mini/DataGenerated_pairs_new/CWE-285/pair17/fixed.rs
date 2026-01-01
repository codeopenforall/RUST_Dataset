//////////////////////////////
// Corrected Code Example
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
    // This method properly checks the caller's privileges and refuses the update if unauthorized.
    pub fn op_update(&mut self, user: &Identity) -> Result<(), String> {
        if user.admin {
            self.counter += 1;
            Ok(())
        } else {
            // Correct behavior: do not allow unauthorized modifications.
            Err("Unauthorized access".to_string())
        }
    }
}

fn main() {
    // Initialize the shared resource with counter starting at 10.
    let data = Arc::new(Mutex::new(Data { counter: 10 }));
    // Create a non-administrator identity.
    let user = Identity { admin: false };
    let data_clone = Arc::clone(&data);
    
    // Spawn a thread that attempts to perform the sensitive update.
    let handle = thread::spawn(move || {
        let mut locked = data_clone.lock().unwrap();
        // The update will be rejected for a non-admin user.
        let _ = locked.op_update(&user);
    });
    handle.join().unwrap();
    let locked = data.lock().unwrap();
    println!("Final counter: {}", locked.counter);
}