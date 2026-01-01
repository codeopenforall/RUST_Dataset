use std::env;
use std::thread;
use std::panic;
pub struct Engine;
impl Engine {
    pub unsafe fn explore(&self, data: *const u8, len: usize, idx: usize) -> i32 {
        if idx >= len {
            return 0;
        }
        let byte = *data.add(idx);
        if byte == b'(' {
            return 1 + self.explore(data, len, idx + 1);
        } else {
            return 0;
        }
    }
    pub fn journey(&self, input: &[u8]) -> i32 {
        unsafe { self.explore(input.as_ptr(), input.len(), 0) }
    }
}
pub fn evaluate(input: &str) -> Result<i32, &'static str> {
    let engine = Engine;
    let inp = input.as_bytes().to_vec(); 
    let handler = thread::spawn(move || {
        engine.journey(&inp)
    });
    match handler.join() {
        Ok(res) => Ok(res),
        Err(_) => Err("Unexpected recursive overflow occurred"),
    }
}
fn main() {
    let input = env::args().nth(1).unwrap_or_else(|| "(".repeat(10000));
    match evaluate(&input) {
        Ok(res) => println!("Computation successful, result: {}", res),
        Err(err) => println!("Computation failed with error: {}", err),
    }
}
