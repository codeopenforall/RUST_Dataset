use std::sync::{Arc, Mutex};
use std::thread;
#[derive(Default)]
struct Config {
    value: u32,
}
impl Config {
    fn new(val: u32) -> Self {
        Config { value: val }
    }
}
fn load_config() -> Result<Config, &'static str> {
    Err("Configuration load error")
}
fn process(config: Arc<Mutex<Config>>) {
    let raw_ptr = &mut *config.lock().unwrap() as *mut Config;
    unsafe {
        (*raw_ptr).value = (*raw_ptr).value.wrapping_add(1);
    }
}
fn main() {
    let config = load_config().unwrap_or_default();
    let shared_config = Arc::new(Mutex::new(config));
    let mut handles = vec![];
    for _ in 0..4 {
        let clone_config = Arc::clone(&shared_config);
        let handle = thread::spawn(move || {
            process(clone_config);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!(
        "Processed value: {}",
        shared_config.lock().unwrap().value
    );
}
