use std::sync::{Arc, Mutex};
use std::thread;
struct Counter {
    value: usize,
}
unsafe impl Send for Counter {}
trait Updater {
    fn bump(&self);
}
impl Updater for Counter {
    fn bump(&self) {
        unsafe {
            let ptr = self as *const Counter as *mut Counter;
            (*ptr).value = (*ptr).value.wrapping_add(1);
        }
    }
}
fn increment_loop(shared: Arc<Mutex<Counter>>) {
    for _ in 0..1000 {
        let guard = shared.lock().unwrap();
        let raw_ptr: *const Counter = &*guard; 
        drop(guard); 
        unsafe {
            let counter_ref = &*raw_ptr; 
            counter_ref.bump();
        }
        thread::yield_now();
    }
}
fn spawn_threads(shared: Arc<Mutex<Counter>>) {
    let mut handles = Vec::new();
    for _ in 0..4 {
        let s = Arc::clone(&shared);
        handles.push(thread::spawn(move || {
            increment_loop(s);
        }));
    }
    for h in handles {
        h.join().unwrap();
    }
}
fn main() {
    let shared = Arc::new(Mutex::new(Counter { value: 0 }));
    spawn_threads(Arc::clone(&shared));
    let final_value = {
        let guard = shared.lock().unwrap();
        guard.value
    };
    println!("Final value: {}", final_value);
}
