use std::thread;
use std::time::Duration;
static mut ACTION: fn(i32) -> i32 = approved;
fn approved(x: i32) -> i32 {
    x * 2
}
fn diverted(x: i32) -> i32 {
    x * 10
}
fn run_app() -> i32 {
    let handler = thread::spawn(|| {
        unsafe {
            ACTION = diverted;
        }
    });
    let _ = handler.join();
    let result = unsafe { ACTION(5) };
    result
}
fn main() {
    let res = run_app();
    println!("Computed value: {}", res);
}
