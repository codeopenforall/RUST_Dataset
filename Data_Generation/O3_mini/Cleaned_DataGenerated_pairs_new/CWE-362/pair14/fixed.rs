use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
pub struct Shared {
    count: AtomicU32,
}
impl Shared {
    pub fn new() -> Self {
        Shared { count: AtomicU32::new(0) }
    }
    pub fn process(&self) {
        if self.count.compare_exchange(0, 1, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            thread::sleep(Duration::from_millis(50));
        }
    }
    pub fn value(&self) -> u32 {
        self.count.load(Ordering::SeqCst)
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
