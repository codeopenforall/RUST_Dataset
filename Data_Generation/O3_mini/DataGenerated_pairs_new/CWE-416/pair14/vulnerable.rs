/* 
   Note: This code compiles but contains a use‐after‐free vulnerability.
   It creates a resource, converts it to a raw pointer, frees the resource,
   then later dereferences the freed pointer.
*/
use std::boxed::Box;

struct Resource {
    value: i32,
}

impl Resource {
    fn new(v: i32) -> Self {
        Resource { value: v }
    }
}

fn process() -> i32 {
    unsafe {
        // Line 8: Allocate a resource on the heap.
        let resource = Box::new(Resource::new(42));
        // Line 9: Convert the Box into a raw pointer.
        let raw_ptr: *mut Resource = Box::into_raw(resource);
        // Line 10: Reconstruct the Box to drop (free) the resource.
        let _ = Box::from_raw(raw_ptr); // Resource is now freed.
        // Line 11: Dereference the freed pointer (use-after-free).
        (*raw_ptr).value
    }
}

fn main() {
    let result = process();
    println!("Result: {}", result);
}