use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
struct Shared {
    counter: UnsafeCell<u32>,
}
unsafe impl Sync for Shared {}
impl Shared {
    fn new() -> Self {
        Shared {
            counter: UnsafeCell::new(0),
        }
    }
}
fn run_app() -> u32 {
    let shared = Arc::new(Shared::new());
    let mut handles = Vec::new();
    let iterations_per_thread = 1000;
    for _ in 0..10 {
        let data = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..iterations_per_thread {
                unsafe {
                    let current = *data.counter.get();
                    if current < 10000 {
                        thread::sleep(Duration::from_micros(1));
                        *data.counter.get() = current + 1;
                    }
                }
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().expect("Thread panicked");
    }
    unsafe { *shared.counter.get() }
}
fn main() {
    let result = run_app();
    println!("Final counter: {}", result);
}
