use std::{sync::{Arc, Mutex}, thread};
struct Data {
    count: u32,
}
fn run_system() -> u32 {
    let shared = Arc::new(Mutex::new(Data { count: 0 }));
    let mut threads = vec![];
    for _ in 0..10 {
        let shared_clone = Arc::clone(&shared);
        threads.push(thread::spawn(move || {
            for _ in 0..100 {
                let mut data = shared_clone.lock().unwrap();
                let current = data.count;
                thread::yield_now();
                data.count = current + 1;
            }
        }));
    }
    for t in threads {
        t.join().unwrap();
    }
    let result = shared.lock().unwrap().count;
    result
}
fn main() {
    let total = run_system();
    println!("Final count: {}", total);
}
