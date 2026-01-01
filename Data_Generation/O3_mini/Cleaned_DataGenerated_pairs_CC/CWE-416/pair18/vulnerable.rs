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
    let raw_ptr = Box::into_raw(boxed);
    unsafe {
        Box::from_raw(raw_ptr);
        let _dummy = Box::new(Data::new(100));
        (*raw_ptr).value
    }
}
fn main() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(10));
        compute()
    });
    let result = handle.join().expect("Thread panicked");
    println!("Computed result: {}", result);
}
