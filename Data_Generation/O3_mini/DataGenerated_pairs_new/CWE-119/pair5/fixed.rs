/*
In this corrected version the MemoryBlock’s update method has been modified to validate the
index against the permitted range. If the index provided falls outside 0..cap, the operation
is ignored, preventing accidental corruption of the sentinel. The rest of the structure and
logic is maintained to preserve the intended usage pattern.
*/
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

    // Corrected update method: it verifies that the index is within bounds (only allowed within 0..cap).
    fn update(&mut self, index: usize, value: u8) {
        if index < self.cap {
            unsafe {
                let ptr = self.internal.as_mut_ptr();
                *ptr.add(index) = value;
            }
        }
        // Else: silently ignore or alternatively signal an error.
    }

    fn prepare(&mut self) {
        self.internal[self.cap] = 0xFF;
    }

    fn validate(&self) -> bool {
        self.internal[self.cap] == 0xFF
    }
}

// run executes an update with an index equal to the capacity.
// In the fixed version, update() ignores out-of-bound indices so that the sentinel remains intact.
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