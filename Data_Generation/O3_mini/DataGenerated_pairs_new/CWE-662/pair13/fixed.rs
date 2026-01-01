//////////////////////////////////////////
// Corrected Example for CWE-662:
// This improved version addresses the lock poisoning issue by
// using a safe_lock() helper function, which recovers the underlying
// data from a poisoned lock rather than panicking. The rest of the
// logic, including unsafe counter incrementing, remains unchanged.
//////////////////////////////////////////
use std::sync::{Arc, Mutex, MutexGuard};
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

// Helper function to safely acquire a lock, recovering from poisoning.
fn safe_lock<'a>(mutex: &'a Mutex<Shared>) -> MutexGuard<'a, Shared> {
    match mutex.lock() {
        Ok(guard) => guard,
        // Recover from a poisoned lock by consuming the error and taking the inner value.
        Err(poisoned) => poisoned.into_inner(),
    }
}

fn run() {
    let data = Arc::new(Mutex::new(Shared::new()));

    // Thread that locks and then panics while holding the lock.
    let data_clone = Arc::clone(&data);
    let thr1 = thread::spawn(move || {
        let mut locked = safe_lock(&data_clone);
        locked.increment();
        panic!("Simulated panic to poison the lock");
    });

    // Allow time for the first thread to poison the lock.
    thread::sleep(Duration::from_millis(50));

    // Second thread acquires the lock safely.
    let data_clone2 = Arc::clone(&data);
    let thr2 = thread::spawn(move || {
        let mut locked = safe_lock(&data_clone2);
        locked.increment();
    });

    let _ = thr1.join();
    let _ = thr2.join();

    // Main thread acquires the lock using our safe helper.
    let locked = safe_lock(&data);
    println!("Final counter: {}", locked.value());
}

fn main() {
    run();
}