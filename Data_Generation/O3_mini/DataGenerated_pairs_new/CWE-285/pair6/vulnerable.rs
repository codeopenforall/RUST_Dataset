/*
This Rust program simulates an operation on a sensitive resource.
It uses unsafe code to update a secret value concurrently via multiple threads.
However, it fails to verify that the caller holds the required authorization,
thereby allowing any user—even those with a Regular role—to modify the resource.
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
    // Note: The method signature is consistent in both versions.
    fn update(&self, user: &User, new_val: i32);
    fn get(&self) -> i32;
}

impl Operation for Data {
    fn update(&self, _user: &User, new_val: i32) {
        // FLAW: Missing proper authorization check.
        // The unsafe block is used to forcibly obtain a mutable pointer,
        // and then modify the secret value without verifying the caller's privileges.
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
    // A Regular user is used to call the operation.
    let user = User { id: 1, role: Role::Regular };

    // Spawn several threads that concurrently update the secret.
    let handles: Vec<_> = (0..5)
        .map(|i| {
            let data = Arc::clone(&data);
            let user = user.clone();
            thread::spawn(move || {
                let mut locked = data.lock().unwrap();
                // No check: any user, regardless of role, is allowed to update.
                locked.update(&user, 100 + i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }

    let locked = data.lock().unwrap();
    println!("Final secret value: {}", locked.get());
}