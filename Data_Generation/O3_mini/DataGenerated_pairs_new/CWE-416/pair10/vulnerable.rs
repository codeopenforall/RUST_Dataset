use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Record {
    value: i32,
}

impl Record {
    fn new(val: i32) -> Self {
        Record { value: val }
    }
}

// This function intentionally uses unsafe code to simulate a use‐after‐free bug.
// It allocates a Record on the heap, converts it to a raw pointer, then frees the memory.
// Afterwards, it dereferences the dangling pointer. Concurrent re‐allocation is used
// to increase the chance that the freed memory is re-used.
unsafe fn compute() -> i32 {
    // Allocate memory for a Record with value 42.
    let boxed = Box::new(Record::new(42));
    // Obtain a raw pointer to the Record.
    let ptr = Box::into_raw(boxed);

    // Spawn a thread that reallocates memory. This may cause the allocator to re-use
    // the freed memory if the timing aligns.
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(10));
        // Re-allocate memory; if the allocator returns the same address, it will corrupt
        // the data pointed to by the dangling pointer.
        let _temp = Box::new(Record::new(100));
        // _temp is dropped at the end of this closure.
    });

    // Explicitly free the original memory by reconstructing the Box.
    // At this point, ptr becomes a dangling pointer.
    unsafe {
        Box::from_raw(ptr);
    }

    // Wait for the auxiliary thread to complete.
    handle.join().unwrap();

    // Use-After-Free: Dereference the pointer after the memory has been freed.
    // This is undefined behavior and may produce an incorrect value.
    unsafe { (*ptr).value }
}

fn main() {
    // In main, we call compute() unsafely.
    let result = unsafe { compute() };
    println!("Result: {}", result);
}