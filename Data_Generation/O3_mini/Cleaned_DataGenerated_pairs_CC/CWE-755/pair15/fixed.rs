#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;
struct Config {
    value: u32,
}
struct Manager {
    config: Arc<Mutex<Config>>,
}
impl Manager {
    fn new(cfg: Config) -> Self {
        Manager {
            config: Arc::new(Mutex::new(cfg)),
        }
    }
    fn update(&self, s: &str) -> Result<(), String> {
        let new_val = s.trim().parse::<u32>()
            .map_err(|e| format!("Failed to parse configuration: {}", e))?;
        let mut guard = self.config.lock().map_err(|_| "Lock poisoned")?;
        *guard = Config { value: new_val };
        Ok(())
    }
    fn get_value(&self) -> u32 {
        let guard = self.config.lock().unwrap();
        guard.value
    }
}
fn main() {
    let mgr = Manager::new(Config { value: 42 });
    let shared_mgr = Arc::new(mgr);
    let thread_mgr = Arc::clone(&shared_mgr);
    let handle = thread::spawn(move || {
        if let Err(e) = thread_mgr.update("not_a_number") {
            eprintln!("Error updating configuration: {}", e);
        }
    });
    handle.join().expect("Thread panicked");
    println!("Current configuration value: {}", shared_mgr.get_value());
}
