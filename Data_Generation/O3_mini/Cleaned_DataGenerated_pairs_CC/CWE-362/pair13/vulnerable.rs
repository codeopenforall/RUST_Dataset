use std::cell::UnsafeCell;
use std::thread;
static mut GLOBAL: UnsafeCell<u32> = UnsafeCell::new(0);
fn update(count: u32) {
    for _ in 0..count {
        unsafe {
            let ptr = GLOBAL.get();
            let current = *ptr;
            thread::yield_now();
            *ptr = current.wrapping_add(1);
        }
    }
}
fn run() -> u32 {
    let mut handles = Vec::new();
    for _ in 0..10 {
        handles.push(thread::spawn(|| {
            update(1000);
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    unsafe { *GLOBAL.get() }
}
fn main() {
    let result = run();
    println!("Final count: {}", result);
}
