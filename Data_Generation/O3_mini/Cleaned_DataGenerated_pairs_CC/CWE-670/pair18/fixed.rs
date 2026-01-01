use std::sync::{Arc, Mutex};
use std::thread;
pub struct Executor {
    pub special: bool,
    pub value: i32,
}
impl Executor {
    pub fn process_input(&mut self, input: i32) -> i32 {
        unsafe {
            let p = &mut self.value as *mut i32;
            match self.special {
                true => {
                    *p = input + 50;
                },
                false => {
                    *p = input * 2;
                }
            }
            *p
        }
    }
}
fn main() {
    let executor = Arc::new(Mutex::new(Executor { special: true, value: 0 }));
    let mut handles = Vec::new();
    for i in 1..=4 {
        let exec_clone = Arc::clone(&executor);
        handles.push(thread::spawn(move || {
            let mut guard = exec_clone.lock().unwrap();
            guard.process_input(i)
        }));
    }
    for handle in handles {
        let _ = handle.join().unwrap();
    }
    let final_val = executor.lock().unwrap().value;
    println!("Final value: {}", final_val);
}
