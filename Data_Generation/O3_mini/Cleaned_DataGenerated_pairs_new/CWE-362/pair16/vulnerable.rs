use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
pub struct Core {
    flag: UnsafeCell<bool>,
}
impl Core {
    pub fn new() -> Self {
        Self { flag: UnsafeCell::new(true) }
    }
    pub fn try_use(&self) -> bool {
        unsafe {
            if *self.flag.get() {
                thread::sleep(Duration::from_micros(10));
                *self.flag.get() = false;
                return true;
            }
        }
        false
    }
}
unsafe impl Sync for Core {}
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
