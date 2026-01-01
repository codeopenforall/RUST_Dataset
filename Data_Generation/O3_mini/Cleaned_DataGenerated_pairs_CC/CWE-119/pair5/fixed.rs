#![allow(unused)]
use std::boxed::Box;
struct MemoryBlock {
    internal: Box<[u8]>,
    cap: usize,
}
impl MemoryBlock {
    fn new(cap: usize) -> Self {
        let total = cap + 1;
        let data = vec![0u8; total].into_boxed_slice();
        MemoryBlock { internal: data, cap }
    }
    fn update(&mut self, index: usize, value: u8) {
        if index < self.cap {
            unsafe {
                let ptr = self.internal.as_mut_ptr();
                *ptr.add(index) = value;
            }
        }
    }
    fn prepare(&mut self) {
        self.internal[self.cap] = 0xFF;
    }
    fn validate(&self) -> bool {
        self.internal[self.cap] == 0xFF
    }
}
fn run() -> bool {
    let mut mb = MemoryBlock::new(10);
    mb.prepare();
    mb.update(10, 42);
    mb.validate()
}
fn main() {
    let result = run();
    println!("Integrity check: {}", result);
}
