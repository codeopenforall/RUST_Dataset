use std::thread;
use std::time::Duration;
static mut SHARED: i32 = 0;
fn run_app() -> i32 {
    let mut handles = vec![];
    for _ in 0..8 {
        let handle = thread::spawn(|| {
            for _ in 0..1000 {
                unsafe {
                    let temp = SHARED;
                    thread::sleep(Duration::from_nanos(1));
                    SHARED = temp + 1;
                }
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    unsafe { SHARED }
}
fn main() {
    let result = run_app();
    println!("Final counter value: {}", result);
}
