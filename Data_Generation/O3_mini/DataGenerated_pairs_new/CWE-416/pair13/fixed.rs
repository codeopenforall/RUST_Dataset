use std::sync::Arc;
use std::thread;

#[derive(Debug)]
pub struct Data {
    pub value: i32,
}

pub fn process() -> i32 {
    // Use Arc to ensure shared ownership and proper lifetime management.
    let data = Arc::new(Data { value: 512 });
    // Clone the Arc to simulate concurrent access if needed.
    let data_clone = Arc::clone(&data);
    // Safe access: the memory is not manually freed, and Arc ensures the data remains valid.
    data_clone.value
}

fn main() {
    let result = process();
    println!("Result: {}", result);
}