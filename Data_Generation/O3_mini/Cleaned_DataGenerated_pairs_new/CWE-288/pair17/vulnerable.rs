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
         unsafe {
             for (key, _value) in &req.headers {
                 if key == "X-Debug" {
                     let bypass_ptr: *const bool = &true;
                     if *bypass_ptr {
                         return true;
                     }
                 }
             }
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
             headers: vec![("X-Debug".to_string(), "true".to_string())],
             username: "user".to_string(),
             password: "badpass".to_string(),
         };
         if service_thread.authenticate(&req) {
             println!("Authenticated: Debug route processed without proper checks");
         } else {
             println!("Authentication rejected on debug route");
         }
    });
    handle.join().unwrap();
    let req2 = Request {
         headers: vec![],
         username: "admin".to_string(),
         password: "badpass".to_string(),
    };
    if service.authenticate(&req2) {
         println!("Normal route: Authentication succeeded");
    } else {
         println!("Normal route: Authentication failed");
    }
}
