#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;
use std::ptr;
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
    fn update(&self, s: &str) {
        let new_val = s.trim().parse::<u32>().unwrap_or_default();
        let mut guard = self.config.lock().unwrap();
        unsafe {
            let ptr: *mut Config = &mut *guard;
            (*ptr).value = new_val;
        }
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
        thread_mgr.update("not_a_number");
    });
    handle.join().unwrap();
    println!("Current configuration value: {}", shared_mgr.get_value());
}
