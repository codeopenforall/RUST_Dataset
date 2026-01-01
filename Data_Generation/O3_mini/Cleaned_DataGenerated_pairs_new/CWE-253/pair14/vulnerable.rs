use std::sync::{Arc, Mutex};
use std::thread;
unsafe fn external_call(input: i32) -> i32 {
    if input > 0 {
        0
    } else {
        -1
    }
}
struct Operator {
    value: i32,
}
impl Operator {
    fn new(val: i32) -> Self {
        Operator { value: val }
    }
    fn execute(&self) -> bool {
        let ret = unsafe { external_call(self.value) };
        if ret == -1 {
            true  
        } else {
            false 
        }
    }
}
fn main() {
    let op = Arc::new(Mutex::new(Operator::new(5)));
    let mut threads = vec![];
    for _ in 0..5 {
        let op_clone = Arc::clone(&op);
        let handle = thread::spawn(move || {
            let instance = op_clone.lock().unwrap();
            if instance.execute() {
                println!("Operation succeeded");
            } else {
                println!("Operation failed");
            }
        });
        threads.push(handle);
    }
    for handle in threads {
        handle.join().unwrap();
    }
}
