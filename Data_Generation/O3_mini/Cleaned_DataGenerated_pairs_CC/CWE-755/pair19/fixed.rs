use std::env;
use std::sync::{Arc, Mutex};
use std::thread;
struct Config {
    secret: i32,
}
impl Config {
    fn load() -> Result<Self, &'static str> {
        let value = env::var("CONFIG_VAL").map_err(|_| "missing CONFIG_VAL")?;
        let secret = value.parse::<i32>().map_err(|_| "CONFIG_VAL not an integer")?;
        if secret == 0 {
            return Err("CONFIG_VAL cannot be zero");
        }
        Ok(Config { secret })
    }
}
fn compute(input: i32, config: &Config) -> i32 {
    input / config.secret
}
fn execute() -> Result<(), &'static str> {
    let config = Config::load()?;
    let shared = Arc::new(Mutex::new(config));
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        let conf = shared_clone.lock().unwrap();
        compute(100, &conf)
    });
    let _res = handle.join().map_err(|_| "Thread panicked")?;
    println!("Execution completed without panic.");
    Ok(())
}
fn main() {
    if let Err(e) = execute() {
        eprintln!("Error encountered: {}", e);
        std::process::exit(1);
    }
}
