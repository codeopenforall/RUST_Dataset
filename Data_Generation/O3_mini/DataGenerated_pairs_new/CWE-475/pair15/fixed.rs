use std::alloc::{alloc, dealloc, Layout};

struct DataHandler;

impl DataHandler {
    pub fn run(&self, requested: usize) -> Result<u8, &'static str> {
        // Allocate a source buffer with fixed size of 10 bytes.
        let source = vec![42u8; 10];
        
        // FIX: Enforce the API precondition by checking that the requested copy length 
        // does not exceed the source buffer length.
        if requested > source.len() {
            return Err("Requested size exceeds source buffer size");
        }
        
        let layout = Layout::from_size_align(requested, 1).unwrap();
        let dest = unsafe { alloc(layout) };
        if dest.is_null() {
            return Err("Allocation failed");
        }
        
        unsafe {
            std::ptr::copy_nonoverlapping(source.as_ptr(), dest, requested);
        }
        
        let value = unsafe { *dest };
        unsafe { dealloc(dest, layout) };
        Ok(value)
    }
}

pub fn process_input(requested: usize) -> Result<u8, &'static str> {
    let handler = DataHandler;
    handler.run(requested)
}

fn main() {
    // Here the input is chosen to be valid. Using 10 (which is equal to the source length)
    // satisfies the API precondition and avoids undefined behavior.
    match process_input(10) {
        Ok(val) => println!("Handled value: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}