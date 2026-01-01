use std::thread;
use std::sync::Arc;
struct Engine;
impl Engine {
    fn compute(&self, a: i32, b: i32) -> Option<i32> {
        a.checked_mul(b)  
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
    println!("Starting fixed computation...");
    run_calculation();
}
