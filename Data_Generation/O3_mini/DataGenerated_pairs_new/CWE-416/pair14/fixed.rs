/*
   This corrected code preserves safe usage: the resource is used via a raw pointer
   before it is deallocated. The order of operations ensures that the pointer is valid
   when dereferenced, thereby eliminating the use‐after‐free issue.
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
        // Allocate the resource.
        let resource = Box::new(Resource::new(42));
        // Convert the Box into a raw pointer.
        let raw_ptr: *mut Resource = Box::into_raw(resource);
        // Dereference the raw pointer while the memory is still valid.
        let result = (*raw_ptr).value;
        // Now, safely free the resource after use.
        let _ = Box::from_raw(raw_ptr);
        result
    }
}

fn main() {
    let result = process();
    println!("Result: {}", result);
}