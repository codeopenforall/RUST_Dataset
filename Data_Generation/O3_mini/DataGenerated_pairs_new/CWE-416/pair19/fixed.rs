struct Memory {
    ptr: *mut i32,
}

impl Memory {
    fn new(val: i32) -> Self {
        // Allocate memory on the heap and store its raw pointer.
        let b = Box::new(val);
        let raw = Box::into_raw(b);
        Memory { ptr: raw }
    }

    // Safe read, performed while the memory is still valid.
    unsafe fn get(&self) -> i32 {
        *self.ptr
    }
}

impl Drop for Memory {
    fn drop(&mut self) {
        // Reconstruct the Box to properly deallocate memory.
        unsafe {
            let _ = Box::from_raw(self.ptr);
        }
    }
}

fn run() -> i32 {
    let mem = Memory::new(256);
    // Read the value while the memory is still valid.
    let value = unsafe { mem.get() };
    drop(mem);
    // Return the safely read value.
    value
}

fn main() {
    println!("Result: {}", run());
}