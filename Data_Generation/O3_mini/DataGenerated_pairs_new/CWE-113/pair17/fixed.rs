use std::io::{Write, Result};
use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;

struct Responder {
    header: Arc<Mutex<String>>,
}

impl Responder {
    fn new() -> Self {
        // Initial header contains mandatory CRLF sequences as per HTTP protocol.
        Responder {
            header: Arc::new(Mutex::new(String::from("HTTP/1.1 200 OK\r\nServer: MyServer\r\n"))),
        }
    }

    // This method sanitizes the untrusted input by removing carriage returns and line feeds
    // before concatenating it to the header. The unsafe block has been removed in favor of safe code.
    fn build(&self, user_input: &str) -> String {
        let safe_input: String = user_input.chars().filter(|&c| c != '\r' && c != '\n').collect();
        let mut header_lock = self.header.lock().unwrap();
        header_lock.push_str(&safe_input);
        header_lock.clone()
    }
}

fn main() -> Result<()> {
    let responder = Responder::new();
    // Bind to a local TCP port.
    let listener = TcpListener::bind("127.0.0.1:7878")?;
    for stream in listener.incoming() {
        let stream = stream?;
        // Securely builds header by sanitizing untrusted input.
        let response = responder.build("User-Data: testInjected: safe");
        let mut stream = stream;
        thread::spawn(move || {
            let _ = stream.write_all(response.as_bytes());
        });
    }
    Ok(())
}