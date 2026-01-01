///////////////////////////////////////////////////////////////
// Vulnerable Code (Race condition via unsynchronized access) //
///////////////////////////////////////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct SharedState {
    flag: UnsafeCell<bool>,
}

// Marking as Sync manually is unsafe because we guarantee to use the 
// structure only in a controlled manner. However, unsynchronized access 
// to 'flag' may lead to a race condition!
unsafe impl Sync for SharedState {}

impl SharedState {
    fn new() -> Self {
        SharedState { flag: UnsafeCell::new(false) }
    }

    // UNSAFE: This function performs a check-then-set without synchronization.
    // Multiple threads might concurrently observe 'flag' as false and then set it,
    // leading to a race condition (TOCTOU issue).
    unsafe fn check_then_set(&self) -> bool {
        if !*self.flag.get() {
            // Simulated delay to widen the window for a race.
            thread::sleep(Duration::from_millis(1));
            *self.flag.get() = true;
            true
        } else {
            false
        }
    }
}

// This function spawns several threads that concurrently call the unsynchronized
// check_then_set method. In a race-free world only a single thread should succeed.
fn run_test() -> usize {
    let shared = Arc::new(SharedState::new());
    let mut handles = Vec::new();

    for _ in 0..10 {
        let s = shared.clone();
        handles.push(thread::spawn(move || unsafe { s.check_then_set() }));
    }
    let results: Vec<bool> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let success_count = results.into_iter().filter(|&x| x).count();
    println!("Number of successful accesses: {}", success_count);
    success_count
}

fn main() {
    // In normal execution, we might get more than one successful access due to the race.
    run_test();
}