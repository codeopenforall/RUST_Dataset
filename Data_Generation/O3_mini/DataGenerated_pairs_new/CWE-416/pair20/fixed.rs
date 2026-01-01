////////////////////////////////////////////
// Fixed Code Sample (Eliminating Use-After-Free)
////////////////////////////////////////////

use std::sync::Arc;
use std::thread;

/// A heap-allocated object containing sensitive data.
struct Object {
    data: i32,
}

impl Object {
    fn new(val: i32) -> Self {
        Object { data: val }
    }
}

/// A handler that safely shares ownership of the Object via Arc.
/// This prevents premature deallocation while the handler is still in use.
struct Handler {
    handle: Arc<Object>,
}

impl Handler {
    fn new(handle: Arc<Object>) -> Self {
        Handler { handle }
    }
    /// Safely fetches the data from the object.
    fn fetch(&self) -> i32 {
        self.handle.data
    }
}

/// Executes the corrected routine.
/// Instead of using raw pointers and manual deallocation,
/// the object is owned by an Arc, and both the caller and the Handler share it.
/// This ensures that the Object is only deallocated when all references are dropped,
/// preventing the Use-After-Free vulnerability.
pub fn run() -> i32 {
    // Allocate an Object with shared ownership.
    let obj = Arc::new(Object::new(123));
    // Create a handler that clones the Arc,
    // ensuring the Object remains valid even if the original handle is dropped.
    let handler = Handler::new(Arc::clone(&obj));
    // Optionally drop the original Arc. The object remains alive through the handler.
    drop(obj);
    // Safely fetch the data.
    handler.fetch()
}

fn main() {
    // Spawn a thread to simulate concurrent use.
    let handle = thread::spawn(|| {
        let val = run();
        println!("Final value: {}", val);
    });
    handle.join().unwrap();
}