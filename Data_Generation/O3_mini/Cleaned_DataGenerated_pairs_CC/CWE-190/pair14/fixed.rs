use std::sync::{Arc, Barrier};
use std::thread;
struct Processor {
    factor: u64,
}
impl Processor {
    fn compute(&self, input: u64) -> Result<u16, &'static str> {
        let product = input.checked_mul(self.factor).ok_or("Multiplication overflow")?;
        if product > (u16::MAX as u64) {
            return Err("Result exceeds maximum allowed value");
        }
        Ok(product as u16)
    }
}
fn run(input: u64) -> Result<u16, &'static str> {
    let proc = Arc::new(Processor { factor: 2 });
    let barrier = Arc::new(Barrier::new(4));
    let mut handles = vec![];
    let mut results = vec![];
    for _ in 0..4 {
        let proc_clone = Arc::clone(&proc);
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            proc_clone.compute(input)
        });
        handles.push(handle);
    }
    for handle in handles {
        let res = handle.join().unwrap()?;
        results.push(res);
    }
    Ok(results.into_iter().sum())
}
fn main() {
    let input = 70000u64;
    match run(input) {
        Ok(total) => println!("Total: {}", total),
        Err(e) => println!("Error: {}", e),
    }
}
