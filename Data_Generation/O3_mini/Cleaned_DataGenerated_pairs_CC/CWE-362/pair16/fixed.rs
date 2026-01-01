use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
pub struct Core {
    flag: AtomicBool,
}
impl Core {
    pub fn new() -> Self {
        Self { flag: AtomicBool::new(true) }
    }
    pub fn try_use(&self) -> bool {
        if self.flag.compare_exchange(true, false, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            thread::sleep(Duration::from_micros(10));
            return true;
        }
        false
    }
}
fn main() {
    let core = Arc::new(Core::new());
    let handles: Vec<_> = (0..10)
        .map(|_| {
            let core = core.clone();
            thread::spawn(move || {
                if core.try_use() {
                    println!("Resource used");
                }
            })
        })
        .collect();
    for h in handles {
        h.join().unwrap();
    }
}
