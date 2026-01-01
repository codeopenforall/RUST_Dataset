struct DataHolder {
    ptr: Option<*mut i32>, // Use Option to track pointer validity.
}

impl DataHolder {
    fn new(value: i32) -> Self {
        let boxed = Box::new(value);
        Self { ptr: Some(Box::into_raw(boxed)) }
    }

    // Safely extracts the value and releases the memory.
    unsafe fn release(&mut self) -> i32 {
        if let Some(raw) = self.ptr.take() {
            // Retrieve the value before deallocation.
            let value = *raw;
            let _ = Box::from_raw(raw);
            value
        } else {
            panic!("Memory already released");
        }
    }
}

pub fn compute_value() -> i32 {
    let mut holder = DataHolder::new(123);
    // Retrieve value from valid memory, then free it.
    unsafe {
        // Read and release safely, avoiding any dangling pointer usage.
        holder.release()
    }
}

fn main() {
    let res = compute_value();
    println!("Computed result: {}", res);
}