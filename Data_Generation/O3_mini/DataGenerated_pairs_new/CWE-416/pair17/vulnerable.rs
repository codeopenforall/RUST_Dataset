/////////////////////////////////////////////////////////////////
// Vulnerable Code Sample demonstrating a use‐after‐free flaw 
// in a concurrent-like context using unsafe raw pointers.
/////////////////////////////////////////////////////////////////

use std::ptr;

struct Data {
    value: i32,
}

impl Data {
    fn new(val: i32) -> Self {
        Data { value: val }
    }
}

// The calculate function intentionally mismanages memory.
// It allocates a Data on the heap, immediately frees it,
// then overwrites the freed memory and uses it afterward.
fn calculate() -> i32 {
    unsafe {
        // Allocate and obtain a raw pointer.
        let ptr = Box::into_raw(Box::new(Data::new(42)));
        // Immediately free the allocated memory.
        // Vulnerability: Memory is freed here.
        Box::from_raw(ptr);
        // Simulate memory reuse: overwrite the freed memory.
        ptr::write(ptr, Data::new(999));
        // Use-after-free: Accessing the memory after it has been freed.
        (*ptr).value
    }
}

fn main() {
    let result = calculate();
    println!("Result: {}", result);
}