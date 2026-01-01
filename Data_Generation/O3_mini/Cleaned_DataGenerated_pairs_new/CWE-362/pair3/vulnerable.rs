use std::cell::UnsafeCell;
use std::sync::Arc;
use std::thread;
const ITER: i32 = 10000;
const NUM_THREADS: i32 = 4;
struct SharedData {
    value: UnsafeCell<i32>,
}
unsafe impl Sync for SharedData {}
impl SharedData {
    fn new() -> Self {
        SharedData {
            value: UnsafeCell::new(0),
        }
    }
    fn update(&self) {
        unsafe {
            let current = *self.value.get();
            let next = current + 1;
            *self.value.get() = next;
        }
    }
    fn read(&self) -> i32 {
        unsafe { *self.value.get() }
    }
}
fn main() {
    let data = Arc::new(SharedData::new());
    let mut threads = Vec::new();
    for _ in 0..NUM_THREADS {
        let shared = Arc::clone(&data);
        let handle = thread::spawn(move || {
            for _ in 0..ITER {
                shared.update();
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
    let result = data.read();
    println!("Final counter: {}", result);
    if result != NUM_THREADS * ITER {
        panic!("Race condition detected: expected {} but found {}", NUM_THREADS * ITER, result);
    }
}
