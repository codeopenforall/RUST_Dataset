/* 
   CWE-190 Example: An unsafe unchecked arithmetic multiplication
   that can wrap-around due to integer overflow.
*/
use std::sync::Arc;
use std::thread;

trait Multiply {
    fn multiply(&self, a: u32, b: u32) -> Result<u32, &'static str>;
}

struct Processor;

impl Multiply for Processor {
    fn multiply(&self, a: u32, b: u32) -> Result<u32, &'static str> {
        // UNSAFE: The conversion from u64 to u32 is unchecked,
        // allowing an overflow to wrap-around.
        unsafe {
            let result = (a as u64 * b as u64) as u32; // Vulnerable operation (lines 13-15)
            Ok(result)
        }
    }
}

fn main() {
    let processor = Arc::new(Processor);
    let handle = {
        let proc_clone = Arc::clone(&processor);
        thread::spawn(move || {
            // Dangerous input known to trigger overflow for u32.
            let a = std::u32::MAX;
            let b = 2;
            proc_clone.multiply(a, b)
        })
    };
    match handle.join().unwrap() {
        Ok(val) => println!("Result: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_overflow() {
        let proc = Processor;
        let a = std::u32::MAX;
        let b = 2;
        let result = proc.multiply(a, b);
        // The vulnerable implementation does not detect overflow,
        // so it returns a wrapped value rather than an error.
        assert!(result.is_err(), "Overflow was not detected: {:?}", result);
    }
}