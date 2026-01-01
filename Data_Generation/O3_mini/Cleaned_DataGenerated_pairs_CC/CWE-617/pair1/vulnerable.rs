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
    fn update(&mut self, multiplier: usize) {
        let ptr: *mut usize = &mut self.value;
        unsafe {
            *ptr = self.value.wrapping_mul(multiplier);
        }
        assert!(self.value < THRESHOLD, "Error: value exceeded safe threshold!");
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(Service::new()));
    let mut threads = vec![];
    for i in 1..=5 {
        let service_clone = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            let mut instance = service_clone.lock().unwrap();
            let factor = if i == 3 { 150 } else { 2 };
            instance.update(factor);
        }));
    }
    for handle in threads {
        let _ = handle.join();
    }
    println!("Final value: {}", shared.lock().unwrap().value);
}
