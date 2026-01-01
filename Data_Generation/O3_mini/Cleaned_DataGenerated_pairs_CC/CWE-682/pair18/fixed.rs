use std::sync::Arc;
use std::thread;
struct Engine {
    base: i32,
    multiplier: i32,
    divisor: i32,
}
impl Engine {
    fn process(&self) -> i32 {
        (self.base * self.multiplier) / self.divisor
    }
}
fn main() {
    let engine = Arc::new(Engine { base: 100, multiplier: 3, divisor: 12 });
    let mut handles = vec![];
    for _ in 0..4 {
        let eng = Arc::clone(&engine);
        handles.push(thread::spawn(move || eng.process()));
    }
    let mut total = 0;
    for h in handles {
        total += h.join().expect("Thread panicked");
    }
    println!("Result: {}", total);
}
