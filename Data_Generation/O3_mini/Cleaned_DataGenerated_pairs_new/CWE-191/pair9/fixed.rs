use std::sync::{Arc, Mutex};
use std::thread;
pub struct Counter {
    count: u32,
}
impl Counter {
    pub fn new(n: u32) -> Self {
        Self { count: n }
    }
    pub fn subtract(&mut self, val: u32) {
        self.count = self.count.checked_sub(val).unwrap_or(0);
    }
    pub fn get_value(&self) -> u32 {
        self.count
    }
}
pub fn execute_calculation() -> u32 {
    let counter = Arc::new(Mutex::new(Counter::new(0)));
    let counter_clone = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut lock = counter_clone.lock().unwrap();
        lock.subtract(1);
    });
    handle.join().unwrap();
    let final_value = counter.lock().unwrap().get_value();
    final_value
}
fn main() {
    let res = execute_calculation();
    println!("Final value: {}", res);
}
