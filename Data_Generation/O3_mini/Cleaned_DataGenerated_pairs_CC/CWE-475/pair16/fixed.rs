--------------------------------------------------
#![allow(unused)]
use std::thread;
trait Task {
    fn execute(&self) -> u32;
}
struct Engine;
impl Engine {
    unsafe fn compute_safe(&self) -> u32 {
        let src: Vec<u32> = vec![10, 20, 30];
        let mut dst: Vec<u32> = vec![0; 3];
        std::ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len());
        dst.iter().sum()
    }
}
impl Task for Engine {
    fn execute(&self) -> u32 {
        let handle = thread::spawn(move || unsafe { self.compute_safe() });
        handle.join().unwrap()
    }
}
fn main() {
    let engine = Engine;
    let result = engine.execute();
    println!("Result: {}", result);
}
--------------------------------------------------
