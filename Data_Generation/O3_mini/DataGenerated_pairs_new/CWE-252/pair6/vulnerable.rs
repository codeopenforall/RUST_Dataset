//////////////////////////////////////////////
// Vulnerable Code (Unchecked Return Value)
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
    // Returns an error if the provided data does not fit.
    unsafe fn copy_data(&self, data: &[u8]) -> Result<(), &'static str> {
        let mut guard = self.buffer.lock().unwrap();
        if data.len() > guard.len() {
            return Err("Buffer too small");
        }
        std::ptr::copy_nonoverlapping(data.as_ptr(), guard.as_mut_ptr(), data.len());
        Ok(())
    }

    // This routine spawns a thread to perform the unsafe copy.
    // It discards the Result from the unsafe operation instead of checking it.
    // The expected outcome is that the buffer should exactly match the source data.
    pub fn run_task() -> bool {
        let instance = DataBuffer::new(10); // allocated capacity is too small
        let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]; // 11 bytes of data
        let expected = {
            // expected vector of 11 bytes.
            let mut tmp = Vec::with_capacity(data.len());
            tmp.extend_from_slice(&data);
            tmp
        };

        // Spawn a thread to call the unsafe copy; the returned error is ignored.
        let inst_clone = DataBuffer {
            buffer: instance.buffer.clone(),
        };
        let dclone = data.clone();
        let handle = thread::spawn(move || {
            unsafe {
                // VULNERABILITY: Ignoring the error result.
                let _ = inst_clone.copy_data(&dclone);
            }
        });
        handle.join().unwrap();

        // Check whether the buffer content matches expected data.
        // Since the error was ignored the copy never happened; the buffer remains with its initial size.
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