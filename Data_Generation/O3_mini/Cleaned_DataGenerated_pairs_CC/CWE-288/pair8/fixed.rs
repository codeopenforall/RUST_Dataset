use std::sync::{Arc, Mutex};
use std::thread;
struct Service {
    debug_mode: bool,
}
impl Service {
    fn new(debug_mode: bool) -> Self {
        Service { debug_mode }
    }
    fn process_request(&self, path: &str, token: &str) -> bool {
        if path == "debug" && self.debug_mode {
            return self.verify(token);
        }
        self.verify(token)
    }
    fn verify(&self, token: &str) -> bool {
        token == "secret"
    }
}
fn main() {
    let service = Arc::new(Service::new(true));
    let responses = Arc::new(Mutex::new(Vec::new()));
    let mut handles = vec![];
    for i in 0..4 {
        let srv = Arc::clone(&service);
        let res = Arc::clone(&responses);
        let handle = thread::spawn(move || {
            let result = if i % 2 == 0 {
                srv.process_request("debug", "bad")
            } else {
                srv.process_request("api", "secret")
            };
            res.lock().unwrap().push(result);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Results: {:?}", responses.lock().unwrap());
}
