use std::thread;
use std::time::Duration;
#[derive(Debug)]
pub struct Data {
    pub value: i32,
}
pub fn process() -> i32 {
    let ptr: *mut Data;
    unsafe {
        let boxed = Box::new(Data { value: 512 });
        ptr = Box::into_raw(boxed);
        Box::from_raw(ptr);
        thread::sleep(Duration::from_millis(100));
        (*ptr).value
    }
}
fn main() {
    let result = process();
    println!("Result: {}", result);
}
