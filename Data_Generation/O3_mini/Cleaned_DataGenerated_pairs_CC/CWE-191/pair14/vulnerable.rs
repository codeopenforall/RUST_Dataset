use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
pub struct Data {
    counter: UnsafeCell<u32>,
}
unsafe impl Sync for Data {}
impl Data {
    pub fn new(val: u32) -> Data {
        Data {
            counter: UnsafeCell::new(val),
        }
    }
    pub fn decrease(&self, sub: u32) -> u32 {
        unsafe {
            let current = *self.counter.get();
            let new_val = current.wrapping_sub(sub);
            *self.counter.get() = new_val;
            new_val
        }
    }
}
pub fn process_operation(initial: u32, sub: u32) -> Result<u32, &'static str> {
    let data = Data::new(initial);
    let shared_data = Arc::new(data);
    let thread_data = Arc::clone(&shared_data);
    let handle = thread::spawn(move || {
        thread_data.decrease(sub)
    });
    let result = handle.join().unwrap();
    Ok(result)
}
fn main() {
    let res = process_operation(100, 200);
    match res {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}
