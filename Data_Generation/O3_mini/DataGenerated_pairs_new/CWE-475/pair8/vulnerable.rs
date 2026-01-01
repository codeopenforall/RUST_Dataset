/* 
This Rust program demonstrates a scenario where an unsafe API precondition is violated by
using an incorrect length when constructing a slice from a raw pointer. The function ‘execute’
incorrectly computes an “extended” length (data.len() + 2) and uses it in an unsafe block. This
mistake may cause undefined behavior by reading memory beyond the allocated range. The unsafe
conversion is executed within a spawned thread to mimic realistic concurrent usage patterns.
*/
use std::thread;

fn execute() -> Vec<u8> {
    let data = vec![1u8, 2, 3, 4];
    // Compute an invalid length by adding extra bytes that were never allocated.
    let extended = data.len() + 2;
    let ptr = data.as_ptr();
    let handle = thread::spawn(move || {
        // Unsafe: creating a slice with a length that exceeds the allocated buffer.
        unsafe {
            std::slice::from_raw_parts(ptr, extended).to_vec()
        }
    });
    handle.join().unwrap()
}

fn main() {
    let result = execute();
    println!("Result: {:?}", result);
}