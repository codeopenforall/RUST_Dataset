use std::ptr;

struct DataHolder {
    ptr: *mut i32,
}

impl DataHolder {
    fn new(value: i32) -> Self {
        let boxed = Box::new(value);
        Self { ptr: Box::into_raw(boxed) }
    }

    // Releases the allocated memory without invalidating (nulling) the raw pointer.
    unsafe fn release(&mut self) {
        // Free the memory via re-boxing but do not update self.ptr.
        let _ = Box::from_raw(self.ptr);
        // NOTE: self.ptr remains dangling.
    }

    // Dereferences the raw pointer.
    unsafe fn read(&self) -> i32 {
        *self.ptr
    }
}

pub fn compute_value() -> i32 {
    let mut holder = DataHolder::new(123);
    unsafe {
        // Free the memory.
        holder.release();
        // Simulate memory reuse by allocating a new value.
        let _dummy = Box::new(456);
        // Use-after-free: Dereferencing the dangling pointer.
        holder.read()
    }
}

fn main() {
    let res = compute_value();
    println!("Computed result: {}", res);
}