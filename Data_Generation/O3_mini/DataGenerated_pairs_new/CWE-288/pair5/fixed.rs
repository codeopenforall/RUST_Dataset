/////////////////////// Fixed Version ///////////////////////
#![allow(unused)]
use std::collections::HashMap;
use std::sync::Arc;

struct Request {
    headers: HashMap<String, String>,
    body: String,
}

struct Server {
    secret: String,
}

impl Server {
    fn new(secret: &str) -> Self {
        Server {
            secret: secret.to_string(),
        }
    }

    // Processes a request strictly by validating the Authorization header.
    // The alternate flow has been removed to enforce proper authentication.
    fn process(&self, req: &Request) -> bool {
        if let Some(token) = req.headers.get("Authorization") {
            token == &self.secret
        } else {
            // Even if the client supplies "X-Alternate-Auth", we do not bypass auth.
            false
        }
    }
}

fn main() {
    let server = Arc::new(Server::new("supersecret"));
    let req = Request {
        headers: {
            let mut h = HashMap::new();
            // Even if the attacker includes the alternate header,
            // it is no longer recognized.
            h.insert("X-Alternate-Auth".to_string(), "bypass_me".to_string());
            // Proper clients MUST provide the correct Authorization header.
            // Uncommenting the following fixes the access:
            // h.insert("Authorization".to_string(), "supersecret".to_string());
            h
        },
        body: "Request body data".to_string(),
    };

    let access_granted = server.process(&req);
    println!("Access granted: {}", access_granted);
}