use std::sync::{Arc, Mutex};
use std::thread;
struct ProcessData {
    value: i32,
}
impl ProcessData {
    fn new(val: i32) -> Self {
        ProcessData { value: val }
    }
    fn alter(&mut self, new_val: i32) {
        if new_val < 100 {
            self.value = new_val;
        }
    }
}
fn main() {
    let data = Arc::new(Mutex::new(ProcessData::new(10)));
    let shared = Arc::clone(&data);
    let handle = thread::spawn(move || {
        if let Ok(mut state) = shared.lock() {
            state.alter(999); 
        }
    });
    handle.join().unwrap();
    let state = data.lock().unwrap();
    assert!(state.value < 100, "Invariant violation: value is too high!");
    println!("Completed with value: {}", state.value);
}
