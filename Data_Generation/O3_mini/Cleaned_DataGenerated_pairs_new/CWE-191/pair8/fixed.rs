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
            if sub > current {
                return Err("Integer underflow detected");
            }
            let res = current - sub;
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
        counter_clone.subtract(3)
    });
    let thread_res = handle.join().unwrap();
    match thread_res {
        Ok(val) => println!("Thread result: {}", val),
        Err(e) => println!("Thread error: {}", e),
    }
    let main_res = counter.subtract(1);
    match main_res {
        Ok(val) => println!("Main thread result: {}", val),
        Err(e) => println!("Main thread error: {}", e),
    }
}
