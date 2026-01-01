use std::sync::{Arc, Mutex};
use std::thread;
struct Shared {
    first: Mutex<i32>,
    second: Mutex<i32>,
}
impl Shared {
    fn new() -> Self {
        Self {
            first: Mutex::new(0),
            second: Mutex::new(0),
        }
    }
    fn update(&self) {
        let mut l1 = self.first.lock().unwrap();
        let mut l2 = self.second.lock().unwrap();
        *l1 += 1;
        *l2 += 1;
    }
    fn finalize(&self) -> i32 {
        let l1 = self.first.lock().unwrap();
        let l2 = self.second.lock().unwrap();
        *l1 + *l2
    }
}
fn main() {
    let shared = Arc::new(Shared::new());
    let mut handles = vec![];
    for _ in 0..2 {
        let clone = Arc::clone(&shared);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                clone.update();
            }
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    let result = shared.finalize();
    println!("Result: {}", result);
}
