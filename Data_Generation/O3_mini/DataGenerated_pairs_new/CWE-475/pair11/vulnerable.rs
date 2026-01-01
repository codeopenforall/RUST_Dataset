////////////////////////////////////////////////////////////////////////////////
// Vulnerable code: This code unsafely writes to an allocated buffer without 
// checking that the index is in bounds. It uses unsafe pointer arithmetic 
// that violates API preconditions and may lead to undefined behavior.
////////////////////////////////////////////////////////////////////////////////
#![allow(unused)]
use std::sync::Arc;
use std::thread;

struct Data {
    buf: *mut u32,
    len: usize,
}

impl Data {
    fn new(size: usize) -> Self {
        // Allocate a boxed slice and leak it to obtain a raw pointer.
        let mut v = vec![0u32; size];
        let boxed = v.into_boxed_slice();
        let ptr = Box::into_raw(boxed) as *mut u32;
        Data { buf: ptr, len: size }
    }
    // Unsafe write: does not check that index < len.
    fn write(&self, index: usize, value: u32) {
        unsafe {
            // Vulnerability: no bounds check, index may be invalid.
            *self.buf.add(index) = value;
        }
    }
    fn sum(&self) -> u32 {
        let mut s = 0;
        unsafe {
            for i in 0..self.len {
                s += *self.buf.add(i);
            }
        }
        s
    }
    fn free(&self) {
        unsafe {
            // Recreate the slice from the raw pointer to free memory.
            let slice = std::slice::from_raw_parts_mut(self.buf, self.len);
            let _ = Box::from_raw(slice);
        }
    }
}

// The harness deliberately uses an index that is out of bounds.
pub fn run_harness() -> Result<u32, &'static str> {
    let data = Data::new(4);
    // This call violates the precondition: valid index must be within 0..4.
    data.write(4, 42);
    let s = data.sum();
    data.free();
    Ok(s)
}

fn main() {
    let res = run_harness();
    match res {
        Ok(sum) => println!("Sum: {}", sum),
        Err(e) => println!("Error: {}", e),
    }
}