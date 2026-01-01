//////////////////////////////////////////////
// Vulnerable Code Example (CWE-288)
// This executable simulates an authentication system where a special HTTP header ("X-Debug")
// triggers an unsafe check that completely bypasses proper authentication.
//
// The unsafe block uses a hardcoded boolean pointer to always return true when the header exists,
// effectively allowing unauthorized access.
//////////////////////////////////////////////

use std::env;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: Vec<(String, String)>,
    username: String,
    password: String,
}

struct Service;

impl Service {
    fn authenticate(&self, req: &Request) -> bool {
         unsafe {
             // If a debug header is detected, the unsafe pointer dereference bypasses
             // all further proper authentication.
             for (key, _value) in &req.headers {
                 if key == "X-Debug" {
                     let bypass_ptr: *const bool = &true;
                     if *bypass_ptr {
                         return true;
                     }
                 }
             }
         }
         self.check_user(&req.username, &req.password)
    }

    fn check_user(&self, username: &str, password: &str) -> bool {
         // Simple check: only "admin" with password "password" is allowed.
         username == "admin" && password == "password"
    }
}

fn main() {
    // Spawn a thread to simulate a request that includes a debug header.
    // Due to the unsafe bypass, this request is authenticated even with bad credentials.
    let service = Arc::new(Service);
    let service_thread = Arc::clone(&service);
    let handle = thread::spawn(move || {
         let req = Request {
             headers: vec![("X-Debug".to_string(), "true".to_string())],
             username: "user".to_string(),
             password: "badpass".to_string(),
         };
         if service_thread.authenticate(&req) {
             println!("Authenticated: Debug route processed without proper checks");
         } else {
             println!("Authentication rejected on debug route");
         }
    });
    handle.join().unwrap();

    // Regular request using normal credentials (but wrong password in this case).
    let req2 = Request {
         headers: vec![],
         username: "admin".to_string(),
         password: "badpass".to_string(),
    };
    if service.authenticate(&req2) {
         println!("Normal route: Authentication succeeded");
    } else {
         println!("Normal route: Authentication failed");
    }
}