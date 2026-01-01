use std::alloc::{alloc, dealloc, Layout};

struct DataProcessor;

impl DataProcessor {
    pub fn execute(&self, requested: usize) -> Result<u8, &'static str> {
        // Allocate a source buffer with fixed size of 10 bytes.
        let source = vec![42u8; 10];
        
        // Prepare to allocate a destination buffer with the user-requested size.
        let layout = Layout::from_size_align(requested, 1).unwrap();
        let dest = unsafe { alloc(layout) };
        if dest.is_null() {
            return Err("Allocation failed");
        }
        
        // FLAW: The API precondition for std::ptr::copy_nonoverlapping is violated.
        // The code attempts to copy 'requested' bytes from a source buffer that has only 10 bytes.
        unsafe {
            std::ptr::copy_nonoverlapping(source.as_ptr(), dest, requested);
        }
        
        let value = unsafe { *dest };
        unsafe { dealloc(dest, layout) };
        Ok(value)
    }
}

pub fn process_input(requested: usize) -> Result<u8, &'static str> {
    let processor = DataProcessor;
    processor.execute(requested)
}

fn main() {
    // In this run, the chosen input (20) exceeds the source buffer size (10)
    // and will trigger undefined behavior.
    match process_input(20) {
        Ok(val) => println!("Processed value: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}