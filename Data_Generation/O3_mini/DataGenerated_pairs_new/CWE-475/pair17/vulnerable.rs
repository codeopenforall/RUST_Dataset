#![allow(unused)]
use std::slice;

struct Container {
    ptr: *const u32,
    len: usize,
}

impl Container {
    fn new(input: &[u32]) -> Self {
        Container {
            ptr: input.as_ptr(),
            len: input.len(),
        }
    }

    // This method incorrectly constructs a slice with one extra element,
    // violating the precondition that the pointer must be valid for the given length.
    fn aggregate(&self) -> u32 {
        unsafe {
            // Vulnerability: using self.len + 1 causes an out‐of‐bounds read.
            let data = slice::from_raw_parts(self.ptr, self.len + 1);
            data.iter().fold(0, |acc, &val| acc + val)
        }
    }
}

fn main() {
    // Create a boxed array of 5 u32 values.
    // We intentionally use only the first 4 elements as the valid data.
    let boxed = Box::new([1, 2, 3, 4, 42]);
    let valid_slice = &boxed[..4];
    let handler = Container::new(valid_slice);
    let result = handler.aggregate();
    println!("Total: {}", result);
    // The valid sum from the first four elements should be 10.
    // If the extra element (42) is added, the sum becomes 52.
    // We treat the outcome 52 as a sign of vulnerability.
    if result == 10 {
        println!("Error: Unexpected valid sum");
        std::process::exit(1);
    }
}