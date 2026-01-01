#![allow(unused)]
use std::sync::Arc;
use std::thread;
struct Operator {
    multiplier: u32,
}
impl Operator {
    unsafe fn compute(&self, input: u32) -> u64 {
        let product = input.wrapping_mul(self.multiplier);
        product as u64
    }
}
fn main() {
    let op = Arc::new(Operator { multiplier: 100_000 });
    let test_input: u32 = 50_000; 
    let op_clone = Arc::clone(&op);
    let handle = thread::spawn(move || {
        unsafe { op_clone.compute(test_input) }
    });
    let result = handle.join().unwrap();
    println!("Computed result: {}", result);
    if result != 5_000_000_000 {
        std::process::exit(1);
    }
    std::process::exit(0);
}
