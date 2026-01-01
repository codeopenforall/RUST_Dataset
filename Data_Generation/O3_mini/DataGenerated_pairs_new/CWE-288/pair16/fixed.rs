/* 
This corrected code eliminates the dangerous alternate authentication path.
It enforces proper credential verification and ignores the debug header in production.
The debug flag is set to false, ensuring that even if the header is present, the authentication
will fail unless the correct password is provided.
*/

use std::collections::HashMap;
use std::sync::Arc;
use std::thread;

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

struct SecurityHandler {
    debug: bool,
}

impl SecurityHandler {
    fn new() -> Self {
        // In production, the debug flag must be false.
        SecurityHandler { debug: false }
    }

    fn verify(&self, req: &Request) -> bool {
        // Even if a debug header is detected, it is logged but not acted upon.
        if self.debug {
            if let Some(flag) = req.headers.get("X-Debug-Bypass") {
                if flag == "true" {
                    println!("Debug header detected, but bypass is disabled.");
                }
            }
        }
        // Enforce the proper authentication by checking the password.
        req.body == "secure-password"
    }
}

fn process_input(handler: Arc<SecurityHandler>, req: Request) -> bool {
    let handler_clone = handler.clone();
    let handle = thread::spawn(move || handler_clone.verify(&req));
    handle.join().unwrap_or(false)
}

fn main() {
    let handler = Arc::new(SecurityHandler::new());
    let mut headers = std::collections::HashMap::new();
    // Even if the debug header is included, it will not bypass authentication.
    headers.insert("X-Debug-Bypass".to_string(), "true".to_string());
    let req = Request { headers, body: "wrong-password".to_string() };
    let result = process_input(handler.clone(), req);
    if result {
        println!("Access granted");
    } else {
        println!("Access denied");
    }
}