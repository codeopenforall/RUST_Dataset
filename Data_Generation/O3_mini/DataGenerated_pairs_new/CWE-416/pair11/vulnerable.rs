use std::ptr;

struct Resource {
    ptr: *mut i32,
}

impl Resource {
    // This method unsafely dereferences the pointer.
    // Caller must guarantee that ptr is valid.
    unsafe fn obtain(&self) -> i32 {
        // Vulnerability: use-after-free occurs here when reading from a freed pointer.
        *self.ptr
    }
}

fn allocate() -> Resource {
    // Allocate an integer on the heap.
    let boxed = Box::new(42);
    // Convert to a raw pointer.
    let raw = Box::into_raw(boxed);
    // Erroneously reconstitute and drop the Box,
    // freeing the allocated memory.
    unsafe { Box::from_raw(raw) };
    // Return a Resource holding a dangling pointer.
    Resource { ptr: raw }
}

fn execute() {
    let resource = allocate();
    // Unsafe block: Using a pointer that has been freed.
    unsafe {
        let val = resource.obtain();
        // The use-after-free may yield an undefined value or trigger a crash.
        println!("Value: {}", val);
    }
}

fn main() {
    execute();
}