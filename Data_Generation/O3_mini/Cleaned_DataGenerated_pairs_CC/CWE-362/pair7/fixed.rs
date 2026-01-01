use std::sync::{Arc, Mutex};
use std::thread;
struct Data {
    value: Mutex<u32>,
}
impl Data {
    fn increment(&self) {
        let mut num = self.value.lock().unwrap();
        *num += 1;
    }
    fn get(&self) -> u32 {
        *self.value.lock().unwrap()
    }
}
fn run() -> u32 {
    let shared = Arc::new(Data { value: Mutex::new(0) });
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
