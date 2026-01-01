use std::ptr;

struct Memory {
    ptr: *mut i32,
}

impl Memory {
    fn new(val: i32) -> Self {
        // Allocate memory on the heap and keep its raw pointer.
        let b = Box::new(val);
        let raw = Box::into_raw(b);
        Memory { ptr: raw }
    }

    // Direct, unsafe access to the memory.
    unsafe fn get(&self) -> i32 {
        *self.ptr
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        // Reconstruct the Box to properly free the memory.
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}

fn run() -> i32 {
    let mem = Memory::new(256);
    let raw = mem.ptr;
    // Free the allocated memory.
    drop(mem);
    // Vulnerability: Using the pointer after the memory has been freed.
    // This unsafe block writes to the freed memory, corrupting its original value.
    unsafe {
        *raw = 9999;
        *raw
    }
}

fn main() {
    println!("Result: {}", run());
}