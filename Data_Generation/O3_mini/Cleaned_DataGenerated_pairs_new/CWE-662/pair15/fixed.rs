use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    value: u32,
}
fn compute() -> u32 {
    let shared = Arc::new(Mutex::new(Data { value: 0 }));
    let mut handles = vec![];
    for _ in 0..4 {
        let s = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..10000 {
                let mut guard = s.lock().unwrap();
                guard.value = guard.value.wrapping_add(1);
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
