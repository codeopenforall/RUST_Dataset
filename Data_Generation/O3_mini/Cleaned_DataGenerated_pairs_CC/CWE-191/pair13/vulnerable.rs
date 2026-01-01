use std::sync::{Arc, Mutex, Barrier};
use std::thread;
pub struct Container {
    pub value: u32,
}
impl Container {
    pub unsafe fn reduce(&mut self, amount: u32) {
        self.value = self.value - amount;
    }
}
pub fn execute(amount: u32) -> u32 {
    let data = Arc::new(Mutex::new(Container { value: 10 }));
    let barrier = Arc::new(Barrier::new(2));
    let data_clone = Arc::clone(&data);
    let barrier_clone = Arc::clone(&barrier);
    let handle = thread::spawn(move || {
        barrier_clone.wait();
        let mut guard = data_clone.lock().unwrap();
        unsafe {
            guard.reduce(amount);
        }
    });
    barrier.wait();
    handle.join().unwrap();
    let guard = data.lock().unwrap();
    guard.value
}
fn main() {
    let result = execute(20);
    println!("Final value: {}", result);
}
