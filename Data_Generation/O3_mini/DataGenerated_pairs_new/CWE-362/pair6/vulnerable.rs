///////////////////////////////////////////////////////////////////////////////
// This code demonstrates a race condition vulnerability (CWE-362) via an 
// unsynchronized shared counter. It uses unsafe blocks to perform a read-modify-write 
// operation on an Unsynchronized shared variable. Concurrent threads increment
// the counter without proper synchronization, resulting in a potential race condition.
///////////////////////////////////////////////////////////////////////////////

use std::thread;
use std::cell::UnsafeCell;

// A shared counter using UnsafeCell to enable interior mutability unsafely.
struct Counter {
    value: UnsafeCell<i32>,
}

// Manually opt-in to thread-safety. This is unsafe because we are promising that
// accesses to the counter are safe despite unsynchronized mutation.
unsafe impl Sync for Counter {}

impl Counter {
    fn new() -> Self {
        Counter { value: UnsafeCell::new(0) }
    }

    fn get(&self) -> i32 {
        unsafe { *self.value.get() }
    }

    // This function performs an unsynchronized increment.
    fn add(&self, delta: i32) {
        // Unsafe unsynchronized read-modify-write vulnerable to race condition.
        unsafe {
            let cur = *self.value.get();
            *self.value.get() = cur + delta;
        }
    }
}

// Executes the race condition by spawning multiple threads that update the counter.
fn run() -> i32 {
    let counter = Counter::new();
    let mut threads = vec![];

    for _ in 0..10 {
        let c = &counter;
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                c.add(1);
            }
        });
        threads.push(handle);
    }

    for t in threads {
        t.join().unwrap();
    }
    
    counter.get()
}

fn main() {
    let result = run();
    println!("Final count is: {}", result);
}