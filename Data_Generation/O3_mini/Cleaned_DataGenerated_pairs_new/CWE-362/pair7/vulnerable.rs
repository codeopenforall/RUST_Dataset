use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
struct Data {
    value: UnsafeCell<u32>,
}
unsafe impl Sync for Data {}
impl Data {
    fn increment(&self) {
        unsafe {
            let ptr = self.value.get();
            let curr = *ptr;
            thread::yield_now();
            *ptr = curr + 1;
        }
    }
    fn get(&self) -> u32 {
        unsafe { *self.value.get() }
    }
}
fn run() -> u32 {
    let shared = Arc::new(Data { value: UnsafeCell::new(0) });
    let mut threads = Vec::new();
    for _i in 0..10 {
        let copy = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                copy.increment();
            }
        });
        threads.push(handle);
    }
    for thr in threads {
        thr.join().unwrap();
    }
    shared.get()
}
fn main() {
    let final_value = run();
    println!("Final count: {}", final_value);
}
