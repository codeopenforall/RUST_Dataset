use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;

pub struct Data {
    pub value: UnsafeCell<u32>,
}

unsafe impl Sync for Data {}

impl Data {
    pub fn new(val: u32) -> Self {
        Data { value: UnsafeCell::new(val) }
    }

    // This function performs subtraction without verifying that the result
    // will not underflow. Using an unsafe block, it directly subtracts the provided
    // value from the internal unsigned integer.
    pub fn subtract(&self, sub: u32) {
        unsafe {
            let current = *self.value.get();
            // Vulnerability: no check for underflow; subtraction can wrap around.
            *self.value.get() = current - sub;
        }
    }

    pub fn get(&self) -> u32 {
        unsafe { *self.value.get() }
    }
}

fn main() {
    let data = Arc::new(Data::new(5)); // Starting at a low initial value.
    let mut threads = vec![];

    // Create multiple threads that subtract 10 from the value.
    // Since 5 - 10 will underflow and wrap around, the final value is incorrect.
    for _ in 0..4 {
        let data_clone = Arc::clone(&data);
        threads.push(thread::spawn(move || {
            data_clone.subtract(10);
        }));
    }

    for th in threads {
        th.join().unwrap();
    }

    println!("Final value: {}", data.get());
}