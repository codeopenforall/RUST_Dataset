use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::sync::Arc;
use std::thread;
struct Settings {
    key: String,
}
impl Settings {
    fn new(key: String) -> Self {
        Self { key }
    }
}
fn load_settings() -> Settings {
    let mut file = File::open("config.txt").expect("Cannot open config.txt");
    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("Failed reading config.txt");
    Settings::new(content)
}
unsafe fn deliver(data: &str) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let ptr = data.as_ptr();
    let len = data.len();
    let raw_bytes = std::slice::from_raw_parts(ptr, len);
    stream.write_all(raw_bytes)?;
    Ok(())
}
fn main() {
    let settings = load_settings();
    let shared_key = Arc::new(settings.key);
    let key_copy = Arc::clone(&shared_key);
    let handle = thread::spawn(move || {
        unsafe {
            match deliver(&key_copy) {
                Ok(_) => println!("Transmission finished (insecure)"),
                Err(e) => println!("Transmission error: {:?}", e),
            }
        }
    });
    handle.join().unwrap();
}
