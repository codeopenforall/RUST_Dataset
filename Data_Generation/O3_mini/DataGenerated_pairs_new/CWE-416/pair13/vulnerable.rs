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
        // Free the memory by reconstructing the Box and abandoning it.
        Box::from_raw(ptr);
        // Simulate some delay to mimic concurrent activity
        thread::sleep(Duration::from_millis(100));
        // Dereference the pointer after the memory has been freed (Use-After-Free)
        (*ptr).value
    }
}

fn main() {
    let result = process();
    println!("Result: {}", result);
}