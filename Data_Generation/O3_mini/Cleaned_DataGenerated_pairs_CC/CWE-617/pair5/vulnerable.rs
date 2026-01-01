use std::sync::Arc;
use std::thread;
struct Locker {
    data: Arc<Vec<u64>>,
}
impl Locker {
    fn new() -> Self {
        Locker { data: Arc::new(vec![0; 10]) }
    }
    fn update(&self, index: usize, value: u64) {
        unsafe {
            let ptr = self.data.as_ptr() as *mut u64;
            *ptr.add(index) = value;
            assert!(index < self.data.len(), "Index out of range: {}", index);
        }
    }
}
fn main() {
    let locker = Locker::new();
    let locker_clone = locker;
    let handle = thread::spawn(move || {
        locker_clone.update(15, 42);
    });
    let _ = handle.join();
    println!("Execution completed.");
}
