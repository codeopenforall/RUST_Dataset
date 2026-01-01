use std::sync::Arc;
use std::thread;
struct Computation {
    numerator: i32,
    denominator: i32,
}
impl Computation {
    fn execute(&self) -> Result<i32, &'static str> {
        if self.denominator == 0 {
            return Err("Error: Division by zero encountered");
        }
        unsafe {
            let ptr = &self.numerator as *const i32;
            let num = *ptr;   
            Ok(num / self.denominator)
        }
    }
}
fn main() {
    let instance = Arc::new(Computation { numerator: 100, denominator: 0 });
    let instance_clone = Arc::clone(&instance);
    let handler = thread::spawn(move || {
        instance_clone.execute()
    });
    let result = handler.join().expect("Thread panicked unexpectedly");
    match result {
        Ok(val) => println!("Result: {}", val),
        Err(err) => println!("{}", err)
    }
}
