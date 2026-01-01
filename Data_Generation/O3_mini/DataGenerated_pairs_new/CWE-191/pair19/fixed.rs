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

    // This function subtracts the given value safely using a saturating subtraction,
    // ensuring that the operation never underflows.
    pub fn subtract(&self, sub: u32) {
        unsafe {
            let current = *self.value.get();
            // Fixed: using saturating_sub to prevent underflow.
            *self.value.get() = current.saturating_sub(sub);
        }
    }

    pub fn get(&self) -> u32 {
        unsafe { *self.value.get() }
    }
}

fn main() {
    let data = Arc::new(Data::new(5)); // Starting with the same initial value.
    let mut threads = vec![];

    // Using multiple threads to perform safe subtraction.
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