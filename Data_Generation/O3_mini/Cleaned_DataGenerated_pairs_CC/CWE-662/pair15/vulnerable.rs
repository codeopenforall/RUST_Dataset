use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    value: u32,
}
impl Data {
    unsafe fn do_increment(ptr: *mut Data) {
        (*ptr).value = (*ptr).value.wrapping_add(1);
    }
}
fn compute() -> u32 {
    let shared = Arc::new(Mutex::new(Data { value: 0 }));
    let mut handles = vec![];
    for _ in 0..4 {
        let s = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..10000 {
                let raw_ptr: *mut Data = {
                    let guard = s.lock().unwrap();
                    &*guard as *const Data as *mut Data
                };
                unsafe {
                    Data::do_increment(raw_ptr);
                }
            }
        });
        handles.push(handle);
    }
    for h in handles {
        let _ = h.join();
    }
    let result = shared.lock().unwrap().value;
    result
}
fn main() {
    let result = compute();
    println!("Final value: {}", result);
}
