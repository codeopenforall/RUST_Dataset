use std::sync::{Arc, Mutex};
use std::thread;
struct Accumulator {
    value: u8,
}
impl Accumulator {
    fn new(init: u8) -> Self {
        Self { value: init }
    }
    fn update(&mut self, add: u8) {
        unsafe {
            self.value = self.value.wrapping_add(add);
        }
    }
    fn get(&self) -> u8 {
        self.value
    }
}
fn run_calculation() -> u8 {
    let mut acc = Accumulator::new(250);
    acc.update(10);
    acc.get()
}
fn main() {
    let shared = Arc::new(Mutex::new(run_calculation()));
    let mut threads = vec![];
    for _ in 0..2 {
        let s = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            let val = *s.lock().unwrap();
            println!("Thread sees value: {}", val);
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    println!("Final result: {}", run_calculation());
}
