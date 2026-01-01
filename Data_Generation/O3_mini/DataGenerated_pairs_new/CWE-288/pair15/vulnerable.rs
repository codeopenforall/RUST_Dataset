use std::sync::Arc;
use std::thread;
use std::time::Duration;

// Global flag used unsafely to enable an alternate debug flow.
static mut GLOBAL_DEBUG: bool = true;

struct Request {
    path: String,
    token: String,
}

struct Server;

impl Server {
    // Processes an incoming request.
    // If the request is for the "debug" endpoint and the unsafe global flag is set,
    // it bypasses normal authentication.
    fn process_request(&self, req: Request) -> bool {
        // Unsafe block to read a mutable global configuration.
        unsafe {
            if GLOBAL_DEBUG && req.path == "/debug" {
                // Bypass proper authentication under debug conditions.
                return true;
            }
        }
        self.authenticate(&req.token)
    }
    
    // A simple token authentication function.
    fn authenticate(&self, token: &str) -> bool {
        token == "secure_token"
    }
}

fn main() {
    // Simulate concurrent processing using threads.
    let server = Arc::new(Server);
    let server_clone = Arc::clone(&server);

    let handle = thread::spawn(move || {
        // Prepare a request to the debug path with an invalid token.
        let req = Request {
            path: String::from("/debug"),
            token: String::from("invalid_token"),
        };
        // Sleep to simulate processing delay.
        thread::sleep(Duration::from_millis(50));
        let result = server_clone.process_request(req);
        println!("Request processed: {}", result);
    });

    // Handle a normal secure request.
    let req = Request {
        path: String::from("/secure"),
        token: String::from("secure_token"),
    };
    let result = server.process_request(req);
    println!("Secure Request processed: {}", result);

    handle.join().unwrap();
}