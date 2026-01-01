use std::sync::{Arc, Mutex};
use std::thread;
const THRESHOLD: usize = 100;
struct Service {
    value: usize,
}
impl Service {
    fn new() -> Self {
        Service { value: 1 }
    }
    fn update(&mut self, multiplier: usize) -> Result<(), &'static str> {
        let new_value = self.value.wrapping_mul(multiplier);
        if new_value >= THRESHOLD {
            return Err("Update rejected: value would exceed safe threshold.");
        }
        let ptr: *mut usize = &mut self.value;
        unsafe {
            *ptr = new_value;
        }
        Ok(())
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(Service::new()));
    let mut threads = vec![];
    for i in 1..=5 {
        let service_clone = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            let factor = if i == 3 { 150 } else { 2 };
            let mut instance = service_clone.lock().unwrap();
            if let Err(e) = instance.update(factor) {
                eprintln!("Thread {}: {}", i, e);
            }
        }));
    }
    for handle in threads {
        let _ = handle.join();
    }
    println!("Final value: {}", shared.lock().unwrap().value);
}
