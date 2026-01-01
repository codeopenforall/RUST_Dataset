//////////////////////////
// Vulnerable Code File //
//////////////////////////
use std::sync::Arc;
use std::thread;

struct Processor {
    data: Vec<u8>,
}

impl Processor {
    fn new(data: Vec<u8>) -> Self {
        Processor { data }
    }

    // This method unsafely reads a value from a buffer using unchecked pointer arithmetic.
    // If index is equal to or greater than the length of the buffer, it will read out‐of‐bounds.
    fn extract(&self, index: usize) -> Option<u8> {
        unsafe {
            let ptr = self.data.as_ptr();
            // Vulnerability: No bounds checking is performed.
            Some(*ptr.add(index))
        }
    }
}

fn run_task(proc: Arc<Processor>, idx: usize) -> Option<u8> {
    // In a concurrent context, spawn a thread to perform the extraction.
    proc.extract(idx)
}

fn main() {
    let data = vec![10, 20, 30]; // Buffer length = 3
    let proc = Arc::new(Processor::new(data));
    let index = 3; // Out-of-bounds index since valid indices are 0, 1, and 2.
    
    let proc_clone = Arc::clone(&proc);
    let handle = thread::spawn(move || run_task(proc_clone, index));
    let result = handle.join().unwrap();
    
    // The extracted value is printed even though the pointer read is out-of-bounds.
    println!("Extracted value: {:?}", result);
}