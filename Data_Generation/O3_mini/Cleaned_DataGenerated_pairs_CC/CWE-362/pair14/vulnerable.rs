use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
pub struct Shared {
    count: UnsafeCell<u32>,
}
unsafe impl Sync for Shared {}
impl Shared {
    pub fn new() -> Self {
        Shared { count: UnsafeCell::new(0) }
    }
    pub fn process(&self) {
        unsafe {
            if *self.count.get() == 0 {
                thread::sleep(Duration::from_millis(50));
                *self.count.get() += 1;
            }
        }
    }
    pub fn value(&self) -> u32 {
        unsafe { *self.count.get() }
    }
}
fn main() {
    let shared = Arc::new(Shared::new());
    let mut handles = vec![];
    for _ in 0..2 {
        let s = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            s.process();
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Final count: {}", shared.value());
}
