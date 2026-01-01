/////////////////////// Vulnerable Code Start ///////////////////////
#![allow(unused)]
use std::sync::{Arc, Mutex};
use std::thread;

struct Engine {
    factor: i32,
}

impl Engine {
    fn new(f: i32) -> Self {
        Engine { factor: f }
    }

    // This function is intended to compute the percentage gain
    // using the formula: ((input - factor) * 100) / input.
    // However, due to an arithmetic precedence error, it ends up performing:
    // (input - factor) / input * 100, which loses the intended precision.
    // Note: This function uses an unsafe block to simulate processing of raw
    // pointer data, as often observed in real-world unsafe code misuse.
    unsafe fn compute(&self, input: i32) -> i32 {
        let diff = input - self.factor;
        // Vulnerability: Here, diff / input is computed first (integer division),
        // then multiplied by 100. The multiplication should happen before the division.
        diff / input * 100
    }
}

fn run(input: i32, factor: i32) -> i32 {
    let eng = Arc::new(Mutex::new(Engine::new(factor)));
    let eng_clone = Arc::clone(&eng);
    let handler = thread::spawn(move || {
        // Lock the engine instance across threads.
        let engine = eng_clone.lock().unwrap();
        // Introduce an unsafe pointer dereference to mimic low-level operations.
        let ptr: *const i32 = &engine.factor as *const i32;
        let val = unsafe { *ptr };
        // Use the computed value in an unsafe block.
        unsafe { engine.compute(input) }
    });
    handler.join().unwrap()
}

fn main() {
    let result = run(120, 100);
    println!("Final result: {}", result);
}
/////////////////////// Vulnerable Code End ///////////////////////