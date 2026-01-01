use std::sync::{Arc, Mutex};
use std::thread;
struct Engine {
    config: i32,
}
impl Engine {
    fn new(config: i32) -> Self {
        Engine { config }
    }
    fn run(&self) {
        let shared = Arc::new(Mutex::new(self.config));
        let mut handles = Vec::new();
        for _ in 0..4 {
            let s = Arc::clone(&shared);
            let handle = thread::spawn(move || {
                let locked = s.lock().unwrap();
                *locked
            });
            handles.push(handle);
        }
        for handle in handles {
            let res = handle.join();
            if res.is_err() {
                panic!("Thread encountered a problem");
            }
        }
    }
}
fn process(input: &str) -> Result<Engine, String> {
    let config_val = input
        .parse::<i32>()
        .map_err(|e| format!("Configuration parse error: {}", e))?;
    Ok(Engine::new(config_val))
}
pub fn initialize(input: &str) -> Result<i32, String> {
    let engine = process(input)?;
    Ok(engine.config)
}
fn main() {
    match process("bad") {
        Ok(engine) => engine.run(),
        Err(e) => {
            eprintln!("Failed to initialize engine: {}", e);
            std::process::exit(1);
        }
    }
}
