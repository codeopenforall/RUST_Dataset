//////////////// Vulnerable Code ////////////////////
// This code intentionally performs an out‐of‐bound write using an unsafe pointer operation.
// The struct is defined with a fixed-size array immediately followed by an adjacent field,
// so writing past the array boundary corrupts the adjacent field.
use std::sync::{Arc, Mutex};
use std::thread;

#[repr(C)]
pub struct MemoryBlock {
    data: [u8; 10],
    flag: u8,
}

impl MemoryBlock {
    // This method unsafely writes to the internal array without checking that the index is valid.
    // When an index equal to the length of `data` (i.e. 10) is provided, it overwrites memory allocated for `flag`.
    pub unsafe fn update(&mut self, index: usize, value: u8) {
        let ptr = self.data.as_mut_ptr();
        // Vulnerability: No bounds check is performed. With index 10 the write corrupts the adjacent field.
        *ptr.add(index) = value;
    }
}

fn main() {
    let block = Arc::new(Mutex::new(MemoryBlock { data: [0; 10], flag: 0 }));
    let block_clone = Arc::clone(&block);
    // Spawn a thread that performs an unsafe update with an out-of-bound index.
    let handle = thread::spawn(move || {
        let mut guarded = block_clone.lock().unwrap();
        unsafe {
            // Trigger the issue by writing to index 10 (out-of-bound), which should update the `flag`
            guarded.update(10, 255);
        }
    });
    handle.join().unwrap();
    let guarded = block.lock().unwrap();
    // For a correct run we'd expect flag to remain 0; here it is likely corrupted.
    println!("flag value: {}", guarded.flag);
}