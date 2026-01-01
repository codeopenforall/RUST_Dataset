use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
struct Config {
    secret: i32,
}
impl Config {
    fn load() -> Self {
        let value = env::var("CONFIG_VAL").unwrap_or_default(); 
        let secret = value.parse::<i32>().unwrap_or_default();   
        Config { secret }
    }
}
fn compute(input: i32, config: &Config) -> i32 {
    unsafe {
        let ptr = &config.secret as *const i32;
        let secret_val = *ptr;
        input / secret_val   
    }
}
fn execute() {
    let config = Config::load();
    let shared = Arc::new(Mutex::new(config));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let conf = shared_clone.lock().unwrap();
        compute(100, &conf)
    });
    let res = handle.join().unwrap();
    println!("Result: {}", res);
}
fn main() {
    execute();
}
