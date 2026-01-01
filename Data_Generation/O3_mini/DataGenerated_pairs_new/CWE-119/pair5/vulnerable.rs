/*
This Rust code implements a custom memory block that holds a buffer with a “sentinel” byte at the end.
It uses an unsafe method for writing to the buffer without properly enforcing bounds.
A caller may inadvertently write to the sentinel (which is not part of the allowed region)
if an out‐of-range index is provided. The main function calls a helper that deliberately uses
an index equal to the “capacity”, which corrupts the sentinel. This mimics a CWE-119 buffer overflow.
*/
#![allow(unused)]
use std::boxed::Box;

struct MemoryBlock {
    internal: Box<[u8]>,
    cap: usize,
}

impl MemoryBlock {
    fn new(cap: usize) -> Self {
        // Allocate a buffer with cap + 1 bytes:
        // the first cap bytes are for valid data and the last byte is a sentinel.
        let total = cap + 1;
        let data = vec![0u8; total].into_boxed_slice();
        MemoryBlock { internal: data, cap }
    }

    // This method writes to the buffer using an unsafe pointer.
    // It does not enforce a bounds check against cap.
    // If the caller writes at index == cap, it overwrites the sentinel.
    fn update(&mut self, index: usize, value: u8) {
        unsafe {
            let ptr = self.internal.as_mut_ptr();
            // Vulnerability: no bounds check; index may overflow into the sentinel slot.
            *ptr.add(index) = value;
        }
    }

    // Prepare sets the sentinel to a protected value.
    fn prepare(&mut self) {
        self.internal[self.cap] = 0xFF;
    }

    // Validate that the sentinel remains unchanged.
    fn validate(&self) -> bool {
        self.internal[self.cap] == 0xFF
    }
}

// run executes an update with an index equal to the size of the accessible region.
// In this vulnerable implementation, the update will corrupt the sentinel, and validate() will fail.
fn run() -> bool {
    let mut mb = MemoryBlock::new(10);
    mb.prepare();
    // Caller erroneously provides an index equal to the capacity (should be 0..cap-1).
    mb.update(10, 42);
    mb.validate()
}

fn main() {
    let result = run();
    println!("Integrity check: {}", result);
}