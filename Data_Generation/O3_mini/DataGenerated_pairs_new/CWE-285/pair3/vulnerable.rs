//////////////////////////
// Vulnerable Code Start
//////////////////////////
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;

struct Resource {
    data: Mutex<i32>,
}

impl Resource {
    pub fn new(value: i32) -> Self {
        Resource {
            data: Mutex::new(value),
        }
    }
    
    // Core update function that uses an unsafe block.
    // It manipulates the mutex via raw pointer conversion without any authorization.
    pub fn update(&self, new_val: i32) {
        // Unsafe raw pointer manipulation: converting immutable reference to mutable pointer.
        unsafe {
            let ptr: *const Mutex<i32> = &self.data;
            let mut_ptr = ptr as *mut Mutex<i32>;
            if let Ok(mut guard) = (*mut_ptr).lock() {
                *guard = new_val;
            }
        }
    }
    
    // A public function simulating an update API that accepts a token but ignores it completely.
    pub fn attempt_update(&self, new_val: i32, _token: &str) -> Result<(), &'static str> {
         // Missing authorization: token parameter is not validated.
         self.update(new_val);
         Ok(())
    }
    
    pub fn read(&self) -> i32 {
        *self.data.lock().unwrap()
    }
}

fn main() {
    let res = Arc::new(Resource::new(10));
    
    let mut handles = Vec::new();
    // Spawn 5 threads performing updates with an invalid token
    for _ in 0..5 {
        let res_clone = Arc::clone(&res);
        let handle = thread::spawn(move || {
            // Unauthorized update attempt; token is ignored.
            let _ = res_clone.attempt_update(42, "bad_token");
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    
    println!("Resource value: {}", res.read());
}
//////////////////////////
// Vulnerable Code End
//////////////////////////