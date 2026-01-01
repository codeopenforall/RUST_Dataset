use std::env;
use std::ptr;
use std::sync::{Arc, Mutex};
use std::thread;
struct Responder {
    headers: Arc<Mutex<Vec<u8>>>,
}
impl Responder {
    fn new() -> Self {
        Responder {
            headers: Arc::new(Mutex::new(Vec::new())),
        }
    }
    fn append_header(&self, key: &str, value: &str) {
        let header_line = format!("{}: {}\r\n", key, value);
        let bytes = header_line.as_bytes();
        let len = bytes.len();
        let mut buffer = Vec::with_capacity(len);
        unsafe {
            buffer.set_len(len);
            ptr::copy_nonoverlapping(bytes.as_ptr(), buffer.as_mut_ptr(), len);
        }
        let headers_cloned = Arc::clone(&self.headers);
        thread::spawn(move || {
            let mut h = headers_cloned.lock().unwrap();
            h.extend_from_slice(&buffer);
        })
        .join()
        .expect("Thread panicked");
    }
    fn build_response(&self) -> String {
        let h = self.headers.lock().unwrap();
        String::from_utf8_lossy(&h).to_string()
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <header value>", args[0]);
        std::process::exit(1);
    }
    let input = &args[1];
    let responder = Responder::new();
    responder.append_header("X-Inject", input);
    let response = responder.build_response();
    println!("{}", response);
}
