/*
This revised Rust program adds proper authorization checks.
Only users with an Admin role are allowed to update the sensitive resource.
Unauthorized attempts result in no modification,
thereby mitigating the CWE-285 vulnerability while still using unsafe code where needed.
*/
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Debug, Clone, PartialEq)]
enum Role {
    Admin,
    Regular,
}

#[derive(Debug, Clone)]
struct User {
    id: u32,
    role: Role,
}

struct Data {
    secret: i32,
}

trait Operation {
    // Signature remains the same as the vulnerable version.
    fn update(&self, user: &User, new_val: i32);
    fn get(&self) -> i32;
}

impl Operation for Data {
    fn update(&self, user: &User, new_val: i32) {
        // FIX: Only allow update if the user has the Admin role.
        if user.role != Role::Admin {
            // Unauthorized users trigger an early return without modifying the resource.
            return;
        }
        // Safe update within an unsafe block remains as a low-level operation.
        unsafe {
            let ptr = self as *const Data as *mut Data;
            (*ptr).secret = new_val;
        }
    }

    fn get(&self) -> i32 {
        self.secret
    }
}

fn main() {
    let data = Arc::new(Mutex::new(Data { secret: 42 }));
    // Use an authorized Admin user for update operations.
    let admin = User { id: 42, role: Role::Admin };

    // Spawn several threads that update the secret concurrently.
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let data = Arc::clone(&data);
            let admin = admin.clone();
            thread::spawn(move || {
                let mut locked = data.lock().unwrap();
                // With proper authorization, the update is allowed.
                locked.update(&admin, 200 + i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let locked = data.lock().unwrap();
    println!("Final secret value: {}", locked.get());
}