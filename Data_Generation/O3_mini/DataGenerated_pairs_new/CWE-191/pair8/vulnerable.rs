use std::thread;
use std::sync::Arc;
use std::cell::UnsafeCell;

struct Counter {
    value: UnsafeCell<u32>,
}

unsafe impl Sync for Counter {}

impl Counter {
    fn new(val: u32) -> Self {
        Self { value: UnsafeCell::new(val) }
    }

    fn subtract(&self, sub: u32) -> Result<u32, &'static str> {
        unsafe {
            let ptr = self.value.get();
            let current = *ptr;
            // Vulnerability: no underflow check; subtraction may wrap-around unexpectedly.
            let res = current.wrapping_sub(sub);
            *ptr = res;
            Ok(res)
        }
    }

    fn get(&self) -> u32 {
        unsafe { *self.value.get() }
    }
}

fn main() {
    let counter = Arc::new(Counter::new(2));

    let counter_clone = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        // Subtracting a value larger than the current value (2 - 3) causes an underflow.
        counter_clone.subtract(3)
    });

    let thread_res = handle.join().unwrap();
    println!("Thread result: {:?}", thread_res);

    let main_res = counter.subtract(1);
    println!("Main thread result: {:?}", main_res);
}