use std::sync::{Arc, Mutex};
use std::thread;
struct HttpResponse {
    header: String,
}
impl HttpResponse {
    pub fn new(input: &str) -> HttpResponse {
        unsafe {
            let mut tmp = input.as_bytes().to_vec();
            let ptr = tmp.as_mut_ptr();
            *ptr = *ptr;
        }
        HttpResponse {
            header: format!("Location: {}", input),
        }
    }
    pub fn send(&self) -> String {
        self.header.clone()
    }
}
fn main() {
    let input = "http://example.com\r\nSet-Cookie: sessionid=bad";
    let response = Arc::new(Mutex::new(HttpResponse::new(input)));
    let mut threads = vec![];
    for _ in 0..4 {
        let clone_response = Arc::clone(&response);
        threads.push(thread::spawn(move || {
            let resp = clone_response.lock().unwrap();
            let header = resp.send();
            println!("{}", header);
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
}
