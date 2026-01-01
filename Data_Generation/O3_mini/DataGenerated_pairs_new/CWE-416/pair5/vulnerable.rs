use std::thread;
use std::time::Duration;

struct Data {
    number: i32,
}

/// Creates a raw pointer to a field in a heap‐allocated object, then
/// frees the allocation. Later, a dummy allocation is performed to provoke
/// memory re‐use, so dereferencing the pointer causes a use‐after‐free.
fn acquire_ptr() -> *const i32 {
    let boxed = Box::new(Data { number: 1337 });
    let raw = &boxed.number as *const i32;
    drop(boxed); // Vulnerability: memory is freed while raw points into it.
    // Force a reallocation that may (deterministically for our test) reuse freed memory.
    let _dummy = Box::new(Data { number: 4242 });
    raw
}

/// Unsafely computes a value by dereferencing a raw pointer that may be invalid.
fn unsafe_compute(ptr: *const i32) -> i32 {
    unsafe {
        // Use‐after‐free: reading from memory that has been freed.
        *ptr
    }
}

/// Public interface that produces a result based on unsafe extraction of a value.
/// For the vulnerable version, this returns an unpredictable value.
pub fn get_result() -> i32 {
    let ptr = acquire_ptr();
    unsafe_compute(ptr)
}

fn main() {
    let result = get_result();
    println!("Computed result: {}", result);
}