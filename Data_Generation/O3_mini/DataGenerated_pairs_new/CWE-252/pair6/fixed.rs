//////////////////////////////////////////////
// Corrected Code (Proper Error Handling)
//////////////////////////////////////////////
use std::sync::{Arc, Mutex};
use std::thread;

struct DataBuffer {
    buffer: Arc<Mutex<Vec<u8>>>,
}

impl DataBuffer {
    fn new(capacity: usize) -> Self {
        DataBuffer {
            buffer: Arc::new(Mutex::new(vec![0; capacity])),
        }
    }

    // Unsafe routine that attempts to copy data into the buffer.
    // Returns an error if the provided data does not fit in the current buffer.
    unsafe fn copy_data(&self, data: &[u8]) -> Result<(), &'static str> {
        let mut guard = self.buffer.lock().unwrap();
        if data.len() > guard.len() {
            return Err("Buffer too small");
        }
        std::ptr::copy_nonoverlapping(data.as_ptr(), guard.as_mut_ptr(), data.len());
        Ok(())
    }

    // In this revised routine, the error from the unsafe copy is inspected.
    // If the buffer is too small, it is resized to accommodate the data, and the copy is retried.
    pub fn run_task() -> bool {
        let instance = DataBuffer::new(10); // initially too small
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]; // 11 bytes of data
        let expected = data.clone();

        // Spawn a thread to perform the copy with proper error checking.
        let inst_clone = DataBuffer {
            buffer: instance.buffer.clone(),
        };
        let dclone = data.clone();
        let handle = thread::spawn(move || {
            unsafe {
                // Check the result of the unsafe operation.
                match inst_clone.copy_data(&dclone) {
                    Ok(()) => {}
                    Err(e) if e == "Buffer too small" => {
                        // On error, resize the buffer to the required length.
                        let mut guard = inst_clone.buffer.lock().unwrap();
                        guard.resize(dclone.len(), 0);
                        // Retry the copy operation after resizing.
                        std::ptr::copy_nonoverlapping(dclone.as_ptr(), guard.as_mut_ptr(), dclone.len());
                    }
                    _ => {}
                }
            }
        });
        handle.join().unwrap();

        // Validate that the buffer now exactly matches the expected data.
        let guard = instance.buffer.lock().unwrap();
        if guard.len() != expected.len() {
            return false;
        }
        for (a, b) in guard.iter().zip(expected.iter()) {
            if a != b {
                return false;
            }
        }
        true
    }
}

fn main() {
    let success = DataBuffer::run_task();
    if success {
        println!("Operation succeeded.");
    } else {
        println!("Operation failed.");
    }
}