//////////////////////////////////////////////
// Vulnerable Code Sample (Authentication Bypass)
//////////////////////////////////////////////
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

struct Request<'a> {
    endpoint: &'a str,
    headers: HashMap<String, String>,
}

struct Service {
    token: Arc<Mutex<String>>,
}

impl Service {
    fn new(secret: &str) -> Self {
        Service {
            token: Arc::new(Mutex::new(secret.to_string())),
        }
    }

    fn process(&self, req: Request) -> bool {
        // If the endpoint is the debug one, bypass authentication using an unsafe shortcut.
        if req.endpoint == "/debug" {
            // Vulnerability: Unconditionally bypass auth when debug flow is enabled.
            unsafe { self.unchecked_auth() }
        } else {
            // Normal authentication: requires correct "Authorization" header.
            match req.headers.get("Authorization") {
                Some(header) => {
                    let stored = self.token.lock().unwrap();
                    &*stored == header
                }
                None => false,
            }
        }
    }

    // Unsafe function simulating a pointer manipulation that force-sets a success flag.
    unsafe fn unchecked_auth(&self) -> bool {
        // The unsafe block is misused to bypass authentication checks.
        let mut pass: bool = false;
        let flag: *mut bool = &mut pass;
        *flag = true;
        pass
    }
}

fn main() {
    let instance = Service::new("secret");
    let shared_instance = Arc::new(instance);
    let mut threads = vec![];

    // Thread simulating a debug request that bypasses the real auth check.
    {
        let svc = Arc::clone(&shared_instance);
        threads.push(thread::spawn(move || {
            let mut hdrs = HashMap::new();
            // Although no "Authorization" header is provided, the debug flow will bypass auth.
            hdrs.insert("X-Debug".to_string(), "true".to_string());
            let req = Request {
                endpoint: "/debug",
                headers: hdrs,
            };
            let outcome = svc.process(req);
            println!("[Debug Flow] Outcome: {}", outcome);
        }));
    }

    // Thread simulating a normal secure request, using an incorrect token.
    {
        let svc = Arc::clone(&shared_instance);
        threads.push(thread::spawn(move || {
            let mut hdrs = HashMap::new();
            hdrs.insert("Authorization".to_string(), "invalid".to_string());
            let req = Request {
                endpoint: "/secure",
                headers: hdrs,
            };
            let outcome = svc.process(req);
            println!("[Secure Flow] Outcome: {}", outcome);
        }));
    }

    for th in threads {
        th.join().unwrap();
    }
}