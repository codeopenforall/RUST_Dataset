#![allow(dead_code)]
use std::thread;
use std::sync::Arc;
struct Engine;
impl Engine {
    fn compute(&self, a: i32, b: i32) -> Option<i32> {
        unsafe {
            let res = std::intrinsics::unchecked_mul(a, b);  
            Some(res)
        }
    }
}
fn run_calculation() {
    let engine = Arc::new(Engine);
    let handles: Vec<_> = (0..4).map(|_| {
        let calc = Arc::clone(&engine);
        thread::spawn(move || {
            calc.compute(50000, 50000)
        })
    }).collect();
    for handle in handles {
        let result = handle.join().unwrap();
        println!("Computed value: {:?}", result);
    }
}
fn main() {
    println!("Starting vulnerable computation...");
    run_calculation();
}
