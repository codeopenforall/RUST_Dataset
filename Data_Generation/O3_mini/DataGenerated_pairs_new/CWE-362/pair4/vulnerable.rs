//////////////////////////////
// Vulnerable Rust Code
//////////////////////////////
use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

struct Data {
    // Using UnsafeCell to allow interior mutability unsafely.
    value: UnsafeCell<u32>,
}

// Marking Data as Sync manually. This is unsound because the code does not synchronize access.
unsafe impl Sync for Data {}

impl Data {
    fn new(val: u32) -> Self {
        Data {
            value: UnsafeCell::new(val),
        }
    }

    // This method attempts to increment the value, but uses an unsafe block without proper synchronization.
    fn update(&self) {
        unsafe {
            let ptr = self.value.get();
            let current = ptr.read(); // Read the current value
            // Introduce a small delay to amplify race conditions.
            thread::sleep(Duration::from_micros(1));
            ptr.write(current + 1); // Write back the new value
        }
    }

    // Returns the current value.
    fn fetch(&self) -> u32 {
        unsafe { self.value.get().read() }
    }
}

fn main() {
    let shared = Arc::new(Data::new(0));
    let mut workers = Vec::new();

    // Spawn 10 threads; each performs 1000 unsynchronized increments.
    for _ in 0..10 {
        let handler = {
            let local = Arc::clone(&shared);
            thread::spawn(move || {
                for _ in 0..1000 {
                    local.update();
                }
            })
        };
        workers.push(handler);
    }

    for worker in workers {
        worker.join().expect("Thread panicked");
    }
    println!("Final value: {}", shared.fetch());
}