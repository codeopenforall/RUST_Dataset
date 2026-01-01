use std::sync::{Arc, Mutex};
use std::thread;
struct Accumulator {
    total: u32,
}
impl Accumulator {
    fn new(initial: u32) -> Self {
        Accumulator { total: initial }
    }
    fn update(&mut self, amount: u32) {
        unsafe {
            let ptr: *mut u32 = &mut self.total;
            *ptr = (*ptr) - amount;
        }
    }
    fn value(&self) -> u32 {
        self.total
    }
}
fn main() {
    let data = Arc::new(Mutex::new(Accumulator::new(0)));
    let mut handles = vec![];
    for _ in 0..4 {
        let data_cloned = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut acc = data_cloned.lock().unwrap();
            acc.update(1);
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let acc = data.lock().unwrap();
    println!("Result: {}", acc.value());
}
