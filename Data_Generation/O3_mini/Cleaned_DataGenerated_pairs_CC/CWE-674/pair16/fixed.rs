use std::env;
use std::ptr;
const MAX_DEPTH: usize = 1000;
struct Processor;
impl Processor {
    unsafe fn traverse(&self, data: *const u8, len: usize, current: usize) -> Result<usize, String> {
        if current >= MAX_DEPTH {
            return Err("Recursion limit reached".to_string());
        }
        if len == 0 {
            return Ok(0);
        }
        let first = ptr::read(data);
        if first == b'(' {
            let inner_depth = self.traverse(data.add(1), len - 1, current + 1)?;
            if len >= inner_depth + 2 && ptr::read(data.add(inner_depth + 1)) == b')' {
                Ok(inner_depth + 1)
            } else {
                Err("Missing closing parenthesis".to_string())
            }
        } else {
            Err("Unexpected token encountered".to_string())
        }
    }
    pub fn process(&self, input: &str) -> Result<usize, String> {
        unsafe { self.traverse(input.as_ptr(), input.len(), 0) }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let input = if args.len() > 1 { &args[1] } else { "(())" };
    let proc = Processor;
    match proc.process(input) {
        Ok(depth) => println!("Computed depth: {}", depth),
        Err(e) => eprintln!("Error: {}", e),
    }
}
