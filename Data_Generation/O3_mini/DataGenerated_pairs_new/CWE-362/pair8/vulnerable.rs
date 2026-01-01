use std::{cell::UnsafeCell, sync::Arc, thread, time::Duration};

/// Represents a shared resource that should be initialized exactly once.
struct Resource {
    /// A flag indicating whether initialization has occurred.
    flag: UnsafeCell<bool>,
    /// A counter tracking the number of initializations.
    init_count: UnsafeCell<u32>,
}

// SAFETY: We manually ensure that concurrent access is allowed, but no proper synchronization is used.
unsafe impl Sync for Resource {}

impl Resource {
    fn new() -> Self {
        Resource {
            flag: UnsafeCell::new(false),
            init_count: UnsafeCell::new(0),
        }
    }

    fn process(&self) {
        // UNSAFE: Using raw pointer interior mutability without atomicity.
        unsafe {
            // Vulnerability: Race condition when checking and setting the flag.
            if !*self.flag.get() {
                // Intentional delay to widen the race window.
                thread::sleep(Duration::from_millis(10));
                // Multiple threads may enter this block concurrently.
                *self.init_count.get() = *self.init_count.get() + 1;
                *self.flag.get() = true;
            }
        }
    }

    fn get_count(&self) -> u32 {
        unsafe { *self.init_count.get() }
    }
}

fn main() {
    let res = Arc::new(Resource::new());
    let mut handles = Vec::new();

    // Spawn multiple threads to exercise the race condition.
    for _ in 0..10 {
        let resource = Arc::clone(&res);
        handles.push(thread::spawn(move || {
            for _ in 0..5 {
                resource.process();
            }
        }));
    }

    for handle in handles {
        let _ = handle.join();
    }

    println!("Initialization count: {}", res.get_count());
}