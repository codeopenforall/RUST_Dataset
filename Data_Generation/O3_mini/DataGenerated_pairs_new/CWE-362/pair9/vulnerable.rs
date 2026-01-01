/////////////////////////// Vulnerable Code ///////////////////////////
use std::thread;
use std::time::Duration;

pub struct Worker {
    pub counter: u32,
    pub finished: bool,
}

impl Worker {
    pub fn new() -> Self {
        Worker { counter: 0, finished: false }
    }

    // The function below uses an unsafe block to access mutable fields via raw pointers.
    // This unsynchronized access creates a race condition: multiple threads may read and update
    // the counter concurrently without proper ordering.
    pub fn process(&mut self) {
        unsafe {
            let ptr_counter = &mut self.counter as *mut u32;
            let ptr_finished = &mut self.finished as *mut bool;
            if *ptr_counter < 10 {
                let tmp = *ptr_counter;
                // Simulate work delay to widen the race window.
                thread::sleep(Duration::from_millis(1));
                *ptr_counter = tmp + 1;
                if *ptr_counter == 10 {
                    *ptr_finished = true;
                }
            }
        }
    }
}

// This function spawns multiple threads that modify the shared Worker instance unsafely.
// The lack of synchronization may lead to lost updates, inconsistent counter values, or
// the finished flag not being set even after sufficient work.
pub fn run_state() -> (u32, bool) {
    let mut worker = Worker::new();
    let worker_ptr: *mut Worker = &mut worker;
    let mut threads = vec![];
    for _ in 0..15 {
        let handle = thread::spawn(move || {
            unsafe {
                (*worker_ptr).process();
            }
        });
        threads.push(handle);
    }
    for t in threads {
        t.join().unwrap();
    }
    (worker.counter, worker.finished)
}

fn main() {
    let (counter, finished) = run_state();
    println!("Counter: {}, Finished: {}", counter, finished);
}