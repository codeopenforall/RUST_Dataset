//////////////////////
// Fixed Code       //
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
    // The bypass flag is now strictly controlled by configuration.
    // In production, this must be false so that no endpoint bypasses authentication.
    debug_bypass: bool,
}

impl Application {
    fn new(debug_bypass: bool) -> Self {
        Application { debug_bypass }
    }

    // Correct authentication routine requiring explicit token validation.
    fn verify(&self, req: &Request) -> bool {
        // Do not allow an alternate bypass path even for debug endpoints.
        if req.path == "/debug" {
            // Previously used unsafe bypass has been removed.
            // Fall through to the standard authentication check.
        }
        // Standard authentication: Check if the provided token exactly equals "secret".
        if let Some(ref token) = req.token {
            return token == "secret";
        }
        false
    }
}

fn main() {
    // In a proper production configuration, debug_bypass is set to false.
    let app = Arc::new(Application::new(false));
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