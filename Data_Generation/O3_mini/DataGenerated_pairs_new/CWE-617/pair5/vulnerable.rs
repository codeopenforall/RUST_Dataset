use std::sync::Arc;
use std::thread;

struct Locker {
    data: Arc<Vec<u64>>,
}

impl Locker {
    fn new() -> Self {
        // A fixed-size vector holding ten elements.
        Locker { data: Arc::new(vec![0; 10]) }
    }

    fn update(&self, index: usize, value: u64) {
        unsafe {
            // Create a mutable pointer from the immutable vector.
            let ptr = self.data.as_ptr() as *mut u64;
            // Write the provided value at the given index without checking if it’s valid.
            *ptr.add(index) = value;
            // A post-write assertion using attacker-influenced index.
            // If an out-of-bound index is provided by an attacker, this assertion fires,
            // resulting in a panic and causing denial-of-service.
            assert!(index < self.data.len(), "Index out of range: {}", index);
        }
    }
}

fn main() {
    let locker = Locker::new();
    // Spawn a thread to simulate concurrent processing.
    let locker_clone = locker;
    let handle = thread::spawn(move || {
        // Attacker-controlled index input: 15 is beyond the valid range [0, 9].
        locker_clone.update(15, 42);
    });
    let _ = handle.join();
    println!("Execution completed.");
}