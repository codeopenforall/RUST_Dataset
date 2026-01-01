use std::sync::{Arc, Mutex};
use std::thread;
struct SharedData {
    count: Mutex<i32>,
}
impl SharedData {
    fn new(val: i32) -> Self {
        SharedData {
            count: Mutex::new(val),
        }
    }
    fn increment(&self) {
        let mut num = self.count.lock().unwrap();
        *num += 1;
    }
    fn get(&self) -> i32 {
        *self.count.lock().unwrap()
    }
}
fn run_logic() -> i32 {
    let shared = Arc::new(SharedData::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let s = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                s.increment();
            }
        });
        handles.push(handle);
    }
    for h in handles {
        h.join().unwrap();
    }
    shared.get()
}
fn main() {
    let final_val = run_logic();
    println!("Final count: {}", final_val);
}
