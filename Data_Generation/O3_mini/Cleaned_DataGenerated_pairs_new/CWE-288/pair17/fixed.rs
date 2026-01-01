use std::env;
use std::sync::Arc;
use std::thread;
struct Request {
    headers: Vec<(String, String)>,
    username: String,
    password: String,
}
struct Service;
impl Service {
    fn authenticate(&self, req: &Request) -> bool {
         if let Some((_, token)) = req.headers.iter().find(|(k, _)| k == "X-Debug") {
             if let Ok(valid_token) = env::var("DEBUG_TOKEN") {
                 if token == &valid_token {
                     return self.check_user(&req.username, &req.password);
                 }
             }
             return false;
         }
         self.check_user(&req.username, &req.password)
    }
    fn check_user(&self, username: &str, password: &str) -> bool {
         username == "admin" && password == "password"
    }
}
fn main() {
    let service = Arc::new(Service);
    let service_thread = Arc::clone(&service);
    let handle = thread::spawn(move || {
         let req = Request {
             headers: vec![("X-Debug".to_string(), "wrong".to_string())],
             username: "user".to_string(),
             password: "badpass".to_string(),
         };
         if service_thread.authenticate(&req) {
             println!("Debug route: Authentication succeeded (unexpected)");
         } else {
             println!("Debug route: Authentication correctly rejected");
         }
    });
    handle.join().unwrap();
    let req2 = Request {
         headers: vec![],
         username: "admin".to_string(),
         password: "password".to_string(),
    };
    if service.authenticate(&req2) {
         println!("Normal route: Authentication succeeded");
    } else {
         println!("Normal route: Authentication failed");
    }
}
