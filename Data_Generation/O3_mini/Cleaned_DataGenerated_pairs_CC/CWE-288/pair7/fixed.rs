use std::collections::HashMap;
use std::sync::Arc;
use std::thread;
struct App {
    valid_token: String,
}
impl App {
    fn new(valid_token: String) -> Self {
        App { valid_token }
    }
    fn process_request(&self, req: &Request) -> bool {
        req.token == self.valid_token
    }
}
struct Request {
    token: String,
    headers: HashMap<String, String>,
}
fn main() {
    let app = Arc::new(App::new("secret".to_string()));
    let req = Request {
        token: "wrong_secret".to_string(),
        headers: {
            let mut h = HashMap::new();
            h.insert("X-Debug".to_string(), "true".to_string());
            h
        },
    };
    let app_clone = Arc::clone(&app);
    let handler = thread::spawn(move || {
        let res = app_clone.process_request(&req);
        println!("Authenticated: {}", res);
        res
    });
    let _ = handler.join().unwrap();
}
