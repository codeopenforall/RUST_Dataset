//////////////////////////////////////////
// Vulnerability Example for CWE-662:
// This program uses an Arc<Mutex<Shared>> to manage a shared counter.
// It employs unsafe blocks to increment the counter directly.
// Two threads are spawned: one thread intentionally panics while
// holding the lock (poisoning it), while another thread as well as
// the main thread later try to lock the mutex using unwrap().
// The use of unwrap() on a poisoned lock leads to a panic and thus
// improper synchronization handling.
//////////////////////////////////////////
use std::sync::{Arc, Mutex};
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
        // Unsafe block simulating direct memory modification.
        unsafe {
            let ptr: *mut i32 = &mut self.counter;
            // Use wrapping_add to avoid undefined behavior on overflow.
            *ptr = (*ptr).wrapping_add(1);
        }
    }

    fn value(&self) -> i32 {
        self.counter
    }
}

fn run() {
    let data = Arc::new(Mutex::new(Shared::new()));

    // Thread that locks and then panics while holding the lock
    // to simulate a poisoning event.
    let data_clone = Arc::clone(&data);
    let thr1 = thread::spawn(move || {
        let mut locked = data_clone.lock().unwrap(); // Vulnerable: unwrap on mutex lock may panic if poisoned.
        locked.increment();
        // Intentionally panic to poison the lock.
        panic!("Simulated panic to poison the lock");
    });

    // Give the first thread time to acquire the lock and panic.
    thread::sleep(Duration::from_millis(50));

    // Second thread tries to acquire the same lock.
    let data_clone2 = Arc::clone(&data);
    let thr2 = thread::spawn(move || {
        let mut locked = data_clone2.lock().unwrap(); // Vulnerable: unwrap here will panic if lock is poisoned.
        locked.increment();
    });

    let _ = thr1.join();
    let _ = thr2.join();

    // Main thread acquires the lock.
    let locked = data.lock().unwrap(); // Vulnerable: unwrap here again, no recovery from poison.
    println!("Final counter: {}", locked.value());
}

fn main() {
    run();
}