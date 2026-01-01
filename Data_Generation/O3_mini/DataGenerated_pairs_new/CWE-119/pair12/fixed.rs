//////////////// Corrected Code ////////////////////
// The corrected version adds a boundary check before performing the update,
// thus preventing memory corruption by ensuring the index is within the valid range.
use std::sync::{Arc, Mutex};
use std::thread;

#[repr(C)]
pub struct MemoryBlock {
    data: [u8; 10],
    flag: u8,
}

impl MemoryBlock {
    // The updated method checks the index bounds before performing an unsafe update.
    pub fn update(&mut self, index: usize, value: u8) {
        if index < self.data.len() {
            unsafe {
                let ptr = self.data.as_mut_ptr();
                *ptr.add(index) = value;
            }
        } else {
            // Out-of-bound indices are safely ignored (or could trigger a controlled error).
        }
    }
}

fn main() {
    let block = Arc::new(Mutex::new(MemoryBlock { data: [0; 10], flag: 0 }));
    let block_clone = Arc::clone(&block);
    // Spawn a thread that attempts an update with an out-of-bound index.
    let handle = thread::spawn(move || {
        let mut guarded = block_clone.lock().unwrap();
        // The out-of-bound call is now safely ignored.
        guarded.update(10, 255);
    });
    handle.join().unwrap();
    let guarded = block.lock().unwrap();
    // The adjacent field 'flag' remains intact.
    println!("flag value: {}", guarded.flag);
}