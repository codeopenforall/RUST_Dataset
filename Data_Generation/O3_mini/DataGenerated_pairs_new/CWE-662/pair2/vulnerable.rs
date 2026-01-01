use std::sync::{Arc, RwLock};
use std::thread;

struct Data {
    count: u32,
}

impl Data {
    // This method unsafely increments the counter by casting an immutable reference
    // into a mutable pointer.
    fn increment(&self) {
        unsafe {
            // Get a raw pointer from self and cast to mutable pointer.
            let ptr = self as *const Data as *mut Data;
            // Simulate an unsynchronized increment, causing potential data races.
            (*ptr).count = (*ptr).count.wrapping_add(1);
        }
    }
}

struct Controller {
    data: Arc<RwLock<Data>>,
}

impl Controller {
    fn new() -> Self {
        Controller {
            data: Arc::new(RwLock::new(Data { count: 0 })),
        }
    }

    // Faulty update: acquires a read lock but performs a mutable operation unsafely,
    // leading to improper synchronization.
    fn update(&self) {
        let guard = self.data.read().unwrap();
        // Unsafely modify the data through a read-only lock.
        guard.increment();
        drop(guard);
    }

    fn get_count(&self) -> u32 {
        self.data.read().unwrap().count
    }
}

fn main() {
    let ctrl = Controller::new();
    let shared_ctrl = Arc::new(ctrl);

    let mut handles = vec![];
    let thread_count = 4;
    let iterations = 10_000;

    for _ in 0..thread_count {
        let ctl = Arc::clone(&shared_ctrl);
        handles.push(thread::spawn(move || {
            for _ in 0..iterations {
                ctl.update();
            }
        }));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let final_count = shared_ctrl.get_count();
    println!("Final count: {}", final_count);
}