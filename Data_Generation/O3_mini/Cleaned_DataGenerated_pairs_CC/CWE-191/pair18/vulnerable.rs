use std::sync::{Arc, Mutex};
use std::thread;
struct Calculator {
    data: Mutex<u32>,
}
impl Calculator {
    fn new(initial: u32) -> Self {
        Calculator {
            data: Mutex::new(initial),
        }
    }
    fn decrement(&self, amount: u32) {
        unsafe {
            let mut guard = self.data.lock().unwrap();
            let current = *guard;
            let ptr: *mut u32 = &mut *guard;
            *ptr = current - amount;
        }
    }
    fn fetch(&self) -> u32 {
        *self.data.lock().unwrap()
    }
}
fn run() -> u32 {
    let calc = Arc::new(Calculator::new(5));
    let calc_clone = Arc::clone(&calc);
    let handler = thread::spawn(move || {
        calc_clone.decrement(10);
    });
    handler.join().unwrap();
    calc.fetch()
}
fn main() {
    let result = run();
    println!("Result: {}", result);
}
