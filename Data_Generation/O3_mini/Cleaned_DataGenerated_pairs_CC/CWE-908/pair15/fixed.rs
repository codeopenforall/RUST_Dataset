use std::{sync::{Arc, Mutex}, thread};
#[derive(Debug)]
pub struct Config {
    pub timeout: u32,
    pub label: &'static str,
}
impl Config {
    pub fn new() -> Self {
        Config {
            timeout: 30,
            label: "default",
        }
    }
}
pub fn make_config() -> Config {
    Config::new()
}
fn main() {
    let config = Arc::new(Mutex::new(make_config()));
    let mut handles = vec![];
    for _ in 0..4 {
        let cfg = Arc::clone(&config);
        let handle = thread::spawn(move || {
            let locked = cfg.lock().unwrap();
            println!("Config timeout: {}, label: {}", locked.timeout, locked.label);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
