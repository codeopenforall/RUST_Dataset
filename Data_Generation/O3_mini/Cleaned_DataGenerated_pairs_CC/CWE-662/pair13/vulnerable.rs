use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Shared {
    counter: i32,
}
impl Shared {
    fn new() -> Self {
        Shared { counter: 0 }
    }
    fn increment(&mut self) {
        unsafe {
            let ptr: *mut i32 = &mut self.counter;
            *ptr = (*ptr).wrapping_add(1);
        }
    }
    fn value(&self) -> i32 {
        self.counter
    }
}
fn run() {
    let data = Arc::new(Mutex::new(Shared::new()));
    let data_clone = Arc::clone(&data);
    let thr1 = thread::spawn(move || {
        let mut locked = data_clone.lock().unwrap(); 
        locked.increment();
        panic!("Simulated panic to poison the lock");
    });
    thread::sleep(Duration::from_millis(50));
    let data_clone2 = Arc::clone(&data);
    let thr2 = thread::spawn(move || {
        let mut locked = data_clone2.lock().unwrap(); 
        locked.increment();
    });
    let _ = thr1.join();
    let _ = thr2.join();
    let locked = data.lock().unwrap(); 
    println!("Final counter: {}", locked.value());
}
fn main() {
    run();
}
