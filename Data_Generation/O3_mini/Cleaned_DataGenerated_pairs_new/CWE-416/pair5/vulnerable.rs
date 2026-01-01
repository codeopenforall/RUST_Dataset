use std::thread;
use std::time::Duration;
struct Data {
    number: i32,
}
fn acquire_ptr() -> *const i32 {
    let boxed = Box::new(Data { number: 1337 });
    let raw = &boxed.number as *const i32;
    drop(boxed); 
    let _dummy = Box::new(Data { number: 4242 });
    raw
}
fn unsafe_compute(ptr: *const i32) -> i32 {
    unsafe {
        *ptr
    }
}
pub fn get_result() -> i32 {
    let ptr = acquire_ptr();
    unsafe_compute(ptr)
}
fn main() {
    let result = get_result();
    println!("Computed result: {}", result);
}
