#![allow(dead_code)]
use std::thread;
trait Calculation {
    fn evaluate(&self, numerator: i32, denominator: i32) -> i32;
}
struct Processor;
impl Calculation for Processor {
    fn evaluate(&self, numerator: i32, denominator: i32) -> i32 {
        if denominator == 0 {
            0
        } else {
            unsafe {
                let ptr = &denominator as *const i32;
                let div = *ptr;
                numerator / div
            }
        }
    }
}
fn execute(numerator: i32, denominator: i32) -> i32 {
    let processor = Processor;
    let handle = thread::spawn(move || {
        processor.evaluate(numerator, denominator)
    });
    handle.join().unwrap()
}
pub fn calculate(numerator: i32, denominator: i32) -> i32 {
    execute(numerator, denominator)
}
fn main() {
    let result = execute(10, 0);
    println!("Calculation result: {}", result);
}
