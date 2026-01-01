////////////////////////////////////////////////////////////////////////////////////////////////////
// Corrected Code Sample addressing CWE-682 Correct Calculation Issue
// This version fixes the operator precedence bug by correctly grouping the addition with the multiplier, 
// so that the intended computation, base * (multiplier + addition), is performed. The code still uses unsafe 
// code, concurrency, and smart pointers to emulate real-world complex scenarios.
use std::sync::Arc;
use std::thread;

struct Engine {
    base: u32,
    multiplier: u32,
}

impl Engine {
    fn new(b: u32, m: u32) -> Self {
        Engine { base: b, multiplier: m }
    }
    
    fn calculate(&self, addition: u32) -> u32 {
        unsafe {
            let ptr_base = &self.base as *const u32;
            let ptr_multiplier = &self.multiplier as *const u32;
            // Correct formula: base * (multiplier + addition)
            (*ptr_base).wrapping_mul((*ptr_multiplier).wrapping_add(addition))
        }
    }
}

fn main() {
    let engine = Arc::new(Engine::new(10, 20));
    let eng_clone = Arc::clone(&engine);
    let handle = thread::spawn(move || {
        eng_clone.calculate(5)
    });
    let result = handle.join().unwrap();
    println!("Result: {}", result);
}