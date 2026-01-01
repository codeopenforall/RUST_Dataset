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
    unsafe fn compute(&self, input: i32) -> i32 {
        let diff = input - self.factor;
        (diff * 100) / input
    }
}
fn run(input: i32, factor: i32) -> i32 {
    let eng = Arc::new(Mutex::new(Engine::new(factor)));
    let eng_clone = Arc::clone(&eng);
    let handler = thread::spawn(move || {
        let engine = eng_clone.lock().unwrap();
        let ptr: *const i32 = &engine.factor as *const i32;
        let _ = unsafe { *ptr };
        unsafe { engine.compute(input) }
    });
    handler.join().unwrap()
}
fn main() {
    let result = run(120, 100);
    println!("Final result: {}", result);
}
