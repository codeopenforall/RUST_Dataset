struct Data {
    ptr: *mut i32,
}

impl Data {
    fn new(val: i32) -> Self {
        let b = Box::new(val);
        Self { ptr: Box::into_raw(b) }
    }
}

impl Clone for Data {
    fn clone(&self) -> Self {
        // Deep cloning: allocate new memory and copy the inner value.
        unsafe {
            let value = *self.ptr;
            Data::new(value)
        }
    }
}

impl Drop for Data {
    fn drop(&mut self) {
        unsafe {
            // Safely reclaim the allocated memory.
            let _ = Box::from_raw(self.ptr);
        }
    }
}

fn execute() {
    let original = Data::new(42);
    // Cloning now produces a completely separate allocation.
    let duplicate = original.clone();
    // Use the allocated values to ensure they are valid.
    let sum = unsafe { *original.ptr + *duplicate.ptr };
    println!("Sum: {}", sum);
}

fn main() {
    execute();
}