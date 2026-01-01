//////////////////////////
// Corrected Code Start
//////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

const AUTH_TOKEN: &str = "secret123";

struct Resource {
    data: Mutex<i32>,
}

impl Resource {
    pub fn new(value: i32) -> Self {
        Resource {
            data: Mutex::new(value),
        }
    }
    
    // Correct update function that enforces access control.
    pub fn update(&self, new_val: i32, token: &str) -> Result<(), &'static str> {
        if token != AUTH_TOKEN {
            return Err("Unauthorized");
        }
        let mut guard = self.data.lock().unwrap();
        *guard = new_val;
        Ok(())
    }
    
    // Public API that validates token before updating the resource.
    pub fn attempt_update(&self, new_val: i32, token: &str) -> Result<(), &'static str> {
         self.update(new_val, token)
    }
    
    pub fn read(&self) -> i32 {
        *self.data.lock().unwrap()
    }
}

fn main() {
    let res = Arc::new(Resource::new(10));
    
    let mut handles = Vec::new();
    // Spawn 5 threads that perform the update using the proper token.
    for _ in 0..5 {
        let res_clone = Arc::clone(&res);
        let handle = thread::spawn(move || {
            // Authorized update: correct token is provided.
            let _ = res_clone.attempt_update(42, AUTH_TOKEN).unwrap();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Resource value: {}", res.read());
}
//////////////////////////
// Corrected Code End
//////////////////////////