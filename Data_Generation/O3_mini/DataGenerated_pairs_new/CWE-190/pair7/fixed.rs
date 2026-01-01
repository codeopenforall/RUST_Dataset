/* 
   CWE-190 Fixed Example: Using checked arithmetic to prevent overflow.
   The multiplication is now performed with explicit overflow checking.
*/
use std::sync::Arc;
use std::thread;

trait Multiply {
    fn multiply(&self, a: u32, b: u32) -> Result<u32, &'static str>;
}

struct Processor;

impl Multiply for Processor {
    fn multiply(&self, a: u32, b: u32) -> Result<u32, &'static str> {
        // Use checked multiplication to safely detect overflow.
        a.checked_mul(b).ok_or("Overflow detected")
    }
}

fn main() {
    let processor = Arc::new(Processor);
    let handle = {
        let proc_clone = Arc::clone(&processor);
        thread::spawn(move || {
            // Dangerous input: multiplication would overflow u32.
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
        // The corrected code properly returns an error when overflow occurs.
        assert!(result.is_err(), "Overflow was not detected safely");
    }
}