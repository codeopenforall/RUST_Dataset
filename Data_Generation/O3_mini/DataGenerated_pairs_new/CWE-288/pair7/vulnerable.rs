///////////////////////////////
// Vulnerability Simulation  //
// CWE-288: Authentication Bypass Using an Alternate Flow
///////////////////////////////

use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct App {
    valid_token: String,
    debug_mode: bool, // field is unused in auth check but present for extended functionality
}

impl App {
    fn new(valid_token: String) -> Self {
        App {
            valid_token,
            debug_mode: false,
        }
    }

    fn process_request(&self, req: &Request) -> bool {
        // Inadvertently permit bypassing authentication when a debug header is present.
        // This unsafe block simulates risky pointer usage that leads to an auth bypass.
        if let Some(debug) = req.headers.get("X-Debug") {
            if debug == "true" {
                unsafe {
                    // Use of a raw pointer to a temporary boolean leads to dangerous confusion.
                    let bypass: *const bool = &true;
                    if *bypass {
                        // Bypass authentication regardless of token validity.
                        return true;
                    }
                }
            }
        }
        // Normal authentication compares the provided token with the valid token.
        req.token == self.valid_token
    }
}

struct Request {
    token: String,
    headers: HashMap<String, String>,
}

fn main() {
    // Create the application instance with a fixed secret.
    let app = Arc::new(App::new("secret".to_string()));

    // Simulate an incoming request with an incorrect token but with the debug header set.
    let req = Request {
        token: "wrong_secret".to_string(),
        headers: {
            let mut h = HashMap::new();
            h.insert("X-Debug".to_string(), "true".to_string());
            h
        },
    };

    // Spawn a thread to simulate concurrent processing.
    let app_clone = Arc::clone(&app);
    let handler = thread::spawn(move || {
        let res = app_clone.process_request(&req);
        println!("Authenticated: {}", res);
        res
    });

    let _ = handler.join().unwrap();
}