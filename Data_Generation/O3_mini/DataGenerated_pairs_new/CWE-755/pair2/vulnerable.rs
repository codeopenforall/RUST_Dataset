////////////////// Vulnerable Code //////////////////
use std::ptr;

struct Data {
    ptr: *mut u8,
    size: usize,
}

impl Data {
    fn new(size: usize) -> Result<Self, &'static str> {
        if size == 0 {
            return Err("Zero size");
        }
        // Create a vector with uninitialized memory.
        let mut vec = Vec::with_capacity(size);
        unsafe { vec.set_len(size); }
        let ptr = vec.as_mut_ptr();
        // Leak the vector to transfer ownership of the buffer.
        std::mem::forget(vec);
        Ok(Data { ptr, size })
    }

    fn write(&self, index: usize, val: u8) -> Result<(), &'static str> {
        if index >= self.size {
            return Err("Index out of bounds");
        }
        // Use unsafe pointer arithmetic.
        unsafe {
            *self.ptr.add(index) = val;
        }
        Ok(())
    }
}

impl Default for Data {
    fn default() -> Self {
        // Return a default “safe” state that is actually a null pointer and zero size.
        Data { ptr: ptr::null_mut(), size: 0 }
    }
}

fn run(input: usize) -> Result<u8, &'static str> {
    // Vulnerability:
    // Errors from Data::new are silently swallowed via unwrap_or_default,
    // resulting in a default (invalid) state if input == 0.
    let d = Data::new(input).unwrap_or_default();
    d.write(0, 42)?;
    Ok(42)
}

fn main() {
    // Normal operation: use a valid non-zero input.
    match run(10) {
        Ok(val) => println!("Operation result: {}", val),
        Err(e) => println!("Operation error: {}", e),
    }
}