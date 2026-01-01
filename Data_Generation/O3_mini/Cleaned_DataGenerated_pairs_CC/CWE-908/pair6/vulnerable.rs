use std::mem::MaybeUninit;
use std::sync::{Arc, Mutex};
use std::thread;
struct Config {
    field1: i32,
    field2: i32,
}
fn load_resource() -> Config {
    let mut config: MaybeUninit<Config> = MaybeUninit::uninit();
    unsafe {
        let config_ptr = config.as_mut_ptr();
        (*config_ptr).field1 = 42;
        config.assume_init()
    }
}
fn process_resource(cfg: &Config) -> i32 {
    cfg.field1 + cfg.field2
}
fn run_resource() -> i32 {
    let shared = Arc::new(Mutex::new(load_resource()));
    let mut handles = Vec::new();
    for _ in 0..4 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            let locked = shared_clone.lock().unwrap();
            process_resource(&locked)
        });
        handles.push(handle);
    }
    let mut total = 0;
    for handle in handles {
        if let Ok(val) = handle.join() {
            total += val;
        }
    }
    total
}
fn main() {
    let result = run_resource();
    println!("Result: {}", result);
}
