/////////////////////////////////////////////////////////////
// Corrected Code (Synchronization via atomic operations) //
/////////////////////////////////////////////////////////////
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct SharedState {
    flag: AtomicBool,
}

impl SharedState {
    fn new() -> Self {
        SharedState { flag: AtomicBool::new(false) }
    }

    // This method atomically performs the check-then-set operation.
    // Using compare_exchange ensures that only one thread can flip the flag.
    fn check_then_set(&self) -> bool {
        if self.flag.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
            // Simulated delay (post atomic set) still present to mimic real workload.
            thread::sleep(Duration::from_millis(1));
            true
        } else {
            false
        }
    }
}

fn run_test() -> usize {
    let shared = Arc::new(SharedState::new());
    let mut handles = Vec::new();

    for _ in 0..10 {
        let s = shared.clone();
        handles.push(thread::spawn(move || s.check_then_set()));
    }
    let results: Vec<bool> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let success_count = results.into_iter().filter(|&x| x).count();
    println!("Number of successful accesses: {}", success_count);
    success_count
}

fn main() {
    // With proper atomic synchronization, exactly one thread should succeed.
    run_test();
}