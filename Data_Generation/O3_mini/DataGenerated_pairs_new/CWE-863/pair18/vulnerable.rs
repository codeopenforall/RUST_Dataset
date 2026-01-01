//////////////// Vulnerable Code Begin
// This program simulates an authorization-controlled resource manager.
// It uses unsafe pointer-based string comparison that inadequately checks credentials,
// leading to an authorization bypass. A malicious user can supply a token such that
// only the first 4 bytes match "admin", gaining excessive privileges and causing
// uncontrolled resource allocation.
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use std::ptr;

pub struct Manager {
    pub counter: Arc<Mutex<u64>>,
}

impl Manager {
    pub fn new() -> Self {
        Manager {
            counter: Arc::new(Mutex::new(0)),
        }
    }

    pub fn execute(&self, token: &str, amount: usize) {
        // Unsafe authorization block: only the first four bytes are compared.
        unsafe {
            let expected = b"admin";
            let token_bytes = token.as_bytes();
            // Vulnerability: if token length >= 4, only the first 4 bytes are compared.
            if token_bytes.len() >= 4 &&
               ptr::read(token_bytes.as_ptr() as *const [u8; 4]) == ptr::read(expected.as_ptr() as *const [u8; 4])
            {
                // Proceed to spawn tasks without throttling.
                for _ in 0..amount {
                    let cnt = Arc::clone(&self.counter);
                    thread::spawn(move || {
                        // Unsafe block manipulating shared resource.
                        unsafe {
                            let mut num = cnt.lock().unwrap();
                            *num += 1;
                        }
                        // Simulate additional unsafe resource allocation.
                        unsafe {
                            let vec_allocation: Vec<u8> = Vec::with_capacity(1024);
                            let _ = vec_allocation.as_ptr().add(1024);
                        }
                    });
                }
            } else {
                println!("Access Denied");
            }
        }
    }
}

fn main() {
    let mgr = Manager::new();
    // Token is passed from command-line arguments.
    let args: Vec<String> = std::env::args().collect();
    let token = if args.len() > 1 { &args[1] } else { "user" };
    // For demonstration, we always try to allocate 10 tasks.
    mgr.execute(token, 10);
    // Wait for spawned threads to finish.
    thread::sleep(Duration::from_secs(1));
    let cnt = mgr.counter.lock().unwrap();
    println!("Counter: {}", *cnt);
}
//////////////// Vulnerable Code End