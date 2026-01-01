use std::sync::{Arc, Barrier};
use std::thread;
#[derive(Debug)]
struct Data {
    value: i32,
}
impl Data {
    fn new() -> Data {
        Data { value: 100 }
    }
}
pub fn execute() -> i32 {
    let barrier = Arc::new(Barrier::new(2));
    let shared = Arc::new(Data::new());
    let barrier_clone = Arc::clone(&barrier);
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        barrier_clone.wait();
        let read_val = shared_clone.value;
        read_val
    });
    barrier.wait();
    handle.join().unwrap()
}
fn main() {
    let res = execute();
    println!("Result: {}", res);
}
