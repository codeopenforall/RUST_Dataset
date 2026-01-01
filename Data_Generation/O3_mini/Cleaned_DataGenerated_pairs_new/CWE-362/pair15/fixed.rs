use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
struct Counter {
    count: u64,
}
impl Counter {
    fn new() -> Self {
        Counter { count: 0 }
    }
}
fn run_concurrent(increments: u64, threads: usize) -> u64 {
    let counter = Arc::new(Mutex::new(Counter::new()));
    let mut handles = Vec::with_capacity(threads);
    for _ in 0..threads {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..increments {
                {
                    let mut data = counter_clone.lock().unwrap();
                    data.count = data.count.wrapping_add(1);
                }
                thread::sleep(Duration::from_micros(1));
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let data = counter.lock().unwrap();
    data.count
}
fn main() {
    let threads = 4;
    let increments = 10000;
    let final_value = run_concurrent(increments, threads);
    println!("Final count: {}", final_value);
}
