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
        if req.endpoint == "/debug" {
            unsafe { self.unchecked_auth() }
        } else {
            match req.headers.get("Authorization") {
                Some(header) => {
                    let stored = self.token.lock().unwrap();
                    &*stored == header
                }
                None => false,
            }
        }
    }
    unsafe fn unchecked_auth(&self) -> bool {
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
    {
        let svc = Arc::clone(&shared_instance);
        threads.push(thread::spawn(move || {
            let mut hdrs = HashMap::new();
            hdrs.insert("X-Debug".to_string(), "true".to_string());
            let req = Request {
                endpoint: "/debug",
                headers: hdrs,
            };
            let outcome = svc.process(req);
            println!("[Debug Flow] Outcome: {}", outcome);
        }));
    }
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
