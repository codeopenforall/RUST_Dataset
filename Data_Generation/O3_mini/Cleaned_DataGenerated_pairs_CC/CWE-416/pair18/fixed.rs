use std::thread;
use std::time::Duration;
struct Data {
    value: i32,
}
impl Data {
    fn new(val: i32) -> Self {
        Data { value: val }
    }
}
fn compute() -> i32 {
    let boxed = Box::new(Data::new(42));
    boxed.value
}
fn main() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(10));
        compute()
    });
    let result = handle.join().expect("Thread panicked");
    println!("Computed result: {}", result);
}
