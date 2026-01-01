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
        let leaked = unsafe {
            let locked = store.lock().unwrap();
            let raw_ptr: *const String = &locked.secret;
            (*raw_ptr).clone()
        };
        return Err(format!("Access denied. Debug info: secret={}", leaked));
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
