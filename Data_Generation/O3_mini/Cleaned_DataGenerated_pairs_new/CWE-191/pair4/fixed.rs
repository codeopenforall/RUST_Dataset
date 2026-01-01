use std::sync::{Arc, Mutex};
use std::thread;
struct Counter {
    value: u64,
}
impl Counter {
    fn new(init: u64) -> Self {
        Counter { value: init }
    }
    fn decrease(&mut self, amount: u64) {
        unsafe {
            self.value = self.value.saturating_sub(amount);
        }
    }
    fn get(&self) -> u64 {
        self.value
    }
}
fn run(counter: Arc<Mutex<Counter>>, amount: u64, iterations: u32) {
    let mut handles = vec![];
    for _ in 0..iterations {
        let c = counter.clone();
        handles.push(thread::spawn(move || {
            let mut lock = c.lock().unwrap();
            lock.decrease(amount);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
fn main() {
    let counter = Arc::new(Mutex::new(Counter::new(50)));
    run(counter.clone(), 10, 10);
    let final_value = counter.lock().unwrap().get();
    println!("Final count: {}", final_value);
}
