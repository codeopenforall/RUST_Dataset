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
fn encode(data: &str, salt: u8) -> Vec<u8> {
    data.bytes().map(|b| b ^ salt).collect()
}
unsafe fn deliver(data: &[u8]) -> Result<(), std::io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    stream.write_all(data)?;
    Ok(())
}
fn main() {
    let settings = load_settings();
    let salt: u8 = 0xAA;
    let protected = encode(&settings.key, salt);
    let shared_data = Arc::new(protected);
    let data_copy = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        unsafe {
            match deliver(&data_copy) {
                Ok(_) => println!("Transmission finished (secure)"),
                Err(e) => println!("Transmission error: {:?}", e),
            }
        }
    });
    handle.join().unwrap();
}
