//////////////////////
// Vulnerable Code  //
//////////////////////
use std::sync::Arc;
use std::thread;

struct Request {
    path: String,
    token: Option<String>,
}

impl Request {
    fn new(path: &str, token: Option<&str>) -> Self {
        Request {
            path: path.to_string(),
            token: token.map(|s| s.to_string()),
        }
    }
}

struct Application {
    // In production, this flag should be false.
    // However, due to misconfiguration the debug bypass flag is enabled.
    debug_bypass: bool,
}

impl Application {
    fn new(debug_bypass: bool) -> Self {
        Application { debug_bypass }
    }

    // Authentication routine: For the sensitive endpoint,
    // requests should provide a token that equals "secret".
    // However, the debug endpoint erroneously bypasses the check.
    fn verify(&self, req: &Request) -> bool {
        // The vulnerability: An alternate authentication path
        // for the debug endpoint is left enabled.
        if req.path == "/debug" {
            // Unsafe block simulating risky pointer usage.
            unsafe {
                // Directly dereferencing a pointer to the debug_bypass field.
                let flag_ptr: *const bool = &self.debug_bypass as *const bool;
                // If the flag is true, the function returns success
                // regardless of proper authentication.
                if *flag_ptr {
                    return true;
                }
            }
        }
        // Normal authentication flow.
        if let Some(ref token) = req.token {
            return token == "secret";
        }
        false
    }
}

fn main() {
    // The application is misconfigured with the debug bypass enabled.
    // In a production scenario, debug_bypass should be false.
    let app = Arc::new(Application::new(true));
    let req = Request::new("/debug", None);

    let app_clone = Arc::clone(&app);
    let handle = thread::spawn(move || {
        if app_clone.verify(&req) {
            println!("Access granted!");
        } else {
            println!("Access denied!");
        }
    });
    handle.join().unwrap();
}