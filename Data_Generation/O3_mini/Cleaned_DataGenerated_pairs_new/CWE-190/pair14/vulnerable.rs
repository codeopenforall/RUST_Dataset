use std::sync::{Arc, Barrier};
use std::thread;
struct Processor {
    factor: u64,
}
impl Processor {
    unsafe fn calculate(&self, input: u64) -> u16 {
        let product = input * self.factor;
        product as u16
    }
}
fn run(input: u64) -> u16 {
    let proc = Arc::new(Processor { factor: 2 });
    let barrier = Arc::new(Barrier::new(4));
    let mut handles = vec![];
    let mut results = vec![];
    for _ in 0..4 {
        let proc_clone = Arc::clone(&proc);
        let barrier_clone = Arc::clone(&barrier);
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            unsafe { proc_clone.calculate(input) }
        });
        handles.push(handle);
    }
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    results.into_iter().sum()
}
fn main() {
    let input = 70000u64;
    let total = run(input);
    println!("Total: {}", total);
}
