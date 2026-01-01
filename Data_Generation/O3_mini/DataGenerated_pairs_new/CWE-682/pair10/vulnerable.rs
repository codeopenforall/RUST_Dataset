////////////////////////////////////////////////////////////////////////////////////////////////////
// Vulnerable Code Sample targeting CWE-682 (Incorrect Calculation due to operator precedence)
// This program spawns a thread using concurrency and employs an unsafe code block with raw pointer
// dereferencing. The calculation logic is intended to compute base * (multiplier + addition),
// but it mistakenly performs (base * multiplier) + addition due to a precedence error.
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
            // Intended formula: base * (multiplier + addition)
            // Bug: incorrect operator precedence leads to: (base * multiplier) + addition
            (*ptr_base).wrapping_mul(*ptr_multiplier).wrapping_add(addition) // Vulnerability here
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