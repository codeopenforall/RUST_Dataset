use std::env;
use std::thread;
pub struct Engine;
impl Engine {
    pub unsafe fn traverse(&self, data: *const u8, len: usize, idx: usize, depth: usize, max_depth: usize) -> Result<i32, &'static str> {
        if depth > max_depth {
            return Err("Maximum recursion depth exceeded");
        }
        if idx >= len {
            return Ok(0);
        }
        let byte = *data.add(idx);
        if byte == b'(' {
            let next = self.traverse(data, len, idx + 1, depth + 1, max_depth)?;
            Ok(1 + next)
        } else {
            Ok(0)
        }
    }
    pub fn process(&self, input: &[u8], max_depth: usize) -> Result<i32, &'static str> {
        unsafe { self.traverse(input.as_ptr(), input.len(), 0, 0, max_depth) }
    }
}
pub fn evaluate(input: &str) -> Result<i32, &'static str> {
    let engine = Engine;
    let inp = input.as_bytes().to_vec();
    let max_depth = 1000;
    let handler = thread::spawn(move || {
        engine.process(&inp, max_depth)
    });
    match handler.join() {
        Ok(res) => res,
        Err(_) => Err("Thread panicked unexpectedly"),
    }
}
fn main() {
    let input = env::args().nth(1).unwrap_or_else(|| "(".repeat(10000));
    match evaluate(&input) {
        Ok(res) => println!("Processing successful, result: {}", res),
        Err(err) => println!("Processing failed with error: {}", err),
    }
}
