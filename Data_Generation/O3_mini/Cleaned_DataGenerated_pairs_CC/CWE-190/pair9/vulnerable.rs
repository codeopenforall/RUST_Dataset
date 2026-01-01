use std::sync::{Arc, Mutex};
use std::thread;
struct Accumulator {
    balance: u32,
}
impl Accumulator {
    fn new(init: u32) -> Self {
        Accumulator { balance: init }
    }
    fn add(&mut self, amount: u32) {
        unsafe {
            self.balance = std::intrinsics::unchecked_add(self.balance, amount);
        }
    }
    fn current(&self) -> u32 {
        self.balance
    }
}
fn run_calculation(init: u32, add: u32, threads: usize) -> u32 {
    let accumulator = Arc::new(Mutex::new(Accumulator::new(init)));
    let mut handles = vec![];
    for _ in 0..threads {
        let acc_clone = Arc::clone(&accumulator);
        handles.push(thread::spawn(move || {
            let mut guard = acc_clone.lock().unwrap();
            guard.add(add);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let guard = accumulator.lock().unwrap();
    guard.current()
}
pub fn try_compute() -> Result<u32, &'static str> {
    let result = run_calculation(u32::MAX - 10, 15, 1);
    Ok(result)
}
fn main() {
    let result = try_compute().unwrap();
    println!("Final result: {}", result);
}
