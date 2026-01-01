use std::sync::Arc;
use std::thread;
struct Config {
    sensitive: String,
}
impl Config {
    fn new() -> Self {
        Config { sensitive: String::from("super_secret_password") }
    }
}
fn load_config() -> Config {
    Config::new()
}
fn basic_encrypt(text: &str, key: u8) -> String {
    text.bytes()
        .map(|b| (b ^ key) as char)
        .collect()
}
fn send(data: &str) -> Result<(), &'static str> {
    println!("Transmitting data: {:?}", data);
    if data.contains("secret") {
        Err("Transmission error: unencrypted sensitive data detected")
    } else {
        Ok(())
    }
}
fn run() -> Result<(), &'static str> {
    let config = Arc::new(load_config());
    let shared_config = Arc::clone(&config);
    let key: u8 = 0xAA; 
    let handle = thread::spawn(move || {
        let encrypted = basic_encrypt(&shared_config.sensitive, key);
        send(&encrypted)
    });
    handle.join().unwrap()
}
fn main() {
    match run() {
        Ok(_) => println!("Operation completed successfully."),
        Err(e) => println!("Operation failed: {}", e),
    }
}
