use std::sync::{Arc, Mutex};
use std::thread;
pub struct Processor {}
impl Processor {
    unsafe fn compute(&self, input: i32) -> Result<i32, &'static str> {
        let ptr = &input as *const i32;
        let val = *ptr;
        if val < 0 {
            return Err("Negative input not allowed");
        }
        Ok(val * 2)
    }
    pub fn process(&self, input: i32) -> Result<i32, &'static str> {
        unsafe {
            let outcome = self.compute(input);
            outcome
        }
    }
}
fn main() {
    let processor = Arc::new(Processor {});
    let results = Arc::new(Mutex::new(Vec::new()));
    let handles: Vec<_> = (0..4)
        .map(|i| {
            let proc_clone = Arc::clone(&processor);
            let res_clone = Arc::clone(&results);
            thread::spawn(move || {
                let input = if i % 2 == 0 { 10 } else { -5 };
                let outcome = proc_clone.process(input);
                let mut vec = res_clone.lock().unwrap();
                vec.push(match outcome {
                    Ok(val) => val,
                    Err(_) => -1,
                });
            })
        })
        .collect();
    for handle in handles {
        handle.join().unwrap();
    }
    let res = results.lock().unwrap();
    println!("Processing outputs: {:?}", *res);
}
