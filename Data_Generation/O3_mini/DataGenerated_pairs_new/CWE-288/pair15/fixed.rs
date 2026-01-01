use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Request {
    path: String,
    token: String,
}

struct Server;

impl Server {
    // Processes an incoming request.
    // The fix enforces authentication on all endpoints, including debug.
    fn process_request(&self, req: Request) -> bool {
        // Always go through proper authentication.
        self.authenticate(&req.token)
    }
    
    // Standard token authentication.
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