use std::sync::{Arc, Mutex};
use std::thread;
struct SecureData {
    value: i32,
}
impl SecureData {
    unsafe fn update(&mut self, delta: i32) -> Result<(), &'static str> {
        if self.value + delta < 0 || self.value + delta > 100 {
            return Err("value out of range");
        }
        self.value += delta;
        Ok(())
    }
}
fn process(data: Arc<Mutex<SecureData>>, delta: i32) {
    let mut handles = Vec::new();
    for _ in 0..5 {
        let data_cloned = Arc::clone(&data);
        let h = thread::spawn(move || {
            unsafe {
                let mut guard = data_cloned.lock().unwrap();
                if guard.update(delta).is_err() {
                    return;
                }
            }
        });
        handles.push(h);
    }
    for h in handles {
        h.join().unwrap();
    }
}
pub fn run() -> i32 {
    let initial = SecureData { value: 50 };
    let shared = Arc::new(Mutex::new(initial));
    process(Arc::clone(&shared), 10);
    process(Arc::clone(&shared), 60);
    let final_val = shared.lock().unwrap().value;
    final_val
}
fn main() {
    let final_value = run();
    println!("Final value: {}", final_value);
}
