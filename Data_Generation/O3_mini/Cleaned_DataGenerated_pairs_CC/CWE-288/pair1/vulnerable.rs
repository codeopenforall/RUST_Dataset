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
    debug_bypass: bool,
}
impl Application {
    fn new(debug_bypass: bool) -> Self {
        Application { debug_bypass }
    }
    fn verify(&self, req: &Request) -> bool {
        if req.path == "/debug" {
            unsafe {
                let flag_ptr: *const bool = &self.debug_bypass as *const bool;
                if *flag_ptr {
                    return true;
                }
            }
        }
        if let Some(ref token) = req.token {
            return token == "secret";
        }
        false
    }
}
fn main() {
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
