use std::sync::Arc;
use std::thread;
struct Computation {
    numerator: i32,
    denominator: i32,
}
impl Computation {
    fn execute(&self) -> i32 {
        unsafe {
            let ptr = &self.numerator as *const i32;
            let num = *ptr;   
            num / self.denominator
        }
    }
}
fn main() {
    let instance = Arc::new(Computation { numerator: 100, denominator: 0 });
    let instance_clone = Arc::clone(&instance);
    let handler = thread::spawn(move || {
        instance_clone.execute()
    });
    let result = handler.join().expect("Thread panicked due to divide-by-zero");
    println!("Result: {}", result);
}
