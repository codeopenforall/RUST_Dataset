///////////////////////////////////////////////////////////////////////
// Vulnerable Rust code sample susceptible to HTTP response splitting //
///////////////////////////////////////////////////////////////////////
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
    
    // UNSAFE: Directly uses untrusted input to format HTTP header.
    // No sanitization is performed so CRLF sequences in 'val' can inject extra headers.
    unsafe fn append(&self, label: &str, val: &str) {
        // Format header using untrusted value
        let composed = format!("{}: {}", label, val);
        // Simulate low-level processing using unsafe pointer manipulation
        let ptr = composed.as_ptr();
        let length = composed.len();
        let bytes = std::slice::from_raw_parts(ptr, length);
        let assembled = String::from_utf8_lossy(bytes).into_owned();
        // Store the header line (shared mutable state, simulated concurrency)
        let mut guard = self.data.lock().unwrap();
        guard.push(assembled);
    }
    
    fn retrieve(&self) -> Vec<String> {
        let guard = self.data.lock().unwrap();
        guard.clone()
    }
}

fn main() {
    // Malicious input containing CRLF to attempt HTTP header splitting
    let attacker_input = "Value\r\nInjected-Header: injected";
    let server = Server::initiate();
    
    // Spawn thread to mimic concurrent header building
    let shared = server.data.clone();
    let handler = thread::spawn(move || {
        unsafe {
            let srv = Server { data: shared };
            srv.append("X-Test", attacker_input);
        }
    });
    
    handler.join().unwrap();
    
    // Simulate final header assembly and response emission
    let headers = server.retrieve();
    for line in headers {
        println!("{}", line);
    }
}