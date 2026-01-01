use std::io::{Write, Result};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
struct Responder {
    header: Arc<Mutex<String>>,
}
impl Responder {
    fn new() -> Self {
        Responder {
            header: Arc::new(Mutex::new(String::from("HTTP/1.1 200 OK\r\nServer: MyServer\r\n"))),
        }
    }
    fn build(&self, user_input: &str) -> String {
        let safe_input: String = user_input.chars().filter(|&c| c != '\r' && c != '\n').collect();
        let mut header_lock = self.header.lock().unwrap();
        header_lock.push_str(&safe_input);
        header_lock.clone()
    }
}
fn main() -> Result<()> {
    let responder = Responder::new();
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        let stream = stream?;
        let response = responder.build("User-Data: testInjected: safe");
        let mut stream = stream;
        thread::spawn(move || {
            let _ = stream.write_all(response.as_bytes());
        });
    }
    Ok(())
}
