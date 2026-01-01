use std::sync::{Arc, Mutex};
use std::thread;
struct Server {
    data: Arc<Mutex<Vec<String>>>,
}
impl Server {
    fn initiate() -> Self {
        Server {
            data: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn cleanse(input: &str) -> String {
        input.chars().filter(|&c| c != '\r' && c != '\n').collect()
    }
    unsafe fn append(&self, label: &str, val: &str) {
        let purified = Server::cleanse(val);
        let composed = format!("{}: {}", label, purified);
        let ptr = composed.as_ptr();
        let length = composed.len();
        let bytes = std::slice::from_raw_parts(ptr, length);
        let assembled = String::from_utf8_lossy(bytes).into_owned();
        let mut guard = self.data.lock().unwrap();
        guard.push(assembled);
    }
    fn retrieve(&self) -> Vec<String> {
        let guard = self.data.lock().unwrap();
        guard.clone()
    }
}
fn main() {
    let attacker_input = "Value\r\nInjected-Header: injected";
    let server = Server::initiate();
    let shared = server.data.clone();
    let handler = thread::spawn(move || {
        unsafe {
            let srv = Server { data: shared };
            srv.append("X-Test", attacker_input);
        }
    });
    handler.join().unwrap();
    let headers = server.retrieve();
    for line in headers {
        println!("{}", line);
    }
}
