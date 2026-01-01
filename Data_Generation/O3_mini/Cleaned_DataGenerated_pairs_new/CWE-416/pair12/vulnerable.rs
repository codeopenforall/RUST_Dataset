#![allow(unused_unsafe)]
use std::thread;
use std::sync::{Arc, Mutex};
trait Arithmetic {
    fn compute(&self) -> i32;
}
struct Container {
    data: i32,
}
impl Arithmetic for Container {
    fn compute(&self) -> i32 {
        self.data + 1
    }
}
fn process() -> i32 {
    let resource = Box::new(Container { data: 99 });
    let raw = Box::into_raw(resource);
    unsafe {
        let _ = Box::from_raw(raw); 
    }
    let result = unsafe { (*raw).compute() }; 
    result
}
fn main() {
    let val = process();
    println!("Result: {}", val);
    let _handle = thread::spawn(|| {
        let boxed = Box::new(Container { data: 50 });
        let raw_ptr = Box::into_raw(boxed);
        unsafe {
            let _ = Box::from_raw(raw_ptr); 
        }
        let wrong_use = unsafe { (*raw_ptr).compute() };
        println!("Thread result: {}", wrong_use);
    });
    thread::sleep(std::time::Duration::from_millis(50));
}
