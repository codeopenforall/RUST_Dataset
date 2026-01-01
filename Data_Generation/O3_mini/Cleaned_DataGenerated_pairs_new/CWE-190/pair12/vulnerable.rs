use std::sync::Arc;
use std::thread;
struct Engine;
trait Operations {
    unsafe fn compute(&self, a: u64, b: u64) -> u32;
}
impl Operations for Engine {
    unsafe fn compute(&self, a: u64, b: u64) -> u32 {
        let product = a * b;
        product as u32
    }
}
fn run() -> u64 {
    let engine = Arc::new(Engine{});
    let a: u64 = 300_000;
    let b: u64 = 30_000; 
    let engine_clone = engine.clone();
    let handler = thread::spawn(move || {
        unsafe { engine_clone.compute(a, b) }
    });
    let result_u32 = handler.join().unwrap();
    result_u32 as u64
}
fn main() {
    let result = run();
    println!("Computed result: {}", result);
}
#[cfg(test)]
mod tests {
    use super::run;
    #[test]
    fn check_result() {
        assert_eq!(run(), 9_000_000_000, "Computation did not yield the expected result");
    }
}
