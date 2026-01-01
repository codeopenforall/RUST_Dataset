#![allow(unused_unsafe)]
use std::sync::{Arc, Mutex};
use std::thread;
struct Engine;
impl Engine {
    pub fn run_input(&self, load: usize) -> Result<(), String> {
        const MAX_LOAD: usize = 100;
        if load > MAX_LOAD {
            return Err("Input load too high".to_string());
        }
        let mut result: usize = 0;
        for i in 0..load {
            unsafe {
                result = result.wrapping_add(i);
            }
        }
        let shared = Arc::new(Mutex::new(result));
        let thread_shared = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            unsafe {
                let mut val = thread_shared.lock().unwrap();
                *val = val.wrapping_mul(2);
            }
        });
        handle.join().unwrap();
        Ok(())
    }
}
fn main() {
    let eng = Engine;
    match eng.run_input(10) {
        Ok(()) => println!("Processing completed successfully."),
        Err(e) => println!("Error: {}", e),
    }
}
