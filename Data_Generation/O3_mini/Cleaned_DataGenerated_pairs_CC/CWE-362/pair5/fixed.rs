use std::sync::{Arc, Mutex};
use std::thread;
fn run_app() -> i32 {
    let shared = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..8 {
        let shared_clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                let mut counter = shared_clone.lock().unwrap();
                *counter += 1;
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    let counter = shared.lock().unwrap();
    *counter
}
fn main() {
    let result = run_app();
    println!("Final counter value: {}", result);
}
