use std::sync::{Arc, Mutex};
use std::thread;
struct Record {
    value: u32,
}
impl Record {
    fn new(init: u32) -> Self {
        Record { value: init }
    }
    fn decrease(&mut self, deduction: u32) -> Result<(), &'static str> {
        if deduction > self.value {
            Err("Integer underflow prevented: deduction exceeds current value")
        } else {
            self.value -= deduction;
            Ok(())
        }
    }
}
pub fn run_process() -> u32 {
    let data = Arc::new(Mutex::new(Record::new(50)));
    let mut threads = Vec::new();
    for _ in 0..5 {
        let data_ref = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let deduction = 20u32;
            let mut guard = data_ref.lock().unwrap();
            let _ = guard.decrease(deduction);
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
    let final_value = data.lock().unwrap().value;
    println!("Final value: {}", final_value);
    final_value
}
fn main() {
    let _ = run_process();
}
