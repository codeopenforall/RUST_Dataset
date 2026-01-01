//////////////////////////////////////////////////////////////////////////////////////////////////////
// The revised program corrects the double free error by ensuring unique ownership of the heap allocation.
// The structure now holds its pointer inside an Option so that the memory is deallocated only once.
// Any attempt to drop the pointer a second time is prevented by taking the ownership in the first drop.
//////////////////////////////////////////////////////////////////////////////////////////////////////

struct SafeData {
    ptr: Option<*mut i32>,
}

impl SafeData {
    fn new(val: i32) -> Self {
        let boxed = Box::new(val);
        SafeData { ptr: Some(Box::into_raw(boxed)) }
    }

    // Provides safe access to the data without compromising ownership.
    fn get(&self) -> i32 {
        unsafe { *self.ptr.expect("Pointer should be valid") }
    }
}

impl Drop for SafeData {
    fn drop(&mut self) {
        // Ensure that the pointer is dropped only once.
        if let Some(p) = self.ptr.take() {
            unsafe {
                let _ = Box::from_raw(p);
            }
        }
    }
}

// The core function for the corrected version.
fn run_app() -> i32 {
    // Create a unique owner of the heap allocation.
    let instance = SafeData::new(100);
    // Use the reference without duplicating the ownership.
    instance.get()
}

fn main() {
    // Execute the corrected application logic.
    let _ = run_app();
}