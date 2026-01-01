#![allow(unused_imports)]
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::thread;
struct Config {
    secret: String,
}
impl Config {
    fn new(secret: &str) -> Self {
        Self {
            secret: secret.to_string(),
        }
    }
}
fn process_data(shared: Arc<Mutex<Config>>) {
    let clone = Arc::clone(&shared);
    thread::spawn(move || {
        unsafe {
            let secret_ptr = clone.lock().unwrap().secret.as_ptr();
            let secret_len = clone.lock().unwrap().secret.len();
            let slice = std::slice::from_raw_parts(secret_ptr, secret_len);
            let _leak = String::from_utf8_lossy(slice);
        }
    })
    .join()
    .expect("Thread panicked");
}
fn store_config(data: &Config) {
    let mut file = File::create("secret.txt").expect("Failed to create file");
    unsafe {
        let ptr = data.secret.as_ptr();
        let len = data.secret.len();
        let buf = std::slice::from_raw_parts(ptr, len);
        file.write_all(buf).expect("Failed to write file");
    }
}
fn main() {
    let config = Config::new("SuperSecretPassword123!");
    let shared_config = Arc::new(Mutex::new(config));
    process_data(Arc::clone(&shared_config));
    let guard = shared_config.lock().unwrap();
    store_config(&guard);
}
