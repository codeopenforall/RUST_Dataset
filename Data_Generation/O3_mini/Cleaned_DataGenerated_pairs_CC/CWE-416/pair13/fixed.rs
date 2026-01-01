use std::sync::Arc;
use std::thread;
#[derive(Debug)]
pub struct Data {
    pub value: i32,
}
pub fn process() -> i32 {
    let data = Arc::new(Data { value: 512 });
    let data_clone = Arc::clone(&data);
    data_clone.value
}
fn main() {
    let result = process();
    println!("Result: {}", result);
}
