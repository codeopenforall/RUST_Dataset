////////////////////////////////////////////
// Vulnerable Code Sample (Use-After-Free)
////////////////////////////////////////////

use std::thread;
use std::ptr;

/// A heap-allocated object containing sensitive data.
struct Object {
    data: i32,
}

impl Object {
    fn new(val: i32) -> Self {
        Object { data: val }
    }
}

/// A handler that stores a raw pointer to an Object.
/// It provides operations to read the data.
struct Handler {
    ptr: *const Object,
}

impl Handler {
    fn new(raw: *const Object) -> Self {
        Handler { ptr: raw }
    }
    /// Unsafely fetches the data from the object.
    /// If the object has been freed already, this creates a Use-After-Free.
    fn fetch(&self) -> i32 {
        unsafe { (*self.ptr).data }
    }
}

/// Executes the core routine.
/// This routine creates an Object on the heap, obtains its raw pointer,
/// frees the allocated memory, and then uses the pointer via a Handler.
/// This is a classic Use-After-Free vulnerability.
pub fn run() -> i32 {
    // Allocate an Object on the heap.
    let obj = Box::new(Object::new(123));
    // Get a raw pointer to the object.
    let raw_ptr = Box::into_raw(obj);
    // Create a handler which holds the raw pointer.
    let handler = Handler::new(raw_ptr);
    // Free the allocated object by converting the raw pointer back into a Box.
    // After this, the memory pointed to by raw_ptr is deallocated.
    unsafe {
        Box::from_raw(raw_ptr);
    }
    // Use the handler to access the data.
    // This is a use-after-free: the handler dereferences a dangling pointer.
    handler.fetch()
}

fn main() {
    // Spawn a thread to simulate concurrent use.
    let handle = thread::spawn(|| {
        // The invocation of run() here may trigger undefined behavior.
        let val = run();
        println!("Final value: {}", val);
    });
    handle.join().unwrap();
}