use std::sync::Arc;
use std::thread;
use std::time::Duration;
struct Data {
    number: i32,
}
fn acquire_shared() -> Arc<Data> {
    Arc::new(Data { number: 1337 })
}
fn safe_compute(shared: &Data) -> i32 {
    shared.number
}
pub fn get_result() -> i32 {
    let shared = acquire_shared();
    safe_compute(&shared)
}
fn main() {
    let result = get_result();
    println!("Computed result: {}", result);
}
