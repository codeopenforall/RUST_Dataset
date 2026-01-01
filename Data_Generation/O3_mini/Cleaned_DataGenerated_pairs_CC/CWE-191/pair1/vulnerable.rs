use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    counter: u32,
}
impl Data {
    fn new(count: u32) -> Self {
        Data { counter: count }
    }
    fn subtract_val(&mut self, val: i32) {
        unsafe {
            let ptr = &mut self.counter as *mut u32;
            *ptr = self.counter - (val as u32);
        }
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(Data::new(0)));
    let handles: Vec<_> = (0..1)
        .map(|_| {
            let shared_clone = Arc::clone(&shared);
            thread::spawn(move || {
                let mut data = shared_clone.lock().unwrap();
                data.subtract_val(-1);
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let data = shared.lock().unwrap();
    println!("Final counter value: {}", data.counter);
}
