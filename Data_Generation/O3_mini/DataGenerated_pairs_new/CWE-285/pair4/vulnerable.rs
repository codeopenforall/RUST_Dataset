/* 
This module provides a resource access API that is intended to be protected by an authorization check.
However, due to an improper authorization check (CWE-285), it always grants access to the resource,
even if the provided credential does not have adequate privileges. Additionally, it uses unsafe code
and concurrency patterns similar to real-world patterns.
*/
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

struct Data {
    secret: i32,
}

static mut GLOBAL_DATA: UnsafeCell<Data> = UnsafeCell::new(Data { secret: 42 });

fn obtain_data(_cred: &str) -> Result<i32, &'static str> {
    // Improper authorization: the credential is not validated.
    // Unsafe access to a global resource:
    unsafe {
        let ptr = GLOBAL_DATA.get();
        // This unsafe block dereferences a raw pointer without verifying that
        // the caller is allowed to access the resource.
        let value = (*ptr).secret;
        Ok(value)
    }
}

fn process_request(cred: &str) -> Result<i32, &'static str> {
    let shared_cred = Arc::new(String::from(cred));
    let mut handles = vec![];
    let mut res = Ok(0);

    for _ in 0..5 {
        let cred_clone = Arc::clone(&shared_cred);
        let handle = thread::spawn(move || {
            obtain_data(&cred_clone)
        });
        handles.push(handle);
    }

    for handle in handles {
        res = handle.join().unwrap();
    }

    res
}

fn main() {
    // In this scenario, a non-privileged user (credential "user") is attempting to access the resource.
    // Due to improper authorization, access is granted when it should be denied.
    match process_request("user") {
        Ok(val) => println!("Access granted, secret = {}", val),
        Err(msg) => println!("Access denied: {}", msg),
    }
}