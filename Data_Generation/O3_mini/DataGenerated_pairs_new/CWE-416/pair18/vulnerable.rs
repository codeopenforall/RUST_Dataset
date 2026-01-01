/////////////////////// Vulnerable Code ///////////////////////
use std::thread;
use std::time::Duration;

struct Data {
    value: i32,
}

impl Data {
    fn new(val: i32) -> Self {
        Data { value: val }
    }
}

fn compute() -> i32 {
    // Allocate a Data instance on the heap.
    let boxed = Box::new(Data::new(42));
    // Obtain a raw pointer from the Box.
    let raw_ptr = Box::into_raw(boxed);

    unsafe {
        // Manually free the memory.
        // This deallocates the Data instance.
        Box::from_raw(raw_ptr);
        // Force an allocation which may reuse the just-freed memory.
        // This is done to increase the chance that the memory at raw_ptr is overwritten.
        let _dummy = Box::new(Data::new(100));
        // USE-AFTER-FREE: Dereference the raw pointer after it has been freed.
        // This leads to undefined behavior.
        (*raw_ptr).value
    }
}

fn main() {
    // Spawn a thread to simulate concurrent execution.
    let handle = thread::spawn(|| {
        // Sleep briefly to mimic concurrency effects.
        thread::sleep(Duration::from_millis(10));
        compute()
    });
    let result = handle.join().expect("Thread panicked");
    println!("Computed result: {}", result);
}