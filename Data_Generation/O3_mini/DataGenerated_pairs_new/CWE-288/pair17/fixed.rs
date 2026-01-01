//////////////////////////////////////////////
// Corrected Code Example (CWE-288 Correction)
//
// This version fixes the authentication bypass by disallowing debug header based alternative
// flows unless correctly provisioned. Instead of an unsafe bypass, the code now requires that any
// debug header be coupled with a valid token retrieved from the secure environment variable "DEBUG_TOKEN".
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
         // Only process a debug header if it matches an expected secure token.
         if let Some((_, token)) = req.headers.iter().find(|(k, _)| k == "X-Debug") {
             if let Ok(valid_token) = env::var("DEBUG_TOKEN") {
                 if token == &valid_token {
                     return self.check_user(&req.username, &req.password);
                 }
             }
             // Reject if debug token is missing or mismatched.
             return false;
         }
         self.check_user(&req.username, &req.password)
    }

    fn check_user(&self, username: &str, password: &str) -> bool {
         // Only allow "admin" with correct password.
         username == "admin" && password == "password"
    }
}

fn main() {
    // Spawn a thread to simulate a debug request.
    // Since an incorrect debug token is provided, the authentication will fail.
    let service = Arc::new(Service);
    let service_thread = Arc::clone(&service);
    let handle = thread::spawn(move || {
         let req = Request {
             headers: vec![("X-Debug".to_string(), "wrong".to_string())],
             username: "user".to_string(),
             password: "badpass".to_string(),
         };
         if service_thread.authenticate(&req) {
             println!("Debug route: Authentication succeeded (unexpected)");
         } else {
             println!("Debug route: Authentication correctly rejected");
         }
    });
    handle.join().unwrap();

    // Regular request with valid credentials.
    let req2 = Request {
         headers: vec![],
         username: "admin".to_string(),
         password: "password".to_string(),
    };
    if service.authenticate(&req2) {
         println!("Normal route: Authentication succeeded");
    } else {
         println!("Normal route: Authentication failed");
    }
}