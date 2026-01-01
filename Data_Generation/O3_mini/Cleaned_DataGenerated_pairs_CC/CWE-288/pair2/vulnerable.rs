use std::sync::{Arc, Mutex};
static mut GLOBAL_DEBUG: bool = true; 
struct Request {
    endpoint: String,
    token: Option<String>,
}
struct Server {
    secret: String, 
    config: Arc<Mutex<Config>>,
}
struct Config {
    require_auth: bool,
}
trait Processor {
    fn process_request(&self, req: &Request) -> bool;
}
impl Server {
    fn new(secret: &str, require_auth: bool) -> Self {
        Server {
            secret: secret.to_string(),
            config: Arc::new(Mutex::new(Config { require_auth })),
        }
    }
}
impl Processor for Server {
    fn process_request(&self, req: &Request) -> bool {
        if req.endpoint == "/admin" {
            let cfg = self.config.lock().unwrap();
            if cfg.require_auth {
                return req.token.as_ref() == Some(&self.secret);
            }
            return true;
        }
        else if req.endpoint == "/debug" {
            unsafe {
                if GLOBAL_DEBUG {
                    return true;
                }
            }
            return req.token.as_ref() == Some(&self.secret);
        }
        false
    }
}
fn main() {
    let server = Server::new("supersecret", true);
    let req_admin = Request {
        endpoint: "/admin".to_string(),
        token: Some("supersecret".to_string()),
    };
    let req_debug = Request {
        endpoint: "/debug".to_string(),
        token: None,
    };
    println!("Admin endpoint access: {}", server.process_request(&req_admin));
    println!("Debug endpoint access: {}", server.process_request(&req_debug));
}
