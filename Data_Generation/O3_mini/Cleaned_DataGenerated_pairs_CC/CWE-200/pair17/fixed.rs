use std::sync::{Arc, Mutex};
use std::thread;
use std::panic;
struct Data {
    secret: String,
}
impl Data {
    fn new(secret: &str) -> Self {
        Data { secret: secret.to_string() }
    }
}
fn process(input: &str, store: &Arc<Mutex<Data>>) -> Result<String, String> {
    if input != "safe" {
        return Err("Access denied.".to_string());
    }
    Ok("Operation successful".to_string())
}
fn main() {
    let store = Arc::new(Mutex::new(Data::new("S3cr3tP@ss")));
    let arg = std::env::args().nth(1).unwrap_or_else(|| "unsafe".to_string());
    let store_clone = Arc::clone(&store);
    let handler = thread::spawn(move || {
        match process(&arg, &store_clone) {
            Ok(message) => println!("{}", message),
            Err(err) => {
                panic!("Thread panic: {}", err);
            }
        }
    });
    let _ = handler.join();
}
