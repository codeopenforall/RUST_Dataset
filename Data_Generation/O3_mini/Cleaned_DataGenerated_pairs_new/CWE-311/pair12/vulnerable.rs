use std::sync::Arc;
use std::thread;
use std::time::Duration;
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
fn send(data: &str) -> Result<(), &'static str> {
    unsafe {
        let ptr = data.as_ptr();
        println!("Transmitting from pointer: {:?}", ptr);
        let leaked = std::slice::from_raw_parts(ptr, data.len());
        println!("Logged Data (vulnerable): {:?}", leaked);
    }
    if data.contains("secret") {
        Err("Transmission error: unencrypted sensitive data detected")
    } else {
        Ok(())
    }
}
fn run() -> Result<(), &'static str> {
    let config = Arc::new(load_config());
    let shared_config = Arc::clone(&config);
    let handle = thread::spawn(move || {
        unsafe {
            let ptr = shared_config.sensitive.as_ptr();
            let slice = std::slice::from_raw_parts(ptr, shared_config.sensitive.len());
            let extracted = String::from_utf8_lossy(slice);
            send(&extracted)
        }
    });
    handle.join().unwrap()
}
fn main() {
    match run() {
        Ok(_) => println!("Operation completed successfully."),
        Err(e) => println!("Operation failed: {}", e),
    }
}
