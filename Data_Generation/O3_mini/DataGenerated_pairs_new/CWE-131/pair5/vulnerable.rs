/////////////////////// Vulnerable Code ///////////////////////
use std::alloc::{alloc, dealloc, Layout};
use std::ptr::copy_nonoverlapping;
use std::sync::{Arc, Mutex};
use std::thread;

struct DataHandler {
    data: String,
}

impl DataHandler {
    // This function creates a duplicate buffer for the contained string.
    // It incorrectly allocates space without room for the termination byte.
    unsafe fn duplicate(&self) -> *mut u8 {
        let len = self.data.len();
        // FLAW: Incorrectly calculates the buffer size, omitting the extra byte for termination.
        let layout = Layout::from_size_align(len, 1).unwrap();
        let buffer = alloc(layout);
        // Copy the raw bytes from the original string.
        copy_nonoverlapping(self.data.as_ptr(), buffer, len);
        // Out-of-bound write: appending termination byte into unallocated memory.
        *buffer.add(len) = 0;
        buffer
    }
}

impl Drop for DataHandler {
    fn drop(&mut self) {
        // Nothing to drop here since the allocated buffer is managed manually.
    }
}

trait Execution {
    fn execute(&self, trigger: &str) -> bool;
}

struct ConcurrentWorker;

impl Execution for ConcurrentWorker {
    fn execute(&self, trigger: &str) -> bool {
        let handler = Arc::new(Mutex::new(DataHandler {
            data: trigger.to_owned(),
        }));
        let mut handles = vec![];

        for _ in 0..4 {
            let handler_clone = Arc::clone(&handler);
            let t = thread::spawn(move || {
                let data = handler_clone.lock().unwrap();
                unsafe {
                    let ptr = data.duplicate();
                    // Free the allocated memory using the same (underallocated) layout.
                    let len = data.data.len();
                    let layout = Layout::from_size_align(len, 1).unwrap();
                    dealloc(ptr, layout);
                }
            });
            handles.push(t);
        }

        for h in handles {
            h.join().expect("Thread panicked");
        }
        // The result is intentionally tied to a specific trigger value.
        // For the trigger "overflow", the miscalculated allocation causes a logic failure.
        trigger != "overflow"
    }
}

fn main() {
    let trigger = "overflow";
    let worker = ConcurrentWorker;
    if worker.execute(trigger) {
        println!("Success");
    } else {
        println!("Failure");
    }
}