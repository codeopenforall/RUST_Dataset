use std::sync::Arc;
use std::thread;
struct Processor {
    numerator: i32,
    denominator: i32,
}
impl Processor {
    unsafe fn operation(&self) -> i32 {
        let numer_ptr: *const i32 = &self.numerator;
        let denom_ptr: *const i32 = &self.denominator;
        *numer_ptr / *denom_ptr
    }
}
pub fn compute(n1: i32, n2: i32) -> Result<i32, &'static str> {
    let proc = Processor {
        numerator: n1,
        denominator: n2,
    };
    let shared = Arc::new(proc);
    let shared_clone = Arc::clone(&shared);
    let handle = thread::spawn(move || {
        unsafe { shared_clone.operation() }
    });
    match handle.join() {
        Ok(res) => Ok(res),
        Err(_) => Err("Unhandled division by zero"),
    }
}
fn main() {
    let arg = std::env::args().nth(1).unwrap_or_else(|| "0".to_string());
    let denom = arg.parse::<i32>().unwrap_or(0);
    match compute(100, denom) {
        Ok(result) => println!("Result: {}", result),
        Err(err) => println!("Error: {}", err),
    }
}
